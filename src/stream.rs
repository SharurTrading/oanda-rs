// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded newline-delimited HTTP streams.

use crate::{
    AccountID, Client, Error, Result,
    admission::StreamLease,
    models::{ClientPrice, PricingHeartbeat, Transaction, TransactionHeartbeat},
    validation::Validate,
};
use reqwest::Response;
use serde::Serialize;
use std::time::Duration;

/// Local hostile-input bound on a single newline-delimited record.
///
/// This is the client's own memory bound, not a provider contract: OANDA
/// documents no record size, and realistic pricing and transaction payloads are
/// orders of magnitude smaller. A record over this limit ends the generation
/// with an explicit gap. There is deliberately no bound on the transport chunk
/// size: HTTP chunk segmentation is not a provider record boundary. The separate
/// total backlog cap below ends an oversized generation explicitly; it never
/// evicts records to make room for later ones.
const MAX_RECORD_BYTES: usize = 1024 * 1024;
/// Local hostile-input bound on the total buffered-but-undelivered backlog.
///
/// Records already buffered are never evicted — a slow caller delays delivery
/// rather than losing records — but a fill episode that parks more than this
/// many bytes in the buffer, complete records included, ends the generation
/// with an explicit gap. Without it a provider burst of complete records could
/// occupy memory without bound, because no further reading — and therefore no
/// per-record check — happens while undelivered records remain.
const MAX_BACKLOG_BYTES: usize = 16 * 1024 * 1024;
/// Local maximum silence before ending a stream generation.
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);

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

struct Buffered<E> {
    bytes: Vec<u8>,
    start: usize,
    decoder: fn(&[u8]) -> Result<E>,
}

impl<E> Buffered<E> {
    fn remaining(&self) -> Result<usize> {
        self.bytes
            .len()
            .checked_sub(self.start)
            .ok_or(Error::Invariant("stream buffer cursor exceeds its length"))
    }

    fn next(&mut self) -> Option<Result<E>> {
        let remaining = match self.remaining() {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        if remaining > MAX_BACKLOG_BYTES {
            return Some(Err(Error::ResponseTooLarge));
        }
        loop {
            let Some(offset) = self.bytes[self.start..]
                .iter()
                .position(|byte| *byte == b'\n')
            else {
                return (self.bytes.len() - self.start > MAX_RECORD_BYTES)
                    .then_some(Err(Error::ResponseTooLarge));
            };
            let newline = self.start + offset;
            let end = if newline > self.start && self.bytes[newline - 1] == b'\r' {
                newline - 1
            } else {
                newline
            };
            if end - self.start > MAX_RECORD_BYTES {
                return Some(Err(Error::ResponseTooLarge));
            }
            // Decode the borrowed slice before compaction; the decoder returns owned evidence.
            let event = (end > self.start).then(|| (self.decoder)(&self.bytes[self.start..end]));
            self.start = newline + 1;
            if self.start == self.bytes.len() {
                self.bytes.clear();
                self.start = 0;
            } else if self.start > MAX_RECORD_BYTES {
                self.bytes.drain(..self.start);
                self.start = 0;
            }
            if let Some(event) = event {
                return Some(event);
            }
        }
    }
}

/// A finite, owned prefix of an HTTP stream's already buffered records.
///
/// The HTTP response and admission lease are closed before this value is returned.
/// Iteration is synchronous: it performs no network read and requires no runtime.
/// It yields records in their original order, ending after the first error or
/// after the prefix is exhausted. A nonempty unterminated tail is an explicit
/// error. Dropping this value discards any undelivered prefix, so closure alone
/// does not prove that the caller consumed it.
#[must_use = "consume the finite prefix and handle errors before claiming it was drained"]
pub struct BufferedEvents<E> {
    buffered: Buffered<E>,
    initial_error: Option<Error>,
    finished: bool,
}

impl<E> std::fmt::Debug for BufferedEvents<E> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("BufferedEvents")
            .field(
                "remaining_bytes",
                &self.buffered.bytes.len().checked_sub(self.buffered.start),
            )
            .field("finished", &self.finished)
            .finish_non_exhaustive()
    }
}

