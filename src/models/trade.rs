//! OANDA v20 trade definitions.
// Generated shared imports vary by definition family.
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::ids::*;
#[allow(unused_imports)]
use crate::timestamp::Timestamp;
#[allow(unused_imports)]
use rust_decimal::Decimal;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// The current state of the Trade.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TradeState {
    /// The Trade is currently open
    Open,
    /// The Trade has been fully closed
    Closed,
    /// The Trade will be closed as soon as the trade’s instrument becomes tradeable
    CloseWhenTradeable,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TradeState {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Open => "OPEN",
            Self::Closed => "CLOSED",
            Self::CloseWhenTradeable => "CLOSE_WHEN_TRADEABLE",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TradeState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TradeState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "OPEN" => Self::Open,
            "CLOSED" => Self::Closed,
            "CLOSE_WHEN_TRADEABLE" => Self::CloseWhenTradeable,
            _ => Self::Unknown(value),
        })
    }
}

/// The state to filter the Trades by
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TradeStateFilter {
    /// The Trades that are currently open
    Open,
    /// The Trades that have been fully closed
    Closed,
    /// The Trades that will be closed as soon as the trades’ instrument becomes tradeable
    CloseWhenTradeable,
    /// The Trades that are in any of the possible states listed above.
    All,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TradeStateFilter {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Open => "OPEN",
            Self::Closed => "CLOSED",
            Self::CloseWhenTradeable => "CLOSE_WHEN_TRADEABLE",
            Self::All => "ALL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TradeStateFilter {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TradeStateFilter {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "OPEN" => Self::Open,
            "CLOSED" => Self::Closed,
            "CLOSE_WHEN_TRADEABLE" => Self::CloseWhenTradeable,
            "ALL" => Self::All,
            _ => Self::Unknown(value),
        })
    }
}

/// The specification of a Trade within an Account. This includes the full representation of the Trade’s
/// dependent Orders in addition to the IDs of those Orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// The Trade’s identifier, unique within the Trade’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TradeID>,
    /// The Trade’s Instrument.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The execution price of the Trade.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// The date/time when the Trade was opened.
    #[serde(rename = "openTime", default, skip_serializing_if = "Option::is_none")]
    pub open_time: Option<Timestamp>,
    /// The current state of the Trade.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TradeState>,
    /// The initial size of the Trade. Negative values indicate a short Trade, and positive values indicate a
    /// long Trade.
    #[serde(
        rename = "initialUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_units: Option<Decimal>,
    /// The margin required at the time the Trade was created. Note, this is the ‘pure’ margin required, it is
    /// not the ‘effective’ margin used that factors in the trade risk if a GSLO is attached to the trade.
    #[serde(
        rename = "initialMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<Decimal>,
    /// The number of units currently open for the Trade. This value is reduced to 0.0 as the Trade is closed.
    #[serde(
        rename = "currentUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_units: Option<Decimal>,
    /// The total profit/loss realized on the closed portion of the Trade.
    #[serde(
        rename = "realizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pl: Option<Decimal>,
    /// The unrealized profit/loss on the open portion of the Trade.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// Margin currently used by the Trade.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
    /// The average closing price of the Trade. Only present if the Trade has been closed or reduced at least
    /// once.
    #[serde(
        rename = "averageClosePrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_close_price: Option<Decimal>,
    /// The IDs of the Transactions that have closed portions of this Trade.
    #[serde(
        rename = "closingTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub closing_transaction_ids: Option<Vec<TransactionID>>,
    /// The financing paid/collected for this Trade.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The dividend adjustment paid for this Trade.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The date/time when the Trade was fully closed. Only provided for Trades whose state is CLOSED.
    #[serde(rename = "closeTime", default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<Timestamp>,
    /// The client extensions of the Trade.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// Full representation of the Trade’s Take Profit Order, only provided if such an Order exists.
    #[serde(
        rename = "takeProfitOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order: Option<TakeProfitOrder>,
    /// Full representation of the Trade’s Stop Loss Order, only provided if such an Order exists.
    #[serde(
        rename = "stopLossOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order: Option<StopLossOrder>,
    /// Full representation of the Trade’s Trailing Stop Loss Order, only provided if such an Order exists.
    #[serde(
        rename = "trailingStopLossOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order: Option<TrailingStopLossOrder>,
}

