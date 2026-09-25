//! OANDA v20 transaction definitions.
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

/// A CreateTransaction represents the creation of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “CREATE” in a CreateTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Division that the Account is in
    #[serde(
        rename = "divisionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub division_id: Option<i64>,
    /// The ID of the Site that the Account was created at
    #[serde(rename = "siteID", default, skip_serializing_if = "Option::is_none")]
    pub site_id: Option<i64>,
    /// The ID of the user that the Account was created for
    #[serde(
        rename = "accountUserID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_user_id: Option<i64>,
    /// The number of the Account within the site/division/user
    #[serde(
        rename = "accountNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_number: Option<i64>,
    /// The home currency of the Account
    #[serde(
        rename = "homeCurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_currency: Option<Currency>,
}

/// A CloseTransaction represents the closing of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “CLOSE” in a CloseTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
}

/// A ReopenTransaction represents the re-opening of a closed Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReopenTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “REOPEN” in a ReopenTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
}

/// A ClientConfigureTransaction represents the configuration of an Account by a client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigureTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “CLIENT_CONFIGURE” in a ClientConfigureTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The client-provided alias for the Account.
    #[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// The margin rate override for the Account.
    #[serde(
        rename = "marginRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_rate: Option<Decimal>,
}

/// A ClientConfigureRejectTransaction represents the rejection of configuration of an Account by a client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigureRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “CLIENT_CONFIGURE_REJECT” in a
    /// ClientConfigureRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The client-provided alias for the Account.
    #[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// The margin rate override for the Account.
    #[serde(
        rename = "marginRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_rate: Option<Decimal>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A TransferFundsTransaction represents the transfer of funds in/out of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferFundsTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TRANSFER_FUNDS” in a TransferFundsTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The amount to deposit/withdraw from the Account in the Account’s home currency. A positive value
    /// indicates a deposit, a negative value indicates a withdrawal.
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Decimal>,
    /// The reason that an Account is being funded.
    #[serde(
        rename = "fundingReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_reason: Option<FundingReason>,
    /// An optional comment that may be attached to a fund transfer for audit purposes
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The Account’s balance after funds are transferred.
    #[serde(
        rename = "accountBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_balance: Option<Decimal>,
}

/// A TransferFundsRejectTransaction represents the rejection of the transfer of funds in/out of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferFundsRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TRANSFER_FUNDS_REJECT” in a TransferFundsRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The amount to deposit/withdraw from the Account in the Account’s home currency. A positive value
    /// indicates a deposit, a negative value indicates a withdrawal.
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Decimal>,
    /// The reason that an Account is being funded.
    #[serde(
        rename = "fundingReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_reason: Option<FundingReason>,
    /// An optional comment that may be attached to a fund transfer for audit purposes
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A MarketOrderTransaction represents the creation of a Market Order in the user’s account. A Market Order
/// is an Order that is filled immediately at the current market price. Market Orders can be specialized
/// when they are created to accomplish a specific task: to close a Trade, to closeout a Position or to
/// participate in a Margin closeout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARKET_ORDER” in a MarketOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The Market Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The time-in-force requested for the Market Order. Restricted to FOK or IOC for a MarketOrder.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the Market Order filled at.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_bound: Option<Decimal>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Details of the Trade requested to be closed, only provided when the Market Order is being used to
    /// explicitly close a Trade.
    #[serde(
        rename = "tradeClose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_close: Option<MarketOrderTradeClose>,
    /// Details of the long Position requested to be closed out, only provided when a Market Order is being used
    /// to explicitly closeout a long Position.
    #[serde(
        rename = "longPositionCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the short Position requested to be closed out, only provided when a Market Order is being
    /// used to explicitly closeout a short Position.
    #[serde(
        rename = "shortPositionCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the Margin Closeout that this Market Order was created for
    #[serde(
        rename = "marginCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout: Option<MarketOrderMarginCloseout>,
    /// Details of the delayed Trade close that this Market Order was created for
    #[serde(
        rename = "delayedTradeClose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delayed_trade_close: Option<MarketOrderDelayedTradeClose>,
    /// The reason that the Market Order was created
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A MarketOrderRejectTransaction represents the rejection of the creation of a Market Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARKET_ORDER_REJECT” in a MarketOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The Market Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The time-in-force requested for the Market Order. Restricted to FOK or IOC for a MarketOrder.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the Market Order filled at.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_bound: Option<Decimal>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Details of the Trade requested to be closed, only provided when the Market Order is being used to
    /// explicitly close a Trade.
    #[serde(
        rename = "tradeClose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_close: Option<MarketOrderTradeClose>,
    /// Details of the long Position requested to be closed out, only provided when a Market Order is being used
    /// to explicitly closeout a long Position.
    #[serde(
        rename = "longPositionCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the short Position requested to be closed out, only provided when a Market Order is being
    /// used to explicitly closeout a short Position.
    #[serde(
        rename = "shortPositionCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the Margin Closeout that this Market Order was created for
    #[serde(
        rename = "marginCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout: Option<MarketOrderMarginCloseout>,
    /// Details of the delayed Trade close that this Market Order was created for
    #[serde(
        rename = "delayedTradeClose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delayed_trade_close: Option<MarketOrderDelayedTradeClose>,
    /// The reason that the Market Order was created
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A FixedPriceOrderTransaction represents the creation of a Fixed Price Order in the user’s account. A
/// Fixed Price Order is an Order that is filled immediately at a specified price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedPriceOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “FIXED_PRICE_ORDER” in a FixedPriceOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The Fixed Price Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Fixed Price Order. A positive number of units results in a
    /// long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The price specified for the Fixed Price Order. This price is the exact price that the Fixed Price Order
    /// will be filled at.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// The state that the trade resulting from the Fixed Price Order should be set to.
    #[serde(rename = "tradeState")]
    pub trade_state: String,
    /// The reason that the Fixed Price Order was created
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<FixedPriceOrderReason>,
    /// The client extensions for the Fixed Price Order.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A LimitOrderTransaction represents the creation of a Limit Order in the user’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “LIMIT_ORDER” in a LimitOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The Limit Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Limit Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The price threshold specified for the Limit Order. The Limit Order will only be filled by a market price
    /// that is equal to or better than this price.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The time-in-force requested for the Limit Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Limit Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Limit Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<LimitOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an
    /// existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
}

/// A LimitOrderRejectTransaction represents the rejection of the creation of a Limit Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “LIMIT_ORDER_REJECT” in a LimitOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The Limit Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Limit Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The price threshold specified for the Limit Order. The Limit Order will only be filled by a market price
    /// that is equal to or better than this price.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The time-in-force requested for the Limit Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Limit Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Limit Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<LimitOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// The ID of the Order that this Order was intended to replace (only provided if this Order was intended to
    /// replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A StopOrderTransaction represents the creation of a Stop Order in the user’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “STOP_ORDER” in a StopOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The Stop Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Stop Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The price threshold specified for the Stop Order. The Stop Order will only be filled by a market price
    /// that is equal to or worse than this price.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this Stop Order. If the market gaps and crosses through
    /// both the price and the priceBound, the Stop Order will be cancelled instead of being filled.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the Stop Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Stop Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Stop Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an
    /// existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
}

/// A StopOrderRejectTransaction represents the rejection of the creation of a Stop Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “STOP_ORDER_REJECT” in a StopOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The Stop Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Stop Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The price threshold specified for the Stop Order. The Stop Order will only be filled by a market price
    /// that is equal to or worse than this price.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this Stop Order. If the market gaps and crosses through
    /// both the price and the priceBound, the Stop Order will be cancelled instead of being filled.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the Stop Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Stop Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Stop Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// The ID of the Order that this Order was intended to replace (only provided if this Order was intended to
    /// replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A MarketIfTouchedOrderTransaction represents the creation of a MarketIfTouched Order in the user’s
/// Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketIfTouchedOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARKET_IF_TOUCHED_ORDER” in a
    /// MarketIfTouchedOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The MarketIfTouched Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the MarketIfTouched Order. A positive number of units results in
    /// a long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The price threshold specified for the MarketIfTouched Order. The MarketIfTouched Order will only be
    /// filled by a market price that crosses this price from the direction of the market price at the time when
    /// the Order was created (the initialMarketPrice). Depending on the value of the Order’s price and
    /// initialMarketPrice, the MarketIfTouchedOrder will behave like a Limit or a Stop Order.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this MarketIfTouched Order.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the MarketIfTouched Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// MarketIfTouched Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the MarketIfTouched Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Market-if-touched Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketIfTouchedOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an
    /// existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
}

/// A MarketIfTouchedOrderRejectTransaction represents the rejection of the creation of a MarketIfTouched
/// Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketIfTouchedOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARKET_IF_TOUCHED_ORDER_REJECT” in a
    /// MarketIfTouchedOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The MarketIfTouched Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the MarketIfTouched Order. A positive number of units results in
    /// a long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    pub units: Decimal,
    /// The price threshold specified for the MarketIfTouched Order. The MarketIfTouched Order will only be
    /// filled by a market price that crosses this price from the direction of the market price at the time when
    /// the Order was created (the initialMarketPrice). Depending on the value of the Order’s price and
    /// initialMarketPrice, the MarketIfTouchedOrder will behave like a Limit or a Stop Order.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this MarketIfTouched Order.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the MarketIfTouched Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// MarketIfTouched Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the MarketIfTouched Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Market-if-touched Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketIfTouchedOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is
    /// filled (if such a Trade is created).
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when
    /// the Order is filled (if such a Trade is created).
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened
    /// when the Order is filled (if such a Trade is created).
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// The ID of the Order that this Order was intended to replace (only provided if this Order was intended to
    /// replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A TakeProfitOrderTransaction represents the creation of a TakeProfit Order in the user’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeProfitOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TAKE_PROFIT_ORDER” in a TakeProfitOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the TakeProfit Order. The associated Trade will be closed by a market
    /// price that is equal to or better than this threshold.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The time-in-force requested for the TakeProfit Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// TakeProfit Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the TakeProfit Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Take Profit Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<TakeProfitOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an
    /// existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
}

/// A TakeProfitOrderRejectTransaction represents the rejection of the creation of a TakeProfit Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeProfitOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TAKE_PROFIT_ORDER_REJECT” in a
    /// TakeProfitOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the TakeProfit Order. The associated Trade will be closed by a market
    /// price that is equal to or better than this threshold.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// The time-in-force requested for the TakeProfit Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// TakeProfit Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the TakeProfit Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Take Profit Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<TakeProfitOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order was intended to replace (only provided if this Order was intended to
    /// replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A StopLossOrderTransaction represents the creation of a StopLoss Order in the user’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “STOP_LOSS_ORDER” in a StopLossOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Stop Loss Order. The associated Trade will be closed by a market
    /// price that is equal to or worse than this threshold.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Stop Loss Order
    /// price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the StopLoss Order. Restricted to “GTC”, “GFD” and “GTD” for StopLoss
    /// Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the StopLoss Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the
    /// GuaranteedStopLossOrderMode of the account, if it is REQUIRED, the default will be true, for DISABLED or
    /// ENABLED the default is false. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "guaranteed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed: Option<bool>,
    /// The fee that will be charged if the Stop Loss Order is guaranteed and the Order is filled at the
    /// guaranteed price. The value is determined at Order creation time. It is in price units and is charged
    /// for each unit of the Trade. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "guaranteedExecutionPremium",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_premium: Option<Decimal>,
    /// The reason that the Stop Loss Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopLossOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an
    /// existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
}

/// A StopLossOrderRejectTransaction represents the rejection of the creation of a StopLoss Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “STOP_LOSS_ORDER_REJECT” in a StopLossOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Stop Loss Order. The associated Trade will be closed by a market
    /// price that is equal to or worse than this threshold.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Stop Loss Order
    /// price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the StopLoss Order. Restricted to “GTC”, “GFD” and “GTD” for StopLoss
    /// Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the StopLoss Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the
    /// GuaranteedStopLossOrderMode of the account, if it is REQUIRED, the default will be true, for DISABLED or
    /// ENABLED the default is false. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "guaranteed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed: Option<bool>,
    /// The reason that the Stop Loss Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopLossOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order was intended to replace (only provided if this Order was intended to
    /// replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A GuaranteedStopLossOrderTransaction represents the creation of a GuaranteedStopLoss Order in the user’s
/// Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “GUARANTEED_STOP_LOSS_ORDER” in a
    /// GuaranteedStopLossOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Guaranteed Stop Loss Order. The associated Trade will be closed at
    /// this price.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Guaranteed Stop
    /// Loss Order price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask
    /// is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the GuaranteedStopLoss Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// GuaranteedStopLoss Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the GuaranteedStopLoss Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The fee that will be charged if the Guaranteed Stop Loss Order is filled at the guaranteed price. The
    /// value is determined at Order creation time. It is in price units and is charged for each unit of the
    /// Trade.
    #[serde(
        rename = "guaranteedExecutionPremium",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_premium: Option<Decimal>,
    /// The reason that the Guaranteed Stop Loss Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<GuaranteedStopLossOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an
    /// existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
}

/// A GuaranteedStopLossOrderRejectTransaction represents the rejection of the creation of a
/// GuaranteedStopLoss Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “GUARANTEED_STOP_LOSS_ORDER_REJECT” in a
    /// GuaranteedStopLossOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Guaranteed Stop Loss Order. The associated Trade will be closed at
    /// this price.
    #[serde(rename = "price")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Guaranteed Stop
    /// Loss Order price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask
    /// is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the GuaranteedStopLoss Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// GuaranteedStopLoss Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the GuaranteedStopLoss Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Guaranteed Stop Loss Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<GuaranteedStopLossOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order was intended to replace (only provided if this Order was intended to
    /// replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A TrailingStopLossOrderTransaction represents the creation of a TrailingStopLoss Order in the user’s
/// Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingStopLossOrderTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TRAILING_STOP_LOSS_ORDER” in a
    /// TrailingStopLossOrderTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price distance (in price units) specified for the TrailingStopLoss Order.
    #[serde(rename = "distance")]
    pub distance: Decimal,
    /// The time-in-force requested for the TrailingStopLoss Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// TrailingStopLoss Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the StopLoss Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Trailing Stop Loss Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<TrailingStopLossOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an
    /// existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
}

/// A TrailingStopLossOrderRejectTransaction represents the rejection of the creation of a TrailingStopLoss
/// Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingStopLossOrderRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TRAILING_STOP_LOSS_ORDER_REJECT” in a
    /// TrailingStopLossOrderRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price distance (in price units) specified for the TrailingStopLoss Order.
    #[serde(rename = "distance")]
    pub distance: Decimal,
    /// The time-in-force requested for the TrailingStopLoss Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// TrailingStopLoss Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the StopLoss Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The reason that the Trailing Stop Loss Order was initiated
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<TrailingStopLossOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client
    /// extensions).
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order
    /// was created automatically when another Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order was intended to replace (only provided if this Order was intended to
    /// replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// An OrderFillTransaction represents the filling of an Order in the client’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderFillTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “ORDER_FILL” for an OrderFillTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Order filled.
    #[serde(rename = "orderID", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderID>,
    /// The client Order ID of the Order filled (only provided if the client has assigned one).
    #[serde(
        rename = "clientOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientID>,
    /// The name of the filled Order’s instrument.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The number of units filled by the OrderFill.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Decimal>,
    /// This is the conversion factor in effect for the Account at the time of the OrderFill for converting any
    /// gains realized in Instrument quote units into units of the Account’s home currency. Deprecated: Will be
    /// removed in a future API update.
    #[serde(
        rename = "gainQuoteHomeConversionFactor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gain_quote_home_conversion_factor: Option<Decimal>,
    /// This is the conversion factor in effect for the Account at the time of the OrderFill for converting any
    /// losses realized in Instrument quote units into units of the Account’s home currency. Deprecated: Will be
    /// removed in a future API update.
    #[serde(
        rename = "lossQuoteHomeConversionFactor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loss_quote_home_conversion_factor: Option<Decimal>,
    /// The HomeConversionFactors in effect at the time of the OrderFill.
    #[serde(
        rename = "homeConversionFactors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversion_factors: Option<HomeConversionFactors>,
    /// This field is now deprecated and should no longer be used. The individual tradesClosed, tradeReduced and
    /// tradeOpened fields contain the exact/official price each unit was filled at. Deprecated: Will be removed
    /// in a future API update.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// The price that all of the units of the OrderFill should have been filled at, in the absence of
    /// guaranteed price execution. This factors in the Account’s current ClientPrice, used liquidity and the
    /// units of the OrderFill only. If no Trades were closed with their price clamped for guaranteed stop loss
    /// enforcement, then this value will match the price fields of each Trade opened, closed, and reduced, and
    /// they will all be the exact same.
    #[serde(rename = "fullVWAP", default, skip_serializing_if = "Option::is_none")]
    pub full_vwap: Option<Decimal>,
    /// The price in effect for the account at the time of the Order fill.
    #[serde(rename = "fullPrice", default, skip_serializing_if = "Option::is_none")]
    pub full_price: Option<ClientPrice>,
    /// The reason that an Order was filled
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<OrderFillReason>,
    /// The profit or loss incurred when the Order was filled.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<Decimal>,
    /// The profit or loss incurred when the Order was filled, in the Instrument’s quote currency.
    #[serde(rename = "quotePL", default, skip_serializing_if = "Option::is_none")]
    pub quote_pl: Option<Decimal>,
    /// The financing paid or collected when the Order was filled.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The financing paid or collected when the Order was filled, in the Instrument’s base currency.
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_financing: Option<Decimal>,
    /// The financing paid or collected when the Order was filled, in the Instrument’s quote currency.
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_financing: Option<Decimal>,
    /// The commission charged in the Account’s home currency as a result of filling the Order. The commission
    /// is always represented as a positive quantity of the Account’s home currency, however it reduces the
    /// balance in the Account.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// The total guaranteed execution fees charged for all Trades opened, closed or reduced with guaranteed
    /// Stop Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee: Option<Decimal>,
    /// The total guaranteed execution fees charged for all Trades opened, closed or reduced with guaranteed
    /// Stop Loss Orders, expressed in the Instrument’s quote currency.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_guaranteed_execution_fee: Option<Decimal>,
    /// The Account’s balance after the Order was filled.
    #[serde(
        rename = "accountBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_balance: Option<Decimal>,
    /// The Trade that was opened when the Order was filled (only provided if filling the Order resulted in a
    /// new Trade).
    #[serde(
        rename = "tradeOpened",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened: Option<TradeOpen>,
    /// The Trades that were closed when the Order was filled (only provided if filling the Order resulted in a
    /// closing open Trades).
    #[serde(
        rename = "tradesClosed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trades_closed: Option<Vec<TradeReduce>>,
    /// The Trade that was reduced when the Order was filled (only provided if filling the Order resulted in
    /// reducing an open Trade).
    #[serde(
        rename = "tradeReduced",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced: Option<TradeReduce>,
    /// The half spread cost for the OrderFill, which is the sum of the halfSpreadCost values in the
    /// tradeOpened, tradesClosed and tradeReduced fields. This can be a positive or negative value and is
    /// represented in the home currency of the Account.
    #[serde(
        rename = "halfSpreadCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub half_spread_cost: Option<Decimal>,
}

