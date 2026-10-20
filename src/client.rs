// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Client construction and shared HTTP execution policy.

use crate::validation::Validate;
use crate::{
    AccountID, Admission, ClientRequestID, Environment, Error, GenericRejection, OperationError,
    RequestID, Result, Supplied, admission::REQUESTS_PER_SECOND, models::AcceptDatetimeFormat,
};
use futures_util::StreamExt;
use reqwest::{Method, StatusCode, Url};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::HashSet,
    fmt,
    sync::{Arc, Mutex},
    time::Duration,
};
use zeroize::Zeroizing;

const DEFAULT_MAX_BODY: usize = 8 * 1024 * 1024;
const DEFAULT_COOLDOWN: Duration = Duration::from_secs(1);
const MAX_COOLDOWN: Duration = Duration::from_mins(1);

/// A typed OANDA response with the provider's request and pagination metadata.
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    /// Deserialized response body.
    pub body: T,
    /// OANDA's `RequestID` response header as supplied evidence: `Absent` when
    /// OANDA sent none, `Decoded` when this client read it, `Undecoded` when it
    /// sent one this client cannot read as text.
    pub request_id: Supplied<RequestID>,
    /// The Link header's next-page URL as supplied evidence: `Absent` when no
    /// next page was offered, `Decoded` when validated, `Undecoded` when OANDA
    /// offered one this client could not read. A readable link that leaves this
    /// client's API origin is refused with an error on reads. On a decoded mutation
    /// success, an unusable link is `Undecoded` so its failure cannot erase the accepted
    /// body or incorrectly fence the account.
    pub next_page: Supplied<Url>,
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
    pub token: Zeroizing<String>,
    pub max_body: usize,
    pub datetime_format: AcceptDatetimeFormat,
    pub(crate) admission: Admission,
    fenced: Mutex<HashSet<AccountID>>,
    in_flight: Mutex<HashSet<AccountID>>,
}

#[derive(Clone, Copy)]
pub(crate) enum MutationAdmission<'a> {
    Ordinary(&'a AccountID),
    Exit(&'a AccountID),
}

impl<'a> MutationAdmission<'a> {
    fn account(self) -> &'a AccountID {
        match self {
            Self::Ordinary(account) | Self::Exit(account) => account,
        }
    }
}

/// A prepared request with named admission and evidence parameters.
pub(crate) struct RequestParts<'a> {
    method: Method,
    path: &'a str,
    query: Option<Value>,
    body: Option<Value>,
    mutation: Option<MutationAdmission<'a>>,
    client_request_id: Option<&'a ClientRequestID>,
}

impl<'a> RequestParts<'a> {
    pub(crate) fn new(method: Method, path: &'a str) -> Self {
        Self {
            method,
            path,
            query: None,
            body: None,
            mutation: None,
            client_request_id: None,
        }
    }
    pub(crate) fn query<Q: Validate + Serialize>(mut self, query: &Q) -> Result<Self> {
        query.validate()?;
        self.query = Some(
            serde_json::to_value(query).map_err(|error| Error::InvalidInput(error.to_string()))?,
        );
        Ok(self)
    }
    pub(crate) fn body<B: Validate + Serialize>(mut self, body: &B) -> Result<Self> {
        body.validate()?;
        self.body = Some(
            serde_json::to_value(body).map_err(|error| Error::InvalidInput(error.to_string()))?,
        );
        Ok(self)
    }
    pub(crate) fn mutation(mut self, mutation: MutationAdmission<'a>) -> Self {
        self.mutation = Some(mutation);
        self
    }
    pub(crate) fn client_request_id(mut self, id: Option<&'a ClientRequestID>) -> Self {
        self.client_request_id = id;
        self
    }
}

