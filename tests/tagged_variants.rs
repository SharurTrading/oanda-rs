// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Deserialize and reserialize every documented tagged Order and Transaction variant.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]
use oanda_client::models::{Order, OrderRequest, Transaction};
use serde_json::{Value, json};
#[allow(
    clippy::needless_pass_by_value,
    reason = "the helper owns values to exercise serialization"
)]
fn round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(wire: Value) {
    let value: T = serde_json::from_value(wire.clone()).expect("decode concrete provider variant");
    let result = serde_json::to_value(value).expect("encode concrete provider variant");
    assert_eq!(
        result, wire,
        "the provider payload itself must survive the round trip"
    );
}
#[allow(
    clippy::too_many_lines,
    reason = "one reviewed fixture per documented tagged variant"
)]
fn orders() -> Vec<Value> {
    vec![
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS"}),
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "positionFill": "OPEN_ONLY", "tradeState": "OPEN", "type": "FIXED_PRICE"}),
    ]
}

#[allow(
    clippy::too_many_lines,
    reason = "one reviewed fixture per documented tagged variant"
)]
fn requests() -> Vec<Value> {
    vec![
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS"}),
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS"}),
    ]
}

#[allow(
    clippy::too_many_lines,
    reason = "one reviewed fixture per documented tagged variant"
)]
fn transactions() -> Vec<Value> {
    vec![
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CREATE"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CLOSE"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"REOPEN"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CLIENT_CONFIGURE"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CLIENT_CONFIGURE_REJECT"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRANSFER_FUNDS"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRANSFER_FUNDS_REJECT"}),
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET_ORDER"}),
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET_ORDER_REJECT"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "positionFill": "OPEN_ONLY", "tradeState": "OPEN", "type": "FIXED_PRICE_ORDER"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT_ORDER"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT_ORDER_REJECT"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP_ORDER"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP_ORDER_REJECT"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED_ORDER"}),
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED_ORDER_REJECT"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT_ORDER"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT_ORDER_REJECT"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS_ORDER"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS_ORDER_REJECT"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS_ORDER"}),
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS_ORDER_REJECT"}),
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS_ORDER"}),
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS_ORDER_REJECT"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_FILL"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CANCEL"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CANCEL_REJECT"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CLIENT_EXTENSIONS_MODIFY"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRADE_CLIENT_EXTENSIONS_MODIFY"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"MARGIN_CALL_ENTER"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"MARGIN_CALL_EXTEND"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"MARGIN_CALL_EXIT"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"DELAYED_TRADE_CLOSURE"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"DAILY_FINANCING"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"DIVIDEND_ADJUSTMENT"}),
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"RESET_RESETTABLE_PL"}),
    ]
}

#[test]
fn all_documented_tagged_variants_round_trip() {
    for wire in orders() {
        round_trip::<Order>(wire);
    }
    for wire in requests() {
        round_trip::<OrderRequest>(wire);
    }
    for wire in transactions() {
        round_trip::<Transaction>(wire);
    }
}

#[should_panic(expected = "must survive the round trip")]
#[test]
fn a_normalized_field_value_fails_the_round_trip() {
    round_trip::<Transaction>(json!({"id":"1","time":"2024-01-02T03:04:05Z","type":"CREATE"}));
}

#[test]
fn tagged_arms_match_the_pinned_spec() {
    // The fixtures are hand-maintained, so a union arm the generator lost
    // would simply vanish from this suite. The pinned OpenAPI spec is an
    // independent document: every OrderType and TransactionType value it
    // documents must appear as a fixture here, and every fixture must be a
    // documented value.
    let spec: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/spec/official/v20-openapi.json"
    )))
    .expect("pinned spec parses");
    let expected = |name: &str| {
        let mut values: Vec<String> = spec["definitions"][name]["enum"]
            .as_array()
            .unwrap_or_else(|| panic!("{name} enum missing from the pinned spec"))
            .iter()
            .map(|value| value.as_str().expect("enum value").to_owned())
            .collect();
        values.sort();
        values
    };
    // The website is authoritative and adds the guaranteed-stop order beyond
    // the pinned spec (docs/coverage.md records the drift); the allowance is
    // explicit so any further difference fails here.
    let mut guaranteed = expected("OrderType");
    guaranteed.push("GUARANTEED_STOP_LOSS".to_owned());
    guaranteed.sort();
    let mut transactions_expected = expected("TransactionType");
    transactions_expected.extend(
        [
            "DIVIDEND_ADJUSTMENT",
            "GUARANTEED_STOP_LOSS_ORDER",
            "GUARANTEED_STOP_LOSS_ORDER_REJECT",
        ]
        .map(str::to_owned),
    );
    transactions_expected.sort();
    let orders = fixture_kinds(&orders());
    let requests = fixture_kinds(&requests());
    let transactions = fixture_kinds(&transactions());
    // OrderRequest deliberately omits the one order type OANDA publishes no
    // request definition for; the Order set is the full documented union.
    let mut request_expected = guaranteed.clone();
    request_expected.retain(|kind| kind != "FIXED_PRICE");
    assert_eq!(orders, guaranteed);
    assert_eq!(requests, request_expected);
    assert_eq!(transactions, transactions_expected);
}

/// The actual discriminator values exercised by the round-trip test.
fn fixture_kinds(fixtures: &[Value]) -> Vec<String> {
    let count = fixtures.len();
    let mut kinds: Vec<_> = fixtures
        .iter()
        .map(|wire| {
            wire["type"]
                .as_str()
                .expect("fixture discriminator")
                .to_owned()
        })
        .collect();
    kinds.sort();
    kinds.dedup();
    assert_eq!(
        kinds.len(),
        count,
        "every fixture must cover a distinct variant"
    );
    kinds
}
