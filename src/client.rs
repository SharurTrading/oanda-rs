//! Client construction and shared HTTP execution policy.

use crate::{
    AccountID, ClientRequestID, Environment, Error, GenericRejection, OperationError, Result,
    Supplied, models::AcceptDatetimeFormat,
};
use futures_util::StreamExt;
use reqwest::{Method, StatusCode, Url};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::HashSet,
    fmt,
    ops::Deref,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const DEFAULT_MAX_BODY: usize = 8 * 1024 * 1024;
const REQUESTS_PER_SECOND: u32 = 100;
const DEFAULT_COOLDOWN: Duration = Duration::from_secs(1);
const MAX_COOLDOWN: Duration = Duration::from_mins(1);

/// A typed OANDA response with the provider's request and pagination metadata.
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    /// Deserialized response body.
    pub body: T,
    /// OANDA `RequestID` response header, when supplied.
    pub request_id: Option<String>,
    /// Validated next-page URL from the Link header, when supplied.
    pub next_page: Option<Url>,
}

impl<T> Deref for ApiResponse<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

/// Shareable OANDA v20 client.
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("environment", &self.inner.environment)
            .finish_non_exhaustive()
    }
}

pub(crate) struct Inner {
    pub environment: Environment,
    pub rest: Url,
    pub stream: Url,
    pub http: reqwest::Client,
    pub stream_http: reqwest::Client,
    pub token: String,
    pub max_body: usize,
    pub datetime_format: AcceptDatetimeFormat,
    rate: Mutex<Rate>,
    fenced: Mutex<HashSet<String>>,
    in_flight: Mutex<HashSet<String>>,
    pub stream_slots: Mutex<StreamSlots>,
}

struct Rate {
    start: Instant,
    used: u32,
    cooldown_until: Option<Instant>,
}
pub(crate) struct StreamSlots {
    pub active: usize,
    pub last_start: Option<Instant>,
}

/// Validated client configuration.
pub struct ClientBuilder {
    environment: Environment,
    token: String,
    rest_override: Option<Url>,
    stream_override: Option<Url>,
    max_body: usize,
    timeout: Duration,
    datetime_format: AcceptDatetimeFormat,
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("environment", &self.environment)
            .field("max_body", &self.max_body)
            .finish_non_exhaustive()
    }
}

