//! Bounded newline-delimited HTTP streams.

use crate::{
    AccountID, Client, Error, Result,
    models::{ClientPrice, PricingHeartbeat, Transaction, TransactionHeartbeat},
};
use reqwest::Response;
use serde::Serialize;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// Local hostile-input bound on a single newline-delimited record.
///
/// This is the client's own memory bound, not a provider contract: OANDA
/// documents no record size, and realistic pricing and transaction payloads are
/// orders of magnitude smaller. A record over this limit ends the generation
/// with an explicit gap. There is deliberately no bound on the transport chunk
/// size or on the buffered backlog — HTTP chunk segmentation is not a provider
/// record boundary, and a slow caller must never cause records to be evicted,
/// because dropping records would invent a continuity gap that the provider did
/// not create.
const MAX_RECORD_BYTES: usize = 1024 * 1024;

/// One event from OANDA's sampled pricing stream.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum PriceStreamEvent {
    /// A sampled account price.
    Price(Box<ClientPrice>),
    /// A stream heartbeat.
    Heartbeat(PricingHeartbeat),
}

/// One event from OANDA's transaction stream.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TransactionStreamEvent {
    /// A typed transaction variant.
    Transaction(Box<Transaction>),
    /// A stream heartbeat.
    Heartbeat(TransactionHeartbeat),
}

/// An owned HTTP stream. Dropping it closes the connection and releases its slot.
pub struct HttpStream<E> {
    response: Option<Response>,
    buffer: Vec<u8>,
    start: usize,
    decoder: fn(&[u8]) -> Result<E>,
    ended: bool,
    lease: Option<StreamLease>,
}

impl<E> HttpStream<E> {
    fn finish(&mut self) {
        self.ended = true;
        self.response.take();
        self.lease.take();
    }
    /// Read the next event. Any error ends this generation and creates a continuity gap.
    pub async fn next_event(&mut self) -> Option<Result<E>> {
        if self.ended {
            return None;
        }
        loop {
            // Records are consumed through a cursor rather than by draining the
            // buffer per record: a large transport read must not turn record
            // delivery into quadratic copying, and buffered records are never
            // evicted, because a dropped record would invent a provider gap.
            if let Some(offset) = self.buffer[self.start..]
                .iter()
                .position(|byte| *byte == b'\n')
            {
                let newline = self.start + offset;
                let mut end = newline;
                if end > self.start && self.buffer[end - 1] == b'\r' {
                    end -= 1;
                }
                if end - self.start > MAX_RECORD_BYTES {
                    self.finish();
                    return Some(Err(Error::ResponseTooLarge));
                }
                let line: Vec<u8> = self.buffer[self.start..end].to_vec();
                self.start = newline + 1;
                if self.start == self.buffer.len() {
                    self.buffer.clear();
                    self.start = 0;
                } else if self.start > MAX_RECORD_BYTES {
                    self.buffer.drain(..self.start);
                    self.start = 0;
                }
                if line.is_empty() {
                    continue;
                }
                let result = (self.decoder)(&line);
                if result.is_err() {
                    self.finish();
                }
                return Some(result);
            }
            // No complete record is buffered. Bound the pending record, then
            // read more transport data.
            if self.buffer.len() - self.start > MAX_RECORD_BYTES {
                self.finish();
                return Some(Err(Error::ResponseTooLarge));
            }
            let Some(response) = self.response.as_mut() else {
                self.finish();
                return None;
            };
            match tokio::time::timeout(Duration::from_secs(30), response.chunk()).await {
                Err(_) => {
                    self.finish();
                    return Some(Err(Error::Transport(
                        "OANDA stream heartbeat timed out; continuity must be recovered".into(),
                    )));
                }
                Ok(Ok(Some(chunk))) => {
                    // An HTTP chunk is a transport read, not a record
                    // boundary: one chunk may carry thousands of complete
                    // records, so its size alone says nothing about provider
                    // data. Records are bounded individually, before any
                    // decode.
                    self.buffer.extend_from_slice(&chunk);
                }
                Ok(Ok(None)) => {
                    self.finish();
                    return Some(Err(Error::Transport(
                        "OANDA stream ended; continuity must be recovered".into(),
                    )));
                }
                Ok(Err(error)) => {
                    self.finish();
                    return Some(Err(Error::Transport(error.to_string())));
                }
            }
        }
    }
}

struct StreamLease {
    inner: Arc<crate::client::Inner>,
}
impl Drop for StreamLease {
    fn drop(&mut self) {
        // A dropped lease cannot report a failure, and skipping the release would
        // report an exhausted connection slot as one the client may reuse. The
        // slot counter holds plain integers, so a poisoned lock is still readable.
        let mut slots = self
            .inner
            .stream_slots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        slots.active = slots.active.saturating_sub(1);
    }
}

