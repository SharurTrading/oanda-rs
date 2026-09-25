//! OANDA v20 account definitions.
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

/// The full details of a client’s Account. This includes full open Trade, open Position and pending Order
/// representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// The Account’s identifier
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<AccountID>,
    /// Client-assigned alias for the Account. Only provided if the Account has an alias set
    #[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// The home currency of the Account
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// ID of the user that created the Account.
    #[serde(
        rename = "createdByUserID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_user_id: Option<i64>,
    /// The date/time when the Account was created.
    #[serde(
        rename = "createdTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_time: Option<Timestamp>,
    /// The current guaranteed Stop Loss Order settings of the Account. This field will only be present if the
    /// guaranteedStopLossOrderMode is not ‘DISABLED’.
    #[serde(
        rename = "guaranteedStopLossOrderParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_parameters: Option<GuaranteedStopLossOrderParameters>,
    /// The current guaranteed Stop Loss Order mode of the Account.
    #[serde(
        rename = "guaranteedStopLossOrderMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderMode>,
    /// The current guaranteed Stop Loss Order mutability setting of the Account. This field will only be
    /// present if the guaranteedStopLossOrderMode is not ‘DISABLED’. Deprecated: Will be removed in a future
    /// API update.
    #[serde(
        rename = "guaranteedStopLossOrderMutability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mutability: Option<GuaranteedStopLossOrderMutability>,
    /// The date/time that the Account’s resettablePL was last reset.
    #[serde(
        rename = "resettablePLTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl_time: Option<Timestamp>,
    /// Client-provided margin rate override for the Account. The effective margin rate of the Account is the
    /// lesser of this value and the OANDA margin rate for the Account’s division. This value is only provided
    /// if a margin rate override exists for the Account.
    #[serde(
        rename = "marginRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_rate: Option<Decimal>,
    /// The number of Trades currently open in the Account.
    #[serde(
        rename = "openTradeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_trade_count: Option<i64>,
    /// The number of Positions currently open in the Account.
    #[serde(
        rename = "openPositionCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_position_count: Option<i64>,
    /// The number of Orders currently pending in the Account.
    #[serde(
        rename = "pendingOrderCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_order_count: Option<i64>,
    /// Flag indicating that the Account has hedging enabled.
    #[serde(
        rename = "hedgingEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hedging_enabled: Option<bool>,
    /// The total unrealized profit/loss for all Trades currently open in the Account.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// The net asset value of the Account. Equal to Account balance + unrealizedPL.
    #[serde(rename = "NAV", default, skip_serializing_if = "Option::is_none")]
    pub nav: Option<Decimal>,
    /// Margin currently used for the Account.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
    /// Margin available for Account currency.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<Decimal>,
    /// The value of the Account’s open positions represented in the Account’s home currency.
    #[serde(
        rename = "positionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_value: Option<Decimal>,
    /// The Account’s margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<Decimal>,
    /// The Account’s margin closeout NAV.
    #[serde(
        rename = "marginCloseoutNAV",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_nav: Option<Decimal>,
    /// The Account’s margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<Decimal>,
    /// The Account’s margin closeout percentage. When this value is 1.0 or above the Account is in a margin
    /// closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<Decimal>,
    /// The value of the Account’s open positions as used for margin closeout calculations represented in the
    /// Account’s home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<Decimal>,
    /// The current WithdrawalLimit for the account which will be zero or a positive value indicating how much
    /// can be withdrawn from the account.
    #[serde(
        rename = "withdrawalLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_limit: Option<Decimal>,
    /// The Account’s margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<Decimal>,
    /// The Account’s margin call percentage. When this value is 1.0 or above the Account is in a margin call
    /// situation.
    #[serde(
        rename = "marginCallPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_percent: Option<Decimal>,
    /// The current balance of the account.
    #[serde(rename = "balance", default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    /// The total profit/loss realized over the lifetime of the Account.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<Decimal>,
    /// The total realized profit/loss for the account since it was last reset by the client.
    #[serde(
        rename = "resettablePL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl: Option<Decimal>,
    /// The total amount of financing paid/collected over the lifetime of the account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The total amount of commission paid over the lifetime of the Account.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// The total amount of dividend adjustment paid over the lifetime of the Account in the Account’s home
    /// currency.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop
    /// Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFees",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<Decimal>,
    /// The date/time when the Account entered a margin call state. Only provided if the Account is in a margin
    /// call.
    #[serde(
        rename = "marginCallEnterTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_enter_time: Option<Timestamp>,
    /// The number of times that the Account’s current margin call was extended.
    #[serde(
        rename = "marginCallExtensionCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_extension_count: Option<i64>,
    /// The date/time of the Account’s last margin call extension.
    #[serde(
        rename = "lastMarginCallExtensionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_margin_call_extension_time: Option<Timestamp>,
    /// The ID of the last Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
    /// The details of the Trades currently open in the Account.
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<TradeSummary>>,
    /// The details all Account Positions.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
    /// The details of the Orders currently pending in the Account.
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
}