/// An OrderCancelTransaction represents the cancellation of an Order in the client’s Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancelTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “ORDER_CANCEL” for an OrderCancelTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Order cancelled
    #[serde(rename = "orderID", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderID>,
    /// The client ID of the Order cancelled (only provided if the Order has a client Order ID).
    #[serde(
        rename = "clientOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<OrderID>,
    /// The reason that the Order was cancelled.
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<OrderCancelReason>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled for replacement).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// An OrderCancelRejectTransaction represents the rejection of the cancellation of an Order in the client’s
/// Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancelRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “ORDER_CANCEL_REJECT” for an OrderCancelRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Order intended to be cancelled
    #[serde(rename = "orderID", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderID>,
    /// The client ID of the Order intended to be cancelled (only provided if the Order has a client Order ID).
    #[serde(
        rename = "clientOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A OrderClientExtensionsModifyTransaction represents the modification of an Order’s Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderClientExtensionsModifyTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “ORDER_CLIENT_EXTENSIONS_MODIFY” for a
    /// OrderClientExtensionsModifyTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Order who’s client extensions are to be modified.
    #[serde(rename = "orderID", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderID>,
    /// The original Client ID of the Order who’s client extensions are to be modified.
    #[serde(
        rename = "clientOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientID>,
    /// The new Client Extensions for the Order.
    #[serde(
        rename = "clientExtensionsModify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions_modify: Option<ClientExtensions>,
    /// The new Client Extensions for the Order’s Trade on fill.
    #[serde(
        rename = "tradeClientExtensionsModify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,
}

/// A OrderClientExtensionsModifyRejectTransaction represents the rejection of the modification of an
/// Order’s Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderClientExtensionsModifyRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT” for a
    /// OrderClientExtensionsModifyRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Order who’s client extensions are to be modified.
    #[serde(rename = "orderID", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderID>,
    /// The original Client ID of the Order who’s client extensions are to be modified.
    #[serde(
        rename = "clientOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientID>,
    /// The new Client Extensions for the Order.
    #[serde(
        rename = "clientExtensionsModify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions_modify: Option<ClientExtensions>,
    /// The new Client Extensions for the Order’s Trade on fill.
    #[serde(
        rename = "tradeClientExtensionsModify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A TradeClientExtensionsModifyTransaction represents the modification of a Trade’s Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeClientExtensionsModifyTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TRADE_CLIENT_EXTENSIONS_MODIFY” for a
    /// TradeClientExtensionsModifyTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade who’s client extensions are to be modified.
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The original Client ID of the Trade who’s client extensions are to be modified.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The new Client Extensions for the Trade.
    #[serde(
        rename = "tradeClientExtensionsModify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,
}

/// A TradeClientExtensionsModifyRejectTransaction represents the rejection of the modification of a Trade’s
/// Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeClientExtensionsModifyRejectTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT” for a
    /// TradeClientExtensionsModifyRejectTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The ID of the Trade who’s client extensions are to be modified.
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The original Client ID of the Trade who’s client extensions are to be modified.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The new Client Extensions for the Trade.
    #[serde(
        rename = "tradeClientExtensionsModify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A MarginCallEnterTransaction is created when an Account enters the margin call state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginCallEnterTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARGIN_CALL_ENTER” for an MarginCallEnterTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
}

/// A MarginCallExtendTransaction is created when the margin call state for an Account has been extended.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginCallExtendTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARGIN_CALL_EXTEND” for an MarginCallExtendTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The number of the extensions to the Account’s current margin call that have been applied. This value
    /// will be set to 1 for the first MarginCallExtend Transaction
    #[serde(
        rename = "extensionNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_number: Option<i64>,
}

/// A MarginCallExitTransaction is created when an Account leaves the margin call state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginCallExitTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARGIN_CALL_EXIT” for an MarginCallExitTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
}

/// A DelayedTradeClosure Transaction is created administratively to indicate open trades that should have
/// been closed but weren’t because the open trades’ instruments were untradeable at the time. Open trades
/// listed in this transaction will be closed once their respective instruments become tradeable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayedTradeClosureTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “DELAYED_TRADE_CLOSURE” for an DelayedTradeClosureTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The reason for the delayed trade closure
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderReason>,
    /// List of Trade ID’s identifying the open trades that will be closed when their respective instruments
    /// become tradeable
    #[serde(rename = "tradeIDs", default, skip_serializing_if = "Option::is_none")]
    pub trade_ids: Option<TradeID>,
}

/// A DailyFinancingTransaction represents the daily payment/collection of financing for an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyFinancingTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “DAILY_FINANCING” for a DailyFinancingTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The amount of financing paid/collected for the Account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The Account’s balance after daily financing.
    #[serde(
        rename = "accountBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_balance: Option<Decimal>,
    /// The account financing mode at the time of the daily financing. This field is no longer in use moving
    /// forward and was replaced by accountFinancingMode in individual positionFinancings since the financing
    /// mode could differ between instruments. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "accountFinancingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_financing_mode: Option<AccountFinancingMode>,
    /// The financing paid/collected for each Position in the Account.
    #[serde(
        rename = "positionFinancings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_financings: Option<Vec<PositionFinancing>>,
}

/// A DividendAdjustment Transaction is created administratively to pay or collect dividend adjustment
/// mounts to or from an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividendAdjustmentTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “DIVIDEND_ADJUSTMENT” for a DividendAdjustmentTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
    /// The name of the instrument for the dividendAdjustment transaction
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The total dividend adjustment amount paid or collected in the Account’s home currency for the Account as
    /// a result of applying the DividendAdjustment Transaction. This is the sum of the dividend adjustments
    /// paid/collected for each OpenTradeDividendAdjustment found within the Transaction.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The total dividend adjustment amount paid or collected in the Instrument’s quote currency for the
    /// Account as a result of applying the DividendAdjustment Transaction. This is the sum of the quote
    /// dividend adjustments paid/collected for each OpenTradeDividendAdjustment found within the Transaction.
    #[serde(
        rename = "quoteDividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_dividend_adjustment: Option<Decimal>,
    /// The HomeConversionFactors in effect at the time of the DividendAdjustment.
    #[serde(
        rename = "homeConversionFactors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversion_factors: Option<HomeConversionFactors>,
    /// The Account balance after applying the DividendAdjustment Transaction
    #[serde(
        rename = "accountBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_balance: Option<Decimal>,
    /// The dividend adjustment payment/collection details for each open Trade, within the Account, for which a
    /// dividend adjustment is to be paid or collected.
    #[serde(
        rename = "openTradeDividendAdjustments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_trade_dividend_adjustments: Option<Vec<OpenTradeDividendAdjustment>>,
}

/// A ResetResettablePLTransaction represents the resetting of the Account’s resettable PL counters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetResettablePLTransaction {
    /// The Transaction’s Identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the
    /// Account simultaneously.
    #[serde(rename = "batchID", default, skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “RESET_RESETTABLE_PL” for a ResetResettablePLTransaction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransactionType>,
}