/// The summary of a Trade within an Account. This representation does not provide the full details of the
/// Trade’s dependent Orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeSummary {
    /// The Trade’s identifier, unique within the Trade’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TradeID>,
    /// The Trade’s Instrument.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The execution price of the Trade.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// The date/time when the Trade was opened.
    #[serde(rename = "openTime", default, skip_serializing_if = "Option::is_none")]
    pub open_time: Option<Timestamp>,
    /// The current state of the Trade.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TradeState>,
    /// The initial size of the Trade. Negative values indicate a short Trade, and positive values indicate a
    /// long Trade.
    #[serde(
        rename = "initialUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_units: Option<Decimal>,
    /// The margin required at the time the Trade was created. Note, this is the ‘pure’ margin required, it is
    /// not the ‘effective’ margin used that factors in the trade risk if a GSLO is attached to the trade.
    #[serde(
        rename = "initialMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<Decimal>,
    /// The number of units currently open for the Trade. This value is reduced to 0.0 as the Trade is closed.
    #[serde(
        rename = "currentUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_units: Option<Decimal>,
    /// The total profit/loss realized on the closed portion of the Trade.
    #[serde(
        rename = "realizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pl: Option<Decimal>,
    /// The unrealized profit/loss on the open portion of the Trade.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// Margin currently used by the Trade.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
    /// The average closing price of the Trade. Only present if the Trade has been closed or reduced at least
    /// once.
    #[serde(
        rename = "averageClosePrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_close_price: Option<Decimal>,
    /// The IDs of the Transactions that have closed portions of this Trade.
    #[serde(
        rename = "closingTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub closing_transaction_ids: Option<Vec<TransactionID>>,
    /// The financing paid/collected for this Trade.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The dividend adjustment paid for this Trade.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The date/time when the Trade was fully closed. Only provided for Trades whose state is CLOSED.
    #[serde(rename = "closeTime", default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<Timestamp>,
    /// The client extensions of the Trade.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// ID of the Trade’s Take Profit Order, only provided if such an Order exists.
    #[serde(
        rename = "takeProfitOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_id: Option<OrderID>,
    /// ID of the Trade’s Stop Loss Order, only provided if such an Order exists.
    #[serde(
        rename = "stopLossOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_id: Option<OrderID>,
    /// ID of the Trade’s Guaranteed Stop Loss Order, only provided if such an Order exists.
    #[serde(
        rename = "guaranteedStopLossOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_id: Option<OrderID>,
    /// ID of the Trade’s Trailing Stop Loss Order, only provided if such an Order exists.
    #[serde(
        rename = "trailingStopLossOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_id: Option<OrderID>,
}

/// The dynamic (calculated) state of an open Trade
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatedTradeState {
    /// The Trade’s ID.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TradeID>,
    /// The Trade’s unrealized profit/loss.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// Margin currently used by the Trade.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
}

/// The classification of TradePLs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TradePL {
    /// An open Trade currently has a positive (profitable) unrealized P/L, or a closed Trade realized a
    /// positive amount of P/L.
    Positive,
    /// An open Trade currently has a negative (losing) unrealized P/L, or a closed Trade realized a negative
    /// amount of P/L.
    Negative,
    /// An open Trade currently has unrealized P/L of zero (neither profitable nor losing), or a closed Trade
    /// realized a P/L amount of zero.
    Zero,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TradePL {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Positive => "POSITIVE",
            Self::Negative => "NEGATIVE",
            Self::Zero => "ZERO",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TradePL {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TradePL {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "POSITIVE" => Self::Positive,
            "NEGATIVE" => Self::Negative,
            "ZERO" => Self::Zero,
            _ => Self::Unknown(value),
        })
    }
}