/// An AccountState Object is used to represent an Account’s current price-dependent state. Price-dependent
/// Account state is dependent on OANDA’s current Prices, and includes things like unrealized PL, NAV and
/// Trailing Stop Loss Order state. Fields will be omitted if their value has not changed since the
/// specified transaction ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountChangesState {
    /// The total unrealized profit/loss for all Trades currently open in the Account.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// The net asset value of the Account. Equal to Account balance + unrealizedPL.
    #[serde(rename = "NAV", default, skip_serializing_if = "Option::is_none")]
    pub nav: Option<Decimal>,
    /// Margin currently used for the Account.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
    /// Margin available for Account currency.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<Decimal>,
    /// The value of the Account’s open positions represented in the Account’s home currency.
    #[serde(
        rename = "positionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_value: Option<Decimal>,
    /// The Account’s margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<Decimal>,
    /// The Account’s margin closeout NAV.
    #[serde(
        rename = "marginCloseoutNAV",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_nav: Option<Decimal>,
    /// The Account’s margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<Decimal>,
    /// The Account’s margin closeout percentage. When this value is 1.0 or above the Account is in a margin
    /// closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<Decimal>,
    /// The value of the Account’s open positions as used for margin closeout calculations represented in the
    /// Account’s home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<Decimal>,
    /// The current WithdrawalLimit for the account which will be zero or a positive value indicating how much
    /// can be withdrawn from the account.
    #[serde(
        rename = "withdrawalLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_limit: Option<Decimal>,
    /// The Account’s margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<Decimal>,
    /// The Account’s margin call percentage. When this value is 1.0 or above the Account is in a margin call
    /// situation.
    #[serde(
        rename = "marginCallPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_percent: Option<Decimal>,
    /// The current balance of the account.
    #[serde(rename = "balance", default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    /// The total profit/loss realized over the lifetime of the Account.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<Decimal>,
    /// The total realized profit/loss for the account since it was last reset by the client.
    #[serde(
        rename = "resettablePL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl: Option<Decimal>,
    /// The total amount of financing paid/collected over the lifetime of the account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The total amount of commission paid over the lifetime of the Account.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// The total amount of dividend adjustment paid over the lifetime of the Account in the Account’s home
    /// currency.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop
    /// Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFees",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<Decimal>,
    /// The date/time when the Account entered a margin call state. Only provided if the Account is in a margin
    /// call.
    #[serde(
        rename = "marginCallEnterTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_enter_time: Option<Timestamp>,
    /// The number of times that the Account’s current margin call was extended.
    #[serde(
        rename = "marginCallExtensionCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_extension_count: Option<i64>,
    /// The date/time of the Account’s last margin call extension.
    #[serde(
        rename = "lastMarginCallExtensionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_margin_call_extension_time: Option<Timestamp>,
    /// The price-dependent state of each pending Order in the Account.
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<DynamicOrderState>>,
    /// The price-dependent state for each open Trade in the Account.
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<CalculatedTradeState>>,
    /// The price-dependent state for each open Position in the Account.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<CalculatedPositionState>>,
}

/// Properties related to an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountProperties {
    /// The Account’s identifier
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<AccountID>,
    /// The Account’s associated MT4 Account ID. This field will not be present if the Account is not an MT4
    /// account.
    #[serde(
        rename = "mt4AccountID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mt4_account_id: Option<i64>,
    /// The Account’s tags
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// The current mutability and hedging settings related to guaranteed Stop Loss orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrderParameters {
    /// The current guaranteed Stop Loss Order mutability setting of the Account when market is open.
    #[serde(
        rename = "mutabilityMarketOpen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mutability_market_open: Option<GuaranteedStopLossOrderMutability>,
    /// The current guaranteed Stop Loss Order mutability setting of the Account when market is halted.
    #[serde(
        rename = "mutabilityMarketHalted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mutability_market_halted: Option<GuaranteedStopLossOrderMutability>,
}