/// The possible types of a Transaction
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransactionType {
    /// Account Create Transaction
    Create,
    /// Account Close Transaction
    Close,
    /// Account Reopen Transaction
    Reopen,
    /// Client Configuration Transaction
    ClientConfigure,
    /// Client Configuration Reject Transaction
    ClientConfigureReject,
    /// Transfer Funds Transaction
    TransferFunds,
    /// Transfer Funds Reject Transaction
    TransferFundsReject,
    /// Market Order Transaction
    MarketOrder,
    /// Market Order Reject Transaction
    MarketOrderReject,
    /// Fixed Price Order Transaction
    FixedPriceOrder,
    /// Limit Order Transaction
    LimitOrder,
    /// Limit Order Reject Transaction
    LimitOrderReject,
    /// Stop Order Transaction
    StopOrder,
    /// Stop Order Reject Transaction
    StopOrderReject,
    /// Market if Touched Order Transaction
    MarketIfTouchedOrder,
    /// Market if Touched Order Reject Transaction
    MarketIfTouchedOrderReject,
    /// Take Profit Order Transaction
    TakeProfitOrder,
    /// Take Profit Order Reject Transaction
    TakeProfitOrderReject,
    /// Stop Loss Order Transaction
    StopLossOrder,
    /// Stop Loss Order Reject Transaction
    StopLossOrderReject,
    /// Guaranteed Stop Loss Order Transaction
    GuaranteedStopLossOrder,
    /// Guaranteed Stop Loss Order Reject Transaction
    GuaranteedStopLossOrderReject,
    /// Trailing Stop Loss Order Transaction
    TrailingStopLossOrder,
    /// Trailing Stop Loss Order Reject Transaction
    TrailingStopLossOrderReject,
    /// Order Fill Transaction
    OrderFill,
    /// Order Cancel Transaction
    OrderCancel,
    /// Order Cancel Reject Transaction
    OrderCancelReject,
    /// Order Client Extensions Modify Transaction
    OrderClientExtensionsModify,
    /// Order Client Extensions Modify Reject Transaction
    OrderClientExtensionsModifyReject,
    /// Trade Client Extensions Modify Transaction
    TradeClientExtensionsModify,
    /// Trade Client Extensions Modify Reject Transaction
    TradeClientExtensionsModifyReject,
    /// Margin Call Enter Transaction
    MarginCallEnter,
    /// Margin Call Extend Transaction
    MarginCallExtend,
    /// Margin Call Exit Transaction
    MarginCallExit,
    /// Delayed Trade Closure Transaction
    DelayedTradeClosure,
    /// Daily Financing Transaction
    DailyFinancing,
    /// Dividend Adjustment Transaction
    DividendAdjustment,
    /// Reset Resettable PL Transaction
    ResetResettablePl,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TransactionType {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Create => "CREATE",
            Self::Close => "CLOSE",
            Self::Reopen => "REOPEN",
            Self::ClientConfigure => "CLIENT_CONFIGURE",
            Self::ClientConfigureReject => "CLIENT_CONFIGURE_REJECT",
            Self::TransferFunds => "TRANSFER_FUNDS",
            Self::TransferFundsReject => "TRANSFER_FUNDS_REJECT",
            Self::MarketOrder => "MARKET_ORDER",
            Self::MarketOrderReject => "MARKET_ORDER_REJECT",
            Self::FixedPriceOrder => "FIXED_PRICE_ORDER",
            Self::LimitOrder => "LIMIT_ORDER",
            Self::LimitOrderReject => "LIMIT_ORDER_REJECT",
            Self::StopOrder => "STOP_ORDER",
            Self::StopOrderReject => "STOP_ORDER_REJECT",
            Self::MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
            Self::MarketIfTouchedOrderReject => "MARKET_IF_TOUCHED_ORDER_REJECT",
            Self::TakeProfitOrder => "TAKE_PROFIT_ORDER",
            Self::TakeProfitOrderReject => "TAKE_PROFIT_ORDER_REJECT",
            Self::StopLossOrder => "STOP_LOSS_ORDER",
            Self::StopLossOrderReject => "STOP_LOSS_ORDER_REJECT",
            Self::GuaranteedStopLossOrder => "GUARANTEED_STOP_LOSS_ORDER",
            Self::GuaranteedStopLossOrderReject => "GUARANTEED_STOP_LOSS_ORDER_REJECT",
            Self::TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
            Self::TrailingStopLossOrderReject => "TRAILING_STOP_LOSS_ORDER_REJECT",
            Self::OrderFill => "ORDER_FILL",
            Self::OrderCancel => "ORDER_CANCEL",
            Self::OrderCancelReject => "ORDER_CANCEL_REJECT",
            Self::OrderClientExtensionsModify => "ORDER_CLIENT_EXTENSIONS_MODIFY",
            Self::OrderClientExtensionsModifyReject => "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
            Self::TradeClientExtensionsModify => "TRADE_CLIENT_EXTENSIONS_MODIFY",
            Self::TradeClientExtensionsModifyReject => "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
            Self::MarginCallEnter => "MARGIN_CALL_ENTER",
            Self::MarginCallExtend => "MARGIN_CALL_EXTEND",
            Self::MarginCallExit => "MARGIN_CALL_EXIT",
            Self::DelayedTradeClosure => "DELAYED_TRADE_CLOSURE",
            Self::DailyFinancing => "DAILY_FINANCING",
            Self::DividendAdjustment => "DIVIDEND_ADJUSTMENT",
            Self::ResetResettablePl => "RESET_RESETTABLE_PL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TransactionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CREATE" => Self::Create,
            "CLOSE" => Self::Close,
            "REOPEN" => Self::Reopen,
            "CLIENT_CONFIGURE" => Self::ClientConfigure,
            "CLIENT_CONFIGURE_REJECT" => Self::ClientConfigureReject,
            "TRANSFER_FUNDS" => Self::TransferFunds,
            "TRANSFER_FUNDS_REJECT" => Self::TransferFundsReject,
            "MARKET_ORDER" => Self::MarketOrder,
            "MARKET_ORDER_REJECT" => Self::MarketOrderReject,
            "FIXED_PRICE_ORDER" => Self::FixedPriceOrder,
            "LIMIT_ORDER" => Self::LimitOrder,
            "LIMIT_ORDER_REJECT" => Self::LimitOrderReject,
            "STOP_ORDER" => Self::StopOrder,
            "STOP_ORDER_REJECT" => Self::StopOrderReject,
            "MARKET_IF_TOUCHED_ORDER" => Self::MarketIfTouchedOrder,
            "MARKET_IF_TOUCHED_ORDER_REJECT" => Self::MarketIfTouchedOrderReject,
            "TAKE_PROFIT_ORDER" => Self::TakeProfitOrder,
            "TAKE_PROFIT_ORDER_REJECT" => Self::TakeProfitOrderReject,
            "STOP_LOSS_ORDER" => Self::StopLossOrder,
            "STOP_LOSS_ORDER_REJECT" => Self::StopLossOrderReject,
            "GUARANTEED_STOP_LOSS_ORDER" => Self::GuaranteedStopLossOrder,
            "GUARANTEED_STOP_LOSS_ORDER_REJECT" => Self::GuaranteedStopLossOrderReject,
            "TRAILING_STOP_LOSS_ORDER" => Self::TrailingStopLossOrder,
            "TRAILING_STOP_LOSS_ORDER_REJECT" => Self::TrailingStopLossOrderReject,
            "ORDER_FILL" => Self::OrderFill,
            "ORDER_CANCEL" => Self::OrderCancel,
            "ORDER_CANCEL_REJECT" => Self::OrderCancelReject,
            "ORDER_CLIENT_EXTENSIONS_MODIFY" => Self::OrderClientExtensionsModify,
            "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT" => Self::OrderClientExtensionsModifyReject,
            "TRADE_CLIENT_EXTENSIONS_MODIFY" => Self::TradeClientExtensionsModify,
            "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT" => Self::TradeClientExtensionsModifyReject,
            "MARGIN_CALL_ENTER" => Self::MarginCallEnter,
            "MARGIN_CALL_EXTEND" => Self::MarginCallExtend,
            "MARGIN_CALL_EXIT" => Self::MarginCallExit,
            "DELAYED_TRADE_CLOSURE" => Self::DelayedTradeClosure,
            "DAILY_FINANCING" => Self::DailyFinancing,
            "DIVIDEND_ADJUSTMENT" => Self::DividendAdjustment,
            "RESET_RESETTABLE_PL" => Self::ResetResettablePl,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that an Account is being funded.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FundingReason {
    /// The client has initiated a funds transfer
    ClientFunding,
    /// Funds are being transferred between two Accounts.
    AccountTransfer,
    /// Funds are being transferred as part of a Division migration
    DivisionMigration,
    /// Funds are being transferred as part of a Site migration
    SiteMigration,
    /// Funds are being transferred as part of an Account adjustment
    Adjustment,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl FundingReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientFunding => "CLIENT_FUNDING",
            Self::AccountTransfer => "ACCOUNT_TRANSFER",
            Self::DivisionMigration => "DIVISION_MIGRATION",
            Self::SiteMigration => "SITE_MIGRATION",
            Self::Adjustment => "ADJUSTMENT",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for FundingReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for FundingReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_FUNDING" => Self::ClientFunding,
            "ACCOUNT_TRANSFER" => Self::AccountTransfer,
            "DIVISION_MIGRATION" => Self::DivisionMigration,
            "SITE_MIGRATION" => Self::SiteMigration,
            "ADJUSTMENT" => Self::Adjustment,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Market Order was created
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MarketOrderReason {
    /// The Market Order was created at the request of a client
    ClientOrder,
    /// The Market Order was created to close a Trade at the request of a client
    TradeClose,
    /// The Market Order was created to close a Position at the request of a client
    PositionCloseout,
    /// The Market Order was created as part of a Margin Closeout
    MarginCloseout,
    /// The Market Order was created to close a trade marked for delayed closure
    DelayedTradeClose,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl MarketOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::TradeClose => "TRADE_CLOSE",
            Self::PositionCloseout => "POSITION_CLOSEOUT",
            Self::MarginCloseout => "MARGIN_CLOSEOUT",
            Self::DelayedTradeClose => "DELAYED_TRADE_CLOSE",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for MarketOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MarketOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "TRADE_CLOSE" => Self::TradeClose,
            "POSITION_CLOSEOUT" => Self::PositionCloseout,
            "MARGIN_CLOSEOUT" => Self::MarginCloseout,
            "DELAYED_TRADE_CLOSE" => Self::DelayedTradeClose,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Fixed Price Order was created
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FixedPriceOrderReason {
    /// The Fixed Price Order was created as part of a platform account migration
    PlatformAccountMigration,
    /// The Fixed Price Order was created to close a Trade as part of division account migration
    TradeCloseDivisionAccountMigration,
    /// The Fixed Price Order was created to close a Trade administratively
    TradeCloseAdministrativeAction,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl FixedPriceOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::PlatformAccountMigration => "PLATFORM_ACCOUNT_MIGRATION",
            Self::TradeCloseDivisionAccountMigration => "TRADE_CLOSE_DIVISION_ACCOUNT_MIGRATION",
            Self::TradeCloseAdministrativeAction => "TRADE_CLOSE_ADMINISTRATIVE_ACTION",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for FixedPriceOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for FixedPriceOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "PLATFORM_ACCOUNT_MIGRATION" => Self::PlatformAccountMigration,
            "TRADE_CLOSE_DIVISION_ACCOUNT_MIGRATION" => Self::TradeCloseDivisionAccountMigration,
            "TRADE_CLOSE_ADMINISTRATIVE_ACTION" => Self::TradeCloseAdministrativeAction,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Limit Order was initiated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LimitOrderReason {
    /// The Limit Order was initiated at the request of a client
    ClientOrder,
    /// The Limit Order was initiated as a replacement for an existing Order
    Replacement,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl LimitOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::Replacement => "REPLACEMENT",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for LimitOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for LimitOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "REPLACEMENT" => Self::Replacement,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Stop Order was initiated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StopOrderReason {
    /// The Stop Order was initiated at the request of a client
    ClientOrder,
    /// The Stop Order was initiated as a replacement for an existing Order
    Replacement,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl StopOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::Replacement => "REPLACEMENT",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for StopOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StopOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "REPLACEMENT" => Self::Replacement,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Market-if-touched Order was initiated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MarketIfTouchedOrderReason {
    /// The Market-if-touched Order was initiated at the request of a client
    ClientOrder,
    /// The Market-if-touched Order was initiated as a replacement for an existing Order
    Replacement,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl MarketIfTouchedOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::Replacement => "REPLACEMENT",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for MarketIfTouchedOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MarketIfTouchedOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "REPLACEMENT" => Self::Replacement,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Take Profit Order was initiated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TakeProfitOrderReason {
    /// The Take Profit Order was initiated at the request of a client
    ClientOrder,
    /// The Take Profit Order was initiated as a replacement for an existing Order
    Replacement,
    /// The Take Profit Order was initiated automatically when an Order was filled that opened a new Trade
    /// requiring a Take Profit Order.
    OnFill,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TakeProfitOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::Replacement => "REPLACEMENT",
            Self::OnFill => "ON_FILL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TakeProfitOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TakeProfitOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "REPLACEMENT" => Self::Replacement,
            "ON_FILL" => Self::OnFill,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Stop Loss Order was initiated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StopLossOrderReason {
    /// The Stop Loss Order was initiated at the request of a client
    ClientOrder,
    /// The Stop Loss Order was initiated as a replacement for an existing Order
    Replacement,
    /// The Stop Loss Order was initiated automatically when an Order was filled that opened a new Trade
    /// requiring a Stop Loss Order.
    OnFill,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl StopLossOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::Replacement => "REPLACEMENT",
            Self::OnFill => "ON_FILL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for StopLossOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StopLossOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "REPLACEMENT" => Self::Replacement,
            "ON_FILL" => Self::OnFill,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Guaranteed Stop Loss Order was initiated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GuaranteedStopLossOrderReason {
    /// The Guaranteed Stop Loss Order was initiated at the request of a client
    ClientOrder,
    /// The Guaranteed Stop Loss Order was initiated as a replacement for an existing Order
    Replacement,
    /// The Guaranteed Stop Loss Order was initiated automatically when an Order was filled that opened a new
    /// Trade requiring a Guaranteed Stop Loss Order.
    OnFill,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl GuaranteedStopLossOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::Replacement => "REPLACEMENT",
            Self::OnFill => "ON_FILL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for GuaranteedStopLossOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for GuaranteedStopLossOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "REPLACEMENT" => Self::Replacement,
            "ON_FILL" => Self::OnFill,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that the Trailing Stop Loss Order was initiated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TrailingStopLossOrderReason {
    /// The Trailing Stop Loss Order was initiated at the request of a client
    ClientOrder,
    /// The Trailing Stop Loss Order was initiated as a replacement for an existing Order
    Replacement,
    /// The Trailing Stop Loss Order was initiated automatically when an Order was filled that opened a new
    /// Trade requiring a Trailing Stop Loss Order.
    OnFill,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TrailingStopLossOrderReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientOrder => "CLIENT_ORDER",
            Self::Replacement => "REPLACEMENT",
            Self::OnFill => "ON_FILL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TrailingStopLossOrderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TrailingStopLossOrderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CLIENT_ORDER" => Self::ClientOrder,
            "REPLACEMENT" => Self::Replacement,
            "ON_FILL" => Self::OnFill,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that an Order was filled
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderFillReason {
    /// The Order filled was a Limit Order
    LimitOrder,
    /// The Order filled was a Stop Order
    StopOrder,
    /// The Order filled was a Market-if-touched Order
    MarketIfTouchedOrder,
    /// The Order filled was a Take Profit Order
    TakeProfitOrder,
    /// The Order filled was a Stop Loss Order
    StopLossOrder,
    /// The Order filled was a Guaranteed Stop Loss Order
    GuaranteedStopLossOrder,
    /// The Order filled was a Trailing Stop Loss Order
    TrailingStopLossOrder,
    /// The Order filled was a Market Order
    MarketOrder,
    /// The Order filled was a Market Order used to explicitly close a Trade
    MarketOrderTradeClose,
    /// The Order filled was a Market Order used to explicitly close a Position
    MarketOrderPositionCloseout,
    /// The Order filled was a Market Order used for a Margin Closeout
    MarketOrderMarginCloseout,
    /// The Order filled was a Market Order used for a delayed Trade close
    MarketOrderDelayedTradeClose,
    /// The Order filled was a Fixed Price Order
    FixedPriceOrder,
    /// The Order filled was a Fixed Price Order created as part of a platform account migration
    FixedPriceOrderPlatformAccountMigration,
    /// The Order filled was a Fixed Price Order created to close a Trade as part of division account migration
    FixedPriceOrderDivisionAccountMigration,
    /// The Order filled was a Fixed Price Order created to close a Trade administratively
    FixedPriceOrderAdministrativeAction,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl OrderFillReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::LimitOrder => "LIMIT_ORDER",
            Self::StopOrder => "STOP_ORDER",
            Self::MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
            Self::TakeProfitOrder => "TAKE_PROFIT_ORDER",
            Self::StopLossOrder => "STOP_LOSS_ORDER",
            Self::GuaranteedStopLossOrder => "GUARANTEED_STOP_LOSS_ORDER",
            Self::TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
            Self::MarketOrder => "MARKET_ORDER",
            Self::MarketOrderTradeClose => "MARKET_ORDER_TRADE_CLOSE",
            Self::MarketOrderPositionCloseout => "MARKET_ORDER_POSITION_CLOSEOUT",
            Self::MarketOrderMarginCloseout => "MARKET_ORDER_MARGIN_CLOSEOUT",
            Self::MarketOrderDelayedTradeClose => "MARKET_ORDER_DELAYED_TRADE_CLOSE",
            Self::FixedPriceOrder => "FIXED_PRICE_ORDER",
            Self::FixedPriceOrderPlatformAccountMigration => {
                "FIXED_PRICE_ORDER_PLATFORM_ACCOUNT_MIGRATION"
            }
            Self::FixedPriceOrderDivisionAccountMigration => {
                "FIXED_PRICE_ORDER_DIVISION_ACCOUNT_MIGRATION"
            }
            Self::FixedPriceOrderAdministrativeAction => "FIXED_PRICE_ORDER_ADMINISTRATIVE_ACTION",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for OrderFillReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OrderFillReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "LIMIT_ORDER" => Self::LimitOrder,
            "STOP_ORDER" => Self::StopOrder,
            "MARKET_IF_TOUCHED_ORDER" => Self::MarketIfTouchedOrder,
            "TAKE_PROFIT_ORDER" => Self::TakeProfitOrder,
            "STOP_LOSS_ORDER" => Self::StopLossOrder,
            "GUARANTEED_STOP_LOSS_ORDER" => Self::GuaranteedStopLossOrder,
            "TRAILING_STOP_LOSS_ORDER" => Self::TrailingStopLossOrder,
            "MARKET_ORDER" => Self::MarketOrder,
            "MARKET_ORDER_TRADE_CLOSE" => Self::MarketOrderTradeClose,
            "MARKET_ORDER_POSITION_CLOSEOUT" => Self::MarketOrderPositionCloseout,
            "MARKET_ORDER_MARGIN_CLOSEOUT" => Self::MarketOrderMarginCloseout,
            "MARKET_ORDER_DELAYED_TRADE_CLOSE" => Self::MarketOrderDelayedTradeClose,
            "FIXED_PRICE_ORDER" => Self::FixedPriceOrder,
            "FIXED_PRICE_ORDER_PLATFORM_ACCOUNT_MIGRATION" => {
                Self::FixedPriceOrderPlatformAccountMigration
            }
            "FIXED_PRICE_ORDER_DIVISION_ACCOUNT_MIGRATION" => {
                Self::FixedPriceOrderDivisionAccountMigration
            }
            "FIXED_PRICE_ORDER_ADMINISTRATIVE_ACTION" => Self::FixedPriceOrderAdministrativeAction,
            _ => Self::Unknown(value),
        })
    }
}

/// The reason that an Order was cancelled.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderCancelReason {
    /// The Order was cancelled because at the time of filling, an unexpected internal server error occurred.
    InternalServerError,
    /// The Order was cancelled because at the time of filling the account was locked.
    AccountLocked,
    /// The order was to be filled, however the account is configured to not allow new positions to be created.
    AccountNewPositionsLocked,
    /// Filling the Order wasn’t possible because it required the creation of a dependent Order and the Account
    /// is locked for Order creation.
    AccountOrderCreationLocked,
    /// Filling the Order was not possible because the Account is locked for filling Orders.
    AccountOrderFillLocked,
    /// The Order was cancelled explicitly at the request of the client.
    ClientRequest,
    /// The Order cancelled because it is being migrated to another account.
    Migration,
    /// Filling the Order wasn’t possible because the Order’s instrument was halted.
    MarketHalted,
    /// The Order is linked to an open Trade that was closed.
    LinkedTradeClosed,
    /// The time in force specified for this order has passed.
    TimeInForceExpired,
    /// Filling the Order wasn’t possible because the Account had insufficient margin.
    InsufficientMargin,
    /// Filling the Order would have resulted in a a FIFO violation.
    FifoViolation,
    /// Filling the Order would have violated the Order’s price bound.
    BoundsViolation,
    /// The Order was cancelled for replacement at the request of the client.
    ClientRequestReplaced,
    /// The Order was cancelled for replacement with an adjusted fillPrice to accommodate for the price movement
    /// caused by a dividendAdjustment.
    DividendAdjustmentReplaced,
    /// Filling the Order wasn’t possible because enough liquidity available.
    InsufficientLiquidity,
    /// Filling the Order would have resulted in the creation of a Take Profit Order with a GTD time in the past.
    TakeProfitOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a Take Profit Order that would have been filled
    /// immediately, closing the new Trade at a loss.
    TakeProfitOnFillLoss,
    /// Filling the Order would result in the creation of a Take Profit Loss Order that would close the new
    /// Trade at a loss when filled.
    LosingTakeProfit,
    /// Filling the Order would have resulted in the creation of a Stop Loss Order with a GTD time in the past.
    StopLossOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a Stop Loss Order that would have been filled
    /// immediately, closing the new Trade at a loss.
    StopLossOnFillLoss,
    /// Filling the Order would result in the creation of a Stop Loss Order whose price would be zero or
    /// negative due to the specified distance.
    StopLossOnFillPriceDistanceMaximumExceeded,
    /// Filling the Order would not result in the creation of Stop Loss Order, however the Account’s
    /// configuration requires that all Trades have a Stop Loss Order attached to them.
    StopLossOnFillRequired,
    /// Filling the Order would not result in the creation of a guaranteed Stop Loss Order, however the
    /// Account’s configuration requires that all Trades have a guaranteed Stop Loss Order attached to them.
    StopLossOnFillGuaranteedRequired,
    /// Filling the Order would result in the creation of a guaranteed Stop Loss Order, however the Account’s
    /// configuration does not allow guaranteed Stop Loss Orders.
    StopLossOnFillGuaranteedNotAllowed,
    /// Filling the Order would result in the creation of a guaranteed Stop Loss Order with a distance smaller
    /// than the configured minimum distance.
    StopLossOnFillGuaranteedMinimumDistanceNotMet,
    /// Filling the Order would result in the creation of a guaranteed Stop Loss Order with trigger price and
    /// number of units that that violates the account’s guaranteed Stop Loss Order level restriction.
    StopLossOnFillGuaranteedLevelRestrictionExceeded,
    /// Filling the Order would result in the creation of a guaranteed Stop Loss Order for a hedged Trade,
    /// however the Account’s configuration does not allow guaranteed Stop Loss Orders for hedged
    /// Trades/Positions.
    StopLossOnFillGuaranteedHedgingNotAllowed,
    /// Filling the Order would result in the creation of a Stop Loss Order whose TimeInForce value is invalid.
    /// A likely cause would be if the Account requires guaranteed stop loss orders and the TimeInForce value
    /// were not GTC.
    StopLossOnFillTimeInForceInvalid,
    /// Filling the Order would result in the creation of a Stop Loss Order whose TriggerCondition value is
    /// invalid. A likely cause would be if the stop loss order is guaranteed and the TimeInForce is not
    /// TRIGGER_DEFAULT or TRIGGER_BID for a long trade, or not TRIGGER_DEFAULT or TRIGGER_ASK for a short trade.
    StopLossOnFillTriggerConditionInvalid,
    /// Filling the Order would have resulted in the creation of a Guaranteed Stop Loss Order with a GTD time in
    /// the past.
    GuaranteedStopLossOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order that would have been
    /// filled immediately, closing the new Trade at a loss.
    GuaranteedStopLossOnFillLoss,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order whose price would be zero
    /// or negative due to the specified distance.
    GuaranteedStopLossOnFillPriceDistanceMaximumExceeded,
    /// Filling the Order would not result in the creation of a Guaranteed Stop Loss Order, however the
    /// Account’s configuration requires that all Trades have a Guaranteed Stop Loss Order attached to them.
    GuaranteedStopLossOnFillRequired,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order, however the Account’s
    /// configuration does not allow Guaranteed Stop Loss Orders.
    GuaranteedStopLossOnFillNotAllowed,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with a distance smaller
    /// than the configured minimum distance.
    GuaranteedStopLossOnFillMinimumDistanceNotMet,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger number of
    /// units that violates the account’s Guaranteed Stop Loss Order level restriction volume.
    GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger price that
    /// violates the account’s Guaranteed Stop Loss Order level restriction price range.
    GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order for a hedged Trade,
    /// however the Account’s configuration does not allow Guaranteed Stop Loss Orders for hedged
    /// Trades/Positions.
    GuaranteedStopLossOnFillHedgingNotAllowed,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order whose TimeInForce value
    /// is invalid. A likely cause would be if the Account requires guaranteed stop loss orders and the
    /// TimeInForce value were not GTC.
    GuaranteedStopLossOnFillTimeInForceInvalid,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order whose TriggerCondition
    /// value is invalid. A likely cause would be the TimeInForce is not TRIGGER_DEFAULT or TRIGGER_BID for a
    /// long trade, or not TRIGGER_DEFAULT or TRIGGER_ASK for a short trade.
    GuaranteedStopLossOnFillTriggerConditionInvalid,
    /// Filling the Order would result in the creation of a Take Profit Order whose price would be zero or
    /// negative due to the specified distance.
    TakeProfitOnFillPriceDistanceMaximumExceeded,
    /// Filling the Order would have resulted in the creation of a Trailing Stop Loss Order with a GTD time in
    /// the past.
    TrailingStopLossOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a new Open Trade with a client Trade ID already in use.
    ClientTradeIdAlreadyExists,
    /// Closing out a position wasn’t fully possible.
    PositionCloseoutFailed,
    /// Filling the Order would cause the maximum open trades allowed for the Account to be exceeded.
    OpenTradesAllowedExceeded,
    /// Filling the Order would have resulted in exceeding the number of pending Orders allowed for the Account.
    PendingOrdersAllowedExceeded,
    /// Filling the Order would have resulted in the creation of a Take Profit Order with a client Order ID that
    /// is already in use.
    TakeProfitOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the creation of a Stop Loss Order with a client Order ID that
    /// is already in use.
    StopLossOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the creation of a Guaranteed Stop Loss Order with a client
    /// Order ID that is already in use.
    GuaranteedStopLossOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the creation of a Trailing Stop Loss Order with a client Order
    /// ID that is already in use.
    TrailingStopLossOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the Account’s maximum position size limit being exceeded for
    /// the Order’s instrument.
    PositionSizeExceeded,
    /// Filling the Order would result in the creation of a Trade, however there already exists an opposing
    /// (hedged) Trade that has a guaranteed Stop Loss Order attached to it. Guaranteed Stop Loss Orders cannot
    /// be combined with hedged positions.
    HedgingGsloViolation,
    /// Filling the order would cause the maximum position value allowed for the account to be exceeded. The
    /// Order has been cancelled as a result.
    AccountPositionValueLimitExceeded,
    /// Filling the order would require the creation of a short trade, however the instrument is configured such
    /// that orders being filled using bid prices can only reduce existing positions. New short positions cannot
    /// be created, but existing long positions may be reduced or closed.
    InstrumentBidReduceOnly,
    /// Filling the order would require the creation of a long trade, however the instrument is configured such
    /// that orders being filled using ask prices can only reduce existing positions. New long positions cannot
    /// be created, but existing short positions may be reduced or closed.
    InstrumentAskReduceOnly,
    /// Filling the order would require using the bid, however the instrument is configured such that the bids
    /// are halted, and so no short orders may be filled.
    InstrumentBidHalted,
    /// Filling the order would require using the ask, however the instrument is configured such that the asks
    /// are halted, and so no long orders may be filled.
    InstrumentAskHalted,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
    /// is long the GSLO would be short, however the bid side is currently halted. GSLOs cannot be created in
    /// this situation.
    StopLossOnFillGuaranteedBidHalted,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
    /// is short the GSLO would be long, however the ask side is currently halted. GSLOs cannot be created in
    /// this situation.
    StopLossOnFillGuaranteedAskHalted,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
    /// is long the GSLO would be short, however the bid side is currently halted. GSLOs cannot be created in
    /// this situation.
    GuaranteedStopLossOnFillBidHalted,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
    /// is short the GSLO would be long, however the ask side is currently halted. GSLOs cannot be created in
    /// this situation.
    GuaranteedStopLossOnFillAskHalted,
    /// Filling the Order would have resulted in a new Trade that violates the FIFO violation safeguard
    /// constraints.
    FifoViolationSafeguardViolation,
    /// Filling the Order would have reduced an existing Trade such that the reduced Trade violates the FIFO
    /// violation safeguard constraints.
    FifoViolationSafeguardPartialCloseViolation,
    /// The Orders on fill would be in violation of the risk management Order mutual exclusivity configuration
    /// specifying that only one risk management Order can be attached to a Trade.
    OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl OrderCancelReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
            Self::AccountLocked => "ACCOUNT_LOCKED",
            Self::AccountNewPositionsLocked => "ACCOUNT_NEW_POSITIONS_LOCKED",
            Self::AccountOrderCreationLocked => "ACCOUNT_ORDER_CREATION_LOCKED",
            Self::AccountOrderFillLocked => "ACCOUNT_ORDER_FILL_LOCKED",
            Self::ClientRequest => "CLIENT_REQUEST",
            Self::Migration => "MIGRATION",
            Self::MarketHalted => "MARKET_HALTED",
            Self::LinkedTradeClosed => "LINKED_TRADE_CLOSED",
            Self::TimeInForceExpired => "TIME_IN_FORCE_EXPIRED",
            Self::InsufficientMargin => "INSUFFICIENT_MARGIN",
            Self::FifoViolation => "FIFO_VIOLATION",
            Self::BoundsViolation => "BOUNDS_VIOLATION",
            Self::ClientRequestReplaced => "CLIENT_REQUEST_REPLACED",
            Self::DividendAdjustmentReplaced => "DIVIDEND_ADJUSTMENT_REPLACED",
            Self::InsufficientLiquidity => "INSUFFICIENT_LIQUIDITY",
            Self::TakeProfitOnFillGtdTimestampInPast => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
            Self::TakeProfitOnFillLoss => "TAKE_PROFIT_ON_FILL_LOSS",
            Self::LosingTakeProfit => "LOSING_TAKE_PROFIT",
            Self::StopLossOnFillGtdTimestampInPast => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
            Self::StopLossOnFillLoss => "STOP_LOSS_ON_FILL_LOSS",
            Self::StopLossOnFillPriceDistanceMaximumExceeded => {
                "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED"
            }
            Self::StopLossOnFillRequired => "STOP_LOSS_ON_FILL_REQUIRED",
            Self::StopLossOnFillGuaranteedRequired => "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
            Self::StopLossOnFillGuaranteedNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
            Self::StopLossOnFillGuaranteedMinimumDistanceNotMet => {
                "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET"
            }
            Self::StopLossOnFillGuaranteedLevelRestrictionExceeded => {
                "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED"
            }
            Self::StopLossOnFillGuaranteedHedgingNotAllowed => {
                "STOP_LOSS_ON_FILL_GUARANTEED_HEDGING_NOT_ALLOWED"
            }
            Self::StopLossOnFillTimeInForceInvalid => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
            Self::StopLossOnFillTriggerConditionInvalid => {
                "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID"
            }
            Self::GuaranteedStopLossOnFillGtdTimestampInPast => {
                "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST"
            }
            Self::GuaranteedStopLossOnFillLoss => "GUARANTEED_STOP_LOSS_ON_FILL_LOSS",
            Self::GuaranteedStopLossOnFillPriceDistanceMaximumExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillRequired => "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED",
            Self::GuaranteedStopLossOnFillNotAllowed => "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED",
            Self::GuaranteedStopLossOnFillMinimumDistanceNotMet => {
                "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET"
            }
            Self::GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillHedgingNotAllowed => {
                "GUARANTEED_STOP_LOSS_ON_FILL_HEDGING_NOT_ALLOWED"
            }
            Self::GuaranteedStopLossOnFillTimeInForceInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID"
            }
            Self::GuaranteedStopLossOnFillTriggerConditionInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID"
            }
            Self::TakeProfitOnFillPriceDistanceMaximumExceeded => {
                "TAKE_PROFIT_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED"
            }
            Self::TrailingStopLossOnFillGtdTimestampInPast => {
                "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST"
            }
            Self::ClientTradeIdAlreadyExists => "CLIENT_TRADE_ID_ALREADY_EXISTS",
            Self::PositionCloseoutFailed => "POSITION_CLOSEOUT_FAILED",
            Self::OpenTradesAllowedExceeded => "OPEN_TRADES_ALLOWED_EXCEEDED",
            Self::PendingOrdersAllowedExceeded => "PENDING_ORDERS_ALLOWED_EXCEEDED",
            Self::TakeProfitOnFillClientOrderIdAlreadyExists => {
                "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS"
            }
            Self::StopLossOnFillClientOrderIdAlreadyExists => {
                "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS"
            }
            Self::GuaranteedStopLossOnFillClientOrderIdAlreadyExists => {
                "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS"
            }
            Self::TrailingStopLossOnFillClientOrderIdAlreadyExists => {
                "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS"
            }
            Self::PositionSizeExceeded => "POSITION_SIZE_EXCEEDED",
            Self::HedgingGsloViolation => "HEDGING_GSLO_VIOLATION",
            Self::AccountPositionValueLimitExceeded => "ACCOUNT_POSITION_VALUE_LIMIT_EXCEEDED",
            Self::InstrumentBidReduceOnly => "INSTRUMENT_BID_REDUCE_ONLY",
            Self::InstrumentAskReduceOnly => "INSTRUMENT_ASK_REDUCE_ONLY",
            Self::InstrumentBidHalted => "INSTRUMENT_BID_HALTED",
            Self::InstrumentAskHalted => "INSTRUMENT_ASK_HALTED",
            Self::StopLossOnFillGuaranteedBidHalted => "STOP_LOSS_ON_FILL_GUARANTEED_BID_HALTED",
            Self::StopLossOnFillGuaranteedAskHalted => "STOP_LOSS_ON_FILL_GUARANTEED_ASK_HALTED",
            Self::GuaranteedStopLossOnFillBidHalted => "GUARANTEED_STOP_LOSS_ON_FILL_BID_HALTED",
            Self::GuaranteedStopLossOnFillAskHalted => "GUARANTEED_STOP_LOSS_ON_FILL_ASK_HALTED",
            Self::FifoViolationSafeguardViolation => "FIFO_VIOLATION_SAFEGUARD_VIOLATION",
            Self::FifoViolationSafeguardPartialCloseViolation => {
                "FIFO_VIOLATION_SAFEGUARD_PARTIAL_CLOSE_VIOLATION"
            }
            Self::OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation => {
                "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION"
            }
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for OrderCancelReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OrderCancelReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "INTERNAL_SERVER_ERROR" => Self::InternalServerError,
            "ACCOUNT_LOCKED" => Self::AccountLocked,
            "ACCOUNT_NEW_POSITIONS_LOCKED" => Self::AccountNewPositionsLocked,
            "ACCOUNT_ORDER_CREATION_LOCKED" => Self::AccountOrderCreationLocked,
            "ACCOUNT_ORDER_FILL_LOCKED" => Self::AccountOrderFillLocked,
            "CLIENT_REQUEST" => Self::ClientRequest,
            "MIGRATION" => Self::Migration,
            "MARKET_HALTED" => Self::MarketHalted,
            "LINKED_TRADE_CLOSED" => Self::LinkedTradeClosed,
            "TIME_IN_FORCE_EXPIRED" => Self::TimeInForceExpired,
            "INSUFFICIENT_MARGIN" => Self::InsufficientMargin,
            "FIFO_VIOLATION" => Self::FifoViolation,
            "BOUNDS_VIOLATION" => Self::BoundsViolation,
            "CLIENT_REQUEST_REPLACED" => Self::ClientRequestReplaced,
            "DIVIDEND_ADJUSTMENT_REPLACED" => Self::DividendAdjustmentReplaced,
            "INSUFFICIENT_LIQUIDITY" => Self::InsufficientLiquidity,
            "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST" => Self::TakeProfitOnFillGtdTimestampInPast,
            "TAKE_PROFIT_ON_FILL_LOSS" => Self::TakeProfitOnFillLoss,
            "LOSING_TAKE_PROFIT" => Self::LosingTakeProfit,
            "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => Self::StopLossOnFillGtdTimestampInPast,
            "STOP_LOSS_ON_FILL_LOSS" => Self::StopLossOnFillLoss,
            "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Self::StopLossOnFillPriceDistanceMaximumExceeded
            }
            "STOP_LOSS_ON_FILL_REQUIRED" => Self::StopLossOnFillRequired,
            "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED" => Self::StopLossOnFillGuaranteedRequired,
            "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED" => Self::StopLossOnFillGuaranteedNotAllowed,
            "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET" => {
                Self::StopLossOnFillGuaranteedMinimumDistanceNotMet
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED" => {
                Self::StopLossOnFillGuaranteedLevelRestrictionExceeded
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_HEDGING_NOT_ALLOWED" => {
                Self::StopLossOnFillGuaranteedHedgingNotAllowed
            }
            "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => Self::StopLossOnFillTimeInForceInvalid,
            "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Self::StopLossOnFillTriggerConditionInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Self::GuaranteedStopLossOnFillGtdTimestampInPast
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_LOSS" => Self::GuaranteedStopLossOnFillLoss,
            "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillPriceDistanceMaximumExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED" => Self::GuaranteedStopLossOnFillRequired,
            "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED" => Self::GuaranteedStopLossOnFillNotAllowed,
            "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET" => {
                Self::GuaranteedStopLossOnFillMinimumDistanceNotMet
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_HEDGING_NOT_ALLOWED" => {
                Self::GuaranteedStopLossOnFillHedgingNotAllowed
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => {
                Self::GuaranteedStopLossOnFillTimeInForceInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Self::GuaranteedStopLossOnFillTriggerConditionInvalid
            }
            "TAKE_PROFIT_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Self::TakeProfitOnFillPriceDistanceMaximumExceeded
            }
            "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Self::TrailingStopLossOnFillGtdTimestampInPast
            }
            "CLIENT_TRADE_ID_ALREADY_EXISTS" => Self::ClientTradeIdAlreadyExists,
            "POSITION_CLOSEOUT_FAILED" => Self::PositionCloseoutFailed,
            "OPEN_TRADES_ALLOWED_EXCEEDED" => Self::OpenTradesAllowedExceeded,
            "PENDING_ORDERS_ALLOWED_EXCEEDED" => Self::PendingOrdersAllowedExceeded,
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Self::TakeProfitOnFillClientOrderIdAlreadyExists
            }
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Self::StopLossOnFillClientOrderIdAlreadyExists
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Self::GuaranteedStopLossOnFillClientOrderIdAlreadyExists
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Self::TrailingStopLossOnFillClientOrderIdAlreadyExists
            }
            "POSITION_SIZE_EXCEEDED" => Self::PositionSizeExceeded,
            "HEDGING_GSLO_VIOLATION" => Self::HedgingGsloViolation,
            "ACCOUNT_POSITION_VALUE_LIMIT_EXCEEDED" => Self::AccountPositionValueLimitExceeded,
            "INSTRUMENT_BID_REDUCE_ONLY" => Self::InstrumentBidReduceOnly,
            "INSTRUMENT_ASK_REDUCE_ONLY" => Self::InstrumentAskReduceOnly,
            "INSTRUMENT_BID_HALTED" => Self::InstrumentBidHalted,
            "INSTRUMENT_ASK_HALTED" => Self::InstrumentAskHalted,
            "STOP_LOSS_ON_FILL_GUARANTEED_BID_HALTED" => Self::StopLossOnFillGuaranteedBidHalted,
            "STOP_LOSS_ON_FILL_GUARANTEED_ASK_HALTED" => Self::StopLossOnFillGuaranteedAskHalted,
            "GUARANTEED_STOP_LOSS_ON_FILL_BID_HALTED" => Self::GuaranteedStopLossOnFillBidHalted,
            "GUARANTEED_STOP_LOSS_ON_FILL_ASK_HALTED" => Self::GuaranteedStopLossOnFillAskHalted,
            "FIFO_VIOLATION_SAFEGUARD_VIOLATION" => Self::FifoViolationSafeguardViolation,
            "FIFO_VIOLATION_SAFEGUARD_PARTIAL_CLOSE_VIOLATION" => {
                Self::FifoViolationSafeguardPartialCloseViolation
            }
            "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION" => {
                Self::OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation
            }
            _ => Self::Unknown(value),
        })
    }
}

