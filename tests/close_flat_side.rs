// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Decodes the transactions OANDA writes for a close of a flat hedged side through every read.
//!
//! The fixtures under `tests/fixtures/close-flat-side/` are synthetic bodies of
//! `GET /v3/accounts/{accountID}/transactions/{id}` in the shape observed on a Practice hedged
//! account on 2026-10-16. Transaction 44 is the `MARKET_ORDER_REJECT` OANDA writes for a
//! `close_position` of a long side that is already flat: it carries no `units`, so every read
//! covering it must decode it with the units absent rather than refuse the whole reply.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use oanda_client::models::{
    MarketOrderReason, MarketOrderRejectTransaction, OrderCancelReason, OrderFillReason,
    OrderPositionFill, TimeInForce, Transaction, TransactionRejectReason,
};
use oanda_client::position::ClosePositionBody;
use oanda_client::transaction::{TransactionsByIdRangeQuery, TransactionsSinceIdQuery};
use oanda_client::{
    AccountID, Client, Environment, InstrumentName, OperationError, OrderID, Supplied,
    TransactionID, TransactionStreamEvent,
};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use std::{str::FromStr, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use url::Url;

const MARKET_ORDER: &str = include_str!("fixtures/close-flat-side/market-order.json");
const ORDER_FILL: &str = include_str!("fixtures/close-flat-side/order-fill.json");
const ORDER_CANCEL: &str = include_str!("fixtures/close-flat-side/order-cancel.json");
const MARKET_ORDER_REJECT: &str = include_str!("fixtures/close-flat-side/market-order-reject.json");

/// The `transaction` object of one fixture body.
fn fixture(body: &str) -> Value {
    let body: Value = serde_json::from_str(body).expect("fixture parses");
    body["transaction"].clone()
}

fn fixture_batch() -> Vec<Value> {
    [MARKET_ORDER, ORDER_FILL, ORDER_CANCEL, MARKET_ORDER_REJECT]
        .into_iter()
        .map(fixture)
        .collect()
}

/// Serve one canned response on loopback and return the request it received.
async fn server(response: Vec<u8>) -> (Url, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let mut bytes = Vec::new();
        let mut buf = [0_u8; 4096];
        loop {
            let read = socket.read(&mut buf).await.expect("read");
            if read == 0 {
                break;
            }
            bytes.extend_from_slice(&buf[..read]);
            if let Some(header_end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                let header_end = header_end + 4;
                let header = String::from_utf8_lossy(&bytes[..header_end]);
                let length = header
                    .lines()
                    .find_map(|l| {
                        l.strip_prefix("content-length: ")
                            .or_else(|| l.strip_prefix("Content-Length: "))
                    })
                    .and_then(|v| v.trim().parse::<usize>().ok())
                    .unwrap_or(0);
                if bytes.len() >= header_end + length {
                    break;
                }
            }
        }
        socket.write_all(&response).await.expect("write");
        socket.flush().await.expect("flush");
        String::from_utf8_lossy(&bytes).into_owned()
    });
    (url, task)
}