/// Validated client configuration.
pub struct ClientBuilder {
    environment: Environment,
    token: Zeroizing<String>,
    rest_override: Option<Url>,
    stream_override: Option<Url>,
    max_body: usize,
    timeout: Duration,
    exit_reserve: Option<u32>,
    admission: Option<Admission>,
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
    /// Replace the shared request reserve for cancellation and position or trade closes.
    ///
    /// Zero releases the reserve. All clients sharing this admission owner observe the replacement;
    /// waiting reads reconsider it immediately. One caller
    /// coordinator should own this policy. Changes preserve spent capacity,
    /// provider cooldowns and already admitted requests. Exit capacity is finite.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `requests` is at least 120, or
    /// [`Error::Invariant`] when the rate lock is poisoned. Invalid changes preserve the policy and admission state.
    pub fn set_exit_reserve(&self, requests: u32) -> Result<()> {
        self.inner.admission.set_exit_reserve(requests)
    }

    /// Start building a client with an injected personal access token.
    pub fn builder(environment: Environment, bearer_token: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            environment,
            token: Zeroizing::new(bearer_token.into()),
            rest_override: None,
            stream_override: None,
            max_body: DEFAULT_MAX_BODY,
            timeout: Duration::from_secs(30),
            exit_reserve: None,
            admission: None,
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
            .map_err(|_| Error::Invariant("reconciliation lock poisoned"))?;
        fenced.remove(account);
        Ok(())
    }