/// Used to pay or collect a dividend adjustment amount for an open Trade within the Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenTradeDividendAdjustment {
    /// The ID of the Trade for which the dividend adjustment is to be paid or collected.
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The dividend adjustment amount to pay or collect for the Trade.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_adjustment: Option<Decimal>,
    /// The dividend adjustment amount to pay or collect for the Trade, in the Instrument’s quote currency.
    #[serde(
        rename = "quoteDividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_dividend_adjustment: Option<Decimal>,
}

/// A ClientExtensions object allows a client to attach a clientID, tag and comment to Orders and Trades in
/// their Account. Do not set, modify, or delete this field if your account is associated with MT4.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientExtensions {
    /// The Client ID of the Order/Trade
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ClientID>,
    /// A tag associated with the Order/Trade
    #[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<ClientTag>,
    /// A comment associated with the Order/Trade
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<ClientComment>,
}

/// TakeProfitDetails specifies the details of a Take Profit Order to be created on behalf of a client. This
/// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
/// dependent Take Profit Order is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeProfitDetails {
    /// The price that the Take Profit Order will be triggered at. Only one of the price and distance fields may
    /// be specified.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// The time in force for the created Take Profit Order. This may only be GTC, GTD or GFD.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<TimeInForce>,
    /// The date when the Take Profit Order will be cancelled on if timeInForce is GTD.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// The Client Extensions to add to the Take Profit Order when created.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

/// StopLossDetails specifies the details of a Stop Loss Order to be created on behalf of a client. This may
/// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
/// Stop Loss Order is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossDetails {
    /// The price that the Stop Loss Order will be triggered at. Only one of the price and distance fields may
    /// be specified.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// Specifies the distance (in price units) from the Trade’s open price to use as the Stop Loss Order price.
    /// Only one of the distance and price fields may be specified.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Decimal>,
    /// The time in force for the created Stop Loss Order. This may only be GTC, GTD or GFD.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<TimeInForce>,
    /// The date when the Stop Loss Order will be cancelled on if timeInForce is GTD.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// The Client Extensions to add to the Stop Loss Order when created.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// Flag indicating that the price for the Stop Loss Order is guaranteed. The default value depends on the
    /// GuaranteedStopLossOrderMode of the account, if it is REQUIRED, the default will be true, for DISABLED or
    /// ENABLED the default is false. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "guaranteed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed: Option<bool>,
}

/// GuaranteedStopLossDetails specifies the details of a Guaranteed Stop Loss Order to be created on behalf
/// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
/// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossDetails {
    /// The price that the Guaranteed Stop Loss Order will be triggered at. Only one of the price and distance
    /// fields may be specified.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// Specifies the distance (in price units) from the Trade’s open price to use as the Guaranteed Stop Loss
    /// Order price. Only one of the distance and price fields may be specified.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Decimal>,
    /// The time in force for the created Guaranteed Stop Loss Order. This may only be GTC, GTD or GFD.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<TimeInForce>,
    /// The date when the Guaranteed Stop Loss Order will be cancelled on if timeInForce is GTD.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// The Client Extensions to add to the Guaranteed Stop Loss Order when created.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

/// TrailingStopLossDetails specifies the details of a Trailing Stop Loss Order to be created on behalf of a
/// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
/// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingStopLossDetails {
    /// The distance (in price units) from the Trade’s fill price that the Trailing Stop Loss Order will be
    /// triggered at.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Decimal>,
    /// The time in force for the created Trailing Stop Loss Order. This may only be GTC, GTD or GFD.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<TimeInForce>,
    /// The date when the Trailing Stop Loss Order will be cancelled on if timeInForce is GTD.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// The Client Extensions to add to the Trailing Stop Loss Order when created.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

/// A TradeOpen object represents a Trade for an instrument that was opened in an Account. It is found
/// embedded in Transactions that affect the position of an instrument in the Account, specifically the
/// OrderFill Transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOpen {
    /// The ID of the Trade that was opened
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The number of units opened by the Trade
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Decimal>,
    /// The average price that the units were opened at.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// This is the fee charged for opening the trade if it has a guaranteed Stop Loss Order attached to it.
    #[serde(
        rename = "guaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee: Option<Decimal>,
    /// This is the fee charged for opening the trade if it has a guaranteed Stop Loss Order attached to it,
    /// expressed in the Instrument’s quote currency.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_guaranteed_execution_fee: Option<Decimal>,
    /// The client extensions for the newly opened Trade
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The half spread cost for the trade open. This can be a positive or negative value and is represented in
    /// the home currency of the Account.
    #[serde(
        rename = "halfSpreadCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub half_spread_cost: Option<Decimal>,
    /// The margin required at the time the Trade was created. Note, this is the ‘pure’ margin required, it is
    /// not the ‘effective’ margin used that factors in the trade risk if a GSLO is attached to the trade.
    #[serde(
        rename = "initialMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<Decimal>,
}

/// A TradeReduce object represents a Trade for an instrument that was reduced (either partially or fully)
/// in an Account. It is found embedded in Transactions that affect the position of an instrument in the
/// account, specifically the OrderFill Transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeReduce {
    /// The ID of the Trade that was reduced or closed
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The number of units that the Trade was reduced by
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Decimal>,
    /// The average price that the units were closed at. This price may be clamped for guaranteed Stop Loss
    /// Orders.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// The PL realized when reducing the Trade
    #[serde(
        rename = "realizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pl: Option<Decimal>,
    /// The financing paid/collected when reducing the Trade
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The base financing paid/collected when reducing the Trade
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_financing: Option<Decimal>,
    /// The quote financing paid/collected when reducing the Trade
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_financing: Option<Decimal>,
    /// The financing rate in effect for the instrument used to calculate the amount of financing paid/collected
    /// when reducing the Trade. This field will only be set if the AccountFinancingMode at the time of the
    /// order fill is SECOND_BY_SECOND_INSTRUMENT. The value is in decimal rather than percentage points, e.g.
    /// 5% is represented as 0.05.
    #[serde(
        rename = "financingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub financing_rate: Option<Decimal>,
    /// This is the fee that is charged for closing the Trade if it has a guaranteed Stop Loss Order attached to
    /// it.
    #[serde(
        rename = "guaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee: Option<Decimal>,
    /// This is the fee that is charged for closing the Trade if it has a guaranteed Stop Loss Order attached to
    /// it, expressed in the Instrument’s quote currency.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_guaranteed_execution_fee: Option<Decimal>,
    /// The half spread cost for the trade reduce/close. This can be a positive or negative value and is
    /// represented in the home currency of the Account.
    #[serde(
        rename = "halfSpreadCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub half_spread_cost: Option<Decimal>,
}

/// A MarketOrderTradeClose specifies the extensions to a Market Order that has been created specifically to
/// close a Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderTradeClose {
    /// The ID of the Trade requested to be closed
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The client ID of the Trade requested to be closed
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<String>,
    /// Indication of how much of the Trade to close. Either “ALL”, or a DecimalNumber reflection a partial
    /// close of the Trade.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

/// Details for the Market Order extensions specific to a Market Order placed that is part of a Market Order
/// Margin Closeout in a client’s account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderMarginCloseout {
    /// The reason the Market Order was created to perform a margin closeout
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderMarginCloseoutReason>,
}

/// The reason that the Market Order was created to perform a margin closeout
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MarketOrderMarginCloseoutReason {
    /// Trade closures resulted from violating OANDA’s margin policy
    MarginCheckViolation,
    /// Trade closures came from a margin closeout event resulting from regulatory conditions placed on the
    /// Account’s margin call
    RegulatoryMarginCallViolation,
    /// Trade closures resulted from violating the margin policy imposed by regulatory requirements
    RegulatoryMarginCheckViolation,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl MarketOrderMarginCloseoutReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::MarginCheckViolation => "MARGIN_CHECK_VIOLATION",
            Self::RegulatoryMarginCallViolation => "REGULATORY_MARGIN_CALL_VIOLATION",
            Self::RegulatoryMarginCheckViolation => "REGULATORY_MARGIN_CHECK_VIOLATION",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for MarketOrderMarginCloseoutReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MarketOrderMarginCloseoutReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "MARGIN_CHECK_VIOLATION" => Self::MarginCheckViolation,
            "REGULATORY_MARGIN_CALL_VIOLATION" => Self::RegulatoryMarginCallViolation,
            "REGULATORY_MARGIN_CHECK_VIOLATION" => Self::RegulatoryMarginCheckViolation,
            _ => Self::Unknown(value),
        })
    }
}

/// Details for the Market Order extensions specific to a Market Order placed with the intent of fully
/// closing a specific open trade that should have already been closed but wasn’t due to halted market
/// conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderDelayedTradeClose {
    /// The ID of the Trade being closed
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The Client ID of the Trade being closed
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<TradeID>,
    /// The Transaction ID of the DelayedTradeClosure transaction to which this Delayed Trade Close belongs to
    #[serde(
        rename = "sourceTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_transaction_id: Option<TransactionID>,
}

/// A MarketOrderPositionCloseout specifies the extensions to a Market Order when it has been created to
/// closeout a specific Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderPositionCloseout {
    /// The instrument of the Position being closed out.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// Indication of how much of the Position to close. Either “ALL”, or a DecimalNumber reflection a partial
    /// close of the Trade. The DecimalNumber must always be positive, and represent a number that doesn’t
    /// exceed the absolute size of the Position.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

/// A LiquidityRegenerationSchedule indicates how liquidity that is used when filling an Order for an
/// instrument is regenerated following the fill. A liquidity regeneration schedule will be in effect until
/// the timestamp of its final step, but may be replaced by a schedule created for an Order of the same
/// instrument that is filled while it is still in effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityRegenerationSchedule {
    /// The steps in the Liquidity Regeneration Schedule
    #[serde(rename = "steps", default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<LiquidityRegenerationScheduleStep>>,
}

/// A liquidity regeneration schedule Step indicates the amount of bid and ask liquidity that is used by the
/// Account at a certain time. These amounts will only change at the timestamp of the following step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityRegenerationScheduleStep {
    /// The timestamp of the schedule step.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,
    /// The amount of bid liquidity used at this step in the schedule.
    #[serde(
        rename = "bidLiquidityUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_liquidity_used: Option<Decimal>,
    /// The amount of ask liquidity used at this step in the schedule.
    #[serde(
        rename = "askLiquidityUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_liquidity_used: Option<Decimal>,
}

/// OpenTradeFinancing is used to pay/collect daily financing charge for an open Trade within an Account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenTradeFinancing {
    /// The ID of the Trade that financing is being paid/collected for.
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The amount of financing paid/collected for the Trade.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The amount of financing paid/collected in the Instrument’s base currency for the Trade.
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_financing: Option<Decimal>,
    /// The amount of financing paid/collected in the Instrument’s quote currency for the Trade.
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_financing: Option<Decimal>,
    /// The financing rate in effect for the instrument used to calculate the the amount of financing
    /// paid/collected for the Trade. This field will only be set if the AccountFinancingMode at the time of the
    /// daily financing is DAILY_INSTRUMENT or SECOND_BY_SECOND_INSTRUMENT. The value is in decimal rather than
    /// percentage points, e.g. 5% is represented as 0.05.
    #[serde(
        rename = "financingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub financing_rate: Option<Decimal>,
}