impl<E> Iterator for BufferedEvents<E> {
    type Item = Result<E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if let Some(error) = self.initial_error.take() {
            self.finished = true;
            return Some(Err(error));
        }
        if let Some(event) = self.buffered.next() {
            self.finished = event.is_err();
            return Some(event);
        }
        self.finished = true;
        match self.buffered.remaining() {
            Ok(0) => None,
            Ok(bytes) => Some(Err(Error::IncompleteStreamRecord { bytes })),
            Err(error) => Some(Err(error)),
        }
    }
}

impl<E> std::iter::FusedIterator for BufferedEvents<E> {}

/// An owned HTTP stream. Dropping it closes the connection and releases its slot.
pub struct HttpStream<E> {
    response: Option<Response>,
    buffered: Buffered<E>,
    ended: bool,
    lease: Option<StreamLease>,
}

impl<E> HttpStream<E> {
    fn finish(&mut self) {
        self.ended = true;
        self.response.take();
        self.lease.take();
    }

    /// Closes network ingress and transfers exactly the already buffered prefix.
    ///
    /// The returned iterator retains the decoder and original ordering, with the
    /// same record and backlog limits. It owns no response or admission lease.
    /// No unread OS/socket/HTTP-body bytes are fetched; this is a local buffer
    /// cut, not evidence of a complete venue sequence or transaction boundary.
    /// Cancel a pending `next_event` borrow before consuming the stream here.
    ///
    /// The iterator yields `StreamAlreadyEnded` if this generation had already
    /// ended, and `IncompleteStreamRecord` for an unterminated tail. Decode and
    /// size errors also end the iterator. Exhaust it successfully before claiming
    /// that the buffered prefix was delivered.
    pub fn close(mut self) -> BufferedEvents<E> {
        let initial_error = self.ended.then_some(Error::StreamAlreadyEnded);
        self.finish();
        BufferedEvents {
            buffered: self.buffered,
            initial_error,
            finished: false,
        }
    }

