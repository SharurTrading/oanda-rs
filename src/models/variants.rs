//! Tagged provider unions for orders and transactions.
use super::*;
use serde::{Deserialize, Serialize};

/// Documented Order variants identified by OANDA’s `type` field.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Order {
    /// `MARKET` provider variant.
    MarketOrder(MarketOrder),
    /// `LIMIT` provider variant.
    LimitOrder(LimitOrder),
    /// `STOP` provider variant.
    StopOrder(StopOrder),
    /// `MARKET_IF_TOUCHED` provider variant.
    MarketIfTouchedOrder(MarketIfTouchedOrder),
    /// `TAKE_PROFIT` provider variant.
    TakeProfitOrder(TakeProfitOrder),
    /// `STOP_LOSS` provider variant.
    StopLossOrder(StopLossOrder),
    /// `GUARANTEED_STOP_LOSS` provider variant.
    GuaranteedStopLossOrder(GuaranteedStopLossOrder),
    /// `TRAILING_STOP_LOSS` provider variant.
    TrailingStopLossOrder(TrailingStopLossOrder),
    /// `FIXED_PRICE` provider variant.
    FixedPriceOrder(FixedPriceOrder),
    /// Future provider variant with its bounded raw value.
    Unknown {
        /// Provider discriminator.
        kind: String,
        /// Bounded provider object.
        raw: serde_json::Value,
    },
}

impl<'de> Deserialize<'de> for Order {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let kind = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| serde::de::Error::custom("missing OANDA type discriminator"))?;
        match kind {
            "MARKET" => serde_json::from_value(value)
                .map(Self::MarketOrder)
                .map_err(serde::de::Error::custom),
            "LIMIT" => serde_json::from_value(value)
                .map(Self::LimitOrder)
                .map_err(serde::de::Error::custom),
            "STOP" => serde_json::from_value(value)
                .map(Self::StopOrder)
                .map_err(serde::de::Error::custom),
            "MARKET_IF_TOUCHED" => serde_json::from_value(value)
                .map(Self::MarketIfTouchedOrder)
                .map_err(serde::de::Error::custom),
            "TAKE_PROFIT" => serde_json::from_value(value)
                .map(Self::TakeProfitOrder)
                .map_err(serde::de::Error::custom),
            "STOP_LOSS" => serde_json::from_value(value)
                .map(Self::StopLossOrder)
                .map_err(serde::de::Error::custom),
            "GUARANTEED_STOP_LOSS" => serde_json::from_value(value)
                .map(Self::GuaranteedStopLossOrder)
                .map_err(serde::de::Error::custom),
            "TRAILING_STOP_LOSS" => serde_json::from_value(value)
                .map(Self::TrailingStopLossOrder)
                .map_err(serde::de::Error::custom),
            "FIXED_PRICE" => serde_json::from_value(value)
                .map(Self::FixedPriceOrder)
                .map_err(serde::de::Error::custom),
            other => Ok(Self::Unknown {
                kind: other.to_owned(),
                raw: value,
            }),
        }
    }
}

