//! OANDA v20 position definitions.
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

/// The specification of a Position within an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// The Position’s Instrument.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// Profit/loss realized by the Position over the lifetime of the Account.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<Decimal>,
    /// The unrealized profit/loss of all open Trades that contribute to this Position.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// Margin currently used by the Position.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
    /// Profit/loss realized by the Position since the Account’s resettablePL was last reset by the client.
    #[serde(
        rename = "resettablePL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl: Option<Decimal>,
    /// The total amount of financing paid/collected for this instrument over the lifetime of the Account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The total amount of commission paid for this instrument over the lifetime of the Account.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// The total amount of dividend adjustment paid for this instrument over the lifetime of the Account.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop
    /// Loss Orders for this instrument.
    #[serde(
        rename = "guaranteedExecutionFees",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<Decimal>,
    /// The details of the long side of the Position.
    #[serde(rename = "long", default, skip_serializing_if = "Option::is_none")]
    pub long: Option<PositionSide>,
    /// The details of the short side of the Position.
    #[serde(rename = "short", default, skip_serializing_if = "Option::is_none")]
    pub short: Option<PositionSide>,
}

/// The representation of a Position for a single direction (long or short).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSide {
    /// Number of units in the position (negative value indicates short position, positive indicates long
    /// position).
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Decimal>,
    /// Volume-weighted average of the underlying Trade open prices for the Position.
    #[serde(
        rename = "averagePrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_price: Option<Decimal>,
    /// List of the open Trade IDs which contribute to the open Position.
    #[serde(rename = "tradeIDs", default, skip_serializing_if = "Option::is_none")]
    pub trade_ids: Option<Vec<TradeID>>,
    /// Profit/loss realized by the PositionSide over the lifetime of the Account.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<Decimal>,
    /// The unrealized profit/loss of all open Trades that contribute to this PositionSide.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// Profit/loss realized by the PositionSide since the Account’s resettablePL was last reset by the client.
    #[serde(
        rename = "resettablePL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl: Option<Decimal>,
    /// The total amount of financing paid/collected for this PositionSide over the lifetime of the Account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The total amount of dividend adjustment paid for the PositionSide over the lifetime of the Account.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop
    /// Loss Orders attached to Trades for this PositionSide.
    #[serde(
        rename = "guaranteedExecutionFees",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<Decimal>,
}

/// The dynamic (calculated) state of a Position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatedPositionState {
    /// The Position’s Instrument.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The Position’s net unrealized profit/loss
    #[serde(
        rename = "netUnrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub net_unrealized_pl: Option<Decimal>,
    /// The unrealized profit/loss of the Position’s long open Trades
    #[serde(
        rename = "longUnrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_unrealized_pl: Option<Decimal>,
    /// The unrealized profit/loss of the Position’s short open Trades
    #[serde(
        rename = "shortUnrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_unrealized_pl: Option<Decimal>,
    /// Margin currently used by the Position.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
}