/// The overall behaviour of the Account regarding guaranteed Stop Loss Orders.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GuaranteedStopLossOrderMode {
    /// The Account is not permitted to create guaranteed Stop Loss Orders.
    Disabled,
    /// The Account is able, but not required to have guaranteed Stop Loss Orders for open Trades.
    Allowed,
    /// The Account is required to have guaranteed Stop Loss Orders for all open Trades.
    Required,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl GuaranteedStopLossOrderMode {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Disabled => "DISABLED",
            Self::Allowed => "ALLOWED",
            Self::Required => "REQUIRED",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for GuaranteedStopLossOrderMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for GuaranteedStopLossOrderMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "DISABLED" => Self::Disabled,
            "ALLOWED" => Self::Allowed,
            "REQUIRED" => Self::Required,
            _ => Self::Unknown(value),
        })
    }
}

/// For Accounts that support guaranteed Stop Loss Orders, describes the actions that can be performed on
/// guaranteed Stop Loss Orders.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GuaranteedStopLossOrderMutability {
    /// Once a guaranteed Stop Loss Order has been created it cannot be replaced or cancelled.
    Fixed,
    /// An existing guaranteed Stop Loss Order can only be replaced, not cancelled.
    Replaceable,
    /// Once a guaranteed Stop Loss Order has been created it can be either replaced or cancelled.
    Cancelable,
    /// An existing guaranteed Stop Loss Order can only be replaced to widen the gap from the current price, not
    /// cancelled.
    PriceWidenOnly,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl GuaranteedStopLossOrderMutability {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Fixed => "FIXED",
            Self::Replaceable => "REPLACEABLE",
            Self::Cancelable => "CANCELABLE",
            Self::PriceWidenOnly => "PRICE_WIDEN_ONLY",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for GuaranteedStopLossOrderMutability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for GuaranteedStopLossOrderMutability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "FIXED" => Self::Fixed,
            "REPLACEABLE" => Self::Replaceable,
            "CANCELABLE" => Self::Cancelable,
            "PRICE_WIDEN_ONLY" => Self::PriceWidenOnly,
            _ => Self::Unknown(value),
        })
    }
}

