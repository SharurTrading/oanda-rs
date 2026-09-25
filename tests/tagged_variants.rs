//! Deserialize and reserialize every documented tagged Order and Transaction variant.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use oanda_client::models::{Order, OrderRequest, Transaction};
use serde_json::{Value, json};
fn round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(wire: Value, kind: &str) {
    let value: T = serde_json::from_value(wire).expect("decode concrete provider variant");
    let result = serde_json::to_value(value).expect("encode concrete provider variant");
    assert_eq!(result.get("type").and_then(Value::as_str), Some(kind));
}
#[test]
fn all_documented_tagged_variants_round_trip() {
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET"}),
        "MARKET",
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT"}),
        "LIMIT",
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP"}),
        "STOP",
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED"}),
        "MARKET_IF_TOUCHED",
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT"}),
        "TAKE_PROFIT",
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS"}),
        "STOP_LOSS",
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS"}),
        "GUARANTEED_STOP_LOSS",
    );
    round_trip::<Order>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS"}),
        "TRAILING_STOP_LOSS",
    );
    round_trip::<Order>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "positionFill": "OPEN_ONLY", "tradeState": "X", "type": "FIXED_PRICE"}),
        "FIXED_PRICE",
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET"}),
        "MARKET",
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT"}),
        "LIMIT",
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP"}),
        "STOP",
    );
    round_trip::<OrderRequest>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED"}),
        "MARKET_IF_TOUCHED",
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT"}),
        "TAKE_PROFIT",
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS"}),
        "STOP_LOSS",
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS"}),
        "GUARANTEED_STOP_LOSS",
    );
    round_trip::<OrderRequest>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS"}),
        "TRAILING_STOP_LOSS",
    );
    round_trip::<Transaction>(json!({"type": "CREATE"}), "CREATE");
    round_trip::<Transaction>(json!({"type": "CLOSE"}), "CLOSE");
    round_trip::<Transaction>(json!({"type": "REOPEN"}), "REOPEN");
    round_trip::<Transaction>(json!({"type": "CLIENT_CONFIGURE"}), "CLIENT_CONFIGURE");
    round_trip::<Transaction>(
        json!({"type": "CLIENT_CONFIGURE_REJECT"}),
        "CLIENT_CONFIGURE_REJECT",
    );
    round_trip::<Transaction>(json!({"type": "TRANSFER_FUNDS"}), "TRANSFER_FUNDS");
    round_trip::<Transaction>(
        json!({"type": "TRANSFER_FUNDS_REJECT"}),
        "TRANSFER_FUNDS_REJECT",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET_ORDER"}),
        "MARKET_ORDER",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "type": "MARKET_ORDER_REJECT"}),
        "MARKET_ORDER_REJECT",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "positionFill": "OPEN_ONLY", "tradeState": "X", "type": "FIXED_PRICE_ORDER"}),
        "FIXED_PRICE_ORDER",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT_ORDER"}),
        "LIMIT_ORDER",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "LIMIT_ORDER_REJECT"}),
        "LIMIT_ORDER_REJECT",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP_ORDER"}),
        "STOP_ORDER",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "STOP_ORDER_REJECT"}),
        "STOP_ORDER_REJECT",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED_ORDER"}),
        "MARKET_IF_TOUCHED_ORDER",
    );
    round_trip::<Transaction>(
        json!({"instrument": "EUR_USD", "units": "1", "price": "1", "timeInForce": "GTC", "positionFill": "OPEN_ONLY", "triggerCondition": "DEFAULT", "type": "MARKET_IF_TOUCHED_ORDER_REJECT"}),
        "MARKET_IF_TOUCHED_ORDER_REJECT",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT_ORDER"}),
        "TAKE_PROFIT_ORDER",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TAKE_PROFIT_ORDER_REJECT"}),
        "TAKE_PROFIT_ORDER_REJECT",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS_ORDER"}),
        "STOP_LOSS_ORDER",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "STOP_LOSS_ORDER_REJECT"}),
        "STOP_LOSS_ORDER_REJECT",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS_ORDER"}),
        "GUARANTEED_STOP_LOSS_ORDER",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "price": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "GUARANTEED_STOP_LOSS_ORDER_REJECT"}),
        "GUARANTEED_STOP_LOSS_ORDER_REJECT",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS_ORDER"}),
        "TRAILING_STOP_LOSS_ORDER",
    );
    round_trip::<Transaction>(
        json!({"tradeID": "1", "distance": "1", "timeInForce": "GTC", "triggerCondition": "DEFAULT", "type": "TRAILING_STOP_LOSS_ORDER_REJECT"}),
        "TRAILING_STOP_LOSS_ORDER_REJECT",
    );
    round_trip::<Transaction>(json!({"type": "ORDER_FILL"}), "ORDER_FILL");
    round_trip::<Transaction>(json!({"type": "ORDER_CANCEL"}), "ORDER_CANCEL");
    round_trip::<Transaction>(
        json!({"type": "ORDER_CANCEL_REJECT"}),
        "ORDER_CANCEL_REJECT",
    );
    round_trip::<Transaction>(
        json!({"type": "ORDER_CLIENT_EXTENSIONS_MODIFY"}),
        "ORDER_CLIENT_EXTENSIONS_MODIFY",
    );
    round_trip::<Transaction>(
        json!({"type": "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT"}),
        "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
    );
    round_trip::<Transaction>(
        json!({"type": "TRADE_CLIENT_EXTENSIONS_MODIFY"}),
        "TRADE_CLIENT_EXTENSIONS_MODIFY",
    );
    round_trip::<Transaction>(
        json!({"type": "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT"}),
        "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
    );
    round_trip::<Transaction>(json!({"type": "MARGIN_CALL_ENTER"}), "MARGIN_CALL_ENTER");
    round_trip::<Transaction>(json!({"type": "MARGIN_CALL_EXTEND"}), "MARGIN_CALL_EXTEND");
    round_trip::<Transaction>(json!({"type": "MARGIN_CALL_EXIT"}), "MARGIN_CALL_EXIT");
    round_trip::<Transaction>(
        json!({"type": "DELAYED_TRADE_CLOSURE"}),
        "DELAYED_TRADE_CLOSURE",
    );
    round_trip::<Transaction>(json!({"type": "DAILY_FINANCING"}), "DAILY_FINANCING");
    round_trip::<Transaction>(
        json!({"type": "DIVIDEND_ADJUSTMENT"}),
        "DIVIDEND_ADJUSTMENT",
    );
    round_trip::<Transaction>(
        json!({"type": "RESET_RESETTABLE_PL"}),
        "RESET_RESETTABLE_PL",
    );
}
