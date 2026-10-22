//! OANDA v20 transaction definitions.
use super::provider_enum::provider_enum;
use super::{
    AccountFinancingMode, ClientPrice, HomeConversionFactors, OrderPositionFill,
    OrderTriggerCondition, TimeInForce, TradeState,
};
use crate::ids::{
    AccountID, ClientComment, ClientID, ClientTag, Currency, InstrumentName, OrderID, RequestID,
    TradeID, TransactionID,
};
use crate::timestamp::Timestamp;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A `CreateTransaction` represents the creation of an Account.
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
    /// The Type of the Transaction. Always set to “CREATE” in a `CreateTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_create_transaction_type")]
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

/// A `CloseTransaction` represents the closing of an Account.
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
    /// The Type of the Transaction. Always set to “CLOSE” in a `CloseTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_close_transaction_type")]
    pub r#type: Option<TransactionType>,
}

/// A `ReopenTransaction` represents the re-opening of a closed Account.
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
    /// The Type of the Transaction. Always set to “REOPEN” in a `ReopenTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_reopen_transaction_type")]
    pub r#type: Option<TransactionType>,
}

/// A `ClientConfigureTransaction` represents the configuration of an Account by a client.
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
    /// The Type of the Transaction. Always set to “`CLIENT_CONFIGURE`” in a `ClientConfigureTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_client_configure_transaction_type")]
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub margin_rate: Option<Decimal>,
}

/// A `ClientConfigureRejectTransaction` represents the rejection of configuration of an Account by a client.
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
    /// The Type of the Transaction. Always set to “`CLIENT_CONFIGURE_REJECT`” in a
    /// `ClientConfigureRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_client_configure_reject_transaction_type")]
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub margin_rate: Option<Decimal>,
    /// The reason that the Reject Transaction was created
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A `TransferFundsTransaction` represents the transfer of funds in/out of an Account.
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
    /// The Type of the Transaction. Always set to “`TRANSFER_FUNDS`” in a `TransferFundsTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_transfer_funds_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The amount to deposit/withdraw from the Account in the Account’s home currency. A positive value
    /// indicates a deposit, a negative value indicates a withdrawal.
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub account_balance: Option<Decimal>,
}

/// A `TransferFundsRejectTransaction` represents the rejection of the transfer of funds in/out of an Account.
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
    /// The Type of the Transaction. Always set to “`TRANSFER_FUNDS_REJECT`” in a `TransferFundsRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_transfer_funds_reject_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The amount to deposit/withdraw from the Account in the Account’s home currency. A positive value
    /// indicates a deposit, a negative value indicates a withdrawal.
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// A `MarketOrderTransaction` represents the creation of a Market Order in the user’s account. A Market Order
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
    /// The Type of the Transaction. Always set to “`MARKET_ORDER`” in a `MarketOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_order_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The Market Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The time-in-force requested for the Market Order. Restricted to FOK or IOC for a `MarketOrder`.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the Market Order filled at.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// A `MarketOrderRejectTransaction` represents the rejection of the creation of a Market Order.
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
    /// The Type of the Transaction. Always set to “`MARKET_ORDER_REJECT`” in a `MarketOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_order_reject_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The Market Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    ///
    /// `None` when OANDA wrote no units. A rejected position closeout of a side that is already flat
    /// (`CLOSEOUT_POSITION_DOESNT_EXIST`) carries none: the requested size is only in
    /// `long_position_closeout` or `short_position_closeout`. The website marks this field required;
    /// OANDA's own reject omits it.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub units: Option<Decimal>,
    /// The time-in-force requested for the Market Order. Restricted to FOK or IOC for a `MarketOrder`.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the Market Order filled at.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// A `FixedPriceOrderTransaction` represents the creation of a Fixed Price Order in the user’s account. A
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
    /// The Type of the Transaction. Always set to “`FIXED_PRICE_ORDER`” in a `FixedPriceOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_fixed_price_order_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The Fixed Price Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Fixed Price Order. A positive number of units results in a
    /// long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price specified for the Fixed Price Order. This price is the exact price that the Fixed Price Order
    /// will be filled at.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// The state that the trade resulting from the Fixed Price Order should be set to.
    #[serde(rename = "tradeState")]
    pub trade_state: TradeState,
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

/// A `LimitOrderTransaction` represents the creation of a Limit Order in the user’s Account.
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
    /// The Type of the Transaction. Always set to “`LIMIT_ORDER`” in a `LimitOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_limit_order_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The Limit Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Limit Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Limit Order. The Limit Order will only be filled by a market price
    /// that is equal to or better than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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

/// A `LimitOrderRejectTransaction` represents the rejection of the creation of a Limit Order.
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
    /// The Type of the Transaction. Always set to “`LIMIT_ORDER_REJECT`” in a `LimitOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_limit_order_reject_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The Limit Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Limit Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Limit Order. The Limit Order will only be filled by a market price
    /// that is equal to or better than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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

/// A `StopOrderTransaction` represents the creation of a Stop Order in the user’s Account.
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
    /// The Type of the Transaction. Always set to “`STOP_ORDER`” in a `StopOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_order_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The Stop Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Stop Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Stop Order. The Stop Order will only be filled by a market price
    /// that is equal to or worse than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this Stop Order. If the market gaps and crosses through
    /// both the price and the priceBound, the Stop Order will be cancelled instead of being filled.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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

/// A `StopOrderRejectTransaction` represents the rejection of the creation of a Stop Order.
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
    /// The Type of the Transaction. Always set to “`STOP_ORDER_REJECT`” in a `StopOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_order_reject_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The Stop Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Stop Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Stop Order. The Stop Order will only be filled by a market price
    /// that is equal to or worse than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this Stop Order. If the market gaps and crosses through
    /// both the price and the priceBound, the Stop Order will be cancelled instead of being filled.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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

/// A `MarketIfTouchedOrderTransaction` represents the creation of a `MarketIfTouched` Order in the user’s
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
    /// The Type of the Transaction. Always set to “`MARKET_IF_TOUCHED_ORDER`” in a
    /// `MarketIfTouchedOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_if_touched_order_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The `MarketIfTouched` Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the `MarketIfTouched` Order. A positive number of units results in
    /// a long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the `MarketIfTouched` Order. The `MarketIfTouched` Order will only be
    /// filled by a market price that crosses this price from the direction of the market price at the time when
    /// the Order was created (the initialMarketPrice). Depending on the value of the Order’s price and
    /// initialMarketPrice, the `MarketIfTouchedOrder` will behave like a Limit or a Stop Order.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this `MarketIfTouched` Order.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the `MarketIfTouched` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `MarketIfTouched` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `MarketIfTouched` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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

/// A `MarketIfTouchedOrderRejectTransaction` represents the rejection of the creation of a `MarketIfTouched`
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
    /// The Type of the Transaction. Always set to “`MARKET_IF_TOUCHED_ORDER_REJECT`” in a
    /// `MarketIfTouchedOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_if_touched_order_reject_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The `MarketIfTouched` Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the `MarketIfTouched` Order. A positive number of units results in
    /// a long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the `MarketIfTouched` Order. The `MarketIfTouched` Order will only be
    /// filled by a market price that crosses this price from the direction of the market price at the time when
    /// the Order was created (the initialMarketPrice). Depending on the value of the Order’s price and
    /// initialMarketPrice, the `MarketIfTouchedOrder` will behave like a Limit or a Stop Order.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this `MarketIfTouched` Order.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the `MarketIfTouched` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `MarketIfTouched` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `MarketIfTouched` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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

/// A `TakeProfitOrderTransaction` represents the creation of a `TakeProfit` Order in the user’s Account.
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
    /// The Type of the Transaction. Always set to “`TAKE_PROFIT_ORDER`” in a `TakeProfitOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_take_profit_order_transaction_type")]
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
    /// The price threshold specified for the `TakeProfit` Order. The associated Trade will be closed by a market
    /// price that is equal to or better than this threshold.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The time-in-force requested for the `TakeProfit` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TakeProfit` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `TakeProfit` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// A `TakeProfitOrderRejectTransaction` represents the rejection of the creation of a `TakeProfit` Order.
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
    /// The Type of the Transaction. Always set to “`TAKE_PROFIT_ORDER_REJECT`” in a
    /// `TakeProfitOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_take_profit_order_reject_transaction_type")]
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
    /// The price threshold specified for the `TakeProfit` Order. The associated Trade will be closed by a market
    /// price that is equal to or better than this threshold.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The time-in-force requested for the `TakeProfit` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TakeProfit` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `TakeProfit` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// A `StopLossOrderTransaction` represents the creation of a `StopLoss` Order in the user’s Account.
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
    /// The Type of the Transaction. Always set to “`STOP_LOSS_ORDER`” in a `StopLossOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_loss_order_transaction_type")]
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
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Stop Loss Order
    /// price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `StopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for `StopLoss`
    /// Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the
    /// `GuaranteedStopLossOrderMode` of the account, if it is REQUIRED, the default will be true, for DISABLED or
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// A `StopLossOrderRejectTransaction` represents the rejection of the creation of a `StopLoss` Order.
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
    /// The Type of the Transaction. Always set to “`STOP_LOSS_ORDER_REJECT`” in a `StopLossOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_loss_order_reject_transaction_type")]
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
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Stop Loss Order
    /// price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `StopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for `StopLoss`
    /// Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the
    /// `GuaranteedStopLossOrderMode` of the account, if it is REQUIRED, the default will be true, for DISABLED or
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// A `GuaranteedStopLossOrderTransaction` represents the creation of a `GuaranteedStopLoss` Order in the user’s
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
    /// The Type of the Transaction. Always set to “`GUARANTEED_STOP_LOSS_ORDER`” in a
    /// `GuaranteedStopLossOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_guaranteed_stop_loss_order_transaction_type")]
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
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Guaranteed Stop
    /// Loss Order price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask
    /// is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `GuaranteedStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `GuaranteedStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `GuaranteedStopLoss` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// A `GuaranteedStopLossOrderRejectTransaction` represents the rejection of the creation of a
/// `GuaranteedStopLoss` Order.
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
    /// The Type of the Transaction. Always set to “`GUARANTEED_STOP_LOSS_ORDER_REJECT`” in a
    /// `GuaranteedStopLossOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_guaranteed_stop_loss_order_reject_transaction_type")]
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
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Guaranteed Stop
    /// Loss Order price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask
    /// is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `GuaranteedStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `GuaranteedStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `GuaranteedStopLoss` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// A `TrailingStopLossOrderTransaction` represents the creation of a `TrailingStopLoss` Order in the user’s
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
    /// The Type of the Transaction. Always set to “`TRAILING_STOP_LOSS_ORDER`” in a
    /// `TrailingStopLossOrderTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_trailing_stop_loss_order_transaction_type")]
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
    /// The price distance (in price units) specified for the `TrailingStopLoss` Order.
    #[serde(rename = "distance")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub distance: Decimal,
    /// The time-in-force requested for the `TrailingStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TrailingStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// A `TrailingStopLossOrderRejectTransaction` represents the rejection of the creation of a `TrailingStopLoss`
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
    /// The Type of the Transaction. Always set to “`TRAILING_STOP_LOSS_ORDER_REJECT`” in a
    /// `TrailingStopLossOrderRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_trailing_stop_loss_order_reject_transaction_type")]
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
    /// The price distance (in price units) specified for the `TrailingStopLoss` Order.
    #[serde(rename = "distance")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub distance: Decimal,
    /// The time-in-force requested for the `TrailingStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TrailingStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
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
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
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
    /// The ID of the `OrderFill` Transaction that caused this Order to be created (only provided if this Order
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