    // Transport success, rejection, and ambiguity handling stay together for auditability.
    #[allow(clippy::too_many_lines)]
    pub(crate) async fn execute<T: DeserializeOwned, R: DeserializeOwned>(
        &self,
        parts: RequestParts<'_>,
    ) -> std::result::Result<ApiResponse<T>, OperationError<R>> {
        let mutation_account = parts.mutation.map(MutationAdmission::account);
        let exit = matches!(parts.mutation, Some(MutationAdmission::Exit(_)));
        let url = Self::build_url(&self.inner.rest, parts.path, parts.query.as_ref())?;
        let mut request = self
            .inner
            .http
            .request(parts.method, url)
            .bearer_auth(self.inner.token.as_str())
            .header(
                "Accept-Datetime-Format",
                self.inner.datetime_format.as_str(),
            );
        if let Some(id) = parts.client_request_id {
            request = request.header("ClientRequestID", id.as_str());
        }
        if let Some(body) = parts.body {
            request = request.json(&body);
        }
        // Finish fallible request construction before account reservation or admission.
        let request = request
            .build()
            .map_err(|error| Error::InvalidInput(error.to_string()))?;
        self.admit(mutation_account.is_some(), exit).await?;
        // Reserve the account before the first network await. This prevents two
        // concurrent clones from sending mutations whose outcomes cannot be ordered.
        let mut guard = mutation_account
            .map(|account| MutationGuard::new(self.inner.clone(), account))
            .transpose()?;
        let response =
            self.inner
                .http
                .execute(request)
                .await
                .map_err(|e| match mutation_account {
                    Some(account) => Error::AmbiguousMutation {
                        account: account.clone(),
                    },
                    None => Error::Transport(e.to_string()),
                })?;
        let status = response.status();
        let request_id = supplied_header(response.headers(), "RequestID");
        let next_page = if status.is_success() {
            parse_next_page(&self.inner.rest, response.headers())
        } else {
            Ok(Supplied::Absent)
        };
        if status == StatusCode::TOO_MANY_REQUESTS {
            // The status proves refusal even if local cooldown state is inconsistent.
            if let Some(ref mut guard) = guard {
                guard.disarm();
            }
            self.install_cooldown(response.headers().get(reqwest::header::RETRY_AFTER))
                .map_err(|source| Error::ProviderRateLimitedInvariant {
                    source: Box::new(source),
                })?;
            return Err(Error::ProviderRateLimited.into());
        }
        let definitive_rejection = matches!(status.as_u16(), 400 | 401 | 403 | 404 | 405);
        if definitive_rejection && let Some(ref mut guard) = guard {
            guard.disarm();
        }
        let bytes = read_bounded(response, self.inner.max_body)
            .await
            .map_err(|error| {
                if definitive_rejection {
                    OperationError::RejectedUnreadable {
                        status: status.as_u16(),
                        request_id: request_id.clone(),
                        source: error,
                    }
                } else if let Some(account) = mutation_account {
                    OperationError::Client(Error::AmbiguousMutation {
                        account: account.clone(),
                    })
                } else {
                    OperationError::Client(error)
                }
            })?;
        if status.is_success() {
            let value = serde_json::from_slice(&bytes).map_err(|e| match mutation_account {
                Some(account) => Error::AmbiguousMutation {
                    account: account.clone(),
                },
                None => Error::Decode(e.to_string()),
            })?;
            if let Some(ref mut guard) = guard {
                guard.disarm();
            }
            return Ok(ApiResponse {
                body: value,
                request_id,
                // Pagination does not determine a successfully decoded mutation outcome.
                // Refuse to expose an unsafe link while preserving the accepted result.
                next_page: if mutation_account.is_some() {
                    next_page.unwrap_or(Supplied::Undecoded)
                } else {
                    next_page?
                },
            });
        }
        if let Some(account) = mutation_account
            && !definitive_rejection
        {
            return Err(Error::AmbiguousMutation {
                account: account.clone(),
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
        // `errorCode` and `errorMessage` are read one key at a time, so a value
        // with an unexpected type erases only its own key and not its sibling —
        // exactly how `open_http_stream` reads a refused stream's reason. A
        // body this client cannot read as JSON yields no reason, and `body`
        // above is what tells the caller one arrived regardless.
        let reason = ProviderError::read(&bytes);
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
        self.execute(RequestParts::new(Method::GET, &path)).await
    }

    pub(crate) async fn admit(&self, mutation: bool, exit: bool) -> Result<()> {
        self.inner.admission.admit(mutation, exit).await
    }

    pub(crate) fn build_url(base: &Url, path: &str, query: Option<&Value>) -> Result<Url> {
        if !path.starts_with("/v3/") {
            return Err(Error::InvalidInput("path must start with /v3/".into()));
        }
        let mut url = base
            .join(path)
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        if let Some(query) = query {
            let object = query
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
    fn install_cooldown(&self, retry_after: Option<&reqwest::header::HeaderValue>) -> Result<()> {
        let cooldown = match retry_after {
            None => DEFAULT_COOLDOWN,
            Some(value) => parse_retry_after(value).unwrap_or(MAX_COOLDOWN),
        };
        self.inner.admission.install_cooldown(cooldown)
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

    /// Reserve requests in the shared local 120-request, rolling one-second REST window for exits.
    ///
    /// A new private owner defaults to zero; an injected owner keeps its policy when
    /// this method is omitted. Only `cancel_order`, `close_trade`, and `close_position`
    /// can use the reserve. All requests still count toward the same aggregate limit
    /// and provider cooldown. The value must be less than 120; `build` validates it.
    #[must_use]
    pub fn exit_reserve(mut self, requests: u32) -> Self {
        self.exit_reserve = Some(requests);
        self
    }

    /// Share admission with other clients using the same caller-selected egress scope.
    ///
    /// This owner has no credentials. An explicitly requested `exit_reserve` must
    /// agree with its current policy; construction never resets shared capacity.
    #[must_use]
    pub fn admission(mut self, owner: Admission) -> Self {
        self.admission = Some(owner);
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
        if self
            .exit_reserve
            .is_some_and(|reserve| reserve >= REQUESTS_PER_SECOND)
        {
            return Err(Error::InvalidInput(
                "exit reserve must be less than 120 requests".into(),
            ));
        }
        let admission = match self.admission {
            Some(owner) => {
                if let Some(requested) = self.exit_reserve
                    && requested != owner.exit_reserve()?
                {
                    return Err(Error::InvalidInput(
                        "exit reserve conflicts with shared admission policy".into(),
                    ));
                }
                owner
            }
            None => Admission::with_exit_reserve(self.exit_reserve.unwrap_or(0))?,
        };
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
            .connector_layer(admission.connection_layer())
            .timeout(self.timeout)
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .no_proxy()
            .build()
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        let stream_http = stream_transport(reqwest::Client::builder(), self.timeout)
            .connector_layer(admission.connection_layer())
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
                admission,
                fenced: Mutex::new(HashSet::new()),
                in_flight: Mutex::new(HashSet::new()),
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

/// Read a provider header as supplied evidence.
///
/// A header OANDA did not send, and one it sent with bytes this client cannot
/// read as text, are different facts; the second must not collapse into the
/// first.
fn supplied_header(headers: &reqwest::header::HeaderMap, name: &str) -> Supplied<RequestID> {
    match headers.get(name) {
        None => Supplied::Absent,
        Some(value) => match value.to_str() {
            Ok(text) => match RequestID::new(text) {
                Ok(id) => Supplied::Decoded(id),
                Err(_) => Supplied::Undecoded,
            },
            Err(_) => Supplied::Undecoded,
        },
    }
}

/// Read the Link header's next-page URL as supplied evidence.
///
/// A Link header OANDA did not send, and one that is readable but offers no
/// `rel="next"` item, are both `Absent`: no next page arrived. A Link this
/// client cannot read as text, or whose next item does not match the
/// documented shape, is `Undecoded`: a next page was offered and could not be
/// surfaced. A next link that decodes but leaves this client's own API origin
/// is an error instead: that value was read and refused, never lost.
fn parse_next_page(base: &Url, headers: &reqwest::header::HeaderMap) -> Result<Supplied<Url>> {
    let Some(link) = headers.get(reqwest::header::LINK) else {
        return Ok(Supplied::Absent);
    };
    let Ok(value) = link.to_str() else {
        return Ok(Supplied::Undecoded);
    };
    let Some(item) = value
        .split(',')
        .find(|item| item.contains("rel=\"next\"") || item.contains("rel=next"))
    else {
        return Ok(Supplied::Absent);
    };
    // The item was readable and offered a next page, so a shape that does not
    // parse is a decoded instruction this client must refuse loudly — the same
    // treatment a readable link that leaves this client's origin gets. A
    // caller paging a result set learns immediately that pagination broke
    // instead of reading Undecoded as "no next page".
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
    Ok(Supplied::Decoded(url))
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
    /// Read each reason key on its own. A struct-level deserialize would fail
    /// on the first mistyped field and erase both keys, reporting a reason
    /// OANDA sent as one it never did.
    fn read(bytes: &[u8]) -> Self {
        let value: serde_json::Value = match serde_json::from_slice(bytes) {
            Ok(value) => value,
            Err(_) => return Self::UNREAD,
        };
        Self {
            error_code: value
                .get("errorCode")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            error_message: value
                .get("errorMessage")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
        }
    }

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
    account: AccountID,
    armed: bool,
}
impl MutationGuard {
    fn new(inner: Arc<Inner>, account: &AccountID) -> Result<Self> {
        // Check every fallible prerequisite while holding the reservation lock, then insert.
        // A poisoned fence lock cannot leave a reservation without an owning guard.
        let mut in_flight = inner
            .in_flight
            .lock()
            .map_err(|_| Error::Invariant("mutation lock poisoned"))?;
        if inner
            .fenced
            .lock()
            .map_err(|_| Error::Invariant("reconciliation lock poisoned"))?
            .contains(account)
        {
            return Err(Error::ReconciliationRequired {
                account: account.clone(),
            });
        }
        if !in_flight.insert(account.clone()) {
            return Err(Error::MutationInFlight {
                account: account.clone(),
            });
        }
        drop(in_flight);
        Ok(Self {
            inner,
            account: account.clone(),
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

fn stream_transport(builder: reqwest::ClientBuilder, timeout: Duration) -> reqwest::ClientBuilder {
    builder
        .http1_only()
        .connect_timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .no_proxy()
}

#[cfg(test)]
// Poisoning a lock requires a panic, and fixture failures are clearer as panics;
// production paths return typed errors.
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use tokio::time::Instant as RateInstant;

    #[tokio::test]
    async fn streaming_uses_http11_when_the_caller_enables_http2() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let address = listener.local_addr().expect("address");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut bytes = Vec::new();
            let mut buffer = [0; 1024];
            loop {
                let count = socket.read(&mut buffer).await.expect("read request");
                if count == 0 {
                    break;
                }
                bytes.extend_from_slice(&buffer[..count]);
                if bytes.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            socket
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                .await
                .expect("response");
            bytes
        });
        let client = stream_transport(
            reqwest::Client::builder().http2_prior_knowledge(),
            Duration::from_secs(5),
        )
        .build()
        .expect("client");
        let response = client
            .get(format!(
                "http://{address}/v3/accounts/fixture/transactions/stream"
            ))
            .send()
            .await;
        let request = server.await.expect("server");
        assert!(
            request.starts_with(b"GET /v3/accounts/fixture/transactions/stream HTTP/1.1\r\n"),
            "request protocol: {}",
            String::from_utf8_lossy(&request)
        );
        assert_eq!(
            response.expect("stream response").version(),
            reqwest::Version::HTTP_11
        );
    }

    #[tokio::test]
    async fn mutation_rate_admission_is_shared_by_clones() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        if let Ok(mut rate) = client.inner.admission.inner.rate.lock() {
            for _ in 0..REQUESTS_PER_SECOND {
                rate.admitted.push_back(RateInstant::now());
            }
        }
        assert!(matches!(
            client.clone().admit(true, false).await,
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
            fenced.contains(&account),
            "the ambiguity must be fenced, not dropped"
        );
        let in_flight = client
            .inner
            .in_flight
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(!in_flight.contains(&account));
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
            fenced.insert(account.clone());
        }
        poison(&client.inner.fenced);
        assert!(matches!(
            client.acknowledge_reconciliation(&account),
            Err(Error::Invariant(_))
        ));
        let fenced = client
            .inner
            .fenced
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(fenced.contains(&account));
    }

    fn assert_cooldown(header: Option<&str>, expected: Duration) {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        let header =
            header.map(|text| reqwest::header::HeaderValue::from_str(text).expect("header"));
        let start = RateInstant::now();
        client
            .install_cooldown(header.as_ref())
            .expect("install cooldown");
        let rate = client
            .inner
            .admission
            .inner
            .rate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let until = rate.cooldown_until.expect("cooldown installed");
        // An RFC 9110 date carries whole seconds, so formatting a later
        // instant floors it and `num_seconds` floors again; a slow runner can
        // legitimately land one second under the nominal span. One second of
        // tolerance each way keeps the bound meaningful without flaking.
        let tolerance = Duration::from_secs(1);
        assert!(
            until + tolerance >= start + expected,
            "cooldown shorter than {expected:?}"
        );
        assert!(
            until <= RateInstant::now() + expected + tolerance,
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

    fn reserved_client() -> Client {
        Client::builder(Environment::Practice, "fixture")
            .exit_reserve(2)
            .build()
            .expect("valid reserve")
    }

    #[test]
    fn exit_reserve_rejects_the_full_window_and_larger_values() {
        for reserve in [120, 121, u32::MAX] {
            assert!(matches!(
                Client::builder(Environment::Practice, "fixture")
                    .exit_reserve(reserve)
                    .build(),
                Err(Error::InvalidInput(_))
            ));
        }
    }

    #[tokio::test(start_paused = true)]
    async fn ordinary_admission_leaves_two_exits_and_preserves_the_aggregate_limit() {
        let client = reserved_client();
        for _ in 0..118 {
            client.admit(false, false).await.expect("ordinary slot");
        }
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        let read = client.admit(false, false);
        tokio::pin!(read);
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        client.clone().admit(true, true).await.expect("first exit");
        client.clone().admit(true, true).await.expect("second exit");
        assert!(matches!(
            client.admit(true, true).await,
            Err(Error::RateLimited)
        ));
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        tokio::time::advance(Duration::from_millis(999)).await;
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        tokio::time::advance(Duration::from_millis(1)).await;
        read.await.expect("ordinary slot after exact expiry");
    }

    #[tokio::test(start_paused = true)]
    async fn concurrent_clones_cannot_exceed_the_two_remaining_exit_slots() {
        let client = reserved_client();
        for _ in 0..118 {
            client.admit(false, false).await.expect("ordinary slot");
        }
        let mut tasks = Vec::new();
        for _ in 0..20 {
            let clone = client.clone();
            tasks.push(tokio::spawn(async move { clone.admit(true, true).await }));
        }
        let mut admitted = 0;
        let mut refused = 0;
        for task in tasks {
            match task.await.expect("join") {
                Ok(()) => admitted += 1,
                Err(Error::RateLimited) => refused += 1,
                result => panic!("unexpected admission: {result:?}"),
            }
        }
        assert_eq!(admitted, 2);
        assert_eq!(refused, 18);
    }

    #[tokio::test(start_paused = true)]
    async fn cancelled_ordinary_waiters_do_not_charge_the_next_window() {
        let client = reserved_client();
        for _ in 0..118 {
            client.admit(false, false).await.expect("ordinary slot");
        }
        {
            let read = client.admit(false, false);
            tokio::pin!(read);
            assert!(futures_util::poll!(read.as_mut()).is_pending());
        }
        tokio::time::advance(Duration::from_secs(1)).await;
        for _ in 0..118 {
            client.admit(true, false).await.expect("new ordinary slot");
        }
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        client.admit(true, true).await.expect("first exit");
        client.admit(true, true).await.expect("second exit");
    }

    #[tokio::test(start_paused = true)]
    async fn provider_cooldown_blocks_both_classes_until_exact_expiry() {
        let client = reserved_client();
        client
            .install_cooldown(Some(&reqwest::header::HeaderValue::from_static("2")))
            .expect("install cooldown");
        assert!(matches!(
            client.clone().admit(true, false).await,
            Err(Error::RateLimited)
        ));
        assert!(matches!(
            client.clone().admit(true, true).await,
            Err(Error::RateLimited)
        ));
        let read = client.admit(false, false);
        tokio::pin!(read);
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        tokio::time::advance(Duration::from_millis(1999)).await;
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        assert!(matches!(
            client.admit(true, true).await,
            Err(Error::RateLimited)
        ));
        tokio::time::advance(Duration::from_millis(1)).await;
        read.await.expect("read after cooldown");
        client.admit(true, true).await.expect("exit after cooldown");
    }

    #[tokio::test(start_paused = true)]
    async fn a_shorter_provider_cooldown_cannot_release_an_existing_deadline_early() {
        let client = reserved_client();
        client
            .install_cooldown(Some(&reqwest::header::HeaderValue::from_static("5")))
            .expect("install cooldown");
        tokio::time::advance(Duration::from_secs(1)).await;
        client
            .install_cooldown(Some(&reqwest::header::HeaderValue::from_static("1")))
            .expect("install cooldown");
        tokio::time::advance(Duration::from_secs(1)).await;
        assert!(matches!(
            client.admit(true, true).await,
            Err(Error::RateLimited)
        ));
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        tokio::time::advance(Duration::from_secs(3)).await;
        client
            .admit(true, true)
            .await
            .expect("original cooldown expired");
    }

    #[tokio::test(start_paused = true)]
    async fn runtime_release_wakes_all_registered_waiters_without_advancing_time() {
        let client = reserved_client();
        for _ in 0..118 {
            client.admit(false, false).await.expect("ordinary slot");
        }
        let first = client.admit(false, false);
        let second = client.admit(false, false);
        tokio::pin!(first, second);
        assert!(futures_util::poll!(first.as_mut()).is_pending());
        assert!(futures_util::poll!(second.as_mut()).is_pending());
        let started = RateInstant::now();
        client.clone().set_exit_reserve(0).expect("release reserve");
        assert!(matches!(
            futures_util::poll!(first.as_mut()),
            std::task::Poll::Ready(Ok(()))
        ));
        assert!(matches!(
            futures_util::poll!(second.as_mut()),
            std::task::Poll::Ready(Ok(()))
        ));
        assert_eq!(RateInstant::now(), started);
        assert!(matches!(
            client.admit(true, true).await,
            Err(Error::RateLimited)
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn runtime_growth_rechecks_waiters_and_preserves_spent_capacity() {
        let client = reserved_client();
        for _ in 0..118 {
            client.admit(false, false).await.expect("ordinary slot");
        }
        let read = client.admit(false, false);
        tokio::pin!(read);
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        client.set_exit_reserve(0).expect("release reserve");
        client
            .clone()
            .set_exit_reserve(119)
            .expect("replace reserve");
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        client.admit(true, true).await.expect("first exit");
        client.admit(true, true).await.expect("second exit");
        assert!(matches!(
            client.admit(true, true).await,
            Err(Error::RateLimited)
        ));
        client
            .set_exit_reserve(0)
            .expect("release exhausted reserve");
        client
            .set_exit_reserve(119)
            .expect("raise exhausted reserve");
        assert!(matches!(
            client.admit(true, true).await,
            Err(Error::RateLimited)
        ));
        client.set_exit_reserve(0).expect("release reserve");
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        tokio::time::advance(Duration::from_secs(1)).await;
        assert!(matches!(
            futures_util::poll!(read.as_mut()),
            std::task::Poll::Ready(Ok(()))
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn runtime_changes_preserve_cooldown_window_and_invalid_policy_state() {
        let client = reserved_client();
        for _ in 0..118 {
            client.admit(false, false).await.expect("ordinary slot");
        }
        client
            .install_cooldown(Some(&reqwest::header::HeaderValue::from_static("2")))
            .expect("install cooldown");
        for reserve in [119, 0, 2] {
            client.set_exit_reserve(reserve).expect("replace reserve");
            assert!(matches!(
                client.admit(true, true).await,
                Err(Error::RateLimited)
            ));
        }
        for reserve in [120, u32::MAX] {
            assert!(matches!(
                client.set_exit_reserve(reserve),
                Err(Error::InvalidInput(_))
            ));
            let rate = client.inner.admission.inner.rate.lock().expect("rate lock");
            assert_eq!(rate.exit_reserve, 2);
            assert_eq!(rate.admitted.len(), 118);
        }
        client.set_exit_reserve(0).expect("release reserve");
        let read = client.admit(false, false);
        tokio::pin!(read);
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        tokio::time::advance(Duration::from_secs(1)).await;
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        tokio::time::advance(Duration::from_secs(1)).await;
        assert!(matches!(
            futures_util::poll!(read.as_mut()),
            std::task::Poll::Ready(Ok(()))
        ));
        for _ in 0..119 {
            client
                .admit(true, true)
                .await
                .expect("remaining aggregate capacity");
        }
        assert!(matches!(
            client.admit(true, true).await,
            Err(Error::RateLimited)
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn runtime_replacement_keeps_each_admission_deadline_and_independent_client() {
        let client = reserved_client();
        for _ in 0..118 {
            client.admit(false, false).await.expect("ordinary slot");
        }
        tokio::time::advance(Duration::from_millis(900)).await;
        client.set_exit_reserve(3).expect("replace reserve");
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        let independent = reserved_client();
        independent
            .admit(true, false)
            .await
            .expect("independent budget");
        client.admit(true, true).await.expect("first exit");
        client.admit(true, true).await.expect("second exit");
        tokio::time::advance(Duration::from_millis(100)).await;
        for _ in 0..115 {
            client
                .admit(true, false)
                .await
                .expect("new ordinary capacity");
        }
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        tokio::time::advance(Duration::from_millis(900)).await;
        client.admit(true, false).await.expect("first exit expired");
        client
            .admit(true, false)
            .await
            .expect("second exit expired");
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
    }

    #[test]
    fn runtime_setter_reports_poison_and_preserves_policy_and_admission_state() {
        let client = reserved_client();
        poison(&client.inner.admission.inner.rate);
        assert!(matches!(
            client.set_exit_reserve(0),
            Err(Error::Invariant(_))
        ));
        let rate = client
            .inner
            .admission
            .inner
            .rate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(rate.exit_reserve, 2);
        assert_eq!(rate.admitted.len(), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn runtime_waiter_rechecks_each_replacement_including_release_then_growth() {
        let client = reserved_client();
        client.set_exit_reserve(119).expect("raise reserve");
        client.admit(false, false).await.expect("one ordinary slot");
        let read = client.admit(false, false);
        tokio::pin!(read);
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        client.set_exit_reserve(0).expect("release reserve");
        client.set_exit_reserve(119).expect("raise reserve");
        assert!(futures_util::poll!(read.as_mut()).is_pending());
        client.set_exit_reserve(118).expect("replace reserve");
        assert!(matches!(
            futures_util::poll!(read.as_mut()),
            std::task::Poll::Ready(Ok(()))
        ));
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
    }
    #[test]
    fn an_unreadable_fence_never_leaves_a_partial_reservation() {
        let client = Client::builder(Environment::Practice, "fixture")
            .build()
            .expect("client");
        let account = AccountID::new("fixture-account").expect("account");
        poison(&client.inner.fenced);
        assert!(matches!(
            MutationGuard::new(client.inner.clone(), &account),
            Err(Error::Invariant(_))
        ));
        assert!(
            client
                .inner
                .in_flight
                .lock()
                .expect("reservation lock")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn cooldown_failure_preserves_429_without_fencing_or_partial_policy_changes() {
        use tokio::io::AsyncWriteExt;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let url = Url::parse(&format!(
            "http://{}/",
            listener.local_addr().expect("address")
        ))
        .expect("URL");
        let client = Client::builder(Environment::Practice, "fixture")
            .endpoints(url.clone(), url)
            .build()
            .expect("client");
        let admission = client.inner.admission.clone();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            crate::test_support::read_request(&mut socket, &mut Vec::new())
                .await
                .expect("request");
            poison(&admission.inner.rate);
            socket.write_all(b"HTTP/1.1 429 Too Many Requests\r\nRetry-After: 2\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").await.expect("429");
        });
        let account = AccountID::new("fixture-account").expect("account");
        let result = client
            .configure_account(
                &account,
                &crate::accounts::ConfigureAccountBody {
                    alias: Some("fixture".into()),
                    margin_rate: None,
                },
            )
            .await;
        assert!(
            matches!(result, Err(OperationError::Client(Error::ProviderRateLimitedInvariant { source })) if matches!(*source, Error::Invariant(_)))
        );
        server.await.expect("server");
        assert!(client.inner.fenced.lock().expect("fence lock").is_empty());
        assert!(
            client
                .inner
                .in_flight
                .lock()
                .expect("reservation lock")
                .is_empty()
        );
        let rate = client
            .inner
            .admission
            .inner
            .rate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(rate.admitted.len(), 1);
        assert!(
            rate.cooldown_until.is_none(),
            "failed installation must preserve policy"
        );
    }
}