    /// Read the next event. Any error ends this generation and creates a continuity gap.
    pub async fn next_event(&mut self) -> Option<Result<E>> {
        if self.ended {
            return None;
        }
        loop {
            if let Some(result) = self.buffered.next() {
                if result.is_err() {
                    self.finish();
                }
                return Some(result);
            }
            let Some(response) = self.response.as_mut() else {
                self.finish();
                return None;
            };
            match tokio::time::timeout(HEARTBEAT_TIMEOUT, response.chunk()).await {
                Err(_) => {
                    self.finish();
                    return Some(Err(Error::Transport(
                        "OANDA stream heartbeat timed out; continuity must be recovered".into(),
                    )));
                }
                Ok(Ok(Some(chunk))) => {
                    self.buffered.bytes.extend_from_slice(&chunk);
                    match self.buffered.remaining() {
                        Ok(bytes) if bytes <= MAX_BACKLOG_BYTES => {}
                        Ok(_) => {
                            self.finish();
                            return Some(Err(Error::ResponseTooLarge));
                        }
                        Err(error) => {
                            self.finish();
                            return Some(Err(error));
                        }
                    }
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

impl Client {
    pub(crate) async fn open_http_stream<E, Q: Validate + Serialize>(
        &self,
        path: &str,
        query: Option<&Q>,
        decoder: fn(&[u8]) -> Result<E>,
    ) -> Result<HttpStream<E>> {
        let query = query
            .map(|query| {
                query.validate()?;
                serde_json::to_value(query).map_err(|error| Error::InvalidInput(error.to_string()))
            })
            .transpose()?;
        let url = Self::build_url(&self.inner.stream, path, query.as_ref())?;
        let lease = self.inner.admission.lease_stream()?;
        let response = self
            .inner
            .stream_http
            .get(url)
            .bearer_auth(self.inner.token.as_str())
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
            buffered: Buffered {
                bytes: Vec::new(),
                start: 0,
                decoder,
            },
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

    fn from_buffer<E>(bytes: Vec<u8>, decoder: fn(&[u8]) -> Result<E>) -> HttpStream<E> {
        HttpStream {
            response: None,
            buffered: Buffered {
                bytes,
                start: 0,
                decoder,
            },
            ended: false,
            lease: None,
        }
    }

    fn heartbeat(id: u32) -> String {
        format!(
            "{{\"type\":\"HEARTBEAT\",\"lastTransactionID\":\"{id}\",\"time\":\"2024-01-02T03:04:05.000000001Z\"}}\r\n"
        )
    }

    fn transaction_id(event: Result<TransactionStreamEvent>) -> String {
        match event.expect("valid synthetic heartbeat") {
            TransactionStreamEvent::Heartbeat(value) => {
                value.last_transaction_id.expect("fixture id").to_string()
            }
            TransactionStreamEvent::Transaction(_) => panic!("fixture contains no transaction"),
        }
    }

    #[tokio::test]
    async fn closing_preserves_the_exact_remaining_complete_prefix_and_original_decoder() {
        let bytes = format!("\r\n{}\n{}{}", heartbeat(10), heartbeat(11), heartbeat(12));
        let mut stream = from_buffer(bytes.into_bytes(), decode_transaction);
        assert_eq!(
            transaction_id(stream.next_event().await.expect("first record")),
            "10"
        );
        let mut buffered = stream.close();
        assert_eq!(
            transaction_id(buffered.next().expect("second record")),
            "11"
        );
        assert_eq!(transaction_id(buffered.next().expect("third record")), "12");
        assert!(buffered.next().is_none());
        assert!(buffered.next().is_none());
        let price = br#"{"type":"PRICE","instrument":"EUR_USD","closeoutBid":"1.2300","bids":[{"price":"1.229900","liquidity":17}]}"#;
        let mut bytes = price.to_vec();
        bytes.extend_from_slice(b"\r\n");
        let mut buffered = from_buffer(bytes, decode_price).close();
        match buffered
            .next()
            .expect("price")
            .expect("typed pricing decode")
        {
            PriceStreamEvent::Price(value) => {
                let bid = value.closeout_bid.expect("fixture bid");
                assert_eq!(bid.to_string(), "1.2300");
                assert_eq!(bid.scale(), 4);
                assert_eq!(
                    value.bids.expect("fixture buckets")[0]
                        .price
                        .expect("bucket price")
                        .to_string(),
                    "1.229900"
                );
            }
            PriceStreamEvent::Heartbeat(_) => panic!("fixture is a price"),
        }
        assert!(buffered.next().is_none());
    }

    #[tokio::test]
    async fn a_large_buffered_burst_drains_in_order_without_a_record_count_limit() {
        let mut bytes = Vec::new();
        for id in 0..30_000 {
            bytes.extend_from_slice(heartbeat(id).as_bytes());
        }
        assert!(bytes.len() > MAX_RECORD_BYTES && bytes.len() < MAX_BACKLOG_BYTES);
        let server = crate::test_support::held_stream(&bytes).await;
        let client = crate::test_support::fixture_client(server.url.clone());
        let account = AccountID::new("101-001-1-001").expect("fixture account");
        let mut stream = client
            .stream_transactions(&account)
            .await
            .expect("real stream");
        tokio::time::timeout(Duration::from_secs(5), preload(&mut stream, bytes.len()))
            .await
            .expect("finite coalesced HTTP prefix");
        let mut buffered = stream.close();
        for id in 0..30_000 {
            assert_eq!(
                transaction_id(buffered.next().expect("retained complete record")),
                id.to_string()
            );
        }
        assert!(buffered.next().is_none());
        peer_closed(server).await;
    }

    #[test]
    fn a_partial_tail_is_explicit_after_the_complete_prefix_and_then_fused() {
        for tail in [b"{\"type\":".as_slice(), b"\r", b" "] {
            let mut bytes = heartbeat(21).into_bytes();
            bytes.extend_from_slice(tail);
            let mut buffered = from_buffer(bytes, decode_transaction).close();
            assert_eq!(
                transaction_id(buffered.next().expect("complete record")),
                "21"
            );
            assert!(
                matches!(buffered.next(), Some(Err(Error::IncompleteStreamRecord { bytes })) if bytes == tail.len())
            );
            assert!(buffered.next().is_none());
            assert!(buffered.next().is_none());
        }
        for empty in [b"".as_slice(), b"\n\r\n"] {
            assert!(
                from_buffer(empty.to_vec(), decode_transaction)
                    .close()
                    .next()
                    .is_none()
            );
        }
    }

    #[test]
    fn malformed_and_oversized_prefixes_end_explicitly_without_skipping_to_later_records() {
        let mut malformed = b"not-json\n".to_vec();
        malformed.extend_from_slice(heartbeat(31).as_bytes());
        let mut buffered = from_buffer(malformed, decode_transaction).close();
        assert!(matches!(buffered.next(), Some(Err(Error::Decode(_)))));
        assert!(buffered.next().is_none());
        for mut oversized in [
            vec![b'x'; MAX_RECORD_BYTES + 1],
            vec![b'x'; MAX_BACKLOG_BYTES + 1],
        ] {
            let mut partial = from_buffer(oversized.clone(), decode_transaction).close();
            assert!(matches!(partial.next(), Some(Err(Error::ResponseTooLarge))));
            assert!(partial.next().is_none());
            oversized.extend_from_slice(b"\n");
            oversized.extend_from_slice(heartbeat(32).as_bytes());
            let mut complete = from_buffer(oversized, decode_transaction).close();
            assert!(matches!(
                complete.next(),
                Some(Err(Error::ResponseTooLarge))
            ));
            assert!(complete.next().is_none());
        }
    }

    #[tokio::test]
    async fn an_already_ended_generation_cannot_report_a_clean_buffered_cut() {
        let mut stream = from_buffer(b"not-json\n".to_vec(), decode_transaction);
        assert!(matches!(
            stream.next_event().await,
            Some(Err(Error::Decode(_)))
        ));
        let mut buffered = stream.close();
        assert!(matches!(
            buffered.next(),
            Some(Err(Error::StreamAlreadyEnded))
        ));
        assert!(buffered.next().is_none());
    }

    #[test]
    fn buffered_debug_keeps_raw_data_private_and_invalid_cursors_explicit() {
        let mut stream = from_buffer(b"sensitive-fixture-body\n".to_vec(), decode_transaction);
        assert!(!format!("{:?}", stream.close()).contains("sensitive-fixture-body"));
        stream = from_buffer(Vec::new(), decode_transaction);
        stream.buffered.start = 1;
        let mut buffered = stream.close();
        assert!(matches!(buffered.next(), Some(Err(Error::Invariant(_)))));
        assert!(buffered.next().is_none());
    }

    // Prepare an exact finite real HTTP body prefix without assuming TCP/chunk
    // segmentation. The cut and iterator run only after these bytes are owned.
    async fn preload(stream: &mut HttpStream<TransactionStreamEvent>, bytes: usize) {
        while stream.buffered.remaining().expect("valid buffer cursor") < bytes {
            let chunk = stream
                .response
                .as_mut()
                .expect("open response")
                .chunk()
                .await
                .expect("real HTTP chunk")
                .expect("held body remains open");
            stream.buffered.bytes.extend_from_slice(&chunk);
        }
        assert_eq!(stream.buffered.remaining().expect("buffer length"), bytes);
    }

    async fn peer_closed(server: crate::test_support::HeldStream) {
        tokio::time::timeout(Duration::from_secs(5), server.closed)
            .await
            .expect("owned HTTP peer did not close")
            .expect("loopback server task");
    }

    #[tokio::test]
    async fn cancelling_a_real_http_read_then_closing_retains_partial_bytes_and_releases_the_peer()
    {
        use futures_util::FutureExt;
        for prefix in [b"".as_slice(), b"{\"type\":"] {
            let server = crate::test_support::held_stream(prefix).await;
            let client = crate::test_support::fixture_client(server.url.clone());
            let account = AccountID::new("101-001-1-001").expect("fixture account");
            let mut stream = client
                .stream_transactions(&account)
                .await
                .expect("real stream");
            tokio::time::timeout(Duration::from_secs(5), preload(&mut stream, prefix.len()))
                .await
                .expect("finite HTTP prefix");
            let mut reading = Box::pin(stream.next_event());
            assert!(reading.as_mut().now_or_never().is_none());
            drop(reading);
            let mut buffered = stream.close();
            if prefix.is_empty() {
                assert!(buffered.next().is_none());
            } else {
                assert!(
                    matches!(buffered.next(), Some(Err(Error::IncompleteStreamRecord { bytes })) if bytes == prefix.len())
                );
            }
            assert!(buffered.next().is_none());
            peer_closed(server).await;
        }
    }

    #[tokio::test]
    async fn real_http_close_releases_capacity_before_drain_and_preserves_sibling_stream_and_rest()
    {
        let owner = crate::Admission::with_exit_reserve(0).expect("admission owner");
        let mut leases = Vec::new();
        for _ in 0..18 {
            leases.push(owner.lease_stream().expect("occupied sibling lease"));
        }
        let prefix = format!("{}{}", heartbeat(41), heartbeat(42));
        let first = crate::test_support::held_stream(prefix.as_bytes()).await;
        let sibling = crate::test_support::held_stream(heartbeat(43).as_bytes()).await;
        let (rest, rest_server) = crate::test_support::server(crate::test_support::http(
            "200 OK",
            "application/json",
            r#"{"accounts":[]}"#,
        ))
        .await;
        let client = Client::builder(Environment::Practice, "fixture-secret")
            .endpoints(rest.clone(), first.url.clone())
            .admission(owner.clone())
            .build()
            .expect("client");
        let other = Client::builder(Environment::Practice, "another-fixture-secret")
            .endpoints(rest, sibling.url.clone())
            .admission(owner.clone())
            .build()
            .expect("sibling client");
        let account = AccountID::new("101-001-1-001").expect("fixture account");
        let mut stream = client
            .stream_transactions(&account)
            .await
            .expect("first real stream");
        let mut sibling_stream = other
            .stream_transactions(&account)
            .await
            .expect("sibling real stream");
        assert!(matches!(owner.lease_stream(), Err(Error::RateLimited)));
        assert_eq!(
            transaction_id(stream.next_event().await.expect("first record")),
            "41"
        );
        tokio::time::timeout(
            Duration::from_secs(5),
            preload(&mut stream, heartbeat(42).len()),
        )
        .await
        .expect("remaining finite prefix");
        let mut buffered = stream.close();
        let reused = owner
            .lease_stream()
            .expect("closed stream slot is reusable before iterator consumption");
        peer_closed(first).await;
        assert_eq!(
            transaction_id(
                sibling_stream
                    .next_event()
                    .await
                    .expect("sibling still live")
            ),
            "43"
        );
        assert!(
            client
                .list_accounts()
                .await
                .expect("REST remains available")
                .body
                .accounts
                .expect("explicit account list")
                .is_empty()
        );
        rest_server.await.expect("REST server");
        assert_eq!(
            transaction_id(buffered.next().expect("undelivered record")),
            "42"
        );
        assert!(buffered.next().is_none());
        drop(reused);
        drop(leases);
        drop(sibling_stream.close());
        peer_closed(sibling).await;
    }

    #[tokio::test]
    async fn real_http_buffered_malformed_oversized_and_unterminated_records_are_not_clean_drains()
    {
        let mut malformed = b"not-json\n".to_vec();
        malformed.extend_from_slice(heartbeat(51).as_bytes());
        let mut oversized = vec![b'x'; MAX_RECORD_BYTES + 1];
        oversized.push(b'\n');
        for (index, prefix) in [malformed, oversized, b"{\"type\":".to_vec()]
            .into_iter()
            .enumerate()
        {
            let server = crate::test_support::held_stream(&prefix).await;
            let client = crate::test_support::fixture_client(server.url.clone());
            let account = AccountID::new("101-001-1-001").expect("fixture account");
            let mut stream = client
                .stream_transactions(&account)
                .await
                .expect("real stream");
            tokio::time::timeout(Duration::from_secs(5), preload(&mut stream, prefix.len()))
                .await
                .expect("finite HTTP prefix");
            let mut buffered = stream.close();
            match (index, buffered.next()) {
                (0, Some(Err(Error::Decode(_)))) | (1, Some(Err(Error::ResponseTooLarge))) => {}
                (2, Some(Err(Error::IncompleteStreamRecord { bytes }))) => {
                    assert_eq!(bytes, prefix.len());
                }
                (_, observed) => panic!("buffered error was lost or changed: {observed:?}"),
            }
            assert!(buffered.next().is_none());
            peer_closed(server).await;
        }
    }

    #[tokio::test]
    async fn an_errored_real_http_generation_cannot_be_closed_as_a_clean_prefix() {
        let (url, server) = crate::test_support::server(crate::test_support::http(
            "200 OK",
            "application/octet-stream",
            "not-json\n",
        ))
        .await;
        let client = crate::test_support::fixture_client(url);
        let account = AccountID::new("101-001-1-001").expect("fixture account");
        let mut stream = client
            .stream_transactions(&account)
            .await
            .expect("real stream");
        assert!(matches!(
            stream.next_event().await,
            Some(Err(Error::Decode(_)))
        ));
        let mut buffered = stream.close();
        assert!(matches!(
            buffered.next(),
            Some(Err(Error::StreamAlreadyEnded))
        ));
        assert!(buffered.next().is_none());
        server.await.expect("real HTTP server");
    }

    #[tokio::test]
    async fn cancelled_stream_owner_releases_its_connection_slot() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        if let Ok(mut slots) = client.inner.admission.inner.stream_slots.lock() {
            slots.active = 1;
        }
        let lease = StreamLease {
            owner: client.inner.admission.clone(),
        };
        let task = tokio::spawn(async move {
            let _lease = lease;
            std::future::pending::<()>().await;
        });
        tokio::task::yield_now().await;
        task.abort();
        let _ = task.await;
        if let Ok(slots) = client.inner.admission.inner.stream_slots.lock() {
            assert_eq!(slots.active, 0);
        }
    }

    #[tokio::test]
    async fn a_backlog_beyond_the_local_bound_ends_the_generation() {
        // A provider burst can park many complete records in the buffer at
        // once, and while undelivered records remain no further reading — and
        // therefore no per-record check — happens. Refusing to deliver from a
        // backlog past the cap is a local, explicitly reported limit, never
        // eviction of records already accepted.
        let mut buffer = Vec::new();
        for index in 0..200_000_u32 {
            buffer.extend_from_slice(
                format!(
                    "{{\"type\":\"HEARTBEAT\",\"lastTransactionID\":\"{index}\",\
                     \"time\":\"2024-01-02T03:04:05.000000000Z\"}}\r\n"
                )
                .as_bytes(),
            );
        }
        assert!(
            buffer.len() > MAX_BACKLOG_BYTES,
            "fixture must exceed the cap"
        );
        let mut stream = HttpStream {
            response: None,
            buffered: Buffered {
                bytes: buffer,
                start: 0,
                decoder: decode_transaction,
            },
            ended: false,
            lease: None,
        };
        assert!(matches!(
            stream.next_event().await,
            Some(Err(Error::ResponseTooLarge))
        ));
        assert!(stream.next_event().await.is_none());
    }

    #[tokio::test]
    async fn a_cancelled_stream_releases_its_slot_even_after_a_poisoned_lock() {
        let client = Client::builder(Environment::Practice, "fixture").build();
        assert!(client.is_ok());
        let Ok(client) = client else { return };
        if let Ok(mut slots) = client.inner.admission.inner.stream_slots.lock() {
            slots.active = 1;
        }
        let lease = StreamLease {
            owner: client.inner.admission.clone(),
        };
        let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Ok(_held) = client.inner.admission.inner.stream_slots.lock() else {
                return;
            };
            panic!("poison the lock");
        }));
        assert!(poisoned.is_err());
        drop(lease);
        let slots = client
            .inner
            .admission
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
