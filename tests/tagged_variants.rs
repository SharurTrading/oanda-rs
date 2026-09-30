//! Deserialize and reserialize every documented tagged Order and Transaction variant.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::needless_pass_by_value
)]
use oanda_client::models::{Order, OrderRequest, Transaction};
use serde_json::{Value, json};
fn round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(wire: Value) {
    let value: T = serde_json::from_value(wire.clone()).expect("decode concrete provider variant");
    let result = serde_json::to_value(value).expect("encode concrete provider variant");
    assert_eq!(
        result, wire,
        "the provider payload itself must survive the round trip"
    );
}
#[test]
fn all_documented_tagged_variants_round_trip() {
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET"}),
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT"}),
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP"}),
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED"}),
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT"}),
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS"}),
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS"}),
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS"}),
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "positionFill": "OPEN_ONLY", "tradeState": "X", "type": "FIXED_PRICE"}),
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET"}),
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT"}),
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP"}),
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED"}),
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT"}),
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS"}),
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS"}),
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CREATE"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CLOSE"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"REOPEN"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CLIENT_CONFIGURE"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"CLIENT_CONFIGURE_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRANSFER_FUNDS"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRANSFER_FUNDS_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "positionFill": "OPEN_ONLY", "tradeState": "X", "type": "FIXED_PRICE_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS_ORDER"}),
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS_ORDER_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_FILL"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CANCEL"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CANCEL_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CLIENT_EXTENSIONS_MODIFY"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRADE_CLIENT_EXTENSIONS_MODIFY"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"MARGIN_CALL_ENTER"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"MARGIN_CALL_EXTEND"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"MARGIN_CALL_EXIT"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"DELAYED_TRADE_CLOSURE"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"DAILY_FINANCING"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"DIVIDEND_ADJUSTMENT"}),
    );
    round_trip::<Transaction>(
        json!({"id":"1","time":"2024-01-02T03:04:05.123456789Z","userID":7,"accountID":"101-001-1-001","batchID":"2","requestID":"3","type":"RESET_RESETTABLE_PL"}),
    );
}

#[test]
fn a_normalized_field_value_fails_the_round_trip() {
    // A time without nanoseconds re-encodes normalized, so the reserialized
    // payload no longer equals what the provider sent. The old assertion,
    // which only checked the serializer-written type tag, let this pass.
    assert_panics_with("must survive the round trip", || {
        round_trip::<Transaction>(json!(
            {"id":"1","time":"2024-01-02T03:04:05Z","type":"CREATE"}
        ));
    });
}

/// Assert the closure panics with the expected assertion message, quietly.
///
/// `catch_unwind` alone leaves the default panic hook installed, so a green
/// run prints the caught panic (and a backtrace under `RUST_BACKTRACE=1`), and
/// `is_err()` alone would accept any unrelated panic. The hook is silenced for
/// the duration and the payload itself is checked.
fn assert_panics_with(fragment: &str, body: impl FnOnce() + std::panic::UnwindSafe) {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(body);
    std::panic::set_hook(previous);
    let panic = result.expect_err("expected this assertion to fail");
    let text = panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&str>().copied())
        .unwrap_or_default();
    assert!(
        text.contains(fragment),
        "panic message {text:?} does not carry {fragment:?}"
    );
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
    let source = include_str!("tagged_variants.rs");
    let orders = fixture_kinds(source, "round_trip::<Order>(");
    let requests = fixture_kinds(source, "round_trip::<OrderRequest>(");
    let transactions = fixture_kinds(source, "round_trip::<Transaction>(");
    // OrderRequest deliberately omits the one order type OANDA publishes no
    // request definition for; the Order set is the full documented union.
    let mut request_expected = guaranteed.clone();
    request_expected.retain(|kind| kind != "FIXED_PRICE");
    assert_eq!(orders, guaranteed);
    assert_eq!(requests, request_expected);
    assert_eq!(transactions, transactions_expected);
}

/// The `type` discriminator of every fixture after each marker, sorted.
fn fixture_kinds(source: &str, marker: &str) -> Vec<String> {
    let mut kinds = Vec::new();
    let mut rest = source;
    while let Some(index) = rest.find(marker) {
        let window = &rest[index..(index + 700).min(rest.len())];
        if let Some(tag) = window.find("\"type\":") {
            let after = &window[tag + "\"type\":".len()..];
            let after = after.trim_start_matches(' ');
            if let Some(rest) = after.strip_prefix('"')
                && let Some(end) = rest.find('"')
            {
                kinds.push(rest[..end].to_owned());
            }
        }
        rest = &rest[index + marker.len()..];
    }
    kinds.sort();
    kinds.dedup();
    kinds
}
