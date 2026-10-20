// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Decodes an `ORDER_FILL` that opens a trade, keeping the cost OANDA sends on `tradeOpened`.
//!
//! The fixture under `tests/fixtures/open-fill/` is synthetic, in the shape of the opening
//! fills of a Practice hedged-account recording from 2026-10-12. Their `tradeOpened` entries
//! carry the undocumented `guaranteedExecutionFeeHomeConversionCost`, which `TradeOpen` must
//! keep, with a stated zero told apart from absence.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]

use oanda_client::models::{OrderFillReason, OrderFillTransaction, TradeOpen, Transaction};
use oanda_client::{AccountID, TradeID, TransactionID};
use serde_json::{Value, json};

mod support;
use support::{fixture_client, http, server};

const ORDER_FILL: &str = include_str!("fixtures/open-fill/order-fill.json");

fn fixture() -> Value {
    let body: Value = serde_json::from_str(ORDER_FILL).expect("fixture parses");
    body["transaction"].clone()
}

fn decode_fill(wire: Value) -> OrderFillTransaction {
    match serde_json::from_value(wire).expect("decode 52") {
        Transaction::OrderFillTransaction(fill) => fill,
        other => panic!("52 is a fill: {other:?}"),
    }
}

fn trade_opened(fill: &OrderFillTransaction) -> &TradeOpen {
    fill.trade_opened.as_ref().expect("tradeOpened")
}

/// The decimal exactly as written on the wire, scale included.
fn wire_text(fill: &OrderFillTransaction) -> Option<String> {
    trade_opened(fill)
        .guaranteed_execution_fee_home_conversion_cost
        .map(|value| value.to_string())
}

#[test]
fn the_opening_fill_re_encodes_every_field_oanda_sent() {
    let wire = fixture();
    let decoded: Transaction = serde_json::from_value(wire.clone()).expect("decode 52");
    assert_eq!(serde_json::to_value(&decoded).expect("encode 52"), wire);
}

#[test]
fn a_stated_zero_cost_on_trade_opened_is_kept_as_zero() {
    let fill = decode_fill(fixture());
    assert_eq!(fill.reason, Some(OrderFillReason::MarketOrder));
    assert_eq!(
        trade_opened(&fill).trade_id.as_ref().map(TradeID::as_str),
        Some("52")
    );
    assert_eq!(wire_text(&fill).as_deref(), Some("0"));
}

#[test]
fn an_absent_cost_on_trade_opened_stays_absent() {
    let mut wire = fixture();
    let opened = wire["tradeOpened"]
        .as_object_mut()
        .expect("tradeOpened object");
    assert!(
        opened
            .remove("guaranteedExecutionFeeHomeConversionCost")
            .is_some(),
        "fixture carries the cost"
    );
    let fill = decode_fill(wire.clone());
    assert_eq!(wire_text(&fill), None);
    let encoded = serde_json::to_value(Transaction::OrderFillTransaction(fill)).expect("encode 52");
    assert_eq!(encoded, wire);
}

#[test]
fn a_non_zero_cost_on_trade_opened_decodes_exactly() {
    // Synthetic: no non-zero value has been recorded.
    let mut wire = fixture();
    wire["tradeOpened"]["guaranteedExecutionFeeHomeConversionCost"] = json!("-0.000012345");
    assert_eq!(
        wire_text(&decode_fill(wire.clone())).as_deref(),
        Some("-0.000012345")
    );
    wire["tradeOpened"]["guaranteedExecutionFeeHomeConversionCost"] = json!("0.10");
    assert_eq!(wire_text(&decode_fill(wire)).as_deref(), Some("0.10"));
}

#[tokio::test]
async fn transaction_details_keeps_the_trade_opened_cost() {
    let (url, task) = server(http("200 OK", "application/json", ORDER_FILL)).await;
    let client = fixture_client(url);
    let response = client
        .transaction_details(
            &AccountID::new("101-001-1234567-001").expect("account"),
            &TransactionID::new("52").expect("transaction id"),
        )
        .await
        .expect("transaction 52");
    let Some(Transaction::OrderFillTransaction(fill)) = response.body.transaction else {
        panic!("52 is a fill: {:?}", response.body.transaction);
    };
    assert_eq!(wire_text(&fill).as_deref(), Some("0"));
    task.await.expect("server");
}