/// OpenTradeFinancing is used to pay/collect daily financing charge for a Position within an Account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionFinancing {
    /// The instrument of the Position that financing is being paid/collected for.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The amount of financing paid/collected for the Position.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<Decimal>,
    /// The amount of base financing paid/collected for the Position.
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_financing: Option<Decimal>,
    /// The amount of quote financing paid/collected for the Position.
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_financing: Option<Decimal>,
    /// The HomeConversionFactors in effect for the Position’s Instrument at the time of the DailyFinancing.
    #[serde(
        rename = "homeConversionFactors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversion_factors: Option<HomeConversionFactors>,
    /// The financing paid/collected for each open Trade within the Position.
    #[serde(
        rename = "openTradeFinancings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_trade_financings: Option<Vec<OpenTradeFinancing>>,
    /// The account financing mode at the time of the daily financing.
    #[serde(
        rename = "accountFinancingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_financing_mode: Option<AccountFinancingMode>,
}

/// The reason that a Transaction was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransactionRejectReason {
    /// An unexpected internal server error has occurred
    InternalServerError,
    /// The system was unable to determine the current price for the Order’s instrument
    InstrumentPriceUnknown,
    /// The Account is not active
    AccountNotActive,
    /// The Account is locked
    AccountLocked,
    /// The Account is locked for Order creation
    AccountOrderCreationLocked,
    /// The Account is locked for configuration
    AccountConfigurationLocked,
    /// The Account is locked for deposits
    AccountDepositLocked,
    /// The Account is locked for withdrawals
    AccountWithdrawalLocked,
    /// The Account is locked for Order cancellation
    AccountOrderCancelLocked,
    /// The instrument specified is not tradeable by the Account
    InstrumentNotTradeable,
    /// Creating the Order would result in the maximum number of allowed pending Orders being exceeded
    PendingOrdersAllowedExceeded,
    /// Neither the Order ID nor client Order ID are specified
    OrderIdUnspecified,
    /// The Order specified does not exist
    OrderDoesntExist,
    /// The Order ID and client Order ID specified do not identify the same Order
    OrderIdentifierInconsistency,
    /// Neither the Trade ID nor client Trade ID are specified
    TradeIdUnspecified,
    /// The Trade specified does not exist
    TradeDoesntExist,
    /// The Trade ID and client Trade ID specified do not identify the same Trade
    TradeIdentifierInconsistency,
    /// The Account had insufficient margin to perform the action specified. One possible reason for this is due
    /// to the creation or modification of a guaranteed StopLoss Order.
    InsufficientMargin,
    /// Order instrument has not been specified
    InstrumentMissing,
    /// The instrument specified is unknown
    InstrumentUnknown,
    /// Order units have not been not specified
    UnitsMissing,
    /// Order units specified are invalid
    UnitsInvalid,
    /// The units specified contain more precision than is allowed for the Order’s instrument
    UnitsPrecisionExceeded,
    /// The units specified exceeds the maximum number of units allowed
    UnitsLimitExceeded,
    /// The units specified is less than the minimum number of units required
    UnitsMinimumNotMet,
    /// The price has not been specified
    PriceMissing,
    /// The price specified is invalid
    PriceInvalid,
    /// The price specified contains more precision than is allowed for the instrument
    PricePrecisionExceeded,
    /// The price distance has not been specified
    PriceDistanceMissing,
    /// The price distance specified is invalid
    PriceDistanceInvalid,
    /// The price distance specified contains more precision than is allowed for the instrument
    PriceDistancePrecisionExceeded,
    /// The price distance exceeds that maximum allowed amount
    PriceDistanceMaximumExceeded,
    /// The price distance does not meet the minimum allowed amount
    PriceDistanceMinimumNotMet,
    /// The TimeInForce field has not been specified
    TimeInForceMissing,
    /// The TimeInForce specified is invalid
    TimeInForceInvalid,
    /// The TimeInForce is GTD but no GTD timestamp is provided
    TimeInForceGtdTimestampMissing,
    /// The TimeInForce is GTD but the GTD timestamp is in the past
    TimeInForceGtdTimestampInPast,
    /// The price bound specified is invalid
    PriceBoundInvalid,
    /// The price bound specified contains more precision than is allowed for the Order’s instrument
    PriceBoundPrecisionExceeded,
    /// Multiple Orders on fill share the same client Order ID
    OrdersOnFillDuplicateClientOrderIds,
    /// The Order does not support Trade on fill client extensions because it cannot create a new Trade
    TradeOnFillClientExtensionsNotSupported,
    /// The client Order ID specified is invalid
    ClientOrderIdInvalid,
    /// The client Order ID specified is already assigned to another pending Order
    ClientOrderIdAlreadyExists,
    /// The client Order tag specified is invalid
    ClientOrderTagInvalid,
    /// The client Order comment specified is invalid
    ClientOrderCommentInvalid,
    /// The client Trade ID specified is invalid
    ClientTradeIdInvalid,
    /// The client Trade ID specified is already assigned to another open Trade
    ClientTradeIdAlreadyExists,
    /// The client Trade tag specified is invalid
    ClientTradeTagInvalid,
    /// The client Trade comment is invalid
    ClientTradeCommentInvalid,
    /// The OrderFillPositionAction field has not been specified
    OrderFillPositionActionMissing,
    /// The OrderFillPositionAction specified is invalid
    OrderFillPositionActionInvalid,
    /// The TriggerCondition field has not been specified
    TriggerConditionMissing,
    /// The TriggerCondition specified is invalid
    TriggerConditionInvalid,
    /// The OrderFillPositionAction field has not been specified
    OrderPartialFillOptionMissing,
    /// The OrderFillPositionAction specified is invalid.
    OrderPartialFillOptionInvalid,
    /// When attempting to reissue an order (currently only a MarketIfTouched) that was immediately partially
    /// filled, it is not possible to create a correct pending Order.
    InvalidReissueImmediatePartialFill,
    /// The Orders on fill would be in violation of the risk management Order mutual exclusivity configuration
    /// specifying that only one risk management Order can be attached to a Trade.
    OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Orders on fill would be in violation of the risk management Order mutual exclusivity configuration
    /// specifying that if a GSLO is already attached to a Trade, no other risk management Order can be attached
    /// to a Trade.
    OrdersOnFillRmoMutualExclusivityGsloExcludesOthersViolation,
    /// A Take Profit Order for the specified Trade already exists
    TakeProfitOrderAlreadyExists,
    /// The Take Profit Order would cause the associated Trade to be in violation of the FIFO violation
    /// safeguard constraints.
    TakeProfitOrderWouldViolateFifoViolationSafeguard,
    /// The Take Profit on fill specified does not provide a price
    TakeProfitOnFillPriceMissing,
    /// The Take Profit on fill specified contains an invalid price
    TakeProfitOnFillPriceInvalid,
    /// The Take Profit on fill specified contains a price with more precision than is allowed by the Order’s
    /// instrument
    TakeProfitOnFillPricePrecisionExceeded,
    /// The Take Profit on fill specified does not provide a TimeInForce
    TakeProfitOnFillTimeInForceMissing,
    /// The Take Profit on fill specifies an invalid TimeInForce
    TakeProfitOnFillTimeInForceInvalid,
    /// The Take Profit on fill specifies a GTD TimeInForce but does not provide a GTD timestamp
    TakeProfitOnFillGtdTimestampMissing,
    /// The Take Profit on fill specifies a GTD timestamp that is in the past
    TakeProfitOnFillGtdTimestampInPast,
    /// The Take Profit on fill client Order ID specified is invalid
    TakeProfitOnFillClientOrderIdInvalid,
    /// The Take Profit on fill client Order tag specified is invalid
    TakeProfitOnFillClientOrderTagInvalid,
    /// The Take Profit on fill client Order comment specified is invalid
    TakeProfitOnFillClientOrderCommentInvalid,
    /// The Take Profit on fill specified does not provide a TriggerCondition
    TakeProfitOnFillTriggerConditionMissing,
    /// The Take Profit on fill specifies an invalid TriggerCondition
    TakeProfitOnFillTriggerConditionInvalid,
    /// A Stop Loss Order for the specified Trade already exists
    StopLossOrderAlreadyExists,
    /// An attempt was made to to create a non-guaranteed stop loss order in an account that requires all stop
    /// loss orders to be guaranteed.
    StopLossOrderGuaranteedRequired,
    /// An attempt to create a guaranteed stop loss order with a price that is within the current tradeable
    /// spread.
    StopLossOrderGuaranteedPriceWithinSpread,
    /// An attempt was made to create a guaranteed Stop Loss Order, however the Account’s configuration does not
    /// allow guaranteed Stop Loss Orders.
    StopLossOrderGuaranteedNotAllowed,
    /// An attempt was made to create a guaranteed Stop Loss Order when the market was halted.
    StopLossOrderGuaranteedHaltedCreateViolation,
    /// An attempt was made to re-create a guaranteed Stop Loss Order with a tighter fill price when the market
    /// was halted.
    StopLossOrderGuaranteedHaltedTightenViolation,
    /// An attempt was made to create a guaranteed Stop Loss Order on a hedged Trade (ie there is an existing
    /// open Trade in the opposing direction), however the Account’s configuration does not allow guaranteed
    /// Stop Loss Orders for hedged Trades/Positions.
    StopLossOrderGuaranteedHedgingNotAllowed,
    /// An attempt was made to create a guaranteed Stop Loss Order, however the distance between the current
    /// price and the trigger price does not meet the Account’s configured minimum Guaranteed Stop Loss distance.
    StopLossOrderGuaranteedMinimumDistanceNotMet,
    /// An attempt was made to cancel a Stop Loss Order, however the Account’s configuration requires every
    /// Trade have an associated Stop Loss Order.
    StopLossOrderNotCancelable,
    /// An attempt was made to cancel and replace a Stop Loss Order, however the Account’s configuration
    /// prevents the modification of Stop Loss Orders.
    StopLossOrderNotReplaceable,
    /// An attempt was made to create a guaranteed Stop Loss Order, however doing so would exceed the Account’s
    /// configured guaranteed StopLoss Order level restriction volume.
    StopLossOrderGuaranteedLevelRestrictionExceeded,
    /// The Stop Loss Order request contains both the price and distance fields.
    StopLossOrderPriceAndDistanceBothSpecified,
    /// The Stop Loss Order request contains neither the price nor distance fields.
    StopLossOrderPriceAndDistanceBothMissing,
    /// The Stop Loss Order would cause the associated Trade to be in violation of the FIFO violation safeguard
    /// constraints
    StopLossOrderWouldViolateFifoViolationSafeguard,
    /// The Stop Loss Order would be in violation of the risk management Order mutual exclusivity configuration
    /// specifying that only one risk management order can be attached to a Trade.
    StopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Stop Loss Order would be in violation of the risk management Order mutual exclusivity configuration
    /// specifying that if a GSLO is already attached to a Trade, no other risk management Order can be attached
    /// to the same Trade.
    StopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation,
    /// An attempt to create a pending Order was made with no Stop Loss Order on fill specified and the
    /// Account’s configuration requires that every Trade have an associated Stop Loss Order.
    StopLossOnFillRequiredForPendingOrder,
    /// An attempt to create a pending Order was made with a Stop Loss Order on fill that was explicitly
    /// configured to be guaranteed, however the Account’s configuration does not allow guaranteed Stop Loss
    /// Orders.
    StopLossOnFillGuaranteedNotAllowed,
    /// An attempt to create a pending Order was made with a Stop Loss Order on fill that was explicitly
    /// configured to be not guaranteed, however the Account’s configuration requires guaranteed Stop Loss
    /// Orders.
    StopLossOnFillGuaranteedRequired,
    /// The Stop Loss on fill specified does not provide a price
    StopLossOnFillPriceMissing,
    /// The Stop Loss on fill specifies an invalid price
    StopLossOnFillPriceInvalid,
    /// The Stop Loss on fill specifies a price with more precision than is allowed by the Order’s instrument
    StopLossOnFillPricePrecisionExceeded,
    /// An attempt to create a pending Order was made with the distance between the guaranteed Stop Loss Order
    /// on fill’s price and the pending Order’s price is less than the Account’s configured minimum guaranteed
    /// stop loss distance.
    StopLossOnFillGuaranteedMinimumDistanceNotMet,
    /// An attempt to create a pending Order was made with a guaranteed Stop Loss Order on fill configured, and
    /// the Order’s units exceed the Account’s configured guaranteed StopLoss Order level restriction volume.
    StopLossOnFillGuaranteedLevelRestrictionExceeded,
    /// The Stop Loss on fill distance is invalid
    StopLossOnFillDistanceInvalid,
    /// The Stop Loss on fill price distance exceeds the maximum allowed amount
    StopLossOnFillPriceDistanceMaximumExceeded,
    /// The Stop Loss on fill distance contains more precision than is allowed by the instrument
    StopLossOnFillDistancePrecisionExceeded,
    /// The Stop Loss on fill contains both the price and distance fields.
    StopLossOnFillPriceAndDistanceBothSpecified,
    /// The Stop Loss on fill contains neither the price nor distance fields.
    StopLossOnFillPriceAndDistanceBothMissing,
    /// The Stop Loss on fill specified does not provide a TimeInForce
    StopLossOnFillTimeInForceMissing,
    /// The Stop Loss on fill specifies an invalid TimeInForce
    StopLossOnFillTimeInForceInvalid,
    /// The Stop Loss on fill specifies a GTD TimeInForce but does not provide a GTD timestamp
    StopLossOnFillGtdTimestampMissing,
    /// The Stop Loss on fill specifies a GTD timestamp that is in the past
    StopLossOnFillGtdTimestampInPast,
    /// The Stop Loss on fill client Order ID specified is invalid
    StopLossOnFillClientOrderIdInvalid,
    /// The Stop Loss on fill client Order tag specified is invalid
    StopLossOnFillClientOrderTagInvalid,
    /// The Stop Loss on fill client Order comment specified is invalid
    StopLossOnFillClientOrderCommentInvalid,
    /// The Stop Loss on fill specified does not provide a TriggerCondition
    StopLossOnFillTriggerConditionMissing,
    /// The Stop Loss on fill specifies an invalid TriggerCondition
    StopLossOnFillTriggerConditionInvalid,
    /// A Guaranteed Stop Loss Order for the specified Trade already exists
    GuaranteedStopLossOrderAlreadyExists,
    /// An attempt was made to to create a non-guaranteed stop loss order in an account that requires all stop
    /// loss orders to be guaranteed.
    GuaranteedStopLossOrderRequired,
    /// An attempt to create a guaranteed stop loss order with a price that is within the current tradeable
    /// spread.
    GuaranteedStopLossOrderPriceWithinSpread,
    /// An attempt was made to create a Guaranteed Stop Loss Order, however the Account’s configuration does not
    /// allow Guaranteed Stop Loss Orders.
    GuaranteedStopLossOrderNotAllowed,
    /// An attempt was made to create a Guaranteed Stop Loss Order when the market was halted.
    GuaranteedStopLossOrderHaltedCreateViolation,
    /// An attempt was made to create a Guaranteed Stop Loss Order when the market was open.
    GuaranteedStopLossOrderCreateViolation,
    /// An attempt was made to re-create a Guaranteed Stop Loss Order with a tighter fill price when the market
    /// was halted.
    GuaranteedStopLossOrderHaltedTightenViolation,
    /// An attempt was made to re-create a Guaranteed Stop Loss Order with a tighter fill price when the market
    /// was open.
    GuaranteedStopLossOrderTightenViolation,
    /// An attempt was made to create a Guaranteed Stop Loss Order on a hedged Trade (ie there is an existing
    /// open Trade in the opposing direction), however the Account’s configuration does not allow Guaranteed
    /// Stop Loss Orders for hedged Trades/Positions.
    GuaranteedStopLossOrderHedgingNotAllowed,
    /// An attempt was made to create a Guaranteed Stop Loss Order, however the distance between the current
    /// price and the trigger price does not meet the Account’s configured minimum Guaranteed Stop Loss distance.
    GuaranteedStopLossOrderMinimumDistanceNotMet,
    /// An attempt was made to cancel a Guaranteed Stop Loss Order when the market is open, however the
    /// Account’s configuration requires every Trade have an associated Guaranteed Stop Loss Order.
    GuaranteedStopLossOrderNotCancelable,
    /// An attempt was made to cancel a Guaranteed Stop Loss Order when the market is halted, however the
    /// Account’s configuration requires every Trade have an associated Guaranteed Stop Loss Order.
    GuaranteedStopLossOrderHaltedNotCancelable,
    /// An attempt was made to cancel and replace a Guaranteed Stop Loss Order when the market is open, however
    /// the Account’s configuration prevents the modification of Guaranteed Stop Loss Orders.
    GuaranteedStopLossOrderNotReplaceable,
    /// An attempt was made to cancel and replace a Guaranteed Stop Loss Order when the market is halted,
    /// however the Account’s configuration prevents the modification of Guaranteed Stop Loss Orders.
    GuaranteedStopLossOrderHaltedNotReplaceable,
    /// An attempt was made to create a Guaranteed Stop Loss Order, however doing so would exceed the Account’s
    /// configured guaranteed StopLoss Order level restriction volume.
    GuaranteedStopLossOrderLevelRestrictionVolumeExceeded,
    /// An attempt was made to create a Guaranteed Stop Loss Order, however doing so would exceed the Account’s
    /// configured guaranteed StopLoss Order level restriction price range.
    GuaranteedStopLossOrderLevelRestrictionPriceRangeExceeded,
    /// The Guaranteed Stop Loss Order request contains both the price and distance fields.
    GuaranteedStopLossOrderPriceAndDistanceBothSpecified,
    /// The Guaranteed Stop Loss Order request contains neither the price nor distance fields.
    GuaranteedStopLossOrderPriceAndDistanceBothMissing,
    /// The Guaranteed Stop Loss Order would cause the associated Trade to be in violation of the FIFO violation
    /// safeguard constraints
    GuaranteedStopLossOrderWouldViolateFifoViolationSafeguard,
    /// The Guaranteed Stop Loss Order would be in violation of the risk management Order mutual exclusivity
    /// configuration specifying that only one risk management order can be attached to a Trade.
    GuaranteedStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Guaranteed Stop Loss Order would be in violation of the risk management Order mutual exclusivity
    /// configuration specifying that if a GSLO is already attached to a Trade, no other risk management Order
    /// can be attached to the same Trade.
    GuaranteedStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation,
    /// An attempt to create a pending Order was made with no Guaranteed Stop Loss Order on fill specified and
    /// the Account’s configuration requires that every Trade have an associated Guaranteed Stop Loss Order.
    GuaranteedStopLossOnFillRequiredForPendingOrder,
    /// An attempt to create a pending Order was made with a Guaranteed Stop Loss Order on fill that was
    /// explicitly configured to be guaranteed, however the Account’s configuration does not allow guaranteed
    /// Stop Loss Orders.
    GuaranteedStopLossOnFillNotAllowed,
    /// An attempt to create a pending Order was made with a Guaranteed Stop Loss Order on fill that was
    /// explicitly configured to be not guaranteed, however the Account’s configuration requires Guaranteed Stop
    /// Loss Orders.
    GuaranteedStopLossOnFillRequired,
    /// The Guaranteed Stop Loss on fill specified does not provide a price
    GuaranteedStopLossOnFillPriceMissing,
    /// The Guaranteed Stop Loss on fill specifies an invalid price
    GuaranteedStopLossOnFillPriceInvalid,
    /// The Guaranteed Stop Loss on fill specifies a price with more precision than is allowed by the Order’s
    /// instrument
    GuaranteedStopLossOnFillPricePrecisionExceeded,
    /// An attempt to create a pending Order was made with the distance between the Guaranteed Stop Loss Order
    /// on fill’s price and the pending Order’s price is less than the Account’s configured minimum guaranteed
    /// stop loss distance.
    GuaranteedStopLossOnFillMinimumDistanceNotMet,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger number of
    /// units that violates the account’s Guaranteed Stop Loss Order level restriction volume.
    GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded,
    /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger price that
    /// violates the account’s Guaranteed Stop Loss Order level restriction price range.
    GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded,
    /// The Guaranteed Stop Loss on fill distance is invalid
    GuaranteedStopLossOnFillDistanceInvalid,
    /// The Guaranteed Stop Loss on fill price distance exceeds the maximum allowed amount.
    GuaranteedStopLossOnFillPriceDistanceMaximumExceeded,
    /// The Guaranteed Stop Loss on fill distance contains more precision than is allowed by the instrument
    GuaranteedStopLossOnFillDistancePrecisionExceeded,
    /// The Guaranteed Stop Loss on fill contains both the price and distance fields.
    GuaranteedStopLossOnFillPriceAndDistanceBothSpecified,
    /// The Guaranteed Stop Loss on fill contains neither the price nor distance fields.
    GuaranteedStopLossOnFillPriceAndDistanceBothMissing,
    /// The Guaranteed Stop Loss on fill specified does not provide a TimeInForce
    GuaranteedStopLossOnFillTimeInForceMissing,
    /// The Guaranteed Stop Loss on fill specifies an invalid TimeInForce
    GuaranteedStopLossOnFillTimeInForceInvalid,
    /// The Guaranteed Stop Loss on fill specifies a GTD TimeInForce but does not provide a GTD timestamp
    GuaranteedStopLossOnFillGtdTimestampMissing,
    /// The Guaranteed Stop Loss on fill specifies a GTD timestamp that is in the past.
    GuaranteedStopLossOnFillGtdTimestampInPast,
    /// The Guaranteed Stop Loss on fill client Order ID specified is invalid
    GuaranteedStopLossOnFillClientOrderIdInvalid,
    /// The Guaranteed Stop Loss on fill client Order tag specified is invalid
    GuaranteedStopLossOnFillClientOrderTagInvalid,
    /// The Guaranteed Stop Loss on fill client Order comment specified is invalid.
    GuaranteedStopLossOnFillClientOrderCommentInvalid,
    /// The Guaranteed Stop Loss on fill specified does not provide a TriggerCondition.
    GuaranteedStopLossOnFillTriggerConditionMissing,
    /// The Guaranteed Stop Loss on fill specifies an invalid TriggerCondition.
    GuaranteedStopLossOnFillTriggerConditionInvalid,
    /// A Trailing Stop Loss Order for the specified Trade already exists
    TrailingStopLossOrderAlreadyExists,
    /// The Trailing Stop Loss Order would cause the associated Trade to be in violation of the FIFO violation
    /// safeguard constraints
    TrailingStopLossOrderWouldViolateFifoViolationSafeguard,
    /// The Trailing Stop Loss Order would be in violation of the risk management Order mutual exclusivity
    /// configuration specifying that only one risk management order can be attached to a Trade.
    TrailingStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Trailing Stop Loss Order would be in violation of the risk management Order mutual exclusivity
    /// configuration specifying that if a GSLO is already attached to a Trade, no other risk management Order
    /// can be attached to the same Trade.
    TrailingStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation,
    /// The Trailing Stop Loss on fill specified does not provide a distance
    TrailingStopLossOnFillPriceDistanceMissing,
    /// The Trailing Stop Loss on fill distance is invalid
    TrailingStopLossOnFillPriceDistanceInvalid,
    /// The Trailing Stop Loss on fill distance contains more precision than is allowed by the instrument
    TrailingStopLossOnFillPriceDistancePrecisionExceeded,
    /// The Trailing Stop Loss on fill price distance exceeds the maximum allowed amount
    TrailingStopLossOnFillPriceDistanceMaximumExceeded,
    /// The Trailing Stop Loss on fill price distance does not meet the minimum allowed amount
    TrailingStopLossOnFillPriceDistanceMinimumNotMet,
    /// The Trailing Stop Loss on fill specified does not provide a TimeInForce
    TrailingStopLossOnFillTimeInForceMissing,
    /// The Trailing Stop Loss on fill specifies an invalid TimeInForce
    TrailingStopLossOnFillTimeInForceInvalid,
    /// The Trailing Stop Loss on fill TimeInForce is specified as GTD but no GTD timestamp is provided
    TrailingStopLossOnFillGtdTimestampMissing,
    /// The Trailing Stop Loss on fill GTD timestamp is in the past
    TrailingStopLossOnFillGtdTimestampInPast,
    /// The Trailing Stop Loss on fill client Order ID specified is invalid
    TrailingStopLossOnFillClientOrderIdInvalid,
    /// The Trailing Stop Loss on fill client Order tag specified is invalid
    TrailingStopLossOnFillClientOrderTagInvalid,
    /// The Trailing Stop Loss on fill client Order comment specified is invalid
    TrailingStopLossOnFillClientOrderCommentInvalid,
    /// A client attempted to create either a Trailing Stop Loss order or an order with a Trailing Stop Loss On
    /// Fill specified, which may not yet be supported.
    TrailingStopLossOrdersNotSupported,
    /// The Trailing Stop Loss on fill specified does not provide a TriggerCondition
    TrailingStopLossOnFillTriggerConditionMissing,
    /// The Tailing Stop Loss on fill specifies an invalid TriggerCondition
    TrailingStopLossOnFillTriggerConditionInvalid,
    /// The request to close a Trade does not specify a full or partial close
    CloseTradeTypeMissing,
    /// The request to close a Trade partially did not specify the number of units to close
    CloseTradePartialUnitsMissing,
    /// The request to partially close a Trade specifies a number of units that exceeds the current size of the
    /// given Trade
    CloseTradeUnitsExceedTradeSize,
    /// The Position requested to be closed out does not exist
    CloseoutPositionDoesntExist,
    /// The request to closeout a Position was specified incompletely
    CloseoutPositionIncompleteSpecification,
    /// A partial Position closeout request specifies a number of units that exceeds the current Position
    CloseoutPositionUnitsExceedPositionSize,
    /// The request to closeout a Position could not be fully satisfied
    CloseoutPositionReject,
    /// The request to partially closeout a Position did not specify the number of units to close.
    CloseoutPositionPartialUnitsMissing,
    /// The markup group ID provided is invalid
    MarkupGroupIdInvalid,
    /// The PositionAggregationMode provided is not supported/valid.
    PositionAggregationModeInvalid,
    /// No configuration parameters provided
    AdminConfigureDataMissing,
    /// The margin rate provided is invalid
    MarginRateInvalid,
    /// The margin rate provided would cause an immediate margin closeout
    MarginRateWouldTriggerCloseout,
    /// The account alias string provided is invalid
    AliasInvalid,
    /// No configuration parameters provided
    ClientConfigureDataMissing,
    /// The margin rate provided would cause the Account to enter a margin call state.
    MarginRateWouldTriggerMarginCall,
    /// Funding is not possible because the requested transfer amount is invalid
    AmountInvalid,
    /// The Account does not have sufficient balance to complete the funding request
    InsufficientFunds,
    /// Funding amount has not been specified
    AmountMissing,
    /// Funding reason has not been specified
    FundingReasonMissing,
    /// The list of Order Identifiers provided for a One Cancels All Order contains an Order Identifier that
    /// refers to a Stop Loss Order. OCA groups cannot contain Stop Loss Orders.
    OcaOrderIdsStopLossNotAllowed,
    /// Neither Order nor Trade on Fill client extensions were provided for modification
    ClientExtensionsDataMissing,
    /// The Order to be replaced has a different type than the replacing Order.
    ReplacingOrderInvalid,
    /// The replacing Order refers to a different Trade than the Order that is being replaced.
    ReplacingTradeIdInvalid,
    /// Canceling the order would cause an immediate margin closeout.
    OrderCancelWouldTriggerCloseout,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TransactionRejectReason {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
            Self::InstrumentPriceUnknown => "INSTRUMENT_PRICE_UNKNOWN",
            Self::AccountNotActive => "ACCOUNT_NOT_ACTIVE",
            Self::AccountLocked => "ACCOUNT_LOCKED",
            Self::AccountOrderCreationLocked => "ACCOUNT_ORDER_CREATION_LOCKED",
            Self::AccountConfigurationLocked => "ACCOUNT_CONFIGURATION_LOCKED",
            Self::AccountDepositLocked => "ACCOUNT_DEPOSIT_LOCKED",
            Self::AccountWithdrawalLocked => "ACCOUNT_WITHDRAWAL_LOCKED",
            Self::AccountOrderCancelLocked => "ACCOUNT_ORDER_CANCEL_LOCKED",
            Self::InstrumentNotTradeable => "INSTRUMENT_NOT_TRADEABLE",
            Self::PendingOrdersAllowedExceeded => "PENDING_ORDERS_ALLOWED_EXCEEDED",
            Self::OrderIdUnspecified => "ORDER_ID_UNSPECIFIED",
            Self::OrderDoesntExist => "ORDER_DOESNT_EXIST",
            Self::OrderIdentifierInconsistency => "ORDER_IDENTIFIER_INCONSISTENCY",
            Self::TradeIdUnspecified => "TRADE_ID_UNSPECIFIED",
            Self::TradeDoesntExist => "TRADE_DOESNT_EXIST",
            Self::TradeIdentifierInconsistency => "TRADE_IDENTIFIER_INCONSISTENCY",
            Self::InsufficientMargin => "INSUFFICIENT_MARGIN",
            Self::InstrumentMissing => "INSTRUMENT_MISSING",
            Self::InstrumentUnknown => "INSTRUMENT_UNKNOWN",
            Self::UnitsMissing => "UNITS_MISSING",
            Self::UnitsInvalid => "UNITS_INVALID",
            Self::UnitsPrecisionExceeded => "UNITS_PRECISION_EXCEEDED",
            Self::UnitsLimitExceeded => "UNITS_LIMIT_EXCEEDED",
            Self::UnitsMinimumNotMet => "UNITS_MINIMUM_NOT_MET",
            Self::PriceMissing => "PRICE_MISSING",
            Self::PriceInvalid => "PRICE_INVALID",
            Self::PricePrecisionExceeded => "PRICE_PRECISION_EXCEEDED",
            Self::PriceDistanceMissing => "PRICE_DISTANCE_MISSING",
            Self::PriceDistanceInvalid => "PRICE_DISTANCE_INVALID",
            Self::PriceDistancePrecisionExceeded => "PRICE_DISTANCE_PRECISION_EXCEEDED",
            Self::PriceDistanceMaximumExceeded => "PRICE_DISTANCE_MAXIMUM_EXCEEDED",
            Self::PriceDistanceMinimumNotMet => "PRICE_DISTANCE_MINIMUM_NOT_MET",
            Self::TimeInForceMissing => "TIME_IN_FORCE_MISSING",
            Self::TimeInForceInvalid => "TIME_IN_FORCE_INVALID",
            Self::TimeInForceGtdTimestampMissing => "TIME_IN_FORCE_GTD_TIMESTAMP_MISSING",
            Self::TimeInForceGtdTimestampInPast => "TIME_IN_FORCE_GTD_TIMESTAMP_IN_PAST",
            Self::PriceBoundInvalid => "PRICE_BOUND_INVALID",
            Self::PriceBoundPrecisionExceeded => "PRICE_BOUND_PRECISION_EXCEEDED",
            Self::OrdersOnFillDuplicateClientOrderIds => {
                "ORDERS_ON_FILL_DUPLICATE_CLIENT_ORDER_IDS"
            }
            Self::TradeOnFillClientExtensionsNotSupported => {
                "TRADE_ON_FILL_CLIENT_EXTENSIONS_NOT_SUPPORTED"
            }
            Self::ClientOrderIdInvalid => "CLIENT_ORDER_ID_INVALID",
            Self::ClientOrderIdAlreadyExists => "CLIENT_ORDER_ID_ALREADY_EXISTS",
            Self::ClientOrderTagInvalid => "CLIENT_ORDER_TAG_INVALID",
            Self::ClientOrderCommentInvalid => "CLIENT_ORDER_COMMENT_INVALID",
            Self::ClientTradeIdInvalid => "CLIENT_TRADE_ID_INVALID",
            Self::ClientTradeIdAlreadyExists => "CLIENT_TRADE_ID_ALREADY_EXISTS",
            Self::ClientTradeTagInvalid => "CLIENT_TRADE_TAG_INVALID",
            Self::ClientTradeCommentInvalid => "CLIENT_TRADE_COMMENT_INVALID",
            Self::OrderFillPositionActionMissing => "ORDER_FILL_POSITION_ACTION_MISSING",
            Self::OrderFillPositionActionInvalid => "ORDER_FILL_POSITION_ACTION_INVALID",
            Self::TriggerConditionMissing => "TRIGGER_CONDITION_MISSING",
            Self::TriggerConditionInvalid => "TRIGGER_CONDITION_INVALID",
            Self::OrderPartialFillOptionMissing => "ORDER_PARTIAL_FILL_OPTION_MISSING",
            Self::OrderPartialFillOptionInvalid => "ORDER_PARTIAL_FILL_OPTION_INVALID",
            Self::InvalidReissueImmediatePartialFill => "INVALID_REISSUE_IMMEDIATE_PARTIAL_FILL",
            Self::OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation => {
                "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION"
            }
            Self::OrdersOnFillRmoMutualExclusivityGsloExcludesOthersViolation => {
                "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION"
            }
            Self::TakeProfitOrderAlreadyExists => "TAKE_PROFIT_ORDER_ALREADY_EXISTS",
            Self::TakeProfitOrderWouldViolateFifoViolationSafeguard => {
                "TAKE_PROFIT_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD"
            }
            Self::TakeProfitOnFillPriceMissing => "TAKE_PROFIT_ON_FILL_PRICE_MISSING",
            Self::TakeProfitOnFillPriceInvalid => "TAKE_PROFIT_ON_FILL_PRICE_INVALID",
            Self::TakeProfitOnFillPricePrecisionExceeded => {
                "TAKE_PROFIT_ON_FILL_PRICE_PRECISION_EXCEEDED"
            }
            Self::TakeProfitOnFillTimeInForceMissing => "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_MISSING",
            Self::TakeProfitOnFillTimeInForceInvalid => "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_INVALID",
            Self::TakeProfitOnFillGtdTimestampMissing => {
                "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_MISSING"
            }
            Self::TakeProfitOnFillGtdTimestampInPast => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
            Self::TakeProfitOnFillClientOrderIdInvalid => {
                "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_INVALID"
            }
            Self::TakeProfitOnFillClientOrderTagInvalid => {
                "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_TAG_INVALID"
            }
            Self::TakeProfitOnFillClientOrderCommentInvalid => {
                "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_COMMENT_INVALID"
            }
            Self::TakeProfitOnFillTriggerConditionMissing => {
                "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_MISSING"
            }
            Self::TakeProfitOnFillTriggerConditionInvalid => {
                "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_INVALID"
            }
            Self::StopLossOrderAlreadyExists => "STOP_LOSS_ORDER_ALREADY_EXISTS",
            Self::StopLossOrderGuaranteedRequired => "STOP_LOSS_ORDER_GUARANTEED_REQUIRED",
            Self::StopLossOrderGuaranteedPriceWithinSpread => {
                "STOP_LOSS_ORDER_GUARANTEED_PRICE_WITHIN_SPREAD"
            }
            Self::StopLossOrderGuaranteedNotAllowed => "STOP_LOSS_ORDER_GUARANTEED_NOT_ALLOWED",
            Self::StopLossOrderGuaranteedHaltedCreateViolation => {
                "STOP_LOSS_ORDER_GUARANTEED_HALTED_CREATE_VIOLATION"
            }
            Self::StopLossOrderGuaranteedHaltedTightenViolation => {
                "STOP_LOSS_ORDER_GUARANTEED_HALTED_TIGHTEN_VIOLATION"
            }
            Self::StopLossOrderGuaranteedHedgingNotAllowed => {
                "STOP_LOSS_ORDER_GUARANTEED_HEDGING_NOT_ALLOWED"
            }
            Self::StopLossOrderGuaranteedMinimumDistanceNotMet => {
                "STOP_LOSS_ORDER_GUARANTEED_MINIMUM_DISTANCE_NOT_MET"
            }
            Self::StopLossOrderNotCancelable => "STOP_LOSS_ORDER_NOT_CANCELABLE",
            Self::StopLossOrderNotReplaceable => "STOP_LOSS_ORDER_NOT_REPLACEABLE",
            Self::StopLossOrderGuaranteedLevelRestrictionExceeded => {
                "STOP_LOSS_ORDER_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED"
            }
            Self::StopLossOrderPriceAndDistanceBothSpecified => {
                "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED"
            }
            Self::StopLossOrderPriceAndDistanceBothMissing => {
                "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING"
            }
            Self::StopLossOrderWouldViolateFifoViolationSafeguard => {
                "STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD"
            }
            Self::StopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation => {
                "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION"
            }
            Self::StopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation => {
                "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION"
            }
            Self::StopLossOnFillRequiredForPendingOrder => {
                "STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER"
            }
            Self::StopLossOnFillGuaranteedNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
            Self::StopLossOnFillGuaranteedRequired => "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
            Self::StopLossOnFillPriceMissing => "STOP_LOSS_ON_FILL_PRICE_MISSING",
            Self::StopLossOnFillPriceInvalid => "STOP_LOSS_ON_FILL_PRICE_INVALID",
            Self::StopLossOnFillPricePrecisionExceeded => {
                "STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED"
            }
            Self::StopLossOnFillGuaranteedMinimumDistanceNotMet => {
                "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET"
            }
            Self::StopLossOnFillGuaranteedLevelRestrictionExceeded => {
                "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED"
            }
            Self::StopLossOnFillDistanceInvalid => "STOP_LOSS_ON_FILL_DISTANCE_INVALID",
            Self::StopLossOnFillPriceDistanceMaximumExceeded => {
                "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED"
            }
            Self::StopLossOnFillDistancePrecisionExceeded => {
                "STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED"
            }
            Self::StopLossOnFillPriceAndDistanceBothSpecified => {
                "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED"
            }
            Self::StopLossOnFillPriceAndDistanceBothMissing => {
                "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING"
            }
            Self::StopLossOnFillTimeInForceMissing => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
            Self::StopLossOnFillTimeInForceInvalid => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
            Self::StopLossOnFillGtdTimestampMissing => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
            Self::StopLossOnFillGtdTimestampInPast => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
            Self::StopLossOnFillClientOrderIdInvalid => "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
            Self::StopLossOnFillClientOrderTagInvalid => {
                "STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID"
            }
            Self::StopLossOnFillClientOrderCommentInvalid => {
                "STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID"
            }
            Self::StopLossOnFillTriggerConditionMissing => {
                "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING"
            }
            Self::StopLossOnFillTriggerConditionInvalid => {
                "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID"
            }
            Self::GuaranteedStopLossOrderAlreadyExists => {
                "GUARANTEED_STOP_LOSS_ORDER_ALREADY_EXISTS"
            }
            Self::GuaranteedStopLossOrderRequired => "GUARANTEED_STOP_LOSS_ORDER_REQUIRED",
            Self::GuaranteedStopLossOrderPriceWithinSpread => {
                "GUARANTEED_STOP_LOSS_ORDER_PRICE_WITHIN_SPREAD"
            }
            Self::GuaranteedStopLossOrderNotAllowed => "GUARANTEED_STOP_LOSS_ORDER_NOT_ALLOWED",
            Self::GuaranteedStopLossOrderHaltedCreateViolation => {
                "GUARANTEED_STOP_LOSS_ORDER_HALTED_CREATE_VIOLATION"
            }
            Self::GuaranteedStopLossOrderCreateViolation => {
                "GUARANTEED_STOP_LOSS_ORDER_CREATE_VIOLATION"
            }
            Self::GuaranteedStopLossOrderHaltedTightenViolation => {
                "GUARANTEED_STOP_LOSS_ORDER_HALTED_TIGHTEN_VIOLATION"
            }
            Self::GuaranteedStopLossOrderTightenViolation => {
                "GUARANTEED_STOP_LOSS_ORDER_TIGHTEN_VIOLATION"
            }
            Self::GuaranteedStopLossOrderHedgingNotAllowed => {
                "GUARANTEED_STOP_LOSS_ORDER_HEDGING_NOT_ALLOWED"
            }
            Self::GuaranteedStopLossOrderMinimumDistanceNotMet => {
                "GUARANTEED_STOP_LOSS_ORDER_MINIMUM_DISTANCE_NOT_MET"
            }
            Self::GuaranteedStopLossOrderNotCancelable => {
                "GUARANTEED_STOP_LOSS_ORDER_NOT_CANCELABLE"
            }
            Self::GuaranteedStopLossOrderHaltedNotCancelable => {
                "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_CANCELABLE"
            }
            Self::GuaranteedStopLossOrderNotReplaceable => {
                "GUARANTEED_STOP_LOSS_ORDER_NOT_REPLACEABLE"
            }
            Self::GuaranteedStopLossOrderHaltedNotReplaceable => {
                "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_REPLACEABLE"
            }
            Self::GuaranteedStopLossOrderLevelRestrictionVolumeExceeded => {
                "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_VOLUME_EXCEEDED"
            }
            Self::GuaranteedStopLossOrderLevelRestrictionPriceRangeExceeded => {
                "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED"
            }
            Self::GuaranteedStopLossOrderPriceAndDistanceBothSpecified => {
                "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED"
            }
            Self::GuaranteedStopLossOrderPriceAndDistanceBothMissing => {
                "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING"
            }
            Self::GuaranteedStopLossOrderWouldViolateFifoViolationSafeguard => {
                "GUARANTEED_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD"
            }
            Self::GuaranteedStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation => {
                "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION"
            }
            Self::GuaranteedStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation => {
                "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION"
            }
            Self::GuaranteedStopLossOnFillRequiredForPendingOrder => {
                "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER"
            }
            Self::GuaranteedStopLossOnFillNotAllowed => "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED",
            Self::GuaranteedStopLossOnFillRequired => "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED",
            Self::GuaranteedStopLossOnFillPriceMissing => {
                "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_MISSING"
            }
            Self::GuaranteedStopLossOnFillPriceInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_INVALID"
            }
            Self::GuaranteedStopLossOnFillPricePrecisionExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillMinimumDistanceNotMet => {
                "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET"
            }
            Self::GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillDistanceInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_INVALID"
            }
            Self::GuaranteedStopLossOnFillPriceDistanceMaximumExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillDistancePrecisionExceeded => {
                "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED"
            }
            Self::GuaranteedStopLossOnFillPriceAndDistanceBothSpecified => {
                "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED"
            }
            Self::GuaranteedStopLossOnFillPriceAndDistanceBothMissing => {
                "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING"
            }
            Self::GuaranteedStopLossOnFillTimeInForceMissing => {
                "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING"
            }
            Self::GuaranteedStopLossOnFillTimeInForceInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID"
            }
            Self::GuaranteedStopLossOnFillGtdTimestampMissing => {
                "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING"
            }
            Self::GuaranteedStopLossOnFillGtdTimestampInPast => {
                "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST"
            }
            Self::GuaranteedStopLossOnFillClientOrderIdInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID"
            }
            Self::GuaranteedStopLossOnFillClientOrderTagInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID"
            }
            Self::GuaranteedStopLossOnFillClientOrderCommentInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID"
            }
            Self::GuaranteedStopLossOnFillTriggerConditionMissing => {
                "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING"
            }
            Self::GuaranteedStopLossOnFillTriggerConditionInvalid => {
                "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID"
            }
            Self::TrailingStopLossOrderAlreadyExists => "TRAILING_STOP_LOSS_ORDER_ALREADY_EXISTS",
            Self::TrailingStopLossOrderWouldViolateFifoViolationSafeguard => {
                "TRAILING_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD"
            }
            Self::TrailingStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation => {
                "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION"
            }
            Self::TrailingStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation => {
                "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION"
            }
            Self::TrailingStopLossOnFillPriceDistanceMissing => {
                "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MISSING"
            }
            Self::TrailingStopLossOnFillPriceDistanceInvalid => {
                "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_INVALID"
            }
            Self::TrailingStopLossOnFillPriceDistancePrecisionExceeded => {
                "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_PRECISION_EXCEEDED"
            }
            Self::TrailingStopLossOnFillPriceDistanceMaximumExceeded => {
                "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED"
            }
            Self::TrailingStopLossOnFillPriceDistanceMinimumNotMet => {
                "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MINIMUM_NOT_MET"
            }
            Self::TrailingStopLossOnFillTimeInForceMissing => {
                "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING"
            }
            Self::TrailingStopLossOnFillTimeInForceInvalid => {
                "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID"
            }
            Self::TrailingStopLossOnFillGtdTimestampMissing => {
                "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING"
            }
            Self::TrailingStopLossOnFillGtdTimestampInPast => {
                "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST"
            }
            Self::TrailingStopLossOnFillClientOrderIdInvalid => {
                "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID"
            }
            Self::TrailingStopLossOnFillClientOrderTagInvalid => {
                "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID"
            }
            Self::TrailingStopLossOnFillClientOrderCommentInvalid => {
                "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID"
            }
            Self::TrailingStopLossOrdersNotSupported => "TRAILING_STOP_LOSS_ORDERS_NOT_SUPPORTED",
            Self::TrailingStopLossOnFillTriggerConditionMissing => {
                "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING"
            }
            Self::TrailingStopLossOnFillTriggerConditionInvalid => {
                "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID"
            }
            Self::CloseTradeTypeMissing => "CLOSE_TRADE_TYPE_MISSING",
            Self::CloseTradePartialUnitsMissing => "CLOSE_TRADE_PARTIAL_UNITS_MISSING",
            Self::CloseTradeUnitsExceedTradeSize => "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE",
            Self::CloseoutPositionDoesntExist => "CLOSEOUT_POSITION_DOESNT_EXIST",
            Self::CloseoutPositionIncompleteSpecification => {
                "CLOSEOUT_POSITION_INCOMPLETE_SPECIFICATION"
            }
            Self::CloseoutPositionUnitsExceedPositionSize => {
                "CLOSEOUT_POSITION_UNITS_EXCEED_POSITION_SIZE"
            }
            Self::CloseoutPositionReject => "CLOSEOUT_POSITION_REJECT",
            Self::CloseoutPositionPartialUnitsMissing => "CLOSEOUT_POSITION_PARTIAL_UNITS_MISSING",
            Self::MarkupGroupIdInvalid => "MARKUP_GROUP_ID_INVALID",
            Self::PositionAggregationModeInvalid => "POSITION_AGGREGATION_MODE_INVALID",
            Self::AdminConfigureDataMissing => "ADMIN_CONFIGURE_DATA_MISSING",
            Self::MarginRateInvalid => "MARGIN_RATE_INVALID",
            Self::MarginRateWouldTriggerCloseout => "MARGIN_RATE_WOULD_TRIGGER_CLOSEOUT",
            Self::AliasInvalid => "ALIAS_INVALID",
            Self::ClientConfigureDataMissing => "CLIENT_CONFIGURE_DATA_MISSING",
            Self::MarginRateWouldTriggerMarginCall => "MARGIN_RATE_WOULD_TRIGGER_MARGIN_CALL",
            Self::AmountInvalid => "AMOUNT_INVALID",
            Self::InsufficientFunds => "INSUFFICIENT_FUNDS",
            Self::AmountMissing => "AMOUNT_MISSING",
            Self::FundingReasonMissing => "FUNDING_REASON_MISSING",
            Self::OcaOrderIdsStopLossNotAllowed => "OCA_ORDER_IDS_STOP_LOSS_NOT_ALLOWED",
            Self::ClientExtensionsDataMissing => "CLIENT_EXTENSIONS_DATA_MISSING",
            Self::ReplacingOrderInvalid => "REPLACING_ORDER_INVALID",
            Self::ReplacingTradeIdInvalid => "REPLACING_TRADE_ID_INVALID",
            Self::OrderCancelWouldTriggerCloseout => "ORDER_CANCEL_WOULD_TRIGGER_CLOSEOUT",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TransactionRejectReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TransactionRejectReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "INTERNAL_SERVER_ERROR" => Self::InternalServerError,
            "INSTRUMENT_PRICE_UNKNOWN" => Self::InstrumentPriceUnknown,
            "ACCOUNT_NOT_ACTIVE" => Self::AccountNotActive,
            "ACCOUNT_LOCKED" => Self::AccountLocked,
            "ACCOUNT_ORDER_CREATION_LOCKED" => Self::AccountOrderCreationLocked,
            "ACCOUNT_CONFIGURATION_LOCKED" => Self::AccountConfigurationLocked,
            "ACCOUNT_DEPOSIT_LOCKED" => Self::AccountDepositLocked,
            "ACCOUNT_WITHDRAWAL_LOCKED" => Self::AccountWithdrawalLocked,
            "ACCOUNT_ORDER_CANCEL_LOCKED" => Self::AccountOrderCancelLocked,
            "INSTRUMENT_NOT_TRADEABLE" => Self::InstrumentNotTradeable,
            "PENDING_ORDERS_ALLOWED_EXCEEDED" => Self::PendingOrdersAllowedExceeded,
            "ORDER_ID_UNSPECIFIED" => Self::OrderIdUnspecified,
            "ORDER_DOESNT_EXIST" => Self::OrderDoesntExist,
            "ORDER_IDENTIFIER_INCONSISTENCY" => Self::OrderIdentifierInconsistency,
            "TRADE_ID_UNSPECIFIED" => Self::TradeIdUnspecified,
            "TRADE_DOESNT_EXIST" => Self::TradeDoesntExist,
            "TRADE_IDENTIFIER_INCONSISTENCY" => Self::TradeIdentifierInconsistency,
            "INSUFFICIENT_MARGIN" => Self::InsufficientMargin,
            "INSTRUMENT_MISSING" => Self::InstrumentMissing,
            "INSTRUMENT_UNKNOWN" => Self::InstrumentUnknown,
            "UNITS_MISSING" => Self::UnitsMissing,
            "UNITS_INVALID" => Self::UnitsInvalid,
            "UNITS_PRECISION_EXCEEDED" => Self::UnitsPrecisionExceeded,
            "UNITS_LIMIT_EXCEEDED" => Self::UnitsLimitExceeded,
            "UNITS_MINIMUM_NOT_MET" => Self::UnitsMinimumNotMet,
            "PRICE_MISSING" => Self::PriceMissing,
            "PRICE_INVALID" => Self::PriceInvalid,
            "PRICE_PRECISION_EXCEEDED" => Self::PricePrecisionExceeded,
            "PRICE_DISTANCE_MISSING" => Self::PriceDistanceMissing,
            "PRICE_DISTANCE_INVALID" => Self::PriceDistanceInvalid,
            "PRICE_DISTANCE_PRECISION_EXCEEDED" => Self::PriceDistancePrecisionExceeded,
            "PRICE_DISTANCE_MAXIMUM_EXCEEDED" => Self::PriceDistanceMaximumExceeded,
            "PRICE_DISTANCE_MINIMUM_NOT_MET" => Self::PriceDistanceMinimumNotMet,
            "TIME_IN_FORCE_MISSING" => Self::TimeInForceMissing,
            "TIME_IN_FORCE_INVALID" => Self::TimeInForceInvalid,
            "TIME_IN_FORCE_GTD_TIMESTAMP_MISSING" => Self::TimeInForceGtdTimestampMissing,
            "TIME_IN_FORCE_GTD_TIMESTAMP_IN_PAST" => Self::TimeInForceGtdTimestampInPast,
            "PRICE_BOUND_INVALID" => Self::PriceBoundInvalid,
            "PRICE_BOUND_PRECISION_EXCEEDED" => Self::PriceBoundPrecisionExceeded,
            "ORDERS_ON_FILL_DUPLICATE_CLIENT_ORDER_IDS" => {
                Self::OrdersOnFillDuplicateClientOrderIds
            }
            "TRADE_ON_FILL_CLIENT_EXTENSIONS_NOT_SUPPORTED" => {
                Self::TradeOnFillClientExtensionsNotSupported
            }
            "CLIENT_ORDER_ID_INVALID" => Self::ClientOrderIdInvalid,
            "CLIENT_ORDER_ID_ALREADY_EXISTS" => Self::ClientOrderIdAlreadyExists,
            "CLIENT_ORDER_TAG_INVALID" => Self::ClientOrderTagInvalid,
            "CLIENT_ORDER_COMMENT_INVALID" => Self::ClientOrderCommentInvalid,
            "CLIENT_TRADE_ID_INVALID" => Self::ClientTradeIdInvalid,
            "CLIENT_TRADE_ID_ALREADY_EXISTS" => Self::ClientTradeIdAlreadyExists,
            "CLIENT_TRADE_TAG_INVALID" => Self::ClientTradeTagInvalid,
            "CLIENT_TRADE_COMMENT_INVALID" => Self::ClientTradeCommentInvalid,
            "ORDER_FILL_POSITION_ACTION_MISSING" => Self::OrderFillPositionActionMissing,
            "ORDER_FILL_POSITION_ACTION_INVALID" => Self::OrderFillPositionActionInvalid,
            "TRIGGER_CONDITION_MISSING" => Self::TriggerConditionMissing,
            "TRIGGER_CONDITION_INVALID" => Self::TriggerConditionInvalid,
            "ORDER_PARTIAL_FILL_OPTION_MISSING" => Self::OrderPartialFillOptionMissing,
            "ORDER_PARTIAL_FILL_OPTION_INVALID" => Self::OrderPartialFillOptionInvalid,
            "INVALID_REISSUE_IMMEDIATE_PARTIAL_FILL" => Self::InvalidReissueImmediatePartialFill,
            "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION" => {
                Self::OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation
            }
            "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION" => {
                Self::OrdersOnFillRmoMutualExclusivityGsloExcludesOthersViolation
            }
            "TAKE_PROFIT_ORDER_ALREADY_EXISTS" => Self::TakeProfitOrderAlreadyExists,
            "TAKE_PROFIT_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD" => {
                Self::TakeProfitOrderWouldViolateFifoViolationSafeguard
            }
            "TAKE_PROFIT_ON_FILL_PRICE_MISSING" => Self::TakeProfitOnFillPriceMissing,
            "TAKE_PROFIT_ON_FILL_PRICE_INVALID" => Self::TakeProfitOnFillPriceInvalid,
            "TAKE_PROFIT_ON_FILL_PRICE_PRECISION_EXCEEDED" => {
                Self::TakeProfitOnFillPricePrecisionExceeded
            }
            "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_MISSING" => Self::TakeProfitOnFillTimeInForceMissing,
            "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_INVALID" => Self::TakeProfitOnFillTimeInForceInvalid,
            "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_MISSING" => {
                Self::TakeProfitOnFillGtdTimestampMissing
            }
            "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST" => Self::TakeProfitOnFillGtdTimestampInPast,
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_INVALID" => {
                Self::TakeProfitOnFillClientOrderIdInvalid
            }
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_TAG_INVALID" => {
                Self::TakeProfitOnFillClientOrderTagInvalid
            }
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_COMMENT_INVALID" => {
                Self::TakeProfitOnFillClientOrderCommentInvalid
            }
            "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_MISSING" => {
                Self::TakeProfitOnFillTriggerConditionMissing
            }
            "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Self::TakeProfitOnFillTriggerConditionInvalid
            }
            "STOP_LOSS_ORDER_ALREADY_EXISTS" => Self::StopLossOrderAlreadyExists,
            "STOP_LOSS_ORDER_GUARANTEED_REQUIRED" => Self::StopLossOrderGuaranteedRequired,
            "STOP_LOSS_ORDER_GUARANTEED_PRICE_WITHIN_SPREAD" => {
                Self::StopLossOrderGuaranteedPriceWithinSpread
            }
            "STOP_LOSS_ORDER_GUARANTEED_NOT_ALLOWED" => Self::StopLossOrderGuaranteedNotAllowed,
            "STOP_LOSS_ORDER_GUARANTEED_HALTED_CREATE_VIOLATION" => {
                Self::StopLossOrderGuaranteedHaltedCreateViolation
            }
            "STOP_LOSS_ORDER_GUARANTEED_HALTED_TIGHTEN_VIOLATION" => {
                Self::StopLossOrderGuaranteedHaltedTightenViolation
            }
            "STOP_LOSS_ORDER_GUARANTEED_HEDGING_NOT_ALLOWED" => {
                Self::StopLossOrderGuaranteedHedgingNotAllowed
            }
            "STOP_LOSS_ORDER_GUARANTEED_MINIMUM_DISTANCE_NOT_MET" => {
                Self::StopLossOrderGuaranteedMinimumDistanceNotMet
            }
            "STOP_LOSS_ORDER_NOT_CANCELABLE" => Self::StopLossOrderNotCancelable,
            "STOP_LOSS_ORDER_NOT_REPLACEABLE" => Self::StopLossOrderNotReplaceable,
            "STOP_LOSS_ORDER_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED" => {
                Self::StopLossOrderGuaranteedLevelRestrictionExceeded
            }
            "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED" => {
                Self::StopLossOrderPriceAndDistanceBothSpecified
            }
            "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING" => {
                Self::StopLossOrderPriceAndDistanceBothMissing
            }
            "STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD" => {
                Self::StopLossOrderWouldViolateFifoViolationSafeguard
            }
            "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION" => {
                Self::StopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation
            }
            "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION" => {
                Self::StopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation
            }
            "STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER" => {
                Self::StopLossOnFillRequiredForPendingOrder
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED" => Self::StopLossOnFillGuaranteedNotAllowed,
            "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED" => Self::StopLossOnFillGuaranteedRequired,
            "STOP_LOSS_ON_FILL_PRICE_MISSING" => Self::StopLossOnFillPriceMissing,
            "STOP_LOSS_ON_FILL_PRICE_INVALID" => Self::StopLossOnFillPriceInvalid,
            "STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED" => {
                Self::StopLossOnFillPricePrecisionExceeded
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET" => {
                Self::StopLossOnFillGuaranteedMinimumDistanceNotMet
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED" => {
                Self::StopLossOnFillGuaranteedLevelRestrictionExceeded
            }
            "STOP_LOSS_ON_FILL_DISTANCE_INVALID" => Self::StopLossOnFillDistanceInvalid,
            "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Self::StopLossOnFillPriceDistanceMaximumExceeded
            }
            "STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED" => {
                Self::StopLossOnFillDistancePrecisionExceeded
            }
            "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED" => {
                Self::StopLossOnFillPriceAndDistanceBothSpecified
            }
            "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING" => {
                Self::StopLossOnFillPriceAndDistanceBothMissing
            }
            "STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING" => Self::StopLossOnFillTimeInForceMissing,
            "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => Self::StopLossOnFillTimeInForceInvalid,
            "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING" => Self::StopLossOnFillGtdTimestampMissing,
            "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => Self::StopLossOnFillGtdTimestampInPast,
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID" => Self::StopLossOnFillClientOrderIdInvalid,
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID" => {
                Self::StopLossOnFillClientOrderTagInvalid
            }
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID" => {
                Self::StopLossOnFillClientOrderCommentInvalid
            }
            "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING" => {
                Self::StopLossOnFillTriggerConditionMissing
            }
            "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Self::StopLossOnFillTriggerConditionInvalid
            }
            "GUARANTEED_STOP_LOSS_ORDER_ALREADY_EXISTS" => {
                Self::GuaranteedStopLossOrderAlreadyExists
            }
            "GUARANTEED_STOP_LOSS_ORDER_REQUIRED" => Self::GuaranteedStopLossOrderRequired,
            "GUARANTEED_STOP_LOSS_ORDER_PRICE_WITHIN_SPREAD" => {
                Self::GuaranteedStopLossOrderPriceWithinSpread
            }
            "GUARANTEED_STOP_LOSS_ORDER_NOT_ALLOWED" => Self::GuaranteedStopLossOrderNotAllowed,
            "GUARANTEED_STOP_LOSS_ORDER_HALTED_CREATE_VIOLATION" => {
                Self::GuaranteedStopLossOrderHaltedCreateViolation
            }
            "GUARANTEED_STOP_LOSS_ORDER_CREATE_VIOLATION" => {
                Self::GuaranteedStopLossOrderCreateViolation
            }
            "GUARANTEED_STOP_LOSS_ORDER_HALTED_TIGHTEN_VIOLATION" => {
                Self::GuaranteedStopLossOrderHaltedTightenViolation
            }
            "GUARANTEED_STOP_LOSS_ORDER_TIGHTEN_VIOLATION" => {
                Self::GuaranteedStopLossOrderTightenViolation
            }
            "GUARANTEED_STOP_LOSS_ORDER_HEDGING_NOT_ALLOWED" => {
                Self::GuaranteedStopLossOrderHedgingNotAllowed
            }
            "GUARANTEED_STOP_LOSS_ORDER_MINIMUM_DISTANCE_NOT_MET" => {
                Self::GuaranteedStopLossOrderMinimumDistanceNotMet
            }
            "GUARANTEED_STOP_LOSS_ORDER_NOT_CANCELABLE" => {
                Self::GuaranteedStopLossOrderNotCancelable
            }
            "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_CANCELABLE" => {
                Self::GuaranteedStopLossOrderHaltedNotCancelable
            }
            "GUARANTEED_STOP_LOSS_ORDER_NOT_REPLACEABLE" => {
                Self::GuaranteedStopLossOrderNotReplaceable
            }
            "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_REPLACEABLE" => {
                Self::GuaranteedStopLossOrderHaltedNotReplaceable
            }
            "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_VOLUME_EXCEEDED" => {
                Self::GuaranteedStopLossOrderLevelRestrictionVolumeExceeded
            }
            "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED" => {
                Self::GuaranteedStopLossOrderLevelRestrictionPriceRangeExceeded
            }
            "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED" => {
                Self::GuaranteedStopLossOrderPriceAndDistanceBothSpecified
            }
            "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING" => {
                Self::GuaranteedStopLossOrderPriceAndDistanceBothMissing
            }
            "GUARANTEED_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD" => {
                Self::GuaranteedStopLossOrderWouldViolateFifoViolationSafeguard
            }
            "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION" => {
                Self::GuaranteedStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation
            }
            "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION" => {
                Self::GuaranteedStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER" => {
                Self::GuaranteedStopLossOnFillRequiredForPendingOrder
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED" => Self::GuaranteedStopLossOnFillNotAllowed,
            "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED" => Self::GuaranteedStopLossOnFillRequired,
            "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_MISSING" => {
                Self::GuaranteedStopLossOnFillPriceMissing
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_INVALID" => {
                Self::GuaranteedStopLossOnFillPriceInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillPricePrecisionExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET" => {
                Self::GuaranteedStopLossOnFillMinimumDistanceNotMet
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_INVALID" => {
                Self::GuaranteedStopLossOnFillDistanceInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillPriceDistanceMaximumExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED" => {
                Self::GuaranteedStopLossOnFillDistancePrecisionExceeded
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED" => {
                Self::GuaranteedStopLossOnFillPriceAndDistanceBothSpecified
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING" => {
                Self::GuaranteedStopLossOnFillPriceAndDistanceBothMissing
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING" => {
                Self::GuaranteedStopLossOnFillTimeInForceMissing
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => {
                Self::GuaranteedStopLossOnFillTimeInForceInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING" => {
                Self::GuaranteedStopLossOnFillGtdTimestampMissing
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Self::GuaranteedStopLossOnFillGtdTimestampInPast
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID" => {
                Self::GuaranteedStopLossOnFillClientOrderIdInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID" => {
                Self::GuaranteedStopLossOnFillClientOrderTagInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID" => {
                Self::GuaranteedStopLossOnFillClientOrderCommentInvalid
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING" => {
                Self::GuaranteedStopLossOnFillTriggerConditionMissing
            }
            "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Self::GuaranteedStopLossOnFillTriggerConditionInvalid
            }
            "TRAILING_STOP_LOSS_ORDER_ALREADY_EXISTS" => Self::TrailingStopLossOrderAlreadyExists,
            "TRAILING_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD" => {
                Self::TrailingStopLossOrderWouldViolateFifoViolationSafeguard
            }
            "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION" => {
                Self::TrailingStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation
            }
            "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION" => {
                Self::TrailingStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MISSING" => {
                Self::TrailingStopLossOnFillPriceDistanceMissing
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_INVALID" => {
                Self::TrailingStopLossOnFillPriceDistanceInvalid
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_PRECISION_EXCEEDED" => {
                Self::TrailingStopLossOnFillPriceDistancePrecisionExceeded
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Self::TrailingStopLossOnFillPriceDistanceMaximumExceeded
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MINIMUM_NOT_MET" => {
                Self::TrailingStopLossOnFillPriceDistanceMinimumNotMet
            }
            "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING" => {
                Self::TrailingStopLossOnFillTimeInForceMissing
            }
            "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => {
                Self::TrailingStopLossOnFillTimeInForceInvalid
            }
            "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING" => {
                Self::TrailingStopLossOnFillGtdTimestampMissing
            }
            "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Self::TrailingStopLossOnFillGtdTimestampInPast
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID" => {
                Self::TrailingStopLossOnFillClientOrderIdInvalid
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID" => {
                Self::TrailingStopLossOnFillClientOrderTagInvalid
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID" => {
                Self::TrailingStopLossOnFillClientOrderCommentInvalid
            }
            "TRAILING_STOP_LOSS_ORDERS_NOT_SUPPORTED" => Self::TrailingStopLossOrdersNotSupported,
            "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING" => {
                Self::TrailingStopLossOnFillTriggerConditionMissing
            }
            "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Self::TrailingStopLossOnFillTriggerConditionInvalid
            }
            "CLOSE_TRADE_TYPE_MISSING" => Self::CloseTradeTypeMissing,
            "CLOSE_TRADE_PARTIAL_UNITS_MISSING" => Self::CloseTradePartialUnitsMissing,
            "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE" => Self::CloseTradeUnitsExceedTradeSize,
            "CLOSEOUT_POSITION_DOESNT_EXIST" => Self::CloseoutPositionDoesntExist,
            "CLOSEOUT_POSITION_INCOMPLETE_SPECIFICATION" => {
                Self::CloseoutPositionIncompleteSpecification
            }
            "CLOSEOUT_POSITION_UNITS_EXCEED_POSITION_SIZE" => {
                Self::CloseoutPositionUnitsExceedPositionSize
            }
            "CLOSEOUT_POSITION_REJECT" => Self::CloseoutPositionReject,
            "CLOSEOUT_POSITION_PARTIAL_UNITS_MISSING" => Self::CloseoutPositionPartialUnitsMissing,
            "MARKUP_GROUP_ID_INVALID" => Self::MarkupGroupIdInvalid,
            "POSITION_AGGREGATION_MODE_INVALID" => Self::PositionAggregationModeInvalid,
            "ADMIN_CONFIGURE_DATA_MISSING" => Self::AdminConfigureDataMissing,
            "MARGIN_RATE_INVALID" => Self::MarginRateInvalid,
            "MARGIN_RATE_WOULD_TRIGGER_CLOSEOUT" => Self::MarginRateWouldTriggerCloseout,
            "ALIAS_INVALID" => Self::AliasInvalid,
            "CLIENT_CONFIGURE_DATA_MISSING" => Self::ClientConfigureDataMissing,
            "MARGIN_RATE_WOULD_TRIGGER_MARGIN_CALL" => Self::MarginRateWouldTriggerMarginCall,
            "AMOUNT_INVALID" => Self::AmountInvalid,
            "INSUFFICIENT_FUNDS" => Self::InsufficientFunds,
            "AMOUNT_MISSING" => Self::AmountMissing,
            "FUNDING_REASON_MISSING" => Self::FundingReasonMissing,
            "OCA_ORDER_IDS_STOP_LOSS_NOT_ALLOWED" => Self::OcaOrderIdsStopLossNotAllowed,
            "CLIENT_EXTENSIONS_DATA_MISSING" => Self::ClientExtensionsDataMissing,
            "REPLACING_ORDER_INVALID" => Self::ReplacingOrderInvalid,
            "REPLACING_TRADE_ID_INVALID" => Self::ReplacingTradeIdInvalid,
            "ORDER_CANCEL_WOULD_TRIGGER_CLOSEOUT" => Self::OrderCancelWouldTriggerCloseout,
            _ => Self::Unknown(value),
        })
    }
}