fn http(status: &str, content_type: &str, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

fn fixture_client(url: Url) -> Client {
    Client::builder(Environment::Practice, "fixture-secret")
        .endpoints(url.clone(), url)
        .timeout(Duration::from_secs(2))
        .build()
        .expect("fixture client")
}

fn account() -> AccountID {
    AccountID::new("101-001-1234567-001").expect("account")
}

fn transaction_id(id: &str) -> TransactionID {
    TransactionID::new(id).expect("transaction id")
}

/// Asserts the reject of the flat long side, field by field from the fixture.
fn assert_flat_side_reject(reject: &MarketOrderRejectTransaction) {
    assert_eq!(reject.id.as_ref().map(TransactionID::as_str), Some("44"));
    assert_eq!(reject.instrument.as_str(), "EUR_USD");
    assert_eq!(reject.units, None);
    assert_eq!(reject.time_in_force, TimeInForce::Fok);
    assert_eq!(reject.position_fill, OrderPositionFill::ReduceOnly);
    assert_eq!(reject.reason, Some(MarketOrderReason::PositionCloseout));
    assert_eq!(
        reject.reject_reason,
        Some(TransactionRejectReason::CloseoutPositionDoesntExist)
    );
    let closeout = reject
        .long_position_closeout
        .as_ref()
        .expect("long closeout");
    assert_eq!(
        closeout.instrument.as_ref().map(InstrumentName::as_str),
        Some("EUR_USD")
    );
    assert_eq!(closeout.units.as_deref(), Some("ALL"));
    assert_eq!(reject.short_position_closeout, None);
}

/// Asserts the four fixture transactions decoded as the variants OANDA writes.
fn assert_fixture_batch(transactions: &[Transaction]) {
    let [create, fill, cancel, reject] = transactions else {
        panic!("expected the four fixture transactions, got {transactions:?}");
    };
    let Transaction::MarketOrderTransaction(create) = create else {
        panic!("41 is a market order: {create:?}");
    };
    assert_eq!(create.units, Decimal::from_str("-3").expect("decimal"));
    assert_eq!(create.reason, Some(MarketOrderReason::PositionCloseout));
    let Transaction::OrderFillTransaction(fill) = fill else {
        panic!("42 is a fill: {fill:?}");
    };
    assert_eq!(fill.order_id.as_ref().map(OrderID::as_str), Some("41"));
    assert_eq!(
        fill.reason,
        Some(OrderFillReason::MarketOrderPositionCloseout)
    );
    assert_eq!(
        fill.pl,
        Some(Decimal::from_str("-0.0003").expect("decimal"))
    );
    let Transaction::OrderCancelTransaction(cancel) = cancel else {
        panic!("43 is a cancel: {cancel:?}");
    };
    assert_eq!(cancel.reason, Some(OrderCancelReason::ClientRequest));
    let Transaction::MarketOrderRejectTransaction(reject) = reject else {
        panic!("44 is a market order reject: {reject:?}");
    };
    assert_flat_side_reject(reject);
}

#[test]
fn the_flat_side_reject_re_encodes_without_inventing_units() {
    let wire = fixture(MARKET_ORDER_REJECT);
    let decoded: Transaction = serde_json::from_value(wire.clone()).expect("decode 44");
    assert_eq!(serde_json::to_value(&decoded).expect("encode 44"), wire);
}

#[tokio::test]
async fn transaction_details_decodes_each_fixture_transaction() {
    let mut transactions = Vec::new();
    for (body, id) in [
        (MARKET_ORDER, "41"),
        (ORDER_FILL, "42"),
        (ORDER_CANCEL, "43"),
        (MARKET_ORDER_REJECT, "44"),
    ] {
        let (url, task) = server(http("200 OK", "application/json", body)).await;
        let client = fixture_client(url);
        let response = client
            .transaction_details(&account(), &transaction_id(id))
            .await
            .unwrap_or_else(|error| panic!("transaction {id}: {error:?}"));
        assert_eq!(
            response
                .body
                .last_transaction_id
                .as_ref()
                .map(TransactionID::as_str),
            Some("44")
        );
        transactions.push(response.body.transaction.expect("transaction"));
        task.await.expect("server");
    }
    assert_fixture_batch(&transactions);
}

#[tokio::test]
async fn transactions_since_id_decodes_a_page_holding_the_flat_side_reject() {
    let body = json!({"transactions": fixture_batch(), "lastTransactionID": "44"}).to_string();
    let (url, task) = server(http("200 OK", "application/json", &body)).await;
    let client = fixture_client(url);
    let query = TransactionsSinceIdQuery {
        id: transaction_id("40"),
        r#type: None,
    };
    let response = client
        .transactions_since_id(&account(), &query)
        .await
        .expect("since-id page decodes");
    assert_fixture_batch(&response.body.transactions.expect("transactions"));
    task.await.expect("server");
}

#[tokio::test]
async fn transactions_by_id_range_decodes_a_range_holding_the_flat_side_reject() {
    let body = json!({"transactions": fixture_batch(), "lastTransactionID": "44"}).to_string();
    let (url, task) = server(http("200 OK", "application/json", &body)).await;
    let client = fixture_client(url);
    let query = TransactionsByIdRangeQuery {
        from: transaction_id("41"),
        to: transaction_id("44"),
        r#type: None,
    };
    let response = client
        .transactions_by_id_range(&account(), &query)
        .await
        .expect("range decodes");
    assert_fixture_batch(&response.body.transactions.expect("transactions"));
    task.await.expect("server");
}

#[tokio::test]
async fn the_transaction_stream_delivers_the_flat_side_reject() {
    let body = fixture_batch()
        .iter()
        .map(|transaction| transaction.to_string() + "\n")
        .collect::<String>();
    let (url, task) = server(http("200 OK", "application/octet-stream", &body)).await;
    let client = fixture_client(url);
    let mut stream = client
        .stream_transactions(&account())
        .await
        .expect("stream opens");
    let mut transactions = Vec::new();
    for _ in 0..4 {
        match stream.next_event().await {
            Some(Ok(TransactionStreamEvent::Transaction(transaction))) => {
                transactions.push(*transaction);
            }
            other => panic!("expected a fixture transaction, got {other:?}"),
        }
    }
    assert_fixture_batch(&transactions);
    task.await.expect("server");
}

#[tokio::test]
async fn close_position_rejection_decodes_the_flat_side_reject() {
    // OANDA documents the same ClosePositionRejection body for 400 and 404; the envelope is
    // built from that shape around the fixture reject, and the message is the documented
    // description of CLOSEOUT_POSITION_DOESNT_EXIST.
    for status in ["400 Bad Request", "404 Not Found"] {
        let body = json!({
            "longOrderRejectTransaction": fixture(MARKET_ORDER_REJECT),
            "relatedTransactionIDs": ["44"],
            "lastTransactionID": "44",
            "errorCode": "CLOSEOUT_POSITION_DOESNT_EXIST",
            "errorMessage": "The Position requested to be closed out does not exist",
        })
        .to_string();
        let (url, task) = server(http(status, "application/json", &body)).await;
        let client = fixture_client(url);
        let request = ClosePositionBody {
            long_units: Some("ALL".to_owned()),
            long_client_extensions: None,
            short_units: Some("NONE".to_owned()),
            short_client_extensions: None,
        };
        let error = client
            .close_position(
                &account(),
                &InstrumentName::new("EUR_USD").expect("instrument"),
                &request,
            )
            .await
            .expect_err("rejected");
        let OperationError::Rejected {
            status: code_status,
            code,
            message,
            body,
        } = error
        else {
            panic!("{status}: expected a definitive rejection, got {error:?}");
        };
        assert_eq!(
            code_status,
            if status.starts_with("400") { 400 } else { 404 }
        );
        assert_eq!(code.as_deref(), Some("CLOSEOUT_POSITION_DOESNT_EXIST"));
        assert_eq!(
            message.as_deref(),
            Some("The Position requested to be closed out does not exist")
        );
        let Supplied::Decoded(rejection) = body else {
            panic!("{status}: the rejection body must decode, got {body:?}");
        };
        assert_flat_side_reject(
            rejection
                .long_order_reject_transaction
                .as_ref()
                .expect("long reject"),
        );
        assert_eq!(rejection.short_order_reject_transaction, None);
        assert_eq!(
            rejection
                .related_transaction_ids
                .as_deref()
                .map(|ids| ids.iter().map(TransactionID::as_str).collect::<Vec<_>>()),
            Some(vec!["44"])
        );
        task.await.expect("server");
    }
}