impl Client {
    /// Start building a client with an injected personal access token.
    pub fn builder(environment: Environment, bearer_token: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            environment,
            token: bearer_token.into(),
            rest_override: None,
            stream_override: None,
            max_body: DEFAULT_MAX_BODY,
            timeout: Duration::from_secs(30),
            datetime_format: AcceptDatetimeFormat::Rfc3339,
        }
    }

    /// Allow mutations after the caller has reconciled this account against OANDA state.
    ///
    /// This is an acknowledgement by the caller, not a state query performed by the client.
    ///
    /// # Errors
    ///
    /// Returns an error when the shared fence set cannot be locked. The account then stays
    /// fenced, so its remaining mutations are refused until a client that can lock succeeds.
    pub fn acknowledge_reconciliation(&self, account: &AccountID) -> Result<()> {
        let mut fenced = self
            .inner
            .fenced
            .lock()
            .map_err(|_| Error::InvalidInput("reconciliation lock poisoned".into()))?;
        fenced.remove(account.as_str());
        Ok(())
    }

    // Transport success, rejection, and ambiguity handling stay together for auditability.
    #[allow(clippy::too_many_lines)]
    pub(crate) async fn execute<
        T: DeserializeOwned,
        R: DeserializeOwned,
        Q: Serialize,
        B: Serialize,
    >(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        mutation_account: Option<&AccountID>,
        client_request_id: Option<&ClientRequestID>,
    ) -> std::result::Result<ApiResponse<T>, OperationError<R>> {
        let query_value = query
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        let body_value = body
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        crate::validation::validate(path, query_value.as_ref(), body_value.as_ref())?;
        self.admit(mutation_account.is_some()).await?;
        let url = Self::build_url(&self.inner.rest, path, query)?;
        let mut request = self
            .inner
            .http
            .request(method, url)
            .bearer_auth(&self.inner.token)
            .header(
                "Accept-Datetime-Format",
                self.inner.datetime_format.as_str(),
            );
        if let Some(id) = client_request_id {
            request = request.header("ClientRequestID", id.as_str());
        }
        if let Some(body) = body {
            request = request.json(body);
        }
        // Reserve the account before the first network await. This prevents two
        // concurrent clones from sending mutations whose outcomes cannot be ordered.
        let mut guard = mutation_account
            .map(|account| MutationGuard::new(self.inner.clone(), account))
            .transpose()?;
        let response = request.send().await.map_err(|e| match mutation_account {
            Some(account) => Error::AmbiguousMutation {
                account: account.to_string(),
            },
            None => Error::Transport(e.to_string()),
        })?;
        let status = response.status();
        let request_id = response
            .headers()
            .get("RequestID")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let next_page = if status.is_success() {
            parse_next_page(&self.inner.rest, response.headers()).map_err(|error| {
                match mutation_account {
                    Some(account) => Error::AmbiguousMutation {
                        account: account.to_string(),
                    },
                    None => error,
                }
            })?
        } else {
            None
        };
        if status == StatusCode::TOO_MANY_REQUESTS {
            self.install_cooldown(response.headers().get(reqwest::header::RETRY_AFTER));
            if let Some(ref mut guard) = guard {
                guard.disarm();
            }
            return Err(Error::ProviderRateLimited.into());
        }
        let bytes = read_bounded(response, self.inner.max_body)
            .await
            .map_err(|e| match mutation_account {
                Some(account) => Error::AmbiguousMutation {
                    account: account.to_string(),
                },
                None => e,
            })?;
        if status.is_success() {
            let value = serde_json::from_slice(&bytes).map_err(|e| match mutation_account {
                Some(account) => Error::AmbiguousMutation {
                    account: account.to_string(),
                },
                None => Error::Decode(e.to_string()),
            })?;
            if let Some(ref mut guard) = guard {
                guard.disarm();
            }
            return Ok(ApiResponse {
                body: value,
                request_id,
                next_page,
            });
        }
        if mutation_account.is_some()
            && (status.is_server_error()
                || status == StatusCode::REQUEST_TIMEOUT
                || !matches!(status.as_u16(), 400 | 401 | 403 | 404 | 405))
        {
            return Err(Error::AmbiguousMutation {
                account: mutation_account.map_or(String::new(), ToString::to_string),
            }
            .into());
        }
        let body: Supplied<R> = if bytes.iter().all(u8::is_ascii_whitespace) {
            Supplied::Absent
        } else {
            match serde_json::from_slice::<R>(&bytes) {
                Ok(value) => Supplied::Decoded(value),
                Err(_) => Supplied::Undecoded,
            }
        };
        // `errorCode` and `errorMessage` are read independently of the endpoint's
        // documented rejection schema so a schema mismatch cannot erase a reason
        // OANDA did send. A body this client cannot read yields no reason, and
        // `body` above is what tells the caller one arrived regardless.
        let reason: ProviderError = match serde_json::from_slice::<ProviderError>(&bytes) {
            Ok(reason) => reason,
            Err(_) => ProviderError::UNREAD,
        };
        if let Some(ref mut guard) = guard {
            guard.disarm();
        }
        Err(OperationError::Rejected {
            status: status.as_u16(),
            code: reason.error_code,
            message: reason.error_message,
            body,
        })
    }

    /// Follow a provider pagination link on this client's REST origin.
    ///
    /// The caller supplies the expected typed body for the originating list operation.
    ///
    /// # Errors
    ///
    /// Returns an error for an off-origin link, transport failure, provider rejection, or invalid response.
    pub async fn fetch_page<T: DeserializeOwned>(
        &self,
        next_page: &Url,
    ) -> std::result::Result<ApiResponse<T>, OperationError<GenericRejection>> {
        if next_page.scheme() != self.inner.rest.scheme()
            || next_page.host_str() != self.inner.rest.host_str()
            || next_page.port_or_known_default() != self.inner.rest.port_or_known_default()
            || !next_page.path().starts_with("/v3/")
            || next_page.fragment().is_some()
            || next_page.username() != ""
            || next_page.password().is_some()
        {
            return Err(Error::InvalidInput(
                "pagination URL is outside the OANDA REST origin".into(),
            )
            .into());
        }
        let path = if let Some(query) = next_page.query() {
            format!("{}?{query}", next_page.path())
        } else {
            next_page.path().to_owned()
        };
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }

    pub(crate) async fn admit(&self, mutation: bool) -> Result<()> {
        loop {
            let wait = {
                let mut rate = self
                    .inner
                    .rate
                    .lock()
                    .map_err(|_| Error::InvalidInput("rate lock poisoned".into()))?;
                let now = Instant::now();
                if now.duration_since(rate.start) >= Duration::from_secs(1) {
                    rate.start = now;
                    rate.used = 0;
                }
                if let Some(until) = rate.cooldown_until.filter(|until| *until > now) {
                    until.duration_since(now)
                } else if rate.used < REQUESTS_PER_SECOND {
                    rate.used += 1;
                    return Ok(());
                } else {
                    Duration::from_secs(1).saturating_sub(now.duration_since(rate.start))
                }
            };
            if mutation {
                return Err(Error::RateLimited);
            }
            tokio::time::sleep(wait).await;
        }
    }

    pub(crate) fn build_url<Q: Serialize>(
        base: &Url,
        path: &str,
        query: Option<&Q>,
    ) -> Result<Url> {
        if !path.starts_with("/v3/") {
            return Err(Error::InvalidInput("path must start with /v3/".into()));
        }
        let mut url = base
            .join(path)
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        if let Some(query) = query {
            let value =
                serde_json::to_value(query).map_err(|e| Error::InvalidInput(e.to_string()))?;
            let object = value
                .as_object()
                .ok_or_else(|| Error::InvalidInput("query must be an object".into()))?;
            let mut pairs = url.query_pairs_mut();
            for (key, value) in object {
                if value.is_null() {
                    continue;
                }
                if let Some(items) = value.as_array() {
                    let csv = items
                        .iter()
                        .map(query_atom)
                        .collect::<Result<Vec<_>>>()?
                        .join(",");
                    pairs.append_pair(key, &csv);
                } else {
                    pairs.append_pair(key, &query_atom(value)?);
                }
            }
        }
        Ok(url)
    }

    /// Install the cooldown OANDA's `Retry-After` asks for, shared by every clone.
    ///
    /// A header that is absent is a provider that gave no instruction, so the
    /// documented local default applies. A header that is present but unreadable
    /// is an instruction this client failed to read: it takes the maximum local
    /// cooldown, which is local policy and is never attributed to OANDA.
    fn install_cooldown(&self, retry_after: Option<&reqwest::header::HeaderValue>) {
        let cooldown = match retry_after {
            None => DEFAULT_COOLDOWN,
            Some(value) => parse_retry_after(value).unwrap_or(MAX_COOLDOWN),
        };
        // The critical section touches two plain fields, so a poisoned lock still
        // holds a readable rate window. Skipping the install would report a
        // rate-limited account as one the client may keep calling.
        let mut rate = self
            .inner
            .rate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        rate.cooldown_until = Some(Instant::now() + cooldown);
    }
}