/// A summary representation of a client’s Account. The AccountSummary does not provide a full specification
/// of pending Orders, open Trades and Positions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    /// The Account’s identifier
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<AccountID>,
    /// Client-assigned alias for the Account. Only provided if the Account has an alias set
    #[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// The home currency of the Account
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// ID of the user that created the Account.
    #[serde(
        rename = "createdByUserID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_user_id: Option<i64>,
    /// The date/time when the Account was created.
    #[serde(
        rename = "createdTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_time: Option<Timestamp>,
    /// The current guaranteed Stop Loss Order settings of the Account. This field will only be present if the
    /// guaranteedStopLossOrderMode is not ‘DISABLED’.
    #[serde(
        rename = "guaranteedStopLossOrderParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_parameters: Option<GuaranteedStopLossOrderParameters>,
    /// The current guaranteed Stop Loss Order mode of the Account.
    #[serde(
        rename = "guaranteedStopLossOrderMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderMode>,
    /// The current guaranteed Stop Loss Order mutability setting of the Account. This field will only be
    /// present if the guaranteedStopLossOrderMode is not ‘DISABLED’. Deprecated: Will be removed in a future
    /// API update.
    #[serde(
        rename = "guaranteedStopLossOrderMutability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mutability: Option<GuaranteedStopLossOrderMutability>,
    /// The date/time that the Account’s resettablePL was last reset.
    #[serde(
        rename = "resettablePLTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl_time: Option<Timestamp>,
    /// Client-provided margin rate override for the Account. The effective margin rate of the Account is the
    /// lesser of this value and the OANDA margin rate for the Account’s division. This value is only provided
    /// if a margin rate override exists for the Account.
    #[serde(
        rename = "marginRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_rate: Option<Decimal>,
    /// The number of Trades currently open in the Account.
    #[serde(
        rename = "openTradeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_trade_count: Option<i64>,
    /// The number of Positions currently open in the Account.
    #[serde(
        rename = "openPositionCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_position_count: Option<i64>,
    /// The number of Orders currently pending in the Account.
    #[serde(
        rename = "pendingOrderCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_order_count: Option<i64>,
    /// Flag indicating that the Account has hedging enabled.
    #[serde(
        rename = "hedgingEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hedging_enabled: Option<bool>,
    /// The total unrealized profit/loss for all Trades currently open in the Account.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// The net asset value of the Account. Equal to Account balance + unrealizedPL.
    #[serde(rename = "NAV", default, skip_serializing_if = "Option::is_none")]
    pub nav: Option<Decimal>,
    /// Margin currently used for the Account.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
    /// Margin available for Account currency.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<Decimal>,
    /// The value of the Account’s open positions represented in the Account’s home currency.
    #[serde(
        rename = "positionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_value: Option<Decimal>,
    /// The Account’s margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<Decimal>,
    /// The Account’s margin closeout NAV.
    #[serde(
        rename = "marginCloseoutNAV",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_nav: Option<Decimal>,
    /// The Account’s margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<Decimal>,
    /// The Account’s margin closeout percentage. When this value is 1.0 or above the Account is in a margin
    /// closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<Decimal>,
    /// The value of the Account’s open positions as used for margin closeout calculations represented in the
    /// Account’s home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<Decimal>,
    /// The current WithdrawalLimit for the account which will be zero or a positive value indicating how much
    /// can be withdrawn from the account.
    #[serde(
        rename = "withdrawalLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_limit: Option<Decimal>,
    /// The Account’s margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<Decimal>,
    /// The Account’s margin call percentage. When this value is 1.0 or above the Account is in a margin call
    /// situation.
    #[serde(
        rename = "marginCallPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_percent: Option<Decimal>,
    /// The current balance of the account.
    #[serde(rename = "balance", default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    /// The total profit/loss realized over the lifetime of the Account.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<Decimal>,
    /// The total realized profit/loss for the account since it was last reset by the client.
    #[serde(
        rename = "resettablePL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl: Option<Decimal>,
    /// The total amount of financing paid/collected over the lifetime of the account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The total amount of commission paid over the lifetime of the Account.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// The total amount of dividend adjustment paid over the lifetime of the Account in the Account’s home
    /// currency.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop
    /// Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFees",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<Decimal>,
    /// The date/time when the Account entered a margin call state. Only provided if the Account is in a margin
    /// call.
    #[serde(
        rename = "marginCallEnterTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_enter_time: Option<Timestamp>,
    /// The number of times that the Account’s current margin call was extended.
    #[serde(
        rename = "marginCallExtensionCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_extension_count: Option<i64>,
    /// The date/time of the Account’s last margin call extension.
    #[serde(
        rename = "lastMarginCallExtensionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_margin_call_extension_time: Option<Timestamp>,
    /// The ID of the last Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// The mutable state of a client’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccumulatedAccountState {
    /// The current balance of the account.
    #[serde(rename = "balance", default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    /// The total profit/loss realized over the lifetime of the Account.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<Decimal>,
    /// The total realized profit/loss for the account since it was last reset by the client.
    #[serde(
        rename = "resettablePL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resettable_pl: Option<Decimal>,
    /// The total amount of financing paid/collected over the lifetime of the account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The total amount of commission paid over the lifetime of the Account.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// The total amount of dividend adjustment paid over the lifetime of the Account in the Account’s home
    /// currency.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop
    /// Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFees",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<Decimal>,
    /// The date/time when the Account entered a margin call state. Only provided if the Account is in a margin
    /// call.
    #[serde(
        rename = "marginCallEnterTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_enter_time: Option<Timestamp>,
    /// The number of times that the Account’s current margin call was extended.
    #[serde(
        rename = "marginCallExtensionCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_extension_count: Option<i64>,
    /// The date/time of the Account’s last margin call extension.
    #[serde(
        rename = "lastMarginCallExtensionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_margin_call_extension_time: Option<Timestamp>,
}

/// The dynamically calculated state of a client’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatedAccountState {
    /// The total unrealized profit/loss for all Trades currently open in the Account.
    #[serde(
        rename = "unrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pl: Option<Decimal>,
    /// The net asset value of the Account. Equal to Account balance + unrealizedPL.
    #[serde(rename = "NAV", default, skip_serializing_if = "Option::is_none")]
    pub nav: Option<Decimal>,
    /// Margin currently used for the Account.
    #[serde(
        rename = "marginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_used: Option<Decimal>,
    /// Margin available for Account currency.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<Decimal>,
    /// The value of the Account’s open positions represented in the Account’s home currency.
    #[serde(
        rename = "positionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_value: Option<Decimal>,
    /// The Account’s margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<Decimal>,
    /// The Account’s margin closeout NAV.
    #[serde(
        rename = "marginCloseoutNAV",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_nav: Option<Decimal>,
    /// The Account’s margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<Decimal>,
    /// The Account’s margin closeout percentage. When this value is 1.0 or above the Account is in a margin
    /// closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<Decimal>,
    /// The value of the Account’s open positions as used for margin closeout calculations represented in the
    /// Account’s home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<Decimal>,
    /// The current WithdrawalLimit for the account which will be zero or a positive value indicating how much
    /// can be withdrawn from the account.
    #[serde(
        rename = "withdrawalLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_limit: Option<Decimal>,
    /// The Account’s margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<Decimal>,
    /// The Account’s margin call percentage. When this value is 1.0 or above the Account is in a margin call
    /// situation.
    #[serde(
        rename = "marginCallPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_percent: Option<Decimal>,
}