impl Serialize for Order {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (kind, mut value) = match self {
            Self::MarketOrder(body) => (
                "MARKET",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::LimitOrder(body) => (
                "LIMIT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopOrder(body) => (
                "STOP",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarketIfTouchedOrder(body) => (
                "MARKET_IF_TOUCHED",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TakeProfitOrder(body) => (
                "TAKE_PROFIT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopLossOrder(body) => (
                "STOP_LOSS",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::GuaranteedStopLossOrder(body) => (
                "GUARANTEED_STOP_LOSS",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TrailingStopLossOrder(body) => (
                "TRAILING_STOP_LOSS",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::FixedPriceOrder(body) => (
                "FIXED_PRICE",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::Unknown { kind, raw } => (kind.as_str(), raw.clone()),
        };
        let object = value
            .as_object_mut()
            .ok_or_else(|| serde::ser::Error::custom("OANDA variant must be an object"))?;
        object.insert(
            "type".to_owned(),
            serde_json::Value::String(kind.to_owned()),
        );
        value.serialize(serializer)
    }
}

/// Documented OrderRequest variants identified by OANDA’s `type` field.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum OrderRequest {
    /// `MARKET` provider variant.
    MarketOrderRequest(MarketOrderRequest),
    /// `LIMIT` provider variant.
    LimitOrderRequest(LimitOrderRequest),
    /// `STOP` provider variant.
    StopOrderRequest(StopOrderRequest),
    /// `MARKET_IF_TOUCHED` provider variant.
    MarketIfTouchedOrderRequest(MarketIfTouchedOrderRequest),
    /// `TAKE_PROFIT` provider variant.
    TakeProfitOrderRequest(TakeProfitOrderRequest),
    /// `STOP_LOSS` provider variant.
    StopLossOrderRequest(StopLossOrderRequest),
    /// `GUARANTEED_STOP_LOSS` provider variant.
    GuaranteedStopLossOrderRequest(GuaranteedStopLossOrderRequest),
    /// `TRAILING_STOP_LOSS` provider variant.
    TrailingStopLossOrderRequest(TrailingStopLossOrderRequest),
}

impl<'de> Deserialize<'de> for OrderRequest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let kind = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| serde::de::Error::custom("missing OANDA type discriminator"))?;
        match kind {
            "MARKET" => serde_json::from_value(value)
                .map(Self::MarketOrderRequest)
                .map_err(serde::de::Error::custom),
            "LIMIT" => serde_json::from_value(value)
                .map(Self::LimitOrderRequest)
                .map_err(serde::de::Error::custom),
            "STOP" => serde_json::from_value(value)
                .map(Self::StopOrderRequest)
                .map_err(serde::de::Error::custom),
            "MARKET_IF_TOUCHED" => serde_json::from_value(value)
                .map(Self::MarketIfTouchedOrderRequest)
                .map_err(serde::de::Error::custom),
            "TAKE_PROFIT" => serde_json::from_value(value)
                .map(Self::TakeProfitOrderRequest)
                .map_err(serde::de::Error::custom),
            "STOP_LOSS" => serde_json::from_value(value)
                .map(Self::StopLossOrderRequest)
                .map_err(serde::de::Error::custom),
            "GUARANTEED_STOP_LOSS" => serde_json::from_value(value)
                .map(Self::GuaranteedStopLossOrderRequest)
                .map_err(serde::de::Error::custom),
            "TRAILING_STOP_LOSS" => serde_json::from_value(value)
                .map(Self::TrailingStopLossOrderRequest)
                .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom("unsupported OANDA request type")),
        }
    }
}

impl Serialize for OrderRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (kind, mut value) = match self {
            Self::MarketOrderRequest(body) => (
                "MARKET",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::LimitOrderRequest(body) => (
                "LIMIT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopOrderRequest(body) => (
                "STOP",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarketIfTouchedOrderRequest(body) => (
                "MARKET_IF_TOUCHED",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TakeProfitOrderRequest(body) => (
                "TAKE_PROFIT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopLossOrderRequest(body) => (
                "STOP_LOSS",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::GuaranteedStopLossOrderRequest(body) => (
                "GUARANTEED_STOP_LOSS",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TrailingStopLossOrderRequest(body) => (
                "TRAILING_STOP_LOSS",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
        };
        let object = value
            .as_object_mut()
            .ok_or_else(|| serde::ser::Error::custom("OANDA variant must be an object"))?;
        object.insert(
            "type".to_owned(),
            serde_json::Value::String(kind.to_owned()),
        );
        value.serialize(serializer)
    }
}

/// Documented Transaction variants identified by OANDA’s `type` field.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Transaction {
    /// `CREATE` provider variant.
    CreateTransaction(CreateTransaction),
    /// `CLOSE` provider variant.
    CloseTransaction(CloseTransaction),
    /// `REOPEN` provider variant.
    ReopenTransaction(ReopenTransaction),
    /// `CLIENT_CONFIGURE` provider variant.
    ClientConfigureTransaction(ClientConfigureTransaction),
    /// `CLIENT_CONFIGURE_REJECT` provider variant.
    ClientConfigureRejectTransaction(ClientConfigureRejectTransaction),
    /// `TRANSFER_FUNDS` provider variant.
    TransferFundsTransaction(TransferFundsTransaction),
    /// `TRANSFER_FUNDS_REJECT` provider variant.
    TransferFundsRejectTransaction(TransferFundsRejectTransaction),
    /// `MARKET_ORDER` provider variant.
    MarketOrderTransaction(MarketOrderTransaction),
    /// `MARKET_ORDER_REJECT` provider variant.
    MarketOrderRejectTransaction(MarketOrderRejectTransaction),
    /// `FIXED_PRICE_ORDER` provider variant.
    FixedPriceOrderTransaction(FixedPriceOrderTransaction),
    /// `LIMIT_ORDER` provider variant.
    LimitOrderTransaction(LimitOrderTransaction),
    /// `LIMIT_ORDER_REJECT` provider variant.
    LimitOrderRejectTransaction(LimitOrderRejectTransaction),
    /// `STOP_ORDER` provider variant.
    StopOrderTransaction(StopOrderTransaction),
    /// `STOP_ORDER_REJECT` provider variant.
    StopOrderRejectTransaction(StopOrderRejectTransaction),
    /// `MARKET_IF_TOUCHED_ORDER` provider variant.
    MarketIfTouchedOrderTransaction(MarketIfTouchedOrderTransaction),
    /// `MARKET_IF_TOUCHED_ORDER_REJECT` provider variant.
    MarketIfTouchedOrderRejectTransaction(MarketIfTouchedOrderRejectTransaction),
    /// `TAKE_PROFIT_ORDER` provider variant.
    TakeProfitOrderTransaction(TakeProfitOrderTransaction),
    /// `TAKE_PROFIT_ORDER_REJECT` provider variant.
    TakeProfitOrderRejectTransaction(TakeProfitOrderRejectTransaction),
    /// `STOP_LOSS_ORDER` provider variant.
    StopLossOrderTransaction(StopLossOrderTransaction),
    /// `STOP_LOSS_ORDER_REJECT` provider variant.
    StopLossOrderRejectTransaction(StopLossOrderRejectTransaction),
    /// `GUARANTEED_STOP_LOSS_ORDER` provider variant.
    GuaranteedStopLossOrderTransaction(GuaranteedStopLossOrderTransaction),
    /// `GUARANTEED_STOP_LOSS_ORDER_REJECT` provider variant.
    GuaranteedStopLossOrderRejectTransaction(GuaranteedStopLossOrderRejectTransaction),
    /// `TRAILING_STOP_LOSS_ORDER` provider variant.
    TrailingStopLossOrderTransaction(TrailingStopLossOrderTransaction),
    /// `TRAILING_STOP_LOSS_ORDER_REJECT` provider variant.
    TrailingStopLossOrderRejectTransaction(TrailingStopLossOrderRejectTransaction),
    /// `ORDER_FILL` provider variant.
    OrderFillTransaction(OrderFillTransaction),
    /// `ORDER_CANCEL` provider variant.
    OrderCancelTransaction(OrderCancelTransaction),
    /// `ORDER_CANCEL_REJECT` provider variant.
    OrderCancelRejectTransaction(OrderCancelRejectTransaction),
    /// `ORDER_CLIENT_EXTENSIONS_MODIFY` provider variant.
    OrderClientExtensionsModifyTransaction(OrderClientExtensionsModifyTransaction),
    /// `ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT` provider variant.
    OrderClientExtensionsModifyRejectTransaction(OrderClientExtensionsModifyRejectTransaction),
    /// `TRADE_CLIENT_EXTENSIONS_MODIFY` provider variant.
    TradeClientExtensionsModifyTransaction(TradeClientExtensionsModifyTransaction),
    /// `TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT` provider variant.
    TradeClientExtensionsModifyRejectTransaction(TradeClientExtensionsModifyRejectTransaction),
    /// `MARGIN_CALL_ENTER` provider variant.
    MarginCallEnterTransaction(MarginCallEnterTransaction),
    /// `MARGIN_CALL_EXTEND` provider variant.
    MarginCallExtendTransaction(MarginCallExtendTransaction),
    /// `MARGIN_CALL_EXIT` provider variant.
    MarginCallExitTransaction(MarginCallExitTransaction),
    /// `DELAYED_TRADE_CLOSURE` provider variant.
    DelayedTradeClosureTransaction(DelayedTradeClosureTransaction),
    /// `DAILY_FINANCING` provider variant.
    DailyFinancingTransaction(DailyFinancingTransaction),
    /// `DIVIDEND_ADJUSTMENT` provider variant.
    DividendAdjustmentTransaction(DividendAdjustmentTransaction),
    /// `RESET_RESETTABLE_PL` provider variant.
    ResetResettablePLTransaction(ResetResettablePLTransaction),
    /// Future provider variant with its bounded raw value.
    Unknown {
        /// Provider discriminator.
        kind: String,
        /// Bounded provider object.
        raw: serde_json::Value,
    },
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let kind = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| serde::de::Error::custom("missing OANDA type discriminator"))?;
        match kind {
            "CREATE" => serde_json::from_value(value)
                .map(Self::CreateTransaction)
                .map_err(serde::de::Error::custom),
            "CLOSE" => serde_json::from_value(value)
                .map(Self::CloseTransaction)
                .map_err(serde::de::Error::custom),
            "REOPEN" => serde_json::from_value(value)
                .map(Self::ReopenTransaction)
                .map_err(serde::de::Error::custom),
            "CLIENT_CONFIGURE" => serde_json::from_value(value)
                .map(Self::ClientConfigureTransaction)
                .map_err(serde::de::Error::custom),
            "CLIENT_CONFIGURE_REJECT" => serde_json::from_value(value)
                .map(Self::ClientConfigureRejectTransaction)
                .map_err(serde::de::Error::custom),
            "TRANSFER_FUNDS" => serde_json::from_value(value)
                .map(Self::TransferFundsTransaction)
                .map_err(serde::de::Error::custom),
            "TRANSFER_FUNDS_REJECT" => serde_json::from_value(value)
                .map(Self::TransferFundsRejectTransaction)
                .map_err(serde::de::Error::custom),
            "MARKET_ORDER" => serde_json::from_value(value)
                .map(Self::MarketOrderTransaction)
                .map_err(serde::de::Error::custom),
            "MARKET_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::MarketOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "FIXED_PRICE_ORDER" => serde_json::from_value(value)
                .map(Self::FixedPriceOrderTransaction)
                .map_err(serde::de::Error::custom),
            "LIMIT_ORDER" => serde_json::from_value(value)
                .map(Self::LimitOrderTransaction)
                .map_err(serde::de::Error::custom),
            "LIMIT_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::LimitOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "STOP_ORDER" => serde_json::from_value(value)
                .map(Self::StopOrderTransaction)
                .map_err(serde::de::Error::custom),
            "STOP_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::StopOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "MARKET_IF_TOUCHED_ORDER" => serde_json::from_value(value)
                .map(Self::MarketIfTouchedOrderTransaction)
                .map_err(serde::de::Error::custom),
            "MARKET_IF_TOUCHED_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::MarketIfTouchedOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "TAKE_PROFIT_ORDER" => serde_json::from_value(value)
                .map(Self::TakeProfitOrderTransaction)
                .map_err(serde::de::Error::custom),
            "TAKE_PROFIT_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::TakeProfitOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "STOP_LOSS_ORDER" => serde_json::from_value(value)
                .map(Self::StopLossOrderTransaction)
                .map_err(serde::de::Error::custom),
            "STOP_LOSS_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::StopLossOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "GUARANTEED_STOP_LOSS_ORDER" => serde_json::from_value(value)
                .map(Self::GuaranteedStopLossOrderTransaction)
                .map_err(serde::de::Error::custom),
            "GUARANTEED_STOP_LOSS_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::GuaranteedStopLossOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "TRAILING_STOP_LOSS_ORDER" => serde_json::from_value(value)
                .map(Self::TrailingStopLossOrderTransaction)
                .map_err(serde::de::Error::custom),
            "TRAILING_STOP_LOSS_ORDER_REJECT" => serde_json::from_value(value)
                .map(Self::TrailingStopLossOrderRejectTransaction)
                .map_err(serde::de::Error::custom),
            "ORDER_FILL" => serde_json::from_value(value)
                .map(Self::OrderFillTransaction)
                .map_err(serde::de::Error::custom),
            "ORDER_CANCEL" => serde_json::from_value(value)
                .map(Self::OrderCancelTransaction)
                .map_err(serde::de::Error::custom),
            "ORDER_CANCEL_REJECT" => serde_json::from_value(value)
                .map(Self::OrderCancelRejectTransaction)
                .map_err(serde::de::Error::custom),
            "ORDER_CLIENT_EXTENSIONS_MODIFY" => serde_json::from_value(value)
                .map(Self::OrderClientExtensionsModifyTransaction)
                .map_err(serde::de::Error::custom),
            "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT" => serde_json::from_value(value)
                .map(Self::OrderClientExtensionsModifyRejectTransaction)
                .map_err(serde::de::Error::custom),
            "TRADE_CLIENT_EXTENSIONS_MODIFY" => serde_json::from_value(value)
                .map(Self::TradeClientExtensionsModifyTransaction)
                .map_err(serde::de::Error::custom),
            "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT" => serde_json::from_value(value)
                .map(Self::TradeClientExtensionsModifyRejectTransaction)
                .map_err(serde::de::Error::custom),
            "MARGIN_CALL_ENTER" => serde_json::from_value(value)
                .map(Self::MarginCallEnterTransaction)
                .map_err(serde::de::Error::custom),
            "MARGIN_CALL_EXTEND" => serde_json::from_value(value)
                .map(Self::MarginCallExtendTransaction)
                .map_err(serde::de::Error::custom),
            "MARGIN_CALL_EXIT" => serde_json::from_value(value)
                .map(Self::MarginCallExitTransaction)
                .map_err(serde::de::Error::custom),
            "DELAYED_TRADE_CLOSURE" => serde_json::from_value(value)
                .map(Self::DelayedTradeClosureTransaction)
                .map_err(serde::de::Error::custom),
            "DAILY_FINANCING" => serde_json::from_value(value)
                .map(Self::DailyFinancingTransaction)
                .map_err(serde::de::Error::custom),
            "DIVIDEND_ADJUSTMENT" => serde_json::from_value(value)
                .map(Self::DividendAdjustmentTransaction)
                .map_err(serde::de::Error::custom),
            "RESET_RESETTABLE_PL" => serde_json::from_value(value)
                .map(Self::ResetResettablePLTransaction)
                .map_err(serde::de::Error::custom),
            other => Ok(Self::Unknown {
                kind: other.to_owned(),
                raw: value,
            }),
        }
    }
}

impl Serialize for Transaction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (kind, mut value) = match self {
            Self::CreateTransaction(body) => (
                "CREATE",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::CloseTransaction(body) => (
                "CLOSE",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::ReopenTransaction(body) => (
                "REOPEN",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::ClientConfigureTransaction(body) => (
                "CLIENT_CONFIGURE",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::ClientConfigureRejectTransaction(body) => (
                "CLIENT_CONFIGURE_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TransferFundsTransaction(body) => (
                "TRANSFER_FUNDS",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TransferFundsRejectTransaction(body) => (
                "TRANSFER_FUNDS_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarketOrderTransaction(body) => (
                "MARKET_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarketOrderRejectTransaction(body) => (
                "MARKET_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::FixedPriceOrderTransaction(body) => (
                "FIXED_PRICE_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::LimitOrderTransaction(body) => (
                "LIMIT_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::LimitOrderRejectTransaction(body) => (
                "LIMIT_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopOrderTransaction(body) => (
                "STOP_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopOrderRejectTransaction(body) => (
                "STOP_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarketIfTouchedOrderTransaction(body) => (
                "MARKET_IF_TOUCHED_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarketIfTouchedOrderRejectTransaction(body) => (
                "MARKET_IF_TOUCHED_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TakeProfitOrderTransaction(body) => (
                "TAKE_PROFIT_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TakeProfitOrderRejectTransaction(body) => (
                "TAKE_PROFIT_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopLossOrderTransaction(body) => (
                "STOP_LOSS_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::StopLossOrderRejectTransaction(body) => (
                "STOP_LOSS_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::GuaranteedStopLossOrderTransaction(body) => (
                "GUARANTEED_STOP_LOSS_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::GuaranteedStopLossOrderRejectTransaction(body) => (
                "GUARANTEED_STOP_LOSS_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TrailingStopLossOrderTransaction(body) => (
                "TRAILING_STOP_LOSS_ORDER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TrailingStopLossOrderRejectTransaction(body) => (
                "TRAILING_STOP_LOSS_ORDER_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::OrderFillTransaction(body) => (
                "ORDER_FILL",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::OrderCancelTransaction(body) => (
                "ORDER_CANCEL",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::OrderCancelRejectTransaction(body) => (
                "ORDER_CANCEL_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::OrderClientExtensionsModifyTransaction(body) => (
                "ORDER_CLIENT_EXTENSIONS_MODIFY",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::OrderClientExtensionsModifyRejectTransaction(body) => (
                "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TradeClientExtensionsModifyTransaction(body) => (
                "TRADE_CLIENT_EXTENSIONS_MODIFY",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::TradeClientExtensionsModifyRejectTransaction(body) => (
                "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarginCallEnterTransaction(body) => (
                "MARGIN_CALL_ENTER",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarginCallExtendTransaction(body) => (
                "MARGIN_CALL_EXTEND",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::MarginCallExitTransaction(body) => (
                "MARGIN_CALL_EXIT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::DelayedTradeClosureTransaction(body) => (
                "DELAYED_TRADE_CLOSURE",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::DailyFinancingTransaction(body) => (
                "DAILY_FINANCING",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::DividendAdjustmentTransaction(body) => (
                "DIVIDEND_ADJUSTMENT",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::ResetResettablePLTransaction(body) => (
                "RESET_RESETTABLE_PL",
                serde_json::to_value(body).map_err(serde::ser::Error::custom)?,
            ),
            Self::Unknown { kind, raw } => (kind.as_str(), raw.clone()),
        };
        let object = value
            .as_object_mut()
            .ok_or_else(|| serde::ser::Error::custom("OANDA variant must be an object"))?;
        object.insert(
            "type".to_owned(),
            serde_json::Value::String(kind.to_owned()),
        );
        value.serialize(serializer)
    }
}