/// Read an RFC 9110 `Retry-After` value: delta-seconds or an HTTP-date.
///
/// A date already in the past asks for no wait at all, which is a stated
/// instruction rather than a missing one.
fn parse_retry_after(value: &reqwest::header::HeaderValue) -> Option<Duration> {
    let text = value.to_str().ok()?.trim();
    if let Ok(seconds) = text.parse::<u64>() {
        return Some(Duration::from_secs(seconds).min(MAX_COOLDOWN));
    }
    let when = chrono::DateTime::parse_from_rfc2822(text).ok()?;
    let seconds = u64::try_from(
        (when.with_timezone(&chrono::Utc) - chrono::Utc::now())
            .num_seconds()
            .max(0),
    )
    .ok()?;
    Some(Duration::from_secs(seconds).min(MAX_COOLDOWN))
}

impl ClientBuilder {
    /// Set the maximum non-stream response body size in bytes.
    #[must_use]
    pub fn max_response_bytes(mut self, value: usize) -> Self {
        self.max_body = value;
        self
    }
    /// Set the REST request timeout.
    #[must_use]
    pub fn timeout(mut self, value: Duration) -> Self {
        self.timeout = value;
        self
    }
    /// Select the documented OANDA timestamp wire format for responses.
    #[must_use]
    pub fn datetime_format(mut self, value: AcceptDatetimeFormat) -> Self {
        self.datetime_format = value;
        self
    }
    /// Override both hosts for deterministic loopback fixtures.
    /// Remote endpoints must remain the selected OANDA environment pair.
    #[must_use]
    pub fn endpoints(mut self, rest: Url, stream: Url) -> Self {
        self.rest_override = Some(rest);
        self.stream_override = Some(stream);
        self
    }