impl Client {
    pub(crate) async fn open_http_stream<E, Q: Serialize>(
        &self,
        path: &str,
        query: Option<&Q>,
        decoder: fn(&[u8]) -> Result<E>,
    ) -> Result<HttpStream<E>> {
        self.admit(false).await?;
        {
            let mut slots = self
                .inner
                .stream_slots
                .lock()
                .map_err(|_| Error::InvalidInput("stream slots lock poisoned".into()))?;
            let now = Instant::now();
            if slots.active >= 20
                || slots
                    .last_start
                    .is_some_and(|start| now.duration_since(start) < Duration::from_millis(500))
            {
                return Err(Error::RateLimited);
            }
            slots.active += 1;
            slots.last_start = Some(now);
        }
        let lease = StreamLease {
            inner: self.inner.clone(),
        };
        let url = Self::build_url(&self.inner.stream, path, query)?;
        let response = self
            .inner
            .stream_http
            .get(url)
            .bearer_auth(&self.inner.token)
            .header(
                "Accept-Datetime-Format",
                self.inner.datetime_format.as_str(),
            )
            .send()
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = crate::client::read_bounded(response, self.inner.max_body).await?;
            // A rejection body this client cannot read is not a rejection without a
            // reason. Report the observed status and no reason; never a message or
            // error code OANDA did not send. `Value::get` yields nothing for a
            // document that is not an object, so no shape check is needed.
            let reason = serde_json::from_slice::<serde_json::Value>(&body).ok();
            let code = reason
                .as_ref()
                .and_then(|value| value.get("errorCode"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            let message = reason
                .as_ref()
                .and_then(|value| value.get("errorMessage"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            return Err(Error::Provider {
                status,
                code,
                message,
            });
        }
        Ok(HttpStream {
            response: Some(response),
            buffer: Vec::new(),
            start: 0,
            decoder,
            ended: false,
            lease: Some(lease),
        })
    }

    /// Open the sampled account pricing stream.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid query, exhausted stream capacity, transport failure, or
    /// provider rejection.
    pub async fn stream_pricing(
        &self,
        account: &AccountID,
        query: &crate::pricing::StreamPricingQuery,
    ) -> Result<HttpStream<PriceStreamEvent>> {
        if query.instruments.is_empty() {
            return Err(Error::InvalidInput(
                "pricing instruments must not be empty".into(),
            ));
        }
        let path = format!(
            "/v3/accounts/{}/pricing/stream",
            crate::client::segment(account.as_str())
        );
        self.open_http_stream(&path, Some(query), decode_price)
            .await
    }

    /// Open the account transaction stream. It starts at connection time; use transaction history for recovery.
    ///
    /// # Errors
    ///
    /// Returns an error for exhausted stream capacity, transport failure, or provider rejection.
    pub async fn stream_transactions(
        &self,
        account: &AccountID,
    ) -> Result<HttpStream<TransactionStreamEvent>> {
        let path = format!(
            "/v3/accounts/{}/transactions/stream",
            crate::client::segment(account.as_str())
        );
        self.open_http_stream(&path, None::<&()>, decode_transaction)
            .await
    }
}

fn decode_price(line: &[u8]) -> Result<PriceStreamEvent> {
    let value: serde_json::Value =
        serde_json::from_slice(line).map_err(|e| Error::Decode(e.to_string()))?;
    match value.get("type").and_then(serde_json::Value::as_str) {
        Some("PRICE") => serde_json::from_value(value)
            .map(Box::new)
            .map(PriceStreamEvent::Price)
            .map_err(|e| Error::Decode(e.to_string())),
        Some("HEARTBEAT") => serde_json::from_value(value)
            .map(PriceStreamEvent::Heartbeat)
            .map_err(|e| Error::Decode(e.to_string())),
        _ => Err(Error::Decode("unknown OANDA pricing stream record".into())),
    }
}

fn decode_transaction(line: &[u8]) -> Result<TransactionStreamEvent> {
    let value: serde_json::Value =
        serde_json::from_slice(line).map_err(|e| Error::Decode(e.to_string()))?;
    if value.get("type").and_then(serde_json::Value::as_str) == Some("HEARTBEAT") {
        serde_json::from_value(value)
            .map(TransactionStreamEvent::Heartbeat)
            .map_err(|e| Error::Decode(e.to_string()))
    } else {
        serde_json::from_value(value)
            .map(Box::new)
            .map(TransactionStreamEvent::Transaction)
            .map_err(|e| Error::Decode(e.to_string()))
    }
}

#[cfg(test)]
// Poisoning a lock requires a panic, and fixture failures are clearer as panics;
// production paths return typed errors.
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::Environment;

    #[tokio::test]
    async fn cancelled_stream_owner_releases_its_connection_slot() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        if let Ok(mut slots) = client.inner.stream_slots.lock() {
            slots.active = 1;
        }
        let lease = StreamLease {
            inner: client.inner.clone(),
        };
        let task = tokio::spawn(async move {
            let _lease = lease;
            std::future::pending::<()>().await;
        });
        tokio::task::yield_now().await;
        task.abort();
        let _ = task.await;
        if let Ok(slots) = client.inner.stream_slots.lock() {
            assert_eq!(slots.active, 0);
        }
    }

    #[tokio::test]
    async fn a_cancelled_stream_releases_its_slot_even_after_a_poisoned_lock() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        if let Ok(mut slots) = client.inner.stream_slots.lock() {
            slots.active = 1;
        }
        let lease = StreamLease {
            inner: client.inner.clone(),
        };
        let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Ok(_held) = client.inner.stream_slots.lock() else {
                return;
            };
            panic!("poison the lock");
        }));
        assert!(poisoned.is_err());
        drop(lease);
        let slots = client
            .inner
            .stream_slots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(
            slots.active, 0,
            "a released connection must not be reported as capacity in use"
        );
    }
}