/// A filter that can be used when fetching Transactions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransactionFilter {
    /// Order-related Transactions. These are the Transactions that create, cancel, fill or trigger Orders
    Order,
    /// Funding-related Transactions
    Funding,
    /// Administrative Transactions
    Admin,
    /// Account Create Transaction
    Create,
    /// Account Close Transaction
    Close,
    /// Account Reopen Transaction
    Reopen,
    /// Client Configuration Transaction
    ClientConfigure,
    /// Client Configuration Reject Transaction
    ClientConfigureReject,
    /// Transfer Funds Transaction
    TransferFunds,
    /// Transfer Funds Reject Transaction
    TransferFundsReject,
    /// Market Order Transaction
    MarketOrder,
    /// Market Order Reject Transaction
    MarketOrderReject,
    /// Limit Order Transaction
    LimitOrder,
    /// Limit Order Reject Transaction
    LimitOrderReject,
    /// Stop Order Transaction
    StopOrder,
    /// Stop Order Reject Transaction
    StopOrderReject,
    /// Market if Touched Order Transaction
    MarketIfTouchedOrder,
    /// Market if Touched Order Reject Transaction
    MarketIfTouchedOrderReject,
    /// Take Profit Order Transaction
    TakeProfitOrder,
    /// Take Profit Order Reject Transaction
    TakeProfitOrderReject,
    /// Stop Loss Order Transaction
    StopLossOrder,
    /// Stop Loss Order Reject Transaction
    StopLossOrderReject,
    /// Guaranteed Stop Loss Order Transaction
    GuaranteedStopLossOrder,
    /// Guaranteed Stop Loss Order Reject Transaction
    GuaranteedStopLossOrderReject,
    /// Trailing Stop Loss Order Transaction
    TrailingStopLossOrder,
    /// Trailing Stop Loss Order Reject Transaction
    TrailingStopLossOrderReject,
    /// One Cancels All Order Transaction
    OneCancelsAllOrder,
    /// One Cancels All Order Reject Transaction
    OneCancelsAllOrderReject,
    /// One Cancels All Order Trigger Transaction
    OneCancelsAllOrderTriggered,
    /// Order Fill Transaction
    OrderFill,
    /// Order Cancel Transaction
    OrderCancel,
    /// Order Cancel Reject Transaction
    OrderCancelReject,
    /// Order Client Extensions Modify Transaction
    OrderClientExtensionsModify,
    /// Order Client Extensions Modify Reject Transaction
    OrderClientExtensionsModifyReject,
    /// Trade Client Extensions Modify Transaction
    TradeClientExtensionsModify,
    /// Trade Client Extensions Modify Reject Transaction
    TradeClientExtensionsModifyReject,
    /// Margin Call Enter Transaction
    MarginCallEnter,
    /// Margin Call Extend Transaction
    MarginCallExtend,
    /// Margin Call Exit Transaction
    MarginCallExit,
    /// Delayed Trade Closure Transaction
    DelayedTradeClosure,
    /// Daily Financing Transaction
    DailyFinancing,
    /// Reset Resettable PL Transaction
    ResetResettablePl,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl TransactionFilter {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Order => "ORDER",
            Self::Funding => "FUNDING",
            Self::Admin => "ADMIN",
            Self::Create => "CREATE",
            Self::Close => "CLOSE",
            Self::Reopen => "REOPEN",
            Self::ClientConfigure => "CLIENT_CONFIGURE",
            Self::ClientConfigureReject => "CLIENT_CONFIGURE_REJECT",
            Self::TransferFunds => "TRANSFER_FUNDS",
            Self::TransferFundsReject => "TRANSFER_FUNDS_REJECT",
            Self::MarketOrder => "MARKET_ORDER",
            Self::MarketOrderReject => "MARKET_ORDER_REJECT",
            Self::LimitOrder => "LIMIT_ORDER",
            Self::LimitOrderReject => "LIMIT_ORDER_REJECT",
            Self::StopOrder => "STOP_ORDER",
            Self::StopOrderReject => "STOP_ORDER_REJECT",
            Self::MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
            Self::MarketIfTouchedOrderReject => "MARKET_IF_TOUCHED_ORDER_REJECT",
            Self::TakeProfitOrder => "TAKE_PROFIT_ORDER",
            Self::TakeProfitOrderReject => "TAKE_PROFIT_ORDER_REJECT",
            Self::StopLossOrder => "STOP_LOSS_ORDER",
            Self::StopLossOrderReject => "STOP_LOSS_ORDER_REJECT",
            Self::GuaranteedStopLossOrder => "GUARANTEED_STOP_LOSS_ORDER",
            Self::GuaranteedStopLossOrderReject => "GUARANTEED_STOP_LOSS_ORDER_REJECT",
            Self::TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
            Self::TrailingStopLossOrderReject => "TRAILING_STOP_LOSS_ORDER_REJECT",
            Self::OneCancelsAllOrder => "ONE_CANCELS_ALL_ORDER",
            Self::OneCancelsAllOrderReject => "ONE_CANCELS_ALL_ORDER_REJECT",
            Self::OneCancelsAllOrderTriggered => "ONE_CANCELS_ALL_ORDER_TRIGGERED",
            Self::OrderFill => "ORDER_FILL",
            Self::OrderCancel => "ORDER_CANCEL",
            Self::OrderCancelReject => "ORDER_CANCEL_REJECT",
            Self::OrderClientExtensionsModify => "ORDER_CLIENT_EXTENSIONS_MODIFY",
            Self::OrderClientExtensionsModifyReject => "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
            Self::TradeClientExtensionsModify => "TRADE_CLIENT_EXTENSIONS_MODIFY",
            Self::TradeClientExtensionsModifyReject => "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
            Self::MarginCallEnter => "MARGIN_CALL_ENTER",
            Self::MarginCallExtend => "MARGIN_CALL_EXTEND",
            Self::MarginCallExit => "MARGIN_CALL_EXIT",
            Self::DelayedTradeClosure => "DELAYED_TRADE_CLOSURE",
            Self::DailyFinancing => "DAILY_FINANCING",
            Self::ResetResettablePl => "RESET_RESETTABLE_PL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for TransactionFilter {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TransactionFilter {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ORDER" => Self::Order,
            "FUNDING" => Self::Funding,
            "ADMIN" => Self::Admin,
            "CREATE" => Self::Create,
            "CLOSE" => Self::Close,
            "REOPEN" => Self::Reopen,
            "CLIENT_CONFIGURE" => Self::ClientConfigure,
            "CLIENT_CONFIGURE_REJECT" => Self::ClientConfigureReject,
            "TRANSFER_FUNDS" => Self::TransferFunds,
            "TRANSFER_FUNDS_REJECT" => Self::TransferFundsReject,
            "MARKET_ORDER" => Self::MarketOrder,
            "MARKET_ORDER_REJECT" => Self::MarketOrderReject,
            "LIMIT_ORDER" => Self::LimitOrder,
            "LIMIT_ORDER_REJECT" => Self::LimitOrderReject,
            "STOP_ORDER" => Self::StopOrder,
            "STOP_ORDER_REJECT" => Self::StopOrderReject,
            "MARKET_IF_TOUCHED_ORDER" => Self::MarketIfTouchedOrder,
            "MARKET_IF_TOUCHED_ORDER_REJECT" => Self::MarketIfTouchedOrderReject,
            "TAKE_PROFIT_ORDER" => Self::TakeProfitOrder,
            "TAKE_PROFIT_ORDER_REJECT" => Self::TakeProfitOrderReject,
            "STOP_LOSS_ORDER" => Self::StopLossOrder,
            "STOP_LOSS_ORDER_REJECT" => Self::StopLossOrderReject,
            "GUARANTEED_STOP_LOSS_ORDER" => Self::GuaranteedStopLossOrder,
            "GUARANTEED_STOP_LOSS_ORDER_REJECT" => Self::GuaranteedStopLossOrderReject,
            "TRAILING_STOP_LOSS_ORDER" => Self::TrailingStopLossOrder,
            "TRAILING_STOP_LOSS_ORDER_REJECT" => Self::TrailingStopLossOrderReject,
            "ONE_CANCELS_ALL_ORDER" => Self::OneCancelsAllOrder,
            "ONE_CANCELS_ALL_ORDER_REJECT" => Self::OneCancelsAllOrderReject,
            "ONE_CANCELS_ALL_ORDER_TRIGGERED" => Self::OneCancelsAllOrderTriggered,
            "ORDER_FILL" => Self::OrderFill,
            "ORDER_CANCEL" => Self::OrderCancel,
            "ORDER_CANCEL_REJECT" => Self::OrderCancelReject,
            "ORDER_CLIENT_EXTENSIONS_MODIFY" => Self::OrderClientExtensionsModify,
            "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT" => Self::OrderClientExtensionsModifyReject,
            "TRADE_CLIENT_EXTENSIONS_MODIFY" => Self::TradeClientExtensionsModify,
            "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT" => Self::TradeClientExtensionsModifyReject,
            "MARGIN_CALL_ENTER" => Self::MarginCallEnter,
            "MARGIN_CALL_EXTEND" => Self::MarginCallExtend,
            "MARGIN_CALL_EXIT" => Self::MarginCallExit,
            "DELAYED_TRADE_CLOSURE" => Self::DelayedTradeClosure,
            "DAILY_FINANCING" => Self::DailyFinancing,
            "RESET_RESETTABLE_PL" => Self::ResetResettablePl,
            _ => Self::Unknown(value),
        })
    }
}

/// A TransactionHeartbeat object is injected into the Transaction stream to ensure that the HTTP connection
/// remains active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHeartbeat {
    /// The string “HEARTBEAT”
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
    /// The date/time when the TransactionHeartbeat was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
}