    /// Construct a client without making a network request.
    ///
    /// # Errors
    ///
    /// Returns an error if credentials, limits, or endpoint overrides are invalid, or if the
    /// underlying HTTP clients cannot be constructed.
    pub fn build(self) -> Result<Client> {
        if self.token.trim().is_empty() || self.token.chars().any(char::is_control) {
            return Err(Error::InvalidInput("invalid bearer token".into()));
        }
        if self.max_body == 0 || self.timeout.is_zero() {
            return Err(Error::InvalidInput("limits must be positive".into()));
        }
        let (default_rest, default_stream) = self.environment.urls()?;
        let rest = self.rest_override.unwrap_or_else(|| default_rest.clone());
        let stream = self
            .stream_override
            .unwrap_or_else(|| default_stream.clone());
        validate_url(&rest)?;
        validate_url(&stream)?;
        let paired_loopback = is_loopback(&rest) && is_loopback(&stream);
        if !paired_loopback && (rest != default_rest || stream != default_stream) {
            return Err(Error::InvalidInput(
                "endpoints must use the selected OANDA environment pair or loopback fixtures"
                    .into(),
            ));
        }
        if matches!(self.datetime_format, AcceptDatetimeFormat::Unknown(_)) {
            return Err(Error::InvalidInput("unsupported datetime format".into()));
        }
        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .no_proxy()
            .build()
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        let stream_http = reqwest::Client::builder()
            .connect_timeout(self.timeout)
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .no_proxy()
            .build()
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        Ok(Client {
            inner: Arc::new(Inner {
                environment: self.environment,
                rest,
                stream,
                http,
                stream_http,
                token: self.token,
                max_body: self.max_body,
                datetime_format: self.datetime_format,
                rate: Mutex::new(Rate {
                    start: Instant::now(),
                    used: 0,
                    cooldown_until: None,
                }),
                fenced: Mutex::new(HashSet::new()),
                in_flight: Mutex::new(HashSet::new()),
                stream_slots: Mutex::new(StreamSlots {
                    active: 0,
                    last_start: None,
                }),
            }),
        })
    }
}

fn validate_url(url: &Url) -> Result<()> {
    let host = url
        .host_str()
        .ok_or_else(|| Error::InvalidInput("endpoint must have a host".into()))?;
    let loopback = host == "localhost" || host == "127.0.0.1" || host == "::1";
    if url.scheme() != "https" && !(loopback && url.scheme() == "http") {
        return Err(Error::InvalidInput("remote endpoints require HTTPS".into()));
    }
    if url.path() != "/"
        || url.username() != ""
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(Error::InvalidInput(
            "endpoint URL must not contain credentials, query, or fragment".into(),
        ));
    }
    Ok(())
}

fn is_loopback(url: &Url) -> bool {
    matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "::1"))
}

fn query_atom(value: &serde_json::Value) -> Result<String> {
    match value {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Bool(b) => Ok(b.to_string()),
        _ => Err(Error::InvalidInput("unsupported query value".into())),
    }
}

fn parse_next_page(base: &Url, headers: &reqwest::header::HeaderMap) -> Result<Option<Url>> {
    let Some(link) = headers.get(reqwest::header::LINK) else {
        return Ok(None);
    };
    let value = link
        .to_str()
        .map_err(|_| Error::Decode("invalid Link header".into()))?;
    for item in value.split(',') {
        if !item.contains("rel=\"next\"") && !item.contains("rel=next") {
            continue;
        }
        let Some(start) = item.find('<') else {
            return Err(Error::Decode("invalid pagination link".into()));
        };
        let Some(end) = item[start + 1..].find('>') else {
            return Err(Error::Decode("invalid pagination link".into()));
        };
        let url = base
            .join(&item[start + 1..start + 1 + end])
            .map_err(|e| Error::Decode(e.to_string()))?;
        if url.scheme() != base.scheme()
            || url.host_str() != base.host_str()
            || url.port_or_known_default() != base.port_or_known_default()
            || !url.path().starts_with("/v3/")
        {
            return Err(Error::Decode(
                "pagination link changed origin or API version".into(),
            ));
        }
        return Ok(Some(url));
    }
    Ok(None)
}