/// An `OrderFillTransaction` represents the filling of an Order in the client’s Account.
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
    /// The Type of the Transaction. Always set to “`ORDER_FILL`” for an `OrderFillTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_order_fill_transaction_type")]
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
    /// The number of units filled by the `OrderFill`.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub units: Option<Decimal>,
    /// The number of units the filled Order requested, set against `units`, the number that filled.
    ///
    /// Not documented by OANDA. Observed on a Practice `close_position` closeout fill on
    /// 2026-10-16; `None` when OANDA does not send it.
    #[serde(
        rename = "requestedUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub requested_units: Option<Decimal>,
    /// This is the conversion factor in effect for the Account at the time of the `OrderFill` for converting any
    /// gains realized in Instrument quote units into units of the Account’s home currency. Deprecated: Will be
    /// removed in a future API update.
    #[serde(
        rename = "gainQuoteHomeConversionFactor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub gain_quote_home_conversion_factor: Option<Decimal>,
    /// This is the conversion factor in effect for the Account at the time of the `OrderFill` for converting any
    /// losses realized in Instrument quote units into units of the Account’s home currency. Deprecated: Will be
    /// removed in a future API update.
    #[serde(
        rename = "lossQuoteHomeConversionFactor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub loss_quote_home_conversion_factor: Option<Decimal>,
    /// The `HomeConversionFactors` in effect at the time of the `OrderFill`.
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price: Option<Decimal>,
    /// The price that all of the units of the `OrderFill` should have been filled at, in the absence of
    /// guaranteed price execution. This factors in the Account’s current `ClientPrice`, used liquidity and the
    /// units of the `OrderFill` only. If no Trades were closed with their price clamped for guaranteed stop loss
    /// enforcement, then this value will match the price fields of each Trade opened, closed, and reduced, and
    /// they will all be the exact same.
    #[serde(rename = "fullVWAP", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub full_vwap: Option<Decimal>,
    /// The price in effect for the account at the time of the Order fill.
    #[serde(rename = "fullPrice", default, skip_serializing_if = "Option::is_none")]
    pub full_price: Option<ClientPrice>,
    /// The reason that an Order was filled
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<OrderFillReason>,
    /// The profit or loss incurred when the Order was filled.
    #[serde(rename = "pl", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub pl: Option<Decimal>,
    /// The profit or loss incurred when the Order was filled, in the Instrument’s quote currency.
    #[serde(rename = "quotePL", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_pl: Option<Decimal>,
    /// The financing paid or collected when the Order was filled.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub financing: Option<Decimal>,
    /// The financing paid or collected when the Order was filled, in the Instrument’s base currency.
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub base_financing: Option<Decimal>,
    /// The financing paid or collected when the Order was filled, in the Instrument’s quote currency.
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_financing: Option<Decimal>,
    /// The commission charged in the Account’s home currency as a result of filling the Order. The commission
    /// is always represented as a positive quantity of the Account’s home currency, however it reduces the
    /// balance in the Account.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub commission: Option<Decimal>,
    /// The total guaranteed execution fees charged for all Trades opened, closed or reduced with guaranteed
    /// Stop Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_fee: Option<Decimal>,
    /// The total guaranteed execution fees charged for all Trades opened, closed or reduced with guaranteed
    /// Stop Loss Orders, expressed in the Instrument’s quote currency.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_guaranteed_execution_fee: Option<Decimal>,
    /// The Account’s balance after the Order was filled.
    #[serde(
        rename = "accountBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    /// The half spread cost for the `OrderFill`, which is the sum of the halfSpreadCost values in the
    /// tradeOpened, tradesClosed and tradeReduced fields. This can be a positive or negative value and is
    /// represented in the home currency of the Account.
    #[serde(
        rename = "halfSpreadCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub half_spread_cost: Option<Decimal>,
    /// The cost of converting the fill's `pl` into the Account's home currency, as OANDA names it.
    ///
    /// Not documented by OANDA. Observed on a Practice `close_position` closeout fill on
    /// 2026-10-16, and on each of its `tradesClosed` entries. Its sign and its relation to `pl`
    /// and `accountBalance` have not been recorded. `None` when OANDA does not send it.
    #[serde(
        rename = "plHomeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub pl_home_conversion_cost: Option<Decimal>,
    /// The cost of converting the fill's `baseFinancing` into the Account's home currency, as OANDA
    /// names it.
    ///
    /// Not documented by OANDA; observed as for `pl_home_conversion_cost`.
    #[serde(
        rename = "baseFinancingHomeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub base_financing_home_conversion_cost: Option<Decimal>,
    /// The cost of converting the fill's guaranteed execution fee into the Account's home currency,
    /// as OANDA names it.
    ///
    /// Not documented by OANDA; observed as for `pl_home_conversion_cost`.
    #[serde(
        rename = "guaranteedExecutionFeeHomeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_fee_home_conversion_cost: Option<Decimal>,
    /// The fill's home-currency conversion cost as a whole, as OANDA names it.
    ///
    /// Not documented by OANDA; observed as for `pl_home_conversion_cost`. Whether it is the sum
    /// of the three component costs has not been recorded.
    #[serde(
        rename = "homeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub home_conversion_cost: Option<Decimal>,
}

/// An `OrderCancelTransaction` represents the cancellation of an Order in the client’s Account.
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
    /// The Type of the Transaction. Always set to “`ORDER_CANCEL`” for an `OrderCancelTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_order_cancel_transaction_type")]
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

/// An `OrderCancelRejectTransaction` represents the rejection of the cancellation of an Order in the client’s
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
    /// The Type of the Transaction. Always set to “`ORDER_CANCEL_REJECT`” for an `OrderCancelRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_order_cancel_reject_transaction_type")]
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

/// A `OrderClientExtensionsModifyTransaction` represents the modification of an Order’s Client Extensions.
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
    /// The Type of the Transaction. Always set to “`ORDER_CLIENT_EXTENSIONS_MODIFY`” for a
    /// `OrderClientExtensionsModifyTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_order_client_extensions_modify_transaction_type")]
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

/// A `OrderClientExtensionsModifyRejectTransaction` represents the rejection of the modification of an
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
    /// The Type of the Transaction. Always set to “`ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT`” for a
    /// `OrderClientExtensionsModifyRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_order_client_extensions_modify_reject_transaction_type")]
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

/// A `TradeClientExtensionsModifyTransaction` represents the modification of a Trade’s Client Extensions.
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
    /// The Type of the Transaction. Always set to “`TRADE_CLIENT_EXTENSIONS_MODIFY`” for a
    /// `TradeClientExtensionsModifyTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_trade_client_extensions_modify_transaction_type")]
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

/// A `TradeClientExtensionsModifyRejectTransaction` represents the rejection of the modification of a Trade’s
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
    /// The Type of the Transaction. Always set to “`TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT`” for a
    /// `TradeClientExtensionsModifyRejectTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_trade_client_extensions_modify_reject_transaction_type")]
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

/// A `MarginCallEnterTransaction` is created when an Account enters the margin call state.
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
    /// The Type of the Transaction. Always set to “`MARGIN_CALL_ENTER`” for an `MarginCallEnterTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_margin_call_enter_transaction_type")]
    pub r#type: Option<TransactionType>,
}

/// A `MarginCallExtendTransaction` is created when the margin call state for an Account has been extended.
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
    /// The Type of the Transaction. Always set to “`MARGIN_CALL_EXTEND`” for an `MarginCallExtendTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_margin_call_extend_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The number of the extensions to the Account’s current margin call that have been applied. This value
    /// will be set to 1 for the first `MarginCallExtend` Transaction
    #[serde(
        rename = "extensionNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_number: Option<i64>,
}

/// A `MarginCallExitTransaction` is created when an Account leaves the margin call state.
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
    /// The Type of the Transaction. Always set to “`MARGIN_CALL_EXIT`” for an `MarginCallExitTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_margin_call_exit_transaction_type")]
    pub r#type: Option<TransactionType>,
}

/// A `DelayedTradeClosure` Transaction is created administratively to indicate open trades that should have
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
    /// The Type of the Transaction. Always set to “`DELAYED_TRADE_CLOSURE`” for an `DelayedTradeClosureTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_delayed_trade_closure_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The reason for the delayed trade closure
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderReason>,
    /// List of Trade ID’s identifying the open trades that will be closed when their respective instruments
    /// become tradeable
    #[serde(rename = "tradeIDs", default, skip_serializing_if = "Option::is_none")]
    pub trade_ids: Option<TradeID>,
}

/// A `DailyFinancingTransaction` represents the daily payment/collection of financing for an Account.
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
    /// The Type of the Transaction. Always set to “`DAILY_FINANCING`” for a `DailyFinancingTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_daily_financing_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The amount of financing paid/collected for the Account.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub financing: Option<Decimal>,
    /// The Account’s balance after daily financing.
    #[serde(
        rename = "accountBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// A `DividendAdjustment` Transaction is created administratively to pay or collect dividend adjustment
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
    /// The Type of the Transaction. Always set to “`DIVIDEND_ADJUSTMENT`” for a `DividendAdjustmentTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_dividend_adjustment_transaction_type")]
    pub r#type: Option<TransactionType>,
    /// The name of the instrument for the dividendAdjustment transaction
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The total dividend adjustment amount paid or collected in the Account’s home currency for the Account as
    /// a result of applying the `DividendAdjustment` Transaction. This is the sum of the dividend adjustments
    /// paid/collected for each `OpenTradeDividendAdjustment` found within the Transaction.
    #[serde(
        rename = "dividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub dividend_adjustment: Option<Decimal>,
    /// The total dividend adjustment amount paid or collected in the Instrument’s quote currency for the
    /// Account as a result of applying the `DividendAdjustment` Transaction. This is the sum of the quote
    /// dividend adjustments paid/collected for each `OpenTradeDividendAdjustment` found within the Transaction.
    #[serde(
        rename = "quoteDividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_dividend_adjustment: Option<Decimal>,
    /// The `HomeConversionFactors` in effect at the time of the `DividendAdjustment`.
    #[serde(
        rename = "homeConversionFactors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversion_factors: Option<HomeConversionFactors>,
    /// The Account balance after applying the `DividendAdjustment` Transaction
    #[serde(
        rename = "accountBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// A `ResetResettablePLTransaction` represents the resetting of the Account’s resettable PL counters.
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
    /// The Type of the Transaction. Always set to “`RESET_RESETTABLE_PL`” for a `ResetResettablePLTransaction`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_reset_resettable_p_l_transaction_type")]
    pub r#type: Option<TransactionType>,
}

