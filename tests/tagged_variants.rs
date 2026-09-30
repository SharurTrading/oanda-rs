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
    let result = std::panic::catch_unwind(|| {
        round_trip::<Transaction>(json!(
            {"id":"1","time":"2024-01-02T03:04:05Z","type":"CREATE"}
        ));
    });
    assert!(
        result.is_err(),
        "a payload that does not survive verbatim must fail, not pass"
    );
}