pub(crate) fn segment(value: &str) -> String {
    const PATH_SEGMENT: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    percent_encoding::utf8_percent_encode(value, PATH_SEGMENT).to_string()
}

pub(crate) async fn read_bounded(response: reqwest::Response, max: usize) -> Result<Vec<u8>> {
    let mut stream = response.bytes_stream();
    let mut bytes = Vec::new();
    while let Some(next) = stream.next().await {
        let chunk = next.map_err(|e| Error::Transport(e.to_string()))?;
        if bytes.len().saturating_add(chunk.len()) > max {
            return Err(Error::ResponseTooLarge);
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderError {
    error_code: Option<String>,
    error_message: Option<String>,
}

impl ProviderError {
    /// No reason this client could read. It is distinct from a reason OANDA read
    /// and omitted, and only the rejection body's own state separates the two
    /// for a caller.
    const UNREAD: Self = Self {
        error_code: None,
        error_message: None,
    };
}

struct MutationGuard {
    inner: Arc<Inner>,
    account: String,
    armed: bool,
}
impl MutationGuard {
    fn new(inner: Arc<Inner>, account: &AccountID) -> Result<Self> {
        if inner
            .fenced
            .lock()
            .map_err(|_| Error::InvalidInput("reconciliation lock poisoned".into()))?
            .contains(account.as_str())
        {
            return Err(Error::ReconciliationRequired {
                account: account.to_string(),
            });
        }
        let mut in_flight = inner
            .in_flight
            .lock()
            .map_err(|_| Error::InvalidInput("mutation lock poisoned".into()))?;
        if !in_flight.insert(account.to_string()) {
            return Err(Error::MutationInFlight {
                account: account.to_string(),
            });
        }
        // The previous guard may have fenced the account between the first
        // check and this reservation. Recheck while holding the reservation.
        if inner
            .fenced
            .lock()
            .map_err(|_| Error::InvalidInput("reconciliation lock poisoned".into()))?
            .contains(account.as_str())
        {
            in_flight.remove(account.as_str());
            return Err(Error::ReconciliationRequired {
                account: account.to_string(),
            });
        }
        drop(in_flight);
        Ok(Self {
            inner,
            account: account.to_string(),
            armed: true,
        })
    }
    fn disarm(&mut self) {
        self.armed = false;
    }
}
impl Drop for MutationGuard {
    fn drop(&mut self) {
        // A dropped guard cannot report a failure, so it must not skip the fence:
        // the ambiguity it stands for is exactly what a later mutation must not
        // act on. The critical sections hold plain `HashSet` values, so a
        // poisoned lock still carries the readable account set.
        if self.armed {
            let mut fenced = self
                .inner
                .fenced
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            fenced.insert(self.account.clone());
        }
        let mut in_flight = self
            .inner
            .in_flight
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        in_flight.remove(&self.account);
    }
}

#[cfg(test)]
// Poisoning a lock requires a panic, and fixture failures are clearer as panics;
// production paths return typed errors.
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mutation_rate_admission_is_shared_by_clones() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        if let Ok(mut rate) = client.inner.rate.lock() {
            rate.used = REQUESTS_PER_SECOND;
            rate.start = Instant::now();
        }
        assert!(matches!(
            client.clone().admit(true).await,
            Err(Error::RateLimited)
        ));
    }

    #[test]
    fn account_reservation_and_reconciliation_are_shared() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        let account = AccountID::new("101-001-1-001");
        assert!(account.is_ok());
        let Ok(account) = account else { return };
        let first = MutationGuard::new(client.inner.clone(), &account);
        assert!(first.is_ok());
        let Ok(first) = first else { return };
        assert!(matches!(
            MutationGuard::new(client.clone().inner, &account),
            Err(Error::MutationInFlight { .. })
        ));
        drop(first);
        assert!(matches!(
            MutationGuard::new(client.inner.clone(), &account),
            Err(Error::ReconciliationRequired { .. })
        ));
        let other = AccountID::new("101-001-1-002");
        assert!(other.is_ok());
        if let Ok(other) = other {
            let mut independent = MutationGuard::new(client.inner.clone(), &other);
            assert!(independent.is_ok());
            if let Ok(ref mut guard) = independent {
                guard.disarm();
            }
        }
        client
            .acknowledge_reconciliation(&account)
            .expect("fence released");
        let mut next = MutationGuard::new(client.inner.clone(), &account);
        assert!(next.is_ok());
        if let Ok(ref mut guard) = next {
            guard.disarm();
        }
    }

    fn poison<T>(lock: &Mutex<T>) {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Ok(_held) = lock.lock() else {
                return;
            };
            panic!("poison the lock");
        }));
        assert!(result.is_err());
        assert!(lock.is_poisoned());
    }

    #[test]
    fn an_ambiguous_mutation_is_fenced_even_after_a_poisoned_lock() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        let account = AccountID::new("101-001-1-001");
        assert!(account.is_ok());
        let Ok(account) = account else { return };
        let guard = MutationGuard::new(client.inner.clone(), &account);
        assert!(guard.is_ok());
        let Ok(guard) = guard else { return };
        // The locks fail after the guard is armed, which is the only ordering in
        // which a dropped guard owes the caller a fence.
        poison(&client.inner.fenced);
        poison(&client.inner.in_flight);
        drop(guard);
        let fenced = client
            .inner
            .fenced
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(
            fenced.contains(account.as_str()),
            "the ambiguity must be fenced, not dropped"
        );
        let in_flight = client
            .inner
            .in_flight
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(!in_flight.contains(account.as_str()));
    }

    #[test]
    fn a_failed_acknowledgement_is_reported_and_leaves_the_account_fenced() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        let account = AccountID::new("101-001-1-001");
        assert!(account.is_ok());
        let Ok(account) = account else { return };
        {
            let mut fenced = client
                .inner
                .fenced
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            fenced.insert(account.to_string());
        }
        poison(&client.inner.fenced);
        assert!(matches!(
            client.acknowledge_reconciliation(&account),
            Err(Error::InvalidInput(_))
        ));
        let fenced = client
            .inner
            .fenced
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(fenced.contains(account.as_str()));
    }

    fn assert_cooldown(header: Option<&str>, expected: Duration) {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        poison(&client.inner.rate);
        let header =
            header.map(|text| reqwest::header::HeaderValue::from_str(text).expect("header"));
        let start = Instant::now();
        client.install_cooldown(header.as_ref());
        let rate = client
            .inner
            .rate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let until = rate.cooldown_until.expect("cooldown installed");
        assert!(
            until >= start + expected,
            "cooldown shorter than {expected:?}"
        );
        assert!(
            until <= Instant::now() + expected,
            "cooldown longer than {expected:?}"
        );
    }

    #[test]
    fn retry_after_reads_both_rfc_9110_forms() {
        assert_cooldown(Some("7"), Duration::from_secs(7));
        // An HTTP-date OANDA is entitled to send is read, not treated as garbage.
        let later = chrono::Utc::now() + chrono::Duration::seconds(30);
        assert_cooldown(
            Some(&later.format("%a, %d %b %Y %H:%M:%S GMT").to_string()),
            Duration::from_secs(29),
        );
        // A date already past asks for no wait at all.
        let earlier = chrono::Utc::now() - chrono::Duration::seconds(30);
        assert_cooldown(
            Some(&earlier.format("%a, %d %b %Y %H:%M:%S GMT").to_string()),
            Duration::ZERO,
        );
        assert_cooldown(Some("9999"), MAX_COOLDOWN);
    }

    #[test]
    fn an_unreadable_or_absent_retry_after_is_not_read_as_a_provider_instruction() {
        // No header: the documented local default, not a value OANDA supplied.
        assert_cooldown(None, DEFAULT_COOLDOWN);
        // A header this client cannot read: the local maximum, never a value the
        // provider did not send and never the minimum the old fallback assumed.
        assert_cooldown(Some("soon"), MAX_COOLDOWN);
    }
}