provider_enum! {
    /// The possible types of a Transaction
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum TransactionType {
        /// Account Create Transaction
        Create => "CREATE",
        /// Account Close Transaction
        Close => "CLOSE",
        /// Account Reopen Transaction
        Reopen => "REOPEN",
        /// Client Configuration Transaction
        ClientConfigure => "CLIENT_CONFIGURE",
        /// Client Configuration Reject Transaction
        ClientConfigureReject => "CLIENT_CONFIGURE_REJECT",
        /// Transfer Funds Transaction
        TransferFunds => "TRANSFER_FUNDS",
        /// Transfer Funds Reject Transaction
        TransferFundsReject => "TRANSFER_FUNDS_REJECT",
        /// Market Order Transaction
        MarketOrder => "MARKET_ORDER",
        /// Market Order Reject Transaction
        MarketOrderReject => "MARKET_ORDER_REJECT",
        /// Fixed Price Order Transaction
        FixedPriceOrder => "FIXED_PRICE_ORDER",
        /// Limit Order Transaction
        LimitOrder => "LIMIT_ORDER",
        /// Limit Order Reject Transaction
        LimitOrderReject => "LIMIT_ORDER_REJECT",
        /// Stop Order Transaction
        StopOrder => "STOP_ORDER",
        /// Stop Order Reject Transaction
        StopOrderReject => "STOP_ORDER_REJECT",
        /// Market if Touched Order Transaction
        MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
        /// Market if Touched Order Reject Transaction
        MarketIfTouchedOrderReject => "MARKET_IF_TOUCHED_ORDER_REJECT",
        /// Take Profit Order Transaction
        TakeProfitOrder => "TAKE_PROFIT_ORDER",
        /// Take Profit Order Reject Transaction
        TakeProfitOrderReject => "TAKE_PROFIT_ORDER_REJECT",
        /// Stop Loss Order Transaction
        StopLossOrder => "STOP_LOSS_ORDER",
        /// Stop Loss Order Reject Transaction
        StopLossOrderReject => "STOP_LOSS_ORDER_REJECT",
        /// Guaranteed Stop Loss Order Transaction
        GuaranteedStopLossOrder => "GUARANTEED_STOP_LOSS_ORDER",
        /// Guaranteed Stop Loss Order Reject Transaction
        GuaranteedStopLossOrderReject => "GUARANTEED_STOP_LOSS_ORDER_REJECT",
        /// Trailing Stop Loss Order Transaction
        TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
        /// Trailing Stop Loss Order Reject Transaction
        TrailingStopLossOrderReject => "TRAILING_STOP_LOSS_ORDER_REJECT",
        /// Order Fill Transaction
        OrderFill => "ORDER_FILL",
        /// Order Cancel Transaction
        OrderCancel => "ORDER_CANCEL",
        /// Order Cancel Reject Transaction
        OrderCancelReject => "ORDER_CANCEL_REJECT",
        /// Order Client Extensions Modify Transaction
        OrderClientExtensionsModify => "ORDER_CLIENT_EXTENSIONS_MODIFY",
        /// Order Client Extensions Modify Reject Transaction
        OrderClientExtensionsModifyReject => "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
        /// Trade Client Extensions Modify Transaction
        TradeClientExtensionsModify => "TRADE_CLIENT_EXTENSIONS_MODIFY",
        /// Trade Client Extensions Modify Reject Transaction
        TradeClientExtensionsModifyReject => "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
        /// Margin Call Enter Transaction
        MarginCallEnter => "MARGIN_CALL_ENTER",
        /// Margin Call Extend Transaction
        MarginCallExtend => "MARGIN_CALL_EXTEND",
        /// Margin Call Exit Transaction
        MarginCallExit => "MARGIN_CALL_EXIT",
        /// Delayed Trade Closure Transaction
        DelayedTradeClosure => "DELAYED_TRADE_CLOSURE",
        /// Daily Financing Transaction
        DailyFinancing => "DAILY_FINANCING",
        /// Dividend Adjustment Transaction
        DividendAdjustment => "DIVIDEND_ADJUSTMENT",
        /// Reset Resettable PL Transaction
        ResetResettablePl => "RESET_RESETTABLE_PL",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that an Account is being funded.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum FundingReason {
        /// The client has initiated a funds transfer
        ClientFunding => "CLIENT_FUNDING",
        /// Funds are being transferred between two Accounts.
        AccountTransfer => "ACCOUNT_TRANSFER",
        /// Funds are being transferred as part of a Division migration
        DivisionMigration => "DIVISION_MIGRATION",
        /// Funds are being transferred as part of a Site migration
        SiteMigration => "SITE_MIGRATION",
        /// Funds are being transferred as part of an Account adjustment
        Adjustment => "ADJUSTMENT",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Market Order was created
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum MarketOrderReason {
        /// The Market Order was created at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Market Order was created to close a Trade at the request of a client
        TradeClose => "TRADE_CLOSE",
        /// The Market Order was created to close a Position at the request of a client
        PositionCloseout => "POSITION_CLOSEOUT",
        /// The Market Order was created as part of a Margin Closeout
        MarginCloseout => "MARGIN_CLOSEOUT",
        /// The Market Order was created to close a trade marked for delayed closure
        DelayedTradeClose => "DELAYED_TRADE_CLOSE",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Fixed Price Order was created
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum FixedPriceOrderReason {
        /// The Fixed Price Order was created as part of a platform account migration
        PlatformAccountMigration => "PLATFORM_ACCOUNT_MIGRATION",
        /// The Fixed Price Order was created to close a Trade as part of division account migration
        TradeCloseDivisionAccountMigration => "TRADE_CLOSE_DIVISION_ACCOUNT_MIGRATION",
        /// The Fixed Price Order was created to close a Trade administratively
        TradeCloseAdministrativeAction => "TRADE_CLOSE_ADMINISTRATIVE_ACTION",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Limit Order was initiated
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum LimitOrderReason {
        /// The Limit Order was initiated at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Limit Order was initiated as a replacement for an existing Order
        Replacement => "REPLACEMENT",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Stop Order was initiated
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum StopOrderReason {
        /// The Stop Order was initiated at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Stop Order was initiated as a replacement for an existing Order
        Replacement => "REPLACEMENT",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Market-if-touched Order was initiated
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum MarketIfTouchedOrderReason {
        /// The Market-if-touched Order was initiated at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Market-if-touched Order was initiated as a replacement for an existing Order
        Replacement => "REPLACEMENT",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Take Profit Order was initiated
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum TakeProfitOrderReason {
        /// The Take Profit Order was initiated at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Take Profit Order was initiated as a replacement for an existing Order
        Replacement => "REPLACEMENT",
        /// The Take Profit Order was initiated automatically when an Order was filled that opened a new Trade
        /// requiring a Take Profit Order.
        OnFill => "ON_FILL",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Stop Loss Order was initiated
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum StopLossOrderReason {
        /// The Stop Loss Order was initiated at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Stop Loss Order was initiated as a replacement for an existing Order
        Replacement => "REPLACEMENT",
        /// The Stop Loss Order was initiated automatically when an Order was filled that opened a new Trade
        /// requiring a Stop Loss Order.
        OnFill => "ON_FILL",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Guaranteed Stop Loss Order was initiated
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum GuaranteedStopLossOrderReason {
        /// The Guaranteed Stop Loss Order was initiated at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Guaranteed Stop Loss Order was initiated as a replacement for an existing Order
        Replacement => "REPLACEMENT",
        /// The Guaranteed Stop Loss Order was initiated automatically when an Order was filled that opened a new
        /// Trade requiring a Guaranteed Stop Loss Order.
        OnFill => "ON_FILL",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that the Trailing Stop Loss Order was initiated
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum TrailingStopLossOrderReason {
        /// The Trailing Stop Loss Order was initiated at the request of a client
        ClientOrder => "CLIENT_ORDER",
        /// The Trailing Stop Loss Order was initiated as a replacement for an existing Order
        Replacement => "REPLACEMENT",
        /// The Trailing Stop Loss Order was initiated automatically when an Order was filled that opened a new
        /// Trade requiring a Trailing Stop Loss Order.
        OnFill => "ON_FILL",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that an Order was filled
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum OrderFillReason {
        /// The Order filled was a Limit Order
        LimitOrder => "LIMIT_ORDER",
        /// The Order filled was a Stop Order
        StopOrder => "STOP_ORDER",
        /// The Order filled was a Market-if-touched Order
        MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
        /// The Order filled was a Take Profit Order
        TakeProfitOrder => "TAKE_PROFIT_ORDER",
        /// The Order filled was a Stop Loss Order
        StopLossOrder => "STOP_LOSS_ORDER",
        /// The Order filled was a Guaranteed Stop Loss Order
        GuaranteedStopLossOrder => "GUARANTEED_STOP_LOSS_ORDER",
        /// The Order filled was a Trailing Stop Loss Order
        TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
        /// The Order filled was a Market Order
        MarketOrder => "MARKET_ORDER",
        /// The Order filled was a Market Order used to explicitly close a Trade
        MarketOrderTradeClose => "MARKET_ORDER_TRADE_CLOSE",
        /// The Order filled was a Market Order used to explicitly close a Position
        MarketOrderPositionCloseout => "MARKET_ORDER_POSITION_CLOSEOUT",
        /// The Order filled was a Market Order used for a Margin Closeout
        MarketOrderMarginCloseout => "MARKET_ORDER_MARGIN_CLOSEOUT",
        /// The Order filled was a Market Order used for a delayed Trade close
        MarketOrderDelayedTradeClose => "MARKET_ORDER_DELAYED_TRADE_CLOSE",
        /// The Order filled was a Fixed Price Order
        FixedPriceOrder => "FIXED_PRICE_ORDER",
        /// The Order filled was a Fixed Price Order created as part of a platform account migration
        FixedPriceOrderPlatformAccountMigration => "FIXED_PRICE_ORDER_PLATFORM_ACCOUNT_MIGRATION",
        /// The Order filled was a Fixed Price Order created to close a Trade as part of division account migration
        FixedPriceOrderDivisionAccountMigration => "FIXED_PRICE_ORDER_DIVISION_ACCOUNT_MIGRATION",
        /// The Order filled was a Fixed Price Order created to close a Trade administratively
        FixedPriceOrderAdministrativeAction => "FIXED_PRICE_ORDER_ADMINISTRATIVE_ACTION",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The reason that an Order was cancelled.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum OrderCancelReason {
        /// The Order was cancelled because at the time of filling, an unexpected internal server error occurred.
        InternalServerError => "INTERNAL_SERVER_ERROR",
        /// The Order was cancelled because at the time of filling the account was locked.
        AccountLocked => "ACCOUNT_LOCKED",
        /// The order was to be filled, however the account is configured to not allow new positions to be created.
        AccountNewPositionsLocked => "ACCOUNT_NEW_POSITIONS_LOCKED",
        /// Filling the Order wasn’t possible because it required the creation of a dependent Order and the Account
        /// is locked for Order creation.
        AccountOrderCreationLocked => "ACCOUNT_ORDER_CREATION_LOCKED",
        /// Filling the Order was not possible because the Account is locked for filling Orders.
        AccountOrderFillLocked => "ACCOUNT_ORDER_FILL_LOCKED",
        /// The Order was cancelled explicitly at the request of the client.
        ClientRequest => "CLIENT_REQUEST",
        /// The Order cancelled because it is being migrated to another account.
        Migration => "MIGRATION",
        /// Filling the Order wasn’t possible because the Order’s instrument was halted.
        MarketHalted => "MARKET_HALTED",
        /// The Order is linked to an open Trade that was closed.
        LinkedTradeClosed => "LINKED_TRADE_CLOSED",
        /// The time in force specified for this order has passed.
        TimeInForceExpired => "TIME_IN_FORCE_EXPIRED",
        /// Filling the Order wasn’t possible because the Account had insufficient margin.
        InsufficientMargin => "INSUFFICIENT_MARGIN",
        /// Filling the Order would have resulted in a a FIFO violation.
        FifoViolation => "FIFO_VIOLATION",
        /// Filling the Order would have violated the Order’s price bound.
        BoundsViolation => "BOUNDS_VIOLATION",
        /// The Order was cancelled for replacement at the request of the client.
        ClientRequestReplaced => "CLIENT_REQUEST_REPLACED",
        /// The Order was cancelled for replacement with an adjusted fillPrice to accommodate for the price movement
        /// caused by a dividendAdjustment.
        DividendAdjustmentReplaced => "DIVIDEND_ADJUSTMENT_REPLACED",
        /// Filling the Order wasn’t possible because enough liquidity available.
        InsufficientLiquidity => "INSUFFICIENT_LIQUIDITY",
        /// Filling the Order would have resulted in the creation of a Take Profit Order with a GTD time in the past.
        TakeProfitOnFillGtdTimestampInPast => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// Filling the Order would result in the creation of a Take Profit Order that would have been filled
        /// immediately, closing the new Trade at a loss.
        TakeProfitOnFillLoss => "TAKE_PROFIT_ON_FILL_LOSS",
        /// Filling the Order would result in the creation of a Take Profit Loss Order that would close the new
        /// Trade at a loss when filled.
        LosingTakeProfit => "LOSING_TAKE_PROFIT",
        /// Filling the Order would have resulted in the creation of a Stop Loss Order with a GTD time in the past.
        StopLossOnFillGtdTimestampInPast => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// Filling the Order would result in the creation of a Stop Loss Order that would have been filled
        /// immediately, closing the new Trade at a loss.
        StopLossOnFillLoss => "STOP_LOSS_ON_FILL_LOSS",
        /// Filling the Order would result in the creation of a Stop Loss Order whose price would be zero or
        /// negative due to the specified distance.
        StopLossOnFillPriceDistanceMaximumExceeded => "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        /// Filling the Order would not result in the creation of Stop Loss Order, however the Account’s
        /// configuration requires that all Trades have a Stop Loss Order attached to them.
        StopLossOnFillRequired => "STOP_LOSS_ON_FILL_REQUIRED",
        /// Filling the Order would not result in the creation of a guaranteed Stop Loss Order, however the
        /// Account’s configuration requires that all Trades have a guaranteed Stop Loss Order attached to them.
        StopLossOnFillGuaranteedRequired => "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
        /// Filling the Order would result in the creation of a guaranteed Stop Loss Order, however the Account’s
        /// configuration does not allow guaranteed Stop Loss Orders.
        StopLossOnFillGuaranteedNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
        /// Filling the Order would result in the creation of a guaranteed Stop Loss Order with a distance smaller
        /// than the configured minimum distance.
        StopLossOnFillGuaranteedMinimumDistanceNotMet => "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        /// Filling the Order would result in the creation of a guaranteed Stop Loss Order with trigger price and
        /// number of units that that violates the account’s guaranteed Stop Loss Order level restriction.
        StopLossOnFillGuaranteedLevelRestrictionExceeded => "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        /// Filling the Order would result in the creation of a guaranteed Stop Loss Order for a hedged Trade,
        /// however the Account’s configuration does not allow guaranteed Stop Loss Orders for hedged
        /// Trades/Positions.
        StopLossOnFillGuaranteedHedgingNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_HEDGING_NOT_ALLOWED",
        /// Filling the Order would result in the creation of a Stop Loss Order whose `TimeInForce` value is invalid.
        /// A likely cause would be if the Account requires guaranteed stop loss orders and the `TimeInForce` value
        /// were not GTC.
        StopLossOnFillTimeInForceInvalid => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        /// Filling the Order would result in the creation of a Stop Loss Order whose `TriggerCondition` value is
        /// invalid. A likely cause would be if the stop loss order is guaranteed and the `TimeInForce` is not
        /// `TRIGGER_DEFAULT` or `TRIGGER_BID` for a long trade, or not `TRIGGER_DEFAULT` or `TRIGGER_ASK` for a short trade.
        StopLossOnFillTriggerConditionInvalid => "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        /// Filling the Order would have resulted in the creation of a Guaranteed Stop Loss Order with a GTD time in
        /// the past.
        GuaranteedStopLossOnFillGtdTimestampInPast => "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order that would have been
        /// filled immediately, closing the new Trade at a loss.
        GuaranteedStopLossOnFillLoss => "GUARANTEED_STOP_LOSS_ON_FILL_LOSS",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order whose price would be zero
        /// or negative due to the specified distance.
        GuaranteedStopLossOnFillPriceDistanceMaximumExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        /// Filling the Order would not result in the creation of a Guaranteed Stop Loss Order, however the
        /// Account’s configuration requires that all Trades have a Guaranteed Stop Loss Order attached to them.
        GuaranteedStopLossOnFillRequired => "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order, however the Account’s
        /// configuration does not allow Guaranteed Stop Loss Orders.
        GuaranteedStopLossOnFillNotAllowed => "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with a distance smaller
        /// than the configured minimum distance.
        GuaranteedStopLossOnFillMinimumDistanceNotMet => "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger number of
        /// units that violates the account’s Guaranteed Stop Loss Order level restriction volume.
        GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger price that
        /// violates the account’s Guaranteed Stop Loss Order level restriction price range.
        GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order for a hedged Trade,
        /// however the Account’s configuration does not allow Guaranteed Stop Loss Orders for hedged
        /// Trades/Positions.
        GuaranteedStopLossOnFillHedgingNotAllowed => "GUARANTEED_STOP_LOSS_ON_FILL_HEDGING_NOT_ALLOWED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order whose `TimeInForce` value
        /// is invalid. A likely cause would be if the Account requires guaranteed stop loss orders and the
        /// `TimeInForce` value were not GTC.
        GuaranteedStopLossOnFillTimeInForceInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order whose `TriggerCondition`
        /// value is invalid. A likely cause would be the `TimeInForce` is not `TRIGGER_DEFAULT` or `TRIGGER_BID` for a
        /// long trade, or not `TRIGGER_DEFAULT` or `TRIGGER_ASK` for a short trade.
        GuaranteedStopLossOnFillTriggerConditionInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        /// Filling the Order would result in the creation of a Take Profit Order whose price would be zero or
        /// negative due to the specified distance.
        TakeProfitOnFillPriceDistanceMaximumExceeded => "TAKE_PROFIT_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        /// Filling the Order would have resulted in the creation of a Trailing Stop Loss Order with a GTD time in
        /// the past.
        TrailingStopLossOnFillGtdTimestampInPast => "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// Filling the Order would result in the creation of a new Open Trade with a client Trade ID already in use.
        ClientTradeIdAlreadyExists => "CLIENT_TRADE_ID_ALREADY_EXISTS",
        /// Closing out a position wasn’t fully possible.
        PositionCloseoutFailed => "POSITION_CLOSEOUT_FAILED",
        /// Filling the Order would cause the maximum open trades allowed for the Account to be exceeded.
        OpenTradesAllowedExceeded => "OPEN_TRADES_ALLOWED_EXCEEDED",
        /// Filling the Order would have resulted in exceeding the number of pending Orders allowed for the Account.
        PendingOrdersAllowedExceeded => "PENDING_ORDERS_ALLOWED_EXCEEDED",
        /// Filling the Order would have resulted in the creation of a Take Profit Order with a client Order ID that
        /// is already in use.
        TakeProfitOnFillClientOrderIdAlreadyExists => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        /// Filling the Order would have resulted in the creation of a Stop Loss Order with a client Order ID that
        /// is already in use.
        StopLossOnFillClientOrderIdAlreadyExists => "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        /// Filling the Order would have resulted in the creation of a Guaranteed Stop Loss Order with a client
        /// Order ID that is already in use.
        GuaranteedStopLossOnFillClientOrderIdAlreadyExists => "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        /// Filling the Order would have resulted in the creation of a Trailing Stop Loss Order with a client Order
        /// ID that is already in use.
        TrailingStopLossOnFillClientOrderIdAlreadyExists => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        /// Filling the Order would have resulted in the Account’s maximum position size limit being exceeded for
        /// the Order’s instrument.
        PositionSizeExceeded => "POSITION_SIZE_EXCEEDED",
        /// Filling the Order would result in the creation of a Trade, however there already exists an opposing
        /// (hedged) Trade that has a guaranteed Stop Loss Order attached to it. Guaranteed Stop Loss Orders cannot
        /// be combined with hedged positions.
        HedgingGsloViolation => "HEDGING_GSLO_VIOLATION",
        /// Filling the order would cause the maximum position value allowed for the account to be exceeded. The
        /// Order has been cancelled as a result.
        AccountPositionValueLimitExceeded => "ACCOUNT_POSITION_VALUE_LIMIT_EXCEEDED",
        /// Filling the order would require the creation of a short trade, however the instrument is configured such
        /// that orders being filled using bid prices can only reduce existing positions. New short positions cannot
        /// be created, but existing long positions may be reduced or closed.
        InstrumentBidReduceOnly => "INSTRUMENT_BID_REDUCE_ONLY",
        /// Filling the order would require the creation of a long trade, however the instrument is configured such
        /// that orders being filled using ask prices can only reduce existing positions. New long positions cannot
        /// be created, but existing short positions may be reduced or closed.
        InstrumentAskReduceOnly => "INSTRUMENT_ASK_REDUCE_ONLY",
        /// Filling the order would require using the bid, however the instrument is configured such that the bids
        /// are halted, and so no short orders may be filled.
        InstrumentBidHalted => "INSTRUMENT_BID_HALTED",
        /// Filling the order would require using the ask, however the instrument is configured such that the asks
        /// are halted, and so no long orders may be filled.
        InstrumentAskHalted => "INSTRUMENT_ASK_HALTED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
        /// is long the GSLO would be short, however the bid side is currently halted. GSLOs cannot be created in
        /// this situation.
        StopLossOnFillGuaranteedBidHalted => "STOP_LOSS_ON_FILL_GUARANTEED_BID_HALTED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
        /// is short the GSLO would be long, however the ask side is currently halted. GSLOs cannot be created in
        /// this situation.
        StopLossOnFillGuaranteedAskHalted => "STOP_LOSS_ON_FILL_GUARANTEED_ASK_HALTED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
        /// is long the GSLO would be short, however the bid side is currently halted. GSLOs cannot be created in
        /// this situation.
        GuaranteedStopLossOnFillBidHalted => "GUARANTEED_STOP_LOSS_ON_FILL_BID_HALTED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order (GSLO). Since the trade
        /// is short the GSLO would be long, however the ask side is currently halted. GSLOs cannot be created in
        /// this situation.
        GuaranteedStopLossOnFillAskHalted => "GUARANTEED_STOP_LOSS_ON_FILL_ASK_HALTED",
        /// Filling the Order would have resulted in a new Trade that violates the FIFO violation safeguard
        /// constraints.
        FifoViolationSafeguardViolation => "FIFO_VIOLATION_SAFEGUARD_VIOLATION",
        /// Filling the Order would have reduced an existing Trade such that the reduced Trade violates the FIFO
        /// violation safeguard constraints.
        FifoViolationSafeguardPartialCloseViolation => "FIFO_VIOLATION_SAFEGUARD_PARTIAL_CLOSE_VIOLATION",
        /// The Orders on fill would be in violation of the risk management Order mutual exclusivity configuration
        /// specifying that only one risk management Order can be attached to a Trade.
        OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation => "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub dividend_adjustment: Option<Decimal>,
    /// The dividend adjustment amount to pay or collect for the Trade, in the Instrument’s quote currency.
    #[serde(
        rename = "quoteDividendAdjustment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_dividend_adjustment: Option<Decimal>,
}

/// A `ClientExtensions` object allows a client to attach a clientID, tag and comment to Orders and Trades in
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

/// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
/// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
/// dependent Take Profit Order is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeProfitDetails {
    /// The price that the Take Profit Order will be triggered at. Only one of the price and distance fields may
    /// be specified.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
/// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
/// Stop Loss Order is modified directly through the Trade.
///
/// Both trigger fields remain visible to preserve received evidence. Outbound
/// endpoint admission rejects simultaneous `price` and `distance`; omitting both
/// retains the existing trigger during a dependent-order modification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossDetails {
    /// The price that the Stop Loss Order will be triggered at. Only one of the price and distance fields may
    /// be specified.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price: Option<Decimal>,
    /// Specifies the distance (in price units) from the Trade’s open price to use as the Stop Loss Order price.
    /// Only one of the distance and price fields may be specified.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    /// `GuaranteedStopLossOrderMode` of the account, if it is REQUIRED, the default will be true, for DISABLED or
    /// ENABLED the default is false. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "guaranteed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed: Option<bool>,
}

/// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
/// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
/// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
///
/// Outbound admission rejects simultaneous `price` and `distance`. Omitting both
/// retains the existing trigger; decoding preserves either or both fields as evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossDetails {
    /// The price that the Guaranteed Stop Loss Order will be triggered at. Only one of the price and distance
    /// fields may be specified.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price: Option<Decimal>,
    /// Specifies the distance (in price units) from the Trade’s open price to use as the Guaranteed Stop Loss
    /// Order price. Only one of the distance and price fields may be specified.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
/// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
/// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingStopLossDetails {
    /// The distance (in price units) from the Trade’s fill price that the Trailing Stop Loss Order will be
    /// triggered at.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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

/// A `TradeOpen` object represents a Trade for an instrument that was opened in an Account. It is found
/// embedded in Transactions that affect the position of an instrument in the Account, specifically the
/// `OrderFill` Transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOpen {
    /// The ID of the Trade that was opened
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The number of units opened by the Trade
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub units: Option<Decimal>,
    /// The average price that the units were opened at.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price: Option<Decimal>,
    /// This is the fee charged for opening the trade if it has a guaranteed Stop Loss Order attached to it.
    #[serde(
        rename = "guaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_fee: Option<Decimal>,
    /// This is the fee charged for opening the trade if it has a guaranteed Stop Loss Order attached to it,
    /// expressed in the Instrument’s quote currency.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub half_spread_cost: Option<Decimal>,
    /// The margin required at the time the Trade was created. Note, this is the ‘pure’ margin required, it is
    /// not the ‘effective’ margin used that factors in the trade risk if a GSLO is attached to the trade.
    #[serde(
        rename = "initialMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub initial_margin_required: Option<Decimal>,
    /// The cost of converting this Trade's guaranteed execution fee into the Account's home
    /// currency, as OANDA names it.
    ///
    /// Not documented by OANDA. Observed on every `tradeOpened` entry of a Practice hedged-account
    /// recording on 2026-10-12, as a decimal string. Its sign and how it enters the fill's totals
    /// or the Account balance have not been recorded. `None` when OANDA does not send it, which is
    /// distinct from a stated zero.
    #[serde(
        rename = "guaranteedExecutionFeeHomeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_fee_home_conversion_cost: Option<Decimal>,
}

/// A `TradeReduce` object represents a Trade for an instrument that was reduced (either partially or fully)
/// in an Account. It is found embedded in Transactions that affect the position of an instrument in the
/// account, specifically the `OrderFill` Transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeReduce {
    /// The ID of the Trade that was reduced or closed
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The number of units that the Trade was reduced by
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub units: Option<Decimal>,
    /// The average price that the units were closed at. This price may be clamped for guaranteed Stop Loss
    /// Orders.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price: Option<Decimal>,
    /// The PL realized when reducing the Trade
    #[serde(
        rename = "realizedPL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub realized_pl: Option<Decimal>,
    /// The financing paid/collected when reducing the Trade
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub financing: Option<Decimal>,
    /// The base financing paid/collected when reducing the Trade
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub base_financing: Option<Decimal>,
    /// The quote financing paid/collected when reducing the Trade
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_financing: Option<Decimal>,
    /// The financing rate in effect for the instrument used to calculate the amount of financing paid/collected
    /// when reducing the Trade. This field will only be set if the `AccountFinancingMode` at the time of the
    /// order fill is `SECOND_BY_SECOND_INSTRUMENT`. The value is in decimal rather than percentage points, e.g.
    /// 5% is represented as 0.05.
    #[serde(
        rename = "financingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub financing_rate: Option<Decimal>,
    /// This is the fee that is charged for closing the Trade if it has a guaranteed Stop Loss Order attached to
    /// it.
    #[serde(
        rename = "guaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_fee: Option<Decimal>,
    /// This is the fee that is charged for closing the Trade if it has a guaranteed Stop Loss Order attached to
    /// it, expressed in the Instrument’s quote currency.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_guaranteed_execution_fee: Option<Decimal>,
    /// The half spread cost for the trade reduce/close. This can be a positive or negative value and is
    /// represented in the home currency of the Account.
    #[serde(
        rename = "halfSpreadCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub half_spread_cost: Option<Decimal>,
    /// The cost of converting this Trade's `realizedPL` into the Account's home currency, as OANDA
    /// names it.
    ///
    /// Not documented by OANDA. Observed on the `tradesClosed` entry of a Practice
    /// `close_position` closeout fill on 2026-10-16. Its sign and its relation to `realizedPL`
    /// have not been recorded. `None` when OANDA does not send it.
    #[serde(
        rename = "plHomeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub pl_home_conversion_cost: Option<Decimal>,
    /// The cost of converting this Trade's `baseFinancing` into the Account's home currency, as
    /// OANDA names it.
    ///
    /// Not documented by OANDA; observed as for `pl_home_conversion_cost`.
    #[serde(
        rename = "baseFinancingHomeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub base_financing_home_conversion_cost: Option<Decimal>,
    /// The cost of converting this Trade's guaranteed execution fee into the Account's home
    /// currency, as OANDA names it.
    ///
    /// Not documented by OANDA; observed as for `pl_home_conversion_cost`.
    #[serde(
        rename = "guaranteedExecutionFeeHomeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_fee_home_conversion_cost: Option<Decimal>,
    /// This Trade's home-currency conversion cost as a whole, as OANDA names it.
    ///
    /// Not documented by OANDA; observed as for `pl_home_conversion_cost`. Whether it is the sum
    /// of the three component costs has not been recorded.
    #[serde(
        rename = "homeConversionCost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub home_conversion_cost: Option<Decimal>,
}

/// A `MarketOrderTradeClose` specifies the extensions to a Market Order that has been created specifically to
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
    pub client_trade_id: Option<ClientID>,
    /// Indication of how much of the Trade to close. Either “ALL”, or a `DecimalNumber` reflection a partial
    /// close of the Trade.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<crate::TradeCloseoutUnits>,
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

provider_enum! {
    /// The reason that the Market Order was created to perform a margin closeout
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum MarketOrderMarginCloseoutReason {
        /// Trade closures resulted from violating OANDA’s margin policy
        MarginCheckViolation => "MARGIN_CHECK_VIOLATION",
        /// Trade closures came from a margin closeout event resulting from regulatory conditions placed on the
        /// Account’s margin call
        RegulatoryMarginCallViolation => "REGULATORY_MARGIN_CALL_VIOLATION",
        /// Trade closures resulted from violating the margin policy imposed by regulatory requirements
        RegulatoryMarginCheckViolation => "REGULATORY_MARGIN_CHECK_VIOLATION",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
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
    pub client_trade_id: Option<ClientID>,
    /// The Transaction ID of the `DelayedTradeClosure` transaction to which this Delayed Trade Close belongs to
    #[serde(
        rename = "sourceTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_transaction_id: Option<TransactionID>,
}

/// A `MarketOrderPositionCloseout` specifies the extensions to a Market Order when it has been created to
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
    /// Indication of how much of the Position to close. Either “ALL”, or a `DecimalNumber` reflection a partial
    /// close of the Trade. The `DecimalNumber` must always be positive, and represent a number that doesn’t
    /// exceed the absolute size of the Position.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<crate::TradeCloseoutUnits>,
}

/// A `LiquidityRegenerationSchedule` indicates how liquidity that is used when filling an Order for an
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub bid_liquidity_used: Option<Decimal>,
    /// The amount of ask liquidity used at this step in the schedule.
    #[serde(
        rename = "askLiquidityUsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub ask_liquidity_used: Option<Decimal>,
}

/// `OpenTradeFinancing` is used to pay/collect daily financing charge for an open Trade within an Account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenTradeFinancing {
    /// The ID of the Trade that financing is being paid/collected for.
    #[serde(rename = "tradeID", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeID>,
    /// The amount of financing paid/collected for the Trade.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub financing: Option<Decimal>,
    /// The amount of financing paid/collected in the Instrument’s base currency for the Trade.
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub base_financing: Option<Decimal>,
    /// The amount of financing paid/collected in the Instrument’s quote currency for the Trade.
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_financing: Option<Decimal>,
    /// The financing rate in effect for the instrument used to calculate the the amount of financing
    /// paid/collected for the Trade. This field will only be set if the `AccountFinancingMode` at the time of the
    /// daily financing is `DAILY_INSTRUMENT` or `SECOND_BY_SECOND_INSTRUMENT`. The value is in decimal rather than
    /// percentage points, e.g. 5% is represented as 0.05.
    #[serde(
        rename = "financingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub financing_rate: Option<Decimal>,
}

/// `OpenTradeFinancing` is used to pay/collect daily financing charge for a Position within an Account
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub financing: Option<Decimal>,
    /// The amount of base financing paid/collected for the Position.
    #[serde(
        rename = "baseFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub base_financing: Option<Decimal>,
    /// The amount of quote financing paid/collected for the Position.
    #[serde(
        rename = "quoteFinancing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub quote_financing: Option<Decimal>,
    /// The `HomeConversionFactors` in effect for the Position’s Instrument at the time of the `DailyFinancing`.
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

provider_enum! {
    /// The reason that a Transaction was rejected.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum TransactionRejectReason {
        /// An unexpected internal server error has occurred
        InternalServerError => "INTERNAL_SERVER_ERROR",
        /// The system was unable to determine the current price for the Order’s instrument
        InstrumentPriceUnknown => "INSTRUMENT_PRICE_UNKNOWN",
        /// The Account is not active
        AccountNotActive => "ACCOUNT_NOT_ACTIVE",
        /// The Account is locked
        AccountLocked => "ACCOUNT_LOCKED",
        /// The Account is locked for Order creation
        AccountOrderCreationLocked => "ACCOUNT_ORDER_CREATION_LOCKED",
        /// The Account is locked for configuration
        AccountConfigurationLocked => "ACCOUNT_CONFIGURATION_LOCKED",
        /// The Account is locked for deposits
        AccountDepositLocked => "ACCOUNT_DEPOSIT_LOCKED",
        /// The Account is locked for withdrawals
        AccountWithdrawalLocked => "ACCOUNT_WITHDRAWAL_LOCKED",
        /// The Account is locked for Order cancellation
        AccountOrderCancelLocked => "ACCOUNT_ORDER_CANCEL_LOCKED",
        /// The instrument specified is not tradeable by the Account
        InstrumentNotTradeable => "INSTRUMENT_NOT_TRADEABLE",
        /// Creating the Order would result in the maximum number of allowed pending Orders being exceeded
        PendingOrdersAllowedExceeded => "PENDING_ORDERS_ALLOWED_EXCEEDED",
        /// Neither the Order ID nor client Order ID are specified
        OrderIdUnspecified => "ORDER_ID_UNSPECIFIED",
        /// The Order specified does not exist
        OrderDoesntExist => "ORDER_DOESNT_EXIST",
        /// The Order ID and client Order ID specified do not identify the same Order
        OrderIdentifierInconsistency => "ORDER_IDENTIFIER_INCONSISTENCY",
        /// Neither the Trade ID nor client Trade ID are specified
        TradeIdUnspecified => "TRADE_ID_UNSPECIFIED",
        /// The Trade specified does not exist
        TradeDoesntExist => "TRADE_DOESNT_EXIST",
        /// The Trade ID and client Trade ID specified do not identify the same Trade
        TradeIdentifierInconsistency => "TRADE_IDENTIFIER_INCONSISTENCY",
        /// The Account had insufficient margin to perform the action specified. One possible reason for this is due
        /// to the creation or modification of a guaranteed `StopLoss` Order.
        InsufficientMargin => "INSUFFICIENT_MARGIN",
        /// Order instrument has not been specified
        InstrumentMissing => "INSTRUMENT_MISSING",
        /// The instrument specified is unknown
        InstrumentUnknown => "INSTRUMENT_UNKNOWN",
        /// Order units have not been not specified
        UnitsMissing => "UNITS_MISSING",
        /// Order units specified are invalid
        UnitsInvalid => "UNITS_INVALID",
        /// The units specified contain more precision than is allowed for the Order’s instrument
        UnitsPrecisionExceeded => "UNITS_PRECISION_EXCEEDED",
        /// The units specified exceeds the maximum number of units allowed
        UnitsLimitExceeded => "UNITS_LIMIT_EXCEEDED",
        /// The units specified is less than the minimum number of units required
        UnitsMinimumNotMet => "UNITS_MINIMUM_NOT_MET",
        /// The price has not been specified
        PriceMissing => "PRICE_MISSING",
        /// The price specified is invalid
        PriceInvalid => "PRICE_INVALID",
        /// The price specified contains more precision than is allowed for the instrument
        PricePrecisionExceeded => "PRICE_PRECISION_EXCEEDED",
        /// The price distance has not been specified
        PriceDistanceMissing => "PRICE_DISTANCE_MISSING",
        /// The price distance specified is invalid
        PriceDistanceInvalid => "PRICE_DISTANCE_INVALID",
        /// The price distance specified contains more precision than is allowed for the instrument
        PriceDistancePrecisionExceeded => "PRICE_DISTANCE_PRECISION_EXCEEDED",
        /// The price distance exceeds that maximum allowed amount
        PriceDistanceMaximumExceeded => "PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        /// The price distance does not meet the minimum allowed amount
        PriceDistanceMinimumNotMet => "PRICE_DISTANCE_MINIMUM_NOT_MET",
        /// The `TimeInForce` field has not been specified
        TimeInForceMissing => "TIME_IN_FORCE_MISSING",
        /// The `TimeInForce` specified is invalid
        TimeInForceInvalid => "TIME_IN_FORCE_INVALID",
        /// The `TimeInForce` is GTD but no GTD timestamp is provided
        TimeInForceGtdTimestampMissing => "TIME_IN_FORCE_GTD_TIMESTAMP_MISSING",
        /// The `TimeInForce` is GTD but the GTD timestamp is in the past
        TimeInForceGtdTimestampInPast => "TIME_IN_FORCE_GTD_TIMESTAMP_IN_PAST",
        /// The price bound specified is invalid
        PriceBoundInvalid => "PRICE_BOUND_INVALID",
        /// The price bound specified contains more precision than is allowed for the Order’s instrument
        PriceBoundPrecisionExceeded => "PRICE_BOUND_PRECISION_EXCEEDED",
        /// Multiple Orders on fill share the same client Order ID
        OrdersOnFillDuplicateClientOrderIds => "ORDERS_ON_FILL_DUPLICATE_CLIENT_ORDER_IDS",
        /// The Order does not support Trade on fill client extensions because it cannot create a new Trade
        TradeOnFillClientExtensionsNotSupported => "TRADE_ON_FILL_CLIENT_EXTENSIONS_NOT_SUPPORTED",
        /// The client Order ID specified is invalid
        ClientOrderIdInvalid => "CLIENT_ORDER_ID_INVALID",
        /// The client Order ID specified is already assigned to another pending Order
        ClientOrderIdAlreadyExists => "CLIENT_ORDER_ID_ALREADY_EXISTS",
        /// The client Order tag specified is invalid
        ClientOrderTagInvalid => "CLIENT_ORDER_TAG_INVALID",
        /// The client Order comment specified is invalid
        ClientOrderCommentInvalid => "CLIENT_ORDER_COMMENT_INVALID",
        /// The client Trade ID specified is invalid
        ClientTradeIdInvalid => "CLIENT_TRADE_ID_INVALID",
        /// The client Trade ID specified is already assigned to another open Trade
        ClientTradeIdAlreadyExists => "CLIENT_TRADE_ID_ALREADY_EXISTS",
        /// The client Trade tag specified is invalid
        ClientTradeTagInvalid => "CLIENT_TRADE_TAG_INVALID",
        /// The client Trade comment is invalid
        ClientTradeCommentInvalid => "CLIENT_TRADE_COMMENT_INVALID",
        /// The `OrderFillPositionAction` field has not been specified
        OrderFillPositionActionMissing => "ORDER_FILL_POSITION_ACTION_MISSING",
        /// The `OrderFillPositionAction` specified is invalid
        OrderFillPositionActionInvalid => "ORDER_FILL_POSITION_ACTION_INVALID",
        /// The `TriggerCondition` field has not been specified
        TriggerConditionMissing => "TRIGGER_CONDITION_MISSING",
        /// The `TriggerCondition` specified is invalid
        TriggerConditionInvalid => "TRIGGER_CONDITION_INVALID",
        /// The `OrderFillPositionAction` field has not been specified
        OrderPartialFillOptionMissing => "ORDER_PARTIAL_FILL_OPTION_MISSING",
        /// The `OrderFillPositionAction` specified is invalid.
        OrderPartialFillOptionInvalid => "ORDER_PARTIAL_FILL_OPTION_INVALID",
        /// When attempting to reissue an order (currently only a `MarketIfTouched`) that was immediately partially
        /// filled, it is not possible to create a correct pending Order.
        InvalidReissueImmediatePartialFill => "INVALID_REISSUE_IMMEDIATE_PARTIAL_FILL",
        /// The Orders on fill would be in violation of the risk management Order mutual exclusivity configuration
        /// specifying that only one risk management Order can be attached to a Trade.
        OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation => "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        /// The Orders on fill would be in violation of the risk management Order mutual exclusivity configuration
        /// specifying that if a GSLO is already attached to a Trade, no other risk management Order can be attached
        /// to a Trade.
        OrdersOnFillRmoMutualExclusivityGsloExcludesOthersViolation => "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        /// A Take Profit Order for the specified Trade already exists
        TakeProfitOrderAlreadyExists => "TAKE_PROFIT_ORDER_ALREADY_EXISTS",
        /// The Take Profit Order would cause the associated Trade to be in violation of the FIFO violation
        /// safeguard constraints.
        TakeProfitOrderWouldViolateFifoViolationSafeguard => "TAKE_PROFIT_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        /// The Take Profit on fill specified does not provide a price
        TakeProfitOnFillPriceMissing => "TAKE_PROFIT_ON_FILL_PRICE_MISSING",
        /// The Take Profit on fill specified contains an invalid price
        TakeProfitOnFillPriceInvalid => "TAKE_PROFIT_ON_FILL_PRICE_INVALID",
        /// The Take Profit on fill specified contains a price with more precision than is allowed by the Order’s
        /// instrument
        TakeProfitOnFillPricePrecisionExceeded => "TAKE_PROFIT_ON_FILL_PRICE_PRECISION_EXCEEDED",
        /// The Take Profit on fill specified does not provide a `TimeInForce`
        TakeProfitOnFillTimeInForceMissing => "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_MISSING",
        /// The Take Profit on fill specifies an invalid `TimeInForce`
        TakeProfitOnFillTimeInForceInvalid => "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_INVALID",
        /// The Take Profit on fill specifies a GTD `TimeInForce` but does not provide a GTD timestamp
        TakeProfitOnFillGtdTimestampMissing => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_MISSING",
        /// The Take Profit on fill specifies a GTD timestamp that is in the past
        TakeProfitOnFillGtdTimestampInPast => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// The Take Profit on fill client Order ID specified is invalid
        TakeProfitOnFillClientOrderIdInvalid => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_INVALID",
        /// The Take Profit on fill client Order tag specified is invalid
        TakeProfitOnFillClientOrderTagInvalid => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        /// The Take Profit on fill client Order comment specified is invalid
        TakeProfitOnFillClientOrderCommentInvalid => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        /// The Take Profit on fill specified does not provide a `TriggerCondition`
        TakeProfitOnFillTriggerConditionMissing => "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_MISSING",
        /// The Take Profit on fill specifies an invalid `TriggerCondition`
        TakeProfitOnFillTriggerConditionInvalid => "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_INVALID",
        /// A Stop Loss Order for the specified Trade already exists
        StopLossOrderAlreadyExists => "STOP_LOSS_ORDER_ALREADY_EXISTS",
        /// An attempt was made to to create a non-guaranteed stop loss order in an account that requires all stop
        /// loss orders to be guaranteed.
        StopLossOrderGuaranteedRequired => "STOP_LOSS_ORDER_GUARANTEED_REQUIRED",
        /// An attempt to create a guaranteed stop loss order with a price that is within the current tradeable
        /// spread.
        StopLossOrderGuaranteedPriceWithinSpread => "STOP_LOSS_ORDER_GUARANTEED_PRICE_WITHIN_SPREAD",
        /// An attempt was made to create a guaranteed Stop Loss Order, however the Account’s configuration does not
        /// allow guaranteed Stop Loss Orders.
        StopLossOrderGuaranteedNotAllowed => "STOP_LOSS_ORDER_GUARANTEED_NOT_ALLOWED",
        /// An attempt was made to create a guaranteed Stop Loss Order when the market was halted.
        StopLossOrderGuaranteedHaltedCreateViolation => "STOP_LOSS_ORDER_GUARANTEED_HALTED_CREATE_VIOLATION",
        /// An attempt was made to re-create a guaranteed Stop Loss Order with a tighter fill price when the market
        /// was halted.
        StopLossOrderGuaranteedHaltedTightenViolation => "STOP_LOSS_ORDER_GUARANTEED_HALTED_TIGHTEN_VIOLATION",
        /// An attempt was made to create a guaranteed Stop Loss Order on a hedged Trade (ie there is an existing
        /// open Trade in the opposing direction), however the Account’s configuration does not allow guaranteed
        /// Stop Loss Orders for hedged Trades/Positions.
        StopLossOrderGuaranteedHedgingNotAllowed => "STOP_LOSS_ORDER_GUARANTEED_HEDGING_NOT_ALLOWED",
        /// An attempt was made to create a guaranteed Stop Loss Order, however the distance between the current
        /// price and the trigger price does not meet the Account’s configured minimum Guaranteed Stop Loss distance.
        StopLossOrderGuaranteedMinimumDistanceNotMet => "STOP_LOSS_ORDER_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        /// An attempt was made to cancel a Stop Loss Order, however the Account’s configuration requires every
        /// Trade have an associated Stop Loss Order.
        StopLossOrderNotCancelable => "STOP_LOSS_ORDER_NOT_CANCELABLE",
        /// An attempt was made to cancel and replace a Stop Loss Order, however the Account’s configuration
        /// prevents the modification of Stop Loss Orders.
        StopLossOrderNotReplaceable => "STOP_LOSS_ORDER_NOT_REPLACEABLE",
        /// An attempt was made to create a guaranteed Stop Loss Order, however doing so would exceed the Account’s
        /// configured guaranteed `StopLoss` Order level restriction volume.
        StopLossOrderGuaranteedLevelRestrictionExceeded => "STOP_LOSS_ORDER_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        /// The Stop Loss Order request contains both the price and distance fields.
        StopLossOrderPriceAndDistanceBothSpecified => "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        /// The Stop Loss Order request contains neither the price nor distance fields.
        StopLossOrderPriceAndDistanceBothMissing => "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING",
        /// The Stop Loss Order would cause the associated Trade to be in violation of the FIFO violation safeguard
        /// constraints
        StopLossOrderWouldViolateFifoViolationSafeguard => "STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        /// The Stop Loss Order would be in violation of the risk management Order mutual exclusivity configuration
        /// specifying that only one risk management order can be attached to a Trade.
        StopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation => "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        /// The Stop Loss Order would be in violation of the risk management Order mutual exclusivity configuration
        /// specifying that if a GSLO is already attached to a Trade, no other risk management Order can be attached
        /// to the same Trade.
        StopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation => "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        /// An attempt to create a pending Order was made with no Stop Loss Order on fill specified and the
        /// Account’s configuration requires that every Trade have an associated Stop Loss Order.
        StopLossOnFillRequiredForPendingOrder => "STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER",
        /// An attempt to create a pending Order was made with a Stop Loss Order on fill that was explicitly
        /// configured to be guaranteed, however the Account’s configuration does not allow guaranteed Stop Loss
        /// Orders.
        StopLossOnFillGuaranteedNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
        /// An attempt to create a pending Order was made with a Stop Loss Order on fill that was explicitly
        /// configured to be not guaranteed, however the Account’s configuration requires guaranteed Stop Loss
        /// Orders.
        StopLossOnFillGuaranteedRequired => "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
        /// The Stop Loss on fill specified does not provide a price
        StopLossOnFillPriceMissing => "STOP_LOSS_ON_FILL_PRICE_MISSING",
        /// The Stop Loss on fill specifies an invalid price
        StopLossOnFillPriceInvalid => "STOP_LOSS_ON_FILL_PRICE_INVALID",
        /// The Stop Loss on fill specifies a price with more precision than is allowed by the Order’s instrument
        StopLossOnFillPricePrecisionExceeded => "STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED",
        /// An attempt to create a pending Order was made with the distance between the guaranteed Stop Loss Order
        /// on fill’s price and the pending Order’s price is less than the Account’s configured minimum guaranteed
        /// stop loss distance.
        StopLossOnFillGuaranteedMinimumDistanceNotMet => "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        /// An attempt to create a pending Order was made with a guaranteed Stop Loss Order on fill configured, and
        /// the Order’s units exceed the Account’s configured guaranteed `StopLoss` Order level restriction volume.
        StopLossOnFillGuaranteedLevelRestrictionExceeded => "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        /// The Stop Loss on fill distance is invalid
        StopLossOnFillDistanceInvalid => "STOP_LOSS_ON_FILL_DISTANCE_INVALID",
        /// The Stop Loss on fill price distance exceeds the maximum allowed amount
        StopLossOnFillPriceDistanceMaximumExceeded => "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        /// The Stop Loss on fill distance contains more precision than is allowed by the instrument
        StopLossOnFillDistancePrecisionExceeded => "STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED",
        /// The Stop Loss on fill contains both the price and distance fields.
        StopLossOnFillPriceAndDistanceBothSpecified => "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        /// The Stop Loss on fill contains neither the price nor distance fields.
        StopLossOnFillPriceAndDistanceBothMissing => "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING",
        /// The Stop Loss on fill specified does not provide a `TimeInForce`
        StopLossOnFillTimeInForceMissing => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        /// The Stop Loss on fill specifies an invalid `TimeInForce`
        StopLossOnFillTimeInForceInvalid => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        /// The Stop Loss on fill specifies a GTD `TimeInForce` but does not provide a GTD timestamp
        StopLossOnFillGtdTimestampMissing => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        /// The Stop Loss on fill specifies a GTD timestamp that is in the past
        StopLossOnFillGtdTimestampInPast => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// The Stop Loss on fill client Order ID specified is invalid
        StopLossOnFillClientOrderIdInvalid => "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        /// The Stop Loss on fill client Order tag specified is invalid
        StopLossOnFillClientOrderTagInvalid => "STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        /// The Stop Loss on fill client Order comment specified is invalid
        StopLossOnFillClientOrderCommentInvalid => "STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        /// The Stop Loss on fill specified does not provide a `TriggerCondition`
        StopLossOnFillTriggerConditionMissing => "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        /// The Stop Loss on fill specifies an invalid `TriggerCondition`
        StopLossOnFillTriggerConditionInvalid => "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        /// A Guaranteed Stop Loss Order for the specified Trade already exists
        GuaranteedStopLossOrderAlreadyExists => "GUARANTEED_STOP_LOSS_ORDER_ALREADY_EXISTS",
        /// An attempt was made to to create a non-guaranteed stop loss order in an account that requires all stop
        /// loss orders to be guaranteed.
        GuaranteedStopLossOrderRequired => "GUARANTEED_STOP_LOSS_ORDER_REQUIRED",
        /// An attempt to create a guaranteed stop loss order with a price that is within the current tradeable
        /// spread.
        GuaranteedStopLossOrderPriceWithinSpread => "GUARANTEED_STOP_LOSS_ORDER_PRICE_WITHIN_SPREAD",
        /// An attempt was made to create a Guaranteed Stop Loss Order, however the Account’s configuration does not
        /// allow Guaranteed Stop Loss Orders.
        GuaranteedStopLossOrderNotAllowed => "GUARANTEED_STOP_LOSS_ORDER_NOT_ALLOWED",
        /// An attempt was made to create a Guaranteed Stop Loss Order when the market was halted.
        GuaranteedStopLossOrderHaltedCreateViolation => "GUARANTEED_STOP_LOSS_ORDER_HALTED_CREATE_VIOLATION",
        /// An attempt was made to create a Guaranteed Stop Loss Order when the market was open.
        GuaranteedStopLossOrderCreateViolation => "GUARANTEED_STOP_LOSS_ORDER_CREATE_VIOLATION",
        /// An attempt was made to re-create a Guaranteed Stop Loss Order with a tighter fill price when the market
        /// was halted.
        GuaranteedStopLossOrderHaltedTightenViolation => "GUARANTEED_STOP_LOSS_ORDER_HALTED_TIGHTEN_VIOLATION",
        /// An attempt was made to re-create a Guaranteed Stop Loss Order with a tighter fill price when the market
        /// was open.
        GuaranteedStopLossOrderTightenViolation => "GUARANTEED_STOP_LOSS_ORDER_TIGHTEN_VIOLATION",
        /// An attempt was made to create a Guaranteed Stop Loss Order on a hedged Trade (ie there is an existing
        /// open Trade in the opposing direction), however the Account’s configuration does not allow Guaranteed
        /// Stop Loss Orders for hedged Trades/Positions.
        GuaranteedStopLossOrderHedgingNotAllowed => "GUARANTEED_STOP_LOSS_ORDER_HEDGING_NOT_ALLOWED",
        /// An attempt was made to create a Guaranteed Stop Loss Order, however the distance between the current
        /// price and the trigger price does not meet the Account’s configured minimum Guaranteed Stop Loss distance.
        GuaranteedStopLossOrderMinimumDistanceNotMet => "GUARANTEED_STOP_LOSS_ORDER_MINIMUM_DISTANCE_NOT_MET",
        /// An attempt was made to cancel a Guaranteed Stop Loss Order when the market is open, however the
        /// Account’s configuration requires every Trade have an associated Guaranteed Stop Loss Order.
        GuaranteedStopLossOrderNotCancelable => "GUARANTEED_STOP_LOSS_ORDER_NOT_CANCELABLE",
        /// An attempt was made to cancel a Guaranteed Stop Loss Order when the market is halted, however the
        /// Account’s configuration requires every Trade have an associated Guaranteed Stop Loss Order.
        GuaranteedStopLossOrderHaltedNotCancelable => "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_CANCELABLE",
        /// An attempt was made to cancel and replace a Guaranteed Stop Loss Order when the market is open, however
        /// the Account’s configuration prevents the modification of Guaranteed Stop Loss Orders.
        GuaranteedStopLossOrderNotReplaceable => "GUARANTEED_STOP_LOSS_ORDER_NOT_REPLACEABLE",
        /// An attempt was made to cancel and replace a Guaranteed Stop Loss Order when the market is halted,
        /// however the Account’s configuration prevents the modification of Guaranteed Stop Loss Orders.
        GuaranteedStopLossOrderHaltedNotReplaceable => "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_REPLACEABLE",
        /// An attempt was made to create a Guaranteed Stop Loss Order, however doing so would exceed the Account’s
        /// configured guaranteed `StopLoss` Order level restriction volume.
        GuaranteedStopLossOrderLevelRestrictionVolumeExceeded => "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_VOLUME_EXCEEDED",
        /// An attempt was made to create a Guaranteed Stop Loss Order, however doing so would exceed the Account’s
        /// configured guaranteed `StopLoss` Order level restriction price range.
        GuaranteedStopLossOrderLevelRestrictionPriceRangeExceeded => "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED",
        /// The Guaranteed Stop Loss Order request contains both the price and distance fields.
        GuaranteedStopLossOrderPriceAndDistanceBothSpecified => "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        /// The Guaranteed Stop Loss Order request contains neither the price nor distance fields.
        GuaranteedStopLossOrderPriceAndDistanceBothMissing => "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING",
        /// The Guaranteed Stop Loss Order would cause the associated Trade to be in violation of the FIFO violation
        /// safeguard constraints
        GuaranteedStopLossOrderWouldViolateFifoViolationSafeguard => "GUARANTEED_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        /// The Guaranteed Stop Loss Order would be in violation of the risk management Order mutual exclusivity
        /// configuration specifying that only one risk management order can be attached to a Trade.
        GuaranteedStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation => "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        /// The Guaranteed Stop Loss Order would be in violation of the risk management Order mutual exclusivity
        /// configuration specifying that if a GSLO is already attached to a Trade, no other risk management Order
        /// can be attached to the same Trade.
        GuaranteedStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation => "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        /// An attempt to create a pending Order was made with no Guaranteed Stop Loss Order on fill specified and
        /// the Account’s configuration requires that every Trade have an associated Guaranteed Stop Loss Order.
        GuaranteedStopLossOnFillRequiredForPendingOrder => "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER",
        /// An attempt to create a pending Order was made with a Guaranteed Stop Loss Order on fill that was
        /// explicitly configured to be guaranteed, however the Account’s configuration does not allow guaranteed
        /// Stop Loss Orders.
        GuaranteedStopLossOnFillNotAllowed => "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED",
        /// An attempt to create a pending Order was made with a Guaranteed Stop Loss Order on fill that was
        /// explicitly configured to be not guaranteed, however the Account’s configuration requires Guaranteed Stop
        /// Loss Orders.
        GuaranteedStopLossOnFillRequired => "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED",
        /// The Guaranteed Stop Loss on fill specified does not provide a price
        GuaranteedStopLossOnFillPriceMissing => "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_MISSING",
        /// The Guaranteed Stop Loss on fill specifies an invalid price
        GuaranteedStopLossOnFillPriceInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_INVALID",
        /// The Guaranteed Stop Loss on fill specifies a price with more precision than is allowed by the Order’s
        /// instrument
        GuaranteedStopLossOnFillPricePrecisionExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED",
        /// An attempt to create a pending Order was made with the distance between the Guaranteed Stop Loss Order
        /// on fill’s price and the pending Order’s price is less than the Account’s configured minimum guaranteed
        /// stop loss distance.
        GuaranteedStopLossOnFillMinimumDistanceNotMet => "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger number of
        /// units that violates the account’s Guaranteed Stop Loss Order level restriction volume.
        GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED",
        /// Filling the Order would result in the creation of a Guaranteed Stop Loss Order with trigger price that
        /// violates the account’s Guaranteed Stop Loss Order level restriction price range.
        GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED",
        /// The Guaranteed Stop Loss on fill distance is invalid
        GuaranteedStopLossOnFillDistanceInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_INVALID",
        /// The Guaranteed Stop Loss on fill price distance exceeds the maximum allowed amount.
        GuaranteedStopLossOnFillPriceDistanceMaximumExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        /// The Guaranteed Stop Loss on fill distance contains more precision than is allowed by the instrument
        GuaranteedStopLossOnFillDistancePrecisionExceeded => "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED",
        /// The Guaranteed Stop Loss on fill contains both the price and distance fields.
        GuaranteedStopLossOnFillPriceAndDistanceBothSpecified => "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        /// The Guaranteed Stop Loss on fill contains neither the price nor distance fields.
        GuaranteedStopLossOnFillPriceAndDistanceBothMissing => "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING",
        /// The Guaranteed Stop Loss on fill specified does not provide a `TimeInForce`
        GuaranteedStopLossOnFillTimeInForceMissing => "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        /// The Guaranteed Stop Loss on fill specifies an invalid `TimeInForce`
        GuaranteedStopLossOnFillTimeInForceInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        /// The Guaranteed Stop Loss on fill specifies a GTD `TimeInForce` but does not provide a GTD timestamp
        GuaranteedStopLossOnFillGtdTimestampMissing => "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        /// The Guaranteed Stop Loss on fill specifies a GTD timestamp that is in the past.
        GuaranteedStopLossOnFillGtdTimestampInPast => "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// The Guaranteed Stop Loss on fill client Order ID specified is invalid
        GuaranteedStopLossOnFillClientOrderIdInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        /// The Guaranteed Stop Loss on fill client Order tag specified is invalid
        GuaranteedStopLossOnFillClientOrderTagInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        /// The Guaranteed Stop Loss on fill client Order comment specified is invalid.
        GuaranteedStopLossOnFillClientOrderCommentInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        /// The Guaranteed Stop Loss on fill specified does not provide a `TriggerCondition`.
        GuaranteedStopLossOnFillTriggerConditionMissing => "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        /// The Guaranteed Stop Loss on fill specifies an invalid `TriggerCondition`.
        GuaranteedStopLossOnFillTriggerConditionInvalid => "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        /// A Trailing Stop Loss Order for the specified Trade already exists
        TrailingStopLossOrderAlreadyExists => "TRAILING_STOP_LOSS_ORDER_ALREADY_EXISTS",
        /// The Trailing Stop Loss Order would cause the associated Trade to be in violation of the FIFO violation
        /// safeguard constraints
        TrailingStopLossOrderWouldViolateFifoViolationSafeguard => "TRAILING_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        /// The Trailing Stop Loss Order would be in violation of the risk management Order mutual exclusivity
        /// configuration specifying that only one risk management order can be attached to a Trade.
        TrailingStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation => "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        /// The Trailing Stop Loss Order would be in violation of the risk management Order mutual exclusivity
        /// configuration specifying that if a GSLO is already attached to a Trade, no other risk management Order
        /// can be attached to the same Trade.
        TrailingStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation => "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        /// The Trailing Stop Loss on fill specified does not provide a distance
        TrailingStopLossOnFillPriceDistanceMissing => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MISSING",
        /// The Trailing Stop Loss on fill distance is invalid
        TrailingStopLossOnFillPriceDistanceInvalid => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_INVALID",
        /// The Trailing Stop Loss on fill distance contains more precision than is allowed by the instrument
        TrailingStopLossOnFillPriceDistancePrecisionExceeded => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_PRECISION_EXCEEDED",
        /// The Trailing Stop Loss on fill price distance exceeds the maximum allowed amount
        TrailingStopLossOnFillPriceDistanceMaximumExceeded => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        /// The Trailing Stop Loss on fill price distance does not meet the minimum allowed amount
        TrailingStopLossOnFillPriceDistanceMinimumNotMet => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MINIMUM_NOT_MET",
        /// The Trailing Stop Loss on fill specified does not provide a `TimeInForce`
        TrailingStopLossOnFillTimeInForceMissing => "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        /// The Trailing Stop Loss on fill specifies an invalid `TimeInForce`
        TrailingStopLossOnFillTimeInForceInvalid => "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        /// The Trailing Stop Loss on fill `TimeInForce` is specified as GTD but no GTD timestamp is provided
        TrailingStopLossOnFillGtdTimestampMissing => "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        /// The Trailing Stop Loss on fill GTD timestamp is in the past
        TrailingStopLossOnFillGtdTimestampInPast => "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        /// The Trailing Stop Loss on fill client Order ID specified is invalid
        TrailingStopLossOnFillClientOrderIdInvalid => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        /// The Trailing Stop Loss on fill client Order tag specified is invalid
        TrailingStopLossOnFillClientOrderTagInvalid => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        /// The Trailing Stop Loss on fill client Order comment specified is invalid
        TrailingStopLossOnFillClientOrderCommentInvalid => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        /// A client attempted to create either a Trailing Stop Loss order or an order with a Trailing Stop Loss On
        /// Fill specified, which may not yet be supported.
        TrailingStopLossOrdersNotSupported => "TRAILING_STOP_LOSS_ORDERS_NOT_SUPPORTED",
        /// The Trailing Stop Loss on fill specified does not provide a `TriggerCondition`
        TrailingStopLossOnFillTriggerConditionMissing => "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        /// The Tailing Stop Loss on fill specifies an invalid `TriggerCondition`
        TrailingStopLossOnFillTriggerConditionInvalid => "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        /// The request to close a Trade does not specify a full or partial close
        CloseTradeTypeMissing => "CLOSE_TRADE_TYPE_MISSING",
        /// The request to close a Trade partially did not specify the number of units to close
        CloseTradePartialUnitsMissing => "CLOSE_TRADE_PARTIAL_UNITS_MISSING",
        /// The request to partially close a Trade specifies a number of units that exceeds the current size of the
        /// given Trade
        CloseTradeUnitsExceedTradeSize => "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE",
        /// The Position requested to be closed out does not exist
        CloseoutPositionDoesntExist => "CLOSEOUT_POSITION_DOESNT_EXIST",
        /// The request to closeout a Position was specified incompletely
        CloseoutPositionIncompleteSpecification => "CLOSEOUT_POSITION_INCOMPLETE_SPECIFICATION",
        /// A partial Position closeout request specifies a number of units that exceeds the current Position
        CloseoutPositionUnitsExceedPositionSize => "CLOSEOUT_POSITION_UNITS_EXCEED_POSITION_SIZE",
        /// The request to closeout a Position could not be fully satisfied
        CloseoutPositionReject => "CLOSEOUT_POSITION_REJECT",
        /// The request to partially closeout a Position did not specify the number of units to close.
        CloseoutPositionPartialUnitsMissing => "CLOSEOUT_POSITION_PARTIAL_UNITS_MISSING",
        /// The markup group ID provided is invalid
        MarkupGroupIdInvalid => "MARKUP_GROUP_ID_INVALID",
        /// The `PositionAggregationMode` provided is not supported/valid.
        PositionAggregationModeInvalid => "POSITION_AGGREGATION_MODE_INVALID",
        /// No configuration parameters provided
        AdminConfigureDataMissing => "ADMIN_CONFIGURE_DATA_MISSING",
        /// The margin rate provided is invalid
        MarginRateInvalid => "MARGIN_RATE_INVALID",
        /// The margin rate provided would cause an immediate margin closeout
        MarginRateWouldTriggerCloseout => "MARGIN_RATE_WOULD_TRIGGER_CLOSEOUT",
        /// The account alias string provided is invalid
        AliasInvalid => "ALIAS_INVALID",
        /// No configuration parameters provided
        ClientConfigureDataMissing => "CLIENT_CONFIGURE_DATA_MISSING",
        /// The margin rate provided would cause the Account to enter a margin call state.
        MarginRateWouldTriggerMarginCall => "MARGIN_RATE_WOULD_TRIGGER_MARGIN_CALL",
        /// Funding is not possible because the requested transfer amount is invalid
        AmountInvalid => "AMOUNT_INVALID",
        /// The Account does not have sufficient balance to complete the funding request
        InsufficientFunds => "INSUFFICIENT_FUNDS",
        /// Funding amount has not been specified
        AmountMissing => "AMOUNT_MISSING",
        /// Funding reason has not been specified
        FundingReasonMissing => "FUNDING_REASON_MISSING",
        /// The list of Order Identifiers provided for a One Cancels All Order contains an Order Identifier that
        /// refers to a Stop Loss Order. OCA groups cannot contain Stop Loss Orders.
        OcaOrderIdsStopLossNotAllowed => "OCA_ORDER_IDS_STOP_LOSS_NOT_ALLOWED",
        /// Neither Order nor Trade on Fill client extensions were provided for modification
        ClientExtensionsDataMissing => "CLIENT_EXTENSIONS_DATA_MISSING",
        /// The Order to be replaced has a different type than the replacing Order.
        ReplacingOrderInvalid => "REPLACING_ORDER_INVALID",
        /// The replacing Order refers to a different Trade than the Order that is being replaced.
        ReplacingTradeIdInvalid => "REPLACING_TRADE_ID_INVALID",
        /// Canceling the order would cause an immediate margin closeout.
        OrderCancelWouldTriggerCloseout => "ORDER_CANCEL_WOULD_TRIGGER_CLOSEOUT",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// Unrecognized spellings are retained and serialized unchanged for provider
    /// compatibility; endpoint admission applies any operation-specific restrictions.
    /// A filter that can be used when fetching Transactions
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum TransactionFilter {
        /// Order-related Transactions. These are the Transactions that create, cancel, fill or trigger Orders
        Order => "ORDER",
        /// Funding-related Transactions
        Funding => "FUNDING",
        /// Administrative Transactions
        Admin => "ADMIN",
        /// Account Create Transaction
        Create => "CREATE",
        /// Account Close Transaction
        Close => "CLOSE",
        /// Account Reopen Transaction
        Reopen => "REOPEN",
        /// Client Configuration Transaction
        ClientConfigure => "CLIENT_CONFIGURE",
        /// Client Configuration Reject Transaction
        ClientConfigureReject => "CLIENT_CONFIGURE_REJECT",
        /// Transfer Funds Transaction
        TransferFunds => "TRANSFER_FUNDS",
        /// Transfer Funds Reject Transaction
        TransferFundsReject => "TRANSFER_FUNDS_REJECT",
        /// Market Order Transaction
        MarketOrder => "MARKET_ORDER",
        /// Market Order Reject Transaction
        MarketOrderReject => "MARKET_ORDER_REJECT",
        /// Limit Order Transaction
        LimitOrder => "LIMIT_ORDER",
        /// Limit Order Reject Transaction
        LimitOrderReject => "LIMIT_ORDER_REJECT",
        /// Stop Order Transaction
        StopOrder => "STOP_ORDER",
        /// Stop Order Reject Transaction
        StopOrderReject => "STOP_ORDER_REJECT",
        /// Market if Touched Order Transaction
        MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
        /// Market if Touched Order Reject Transaction
        MarketIfTouchedOrderReject => "MARKET_IF_TOUCHED_ORDER_REJECT",
        /// Take Profit Order Transaction
        TakeProfitOrder => "TAKE_PROFIT_ORDER",
        /// Take Profit Order Reject Transaction
        TakeProfitOrderReject => "TAKE_PROFIT_ORDER_REJECT",
        /// Stop Loss Order Transaction
        StopLossOrder => "STOP_LOSS_ORDER",
        /// Stop Loss Order Reject Transaction
        StopLossOrderReject => "STOP_LOSS_ORDER_REJECT",
        /// Guaranteed Stop Loss Order Transaction
        GuaranteedStopLossOrder => "GUARANTEED_STOP_LOSS_ORDER",
        /// Guaranteed Stop Loss Order Reject Transaction
        GuaranteedStopLossOrderReject => "GUARANTEED_STOP_LOSS_ORDER_REJECT",
        /// Trailing Stop Loss Order Transaction
        TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
        /// Trailing Stop Loss Order Reject Transaction
        TrailingStopLossOrderReject => "TRAILING_STOP_LOSS_ORDER_REJECT",
        /// One Cancels All Order Transaction
        OneCancelsAllOrder => "ONE_CANCELS_ALL_ORDER",
        /// One Cancels All Order Reject Transaction
        OneCancelsAllOrderReject => "ONE_CANCELS_ALL_ORDER_REJECT",
        /// One Cancels All Order Trigger Transaction
        OneCancelsAllOrderTriggered => "ONE_CANCELS_ALL_ORDER_TRIGGERED",
        /// Order Fill Transaction
        OrderFill => "ORDER_FILL",
        /// Order Cancel Transaction
        OrderCancel => "ORDER_CANCEL",
        /// Order Cancel Reject Transaction
        OrderCancelReject => "ORDER_CANCEL_REJECT",
        /// Order Client Extensions Modify Transaction
        OrderClientExtensionsModify => "ORDER_CLIENT_EXTENSIONS_MODIFY",
        /// Order Client Extensions Modify Reject Transaction
        OrderClientExtensionsModifyReject => "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
        /// Trade Client Extensions Modify Transaction
        TradeClientExtensionsModify => "TRADE_CLIENT_EXTENSIONS_MODIFY",
        /// Trade Client Extensions Modify Reject Transaction
        TradeClientExtensionsModifyReject => "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
        /// Margin Call Enter Transaction
        MarginCallEnter => "MARGIN_CALL_ENTER",
        /// Margin Call Extend Transaction
        MarginCallExtend => "MARGIN_CALL_EXTEND",
        /// Margin Call Exit Transaction
        MarginCallExit => "MARGIN_CALL_EXIT",
        /// Delayed Trade Closure Transaction
        DelayedTradeClosure => "DELAYED_TRADE_CLOSURE",
        /// Daily Financing Transaction
        DailyFinancing => "DAILY_FINANCING",
        /// Reset Resettable PL Transaction
        ResetResettablePl => "RESET_RESETTABLE_PL",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

/// A `TransactionHeartbeat` object is injected into the Transaction stream to ensure that the HTTP connection
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
    /// The date/time when the `TransactionHeartbeat` was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
}

// A concrete DTO may omit its documented default, but cannot contradict its own type.
macro_rules! concrete_type_serializer {
    ($name:ident, $expected:literal) => {
        #[expect(
            clippy::ref_option,
            reason = "Serde serialize_with passes the optional field by reference"
        )]
        fn $name<S: serde::Serializer>(
            value: &Option<TransactionType>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            if let Some(observed) = value
                && observed.as_str() != $expected
            {
                return Err(serde::ser::Error::custom(format!(
                    "OANDA type discriminator mismatch: expected {:?}, observed {:?}",
                    $expected,
                    observed.as_str()
                )));
            }
            value.serialize(serializer)
        }
    };
}
concrete_type_serializer!(serialize_create_transaction_type, "CREATE");
concrete_type_serializer!(serialize_close_transaction_type, "CLOSE");
concrete_type_serializer!(serialize_reopen_transaction_type, "REOPEN");
concrete_type_serializer!(
    serialize_client_configure_transaction_type,
    "CLIENT_CONFIGURE"
);
concrete_type_serializer!(
    serialize_client_configure_reject_transaction_type,
    "CLIENT_CONFIGURE_REJECT"
);
concrete_type_serializer!(serialize_transfer_funds_transaction_type, "TRANSFER_FUNDS");
concrete_type_serializer!(
    serialize_transfer_funds_reject_transaction_type,
    "TRANSFER_FUNDS_REJECT"
);
concrete_type_serializer!(serialize_market_order_transaction_type, "MARKET_ORDER");
concrete_type_serializer!(
    serialize_market_order_reject_transaction_type,
    "MARKET_ORDER_REJECT"
);
concrete_type_serializer!(
    serialize_fixed_price_order_transaction_type,
    "FIXED_PRICE_ORDER"
);
concrete_type_serializer!(serialize_limit_order_transaction_type, "LIMIT_ORDER");
concrete_type_serializer!(
    serialize_limit_order_reject_transaction_type,
    "LIMIT_ORDER_REJECT"
);
concrete_type_serializer!(serialize_stop_order_transaction_type, "STOP_ORDER");
concrete_type_serializer!(
    serialize_stop_order_reject_transaction_type,
    "STOP_ORDER_REJECT"
);
concrete_type_serializer!(
    serialize_market_if_touched_order_transaction_type,
    "MARKET_IF_TOUCHED_ORDER"
);
concrete_type_serializer!(
    serialize_market_if_touched_order_reject_transaction_type,
    "MARKET_IF_TOUCHED_ORDER_REJECT"
);
concrete_type_serializer!(
    serialize_take_profit_order_transaction_type,
    "TAKE_PROFIT_ORDER"
);
concrete_type_serializer!(
    serialize_take_profit_order_reject_transaction_type,
    "TAKE_PROFIT_ORDER_REJECT"
);
concrete_type_serializer!(
    serialize_stop_loss_order_transaction_type,
    "STOP_LOSS_ORDER"
);
concrete_type_serializer!(
    serialize_stop_loss_order_reject_transaction_type,
    "STOP_LOSS_ORDER_REJECT"
);
concrete_type_serializer!(
    serialize_guaranteed_stop_loss_order_transaction_type,
    "GUARANTEED_STOP_LOSS_ORDER"
);
concrete_type_serializer!(
    serialize_guaranteed_stop_loss_order_reject_transaction_type,
    "GUARANTEED_STOP_LOSS_ORDER_REJECT"
);
concrete_type_serializer!(
    serialize_trailing_stop_loss_order_transaction_type,
    "TRAILING_STOP_LOSS_ORDER"
);
concrete_type_serializer!(
    serialize_trailing_stop_loss_order_reject_transaction_type,
    "TRAILING_STOP_LOSS_ORDER_REJECT"
);
concrete_type_serializer!(serialize_order_fill_transaction_type, "ORDER_FILL");
concrete_type_serializer!(serialize_order_cancel_transaction_type, "ORDER_CANCEL");
concrete_type_serializer!(
    serialize_order_cancel_reject_transaction_type,
    "ORDER_CANCEL_REJECT"
);
concrete_type_serializer!(
    serialize_order_client_extensions_modify_transaction_type,
    "ORDER_CLIENT_EXTENSIONS_MODIFY"
);
concrete_type_serializer!(
    serialize_order_client_extensions_modify_reject_transaction_type,
    "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT"
);
concrete_type_serializer!(
    serialize_trade_client_extensions_modify_transaction_type,
    "TRADE_CLIENT_EXTENSIONS_MODIFY"
);
concrete_type_serializer!(
    serialize_trade_client_extensions_modify_reject_transaction_type,
    "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT"
);
concrete_type_serializer!(
    serialize_margin_call_enter_transaction_type,
    "MARGIN_CALL_ENTER"
);
concrete_type_serializer!(
    serialize_margin_call_extend_transaction_type,
    "MARGIN_CALL_EXTEND"
);
concrete_type_serializer!(
    serialize_margin_call_exit_transaction_type,
    "MARGIN_CALL_EXIT"
);
concrete_type_serializer!(
    serialize_delayed_trade_closure_transaction_type,
    "DELAYED_TRADE_CLOSURE"
);
concrete_type_serializer!(
    serialize_daily_financing_transaction_type,
    "DAILY_FINANCING"
);
concrete_type_serializer!(
    serialize_dividend_adjustment_transaction_type,
    "DIVIDEND_ADJUSTMENT"
);
concrete_type_serializer!(
    serialize_reset_resettable_p_l_transaction_type,
    "RESET_RESETTABLE_PL"
);

impl crate::validation::Validate for TakeProfitDetails {
    fn validate(&self) -> crate::Result<()> {
        if let Some(time_in_force) = &self.time_in_force {
            super::order::validate_time_in_force(
                time_in_force,
                false,
                self.gtd_time.as_ref(),
                false,
            )?;
        }
        Ok(())
    }
}

impl crate::validation::Validate for StopLossDetails {
    fn validate(&self) -> crate::Result<()> {
        if self.price.is_some() && self.distance.is_some() {
            return Err(crate::Error::InvalidInput(
                "price and distance are mutually exclusive".into(),
            ));
        }
        if let Some(time_in_force) = &self.time_in_force {
            super::order::validate_time_in_force(
                time_in_force,
                false,
                self.gtd_time.as_ref(),
                false,
            )?;
        }
        Ok(())
    }
}

impl crate::validation::Validate for GuaranteedStopLossDetails {
    fn validate(&self) -> crate::Result<()> {
        if self.price.is_some() && self.distance.is_some() {
            return Err(crate::Error::InvalidInput(
                "price and distance are mutually exclusive".into(),
            ));
        }
        if let Some(time_in_force) = &self.time_in_force {
            super::order::validate_time_in_force(
                time_in_force,
                false,
                self.gtd_time.as_ref(),
                false,
            )?;
        }
        Ok(())
    }
}

impl crate::validation::Validate for TrailingStopLossDetails {
    fn validate(&self) -> crate::Result<()> {
        if let Some(time_in_force) = &self.time_in_force {
            super::order::validate_time_in_force(
                time_in_force,
                false,
                self.gtd_time.as_ref(),
                false,
            )?;
        }
        Ok(())
    }
}