/// An AccountChanges Object is used to represent the changes to an Account’s Orders, Trades and Positions
/// since a specified Account TransactionID in the past.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountChanges {
    /// The Orders created. These Orders may have been filled, cancelled or triggered in the same period.
    #[serde(
        rename = "ordersCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orders_created: Option<Vec<Order>>,
    /// The Orders cancelled.
    #[serde(
        rename = "ordersCancelled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orders_cancelled: Option<Vec<Order>>,
    /// The Orders filled.
    #[serde(
        rename = "ordersFilled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orders_filled: Option<Vec<Order>>,
    /// The Orders triggered.
    #[serde(
        rename = "ordersTriggered",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orders_triggered: Option<Vec<Order>>,
    /// The Trades opened.
    #[serde(
        rename = "tradesOpened",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trades_opened: Option<Vec<TradeSummary>>,
    /// The Trades reduced.
    #[serde(
        rename = "tradesReduced",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trades_reduced: Option<Vec<TradeSummary>>,
    /// The Trades closed.
    #[serde(
        rename = "tradesClosed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trades_closed: Option<Vec<TradeSummary>>,
    /// The Positions changed.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
    /// The Transactions that have been generated.
    #[serde(
        rename = "transactions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transactions: Option<Vec<Transaction>>,
}

/// The financing mode of an Account
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AccountFinancingMode {
    /// No financing is paid/charged for open Trades in the Account
    NoFinancing,
    /// Second-by-second financing is paid/charged for open Trades in the Account, both daily and when the the
    /// Trade is closed
    SecondBySecond,
    /// A full day’s worth of financing is paid/charged for open Trades in the Account daily at 5pm New York time
    Daily,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl AccountFinancingMode {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::NoFinancing => "NO_FINANCING",
            Self::SecondBySecond => "SECOND_BY_SECOND",
            Self::Daily => "DAILY",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for AccountFinancingMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AccountFinancingMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NO_FINANCING" => Self::NoFinancing,
            "SECOND_BY_SECOND" => Self::SecondBySecond,
            "DAILY" => Self::Daily,
            _ => Self::Unknown(value),
        })
    }
}

/// Contains the attributes of a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
    /// The user’s OANDA-assigned user ID.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The user-provided username.
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The user’s title.
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The user’s name.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user’s email address.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The OANDA division the user belongs to.
    #[serde(
        rename = "divisionAbbreviation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub division_abbreviation: Option<String>,
    /// The user’s preferred language.
    #[serde(
        rename = "languageAbbreviation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub language_abbreviation: Option<String>,
    /// The home currency of the Account.
    #[serde(
        rename = "homeCurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_currency: Option<Currency>,
}

/// The way that position values for an Account are calculated and aggregated.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PositionAggregationMode {
    /// The Position value or margin for each side (long and short) of the Position are computed independently
    /// and added together.
    AbsoluteSum,
    /// The Position value or margin for each side (long and short) of the Position are computed independently.
    /// The Position value or margin chosen is the maximal absolute value of the two.
    MaximalSide,
    /// The units for each side (long and short) of the Position are netted together and the resulting value
    /// (long or short) is used to compute the Position value or margin.
    NetSum,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl PositionAggregationMode {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::AbsoluteSum => "ABSOLUTE_SUM",
            Self::MaximalSide => "MAXIMAL_SIDE",
            Self::NetSum => "NET_SUM",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for PositionAggregationMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for PositionAggregationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ABSOLUTE_SUM" => Self::AbsoluteSum,
            "MAXIMAL_SIDE" => Self::MaximalSide,
            "NET_SUM" => Self::NetSum,
            _ => Self::Unknown(value),
        })
    }
}
