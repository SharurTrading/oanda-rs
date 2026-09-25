//! OANDA trade endpoint contracts.
// Generated shared imports vary by capability; unused ones are deliberately allowed.
#[allow(unused_imports)]
use crate::Timestamp;
#[allow(unused_imports)]
use crate::ids::*;
#[allow(unused_imports)]
use crate::models::*;
#[allow(unused_imports)]
use crate::{Client, Result};
#[allow(unused_imports)]
use reqwest::Method;
#[allow(unused_imports)]
use rust_decimal::Decimal;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// list_trades query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListTradesQuery {
    /// List of Trade IDs to retrieve.
    #[serde(rename = "ids", default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<TradeID>>,
    /// The state to filter the requested Trades by. \[default=OPEN\]
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TradeStateFilter>,
    /// The instrument to filter the requested Trades by.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The maximum number of Trades to return. \[default=50, maximum=500\]
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// The maximum Trade ID to return. If not provided the most recent Trades in the Account are returned.
    #[serde(rename = "beforeID", default, skip_serializing_if = "Option::is_none")]
    pub before_id: Option<TradeID>,
}

/// list_trades successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTradesResponse {
    /// The list of Trade detail objects
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<Trade>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// list_trades documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListTradesRejection {
    /// Provider error code.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Provider error message.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// List trades.
    pub async fn list_trades(
        &self,
        account_id: &AccountID,
        query: Option<&ListTradesQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<ListTradesResponse>,
        crate::OperationError<ListTradesRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/trades");
        self.execute(Method::GET, &path, query, None::<&()>, None, None)
            .await
    }
}

/// list_open_trades successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOpenTradesResponse {
    /// The Account’s list of open Trades
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<Trade>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// list_open_trades documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListOpenTradesRejection {
    /// Provider error code.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Provider error message.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// List open trades.
    pub async fn list_open_trades(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<ListOpenTradesResponse>,
        crate::OperationError<ListOpenTradesRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/openTrades");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// trade_details successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDetailsResponse {
    /// The details of the requested trade
    #[serde(rename = "trade", default, skip_serializing_if = "Option::is_none")]
    pub trade: Option<Trade>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// trade_details documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradeDetailsRejection {
    /// Provider error code.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Provider error message.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Trade details.
    pub async fn trade_details(
        &self,
        account_id: &AccountID,
        trade_specifier: &TradeSpecifier,
    ) -> std::result::Result<
        crate::ApiResponse<TradeDetailsResponse>,
        crate::OperationError<TradeDetailsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let trade_specifier_encoded = crate::client::segment(trade_specifier.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/trades/{trade_specifier_encoded}");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// close_trade request body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseTradeBody {
    /// Indication of how much of the Trade to close. Either the string “ALL” (indicating that all of the Trade
    /// should be closed), or a DecimalNumber representing the number of units of the open Trade to Close using
    /// a TradeClose MarketOrder. The units specified must always be positive, and the magnitude of the value
    /// cannot exceed the magnitude of the Trade’s open units.
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

/// close_trade successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseTradeResponse {
    /// The MarketOrder Transaction created to close the Trade.
    #[serde(
        rename = "orderCreateTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_create_transaction: Option<MarketOrderTransaction>,
    /// The OrderFill Transaction that fills the Trade-closing MarketOrder and closes the Trade.
    #[serde(
        rename = "orderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction: Option<OrderFillTransaction>,
    /// The OrderCancel Transaction that immediately cancelled the Trade-closing MarketOrder.
    #[serde(
        rename = "orderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// close_trade documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseTradeRejection {
    /// The MarketOrderReject Transaction that rejects the creation of the Trade- closing MarketOrder.
    #[serde(
        rename = "orderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The code of the error that has occurred. This field may not be returned for some errors.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The human-readable description of the error that has occurred.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    /// The ID of the most recent Transaction created for the Account. Only present if the Account exists.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while satisfying the request. Only present if the Account
    /// exists.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
}

impl Client {
    /// Close trade.
    pub async fn close_trade(
        &self,
        account_id: &AccountID,
        trade_specifier: &TradeSpecifier,
        body: &CloseTradeBody,
    ) -> std::result::Result<
        crate::ApiResponse<CloseTradeResponse>,
        crate::OperationError<CloseTradeRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let trade_specifier_encoded = crate::client::segment(trade_specifier.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/trades/{trade_specifier_encoded}/close");
        self.execute(
            Method::PUT,
            &path,
            None::<&()>,
            Some(body),
            Some(account_id),
            None,
        )
        .await
    }
}

/// update_trade_client_extensions request body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTradeClientExtensionsBody {
    /// The Client Extensions to update the Trade with. Do not add, update, or delete the Client Extensions if
    /// your account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

/// update_trade_client_extensions successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTradeClientExtensionsResponse {
    /// The Transaction that updates the Trade’s Client Extensions.
    #[serde(
        rename = "tradeClientExtensionsModifyTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify_transaction: Option<TradeClientExtensionsModifyTransaction>,
    /// The IDs of all Transactions that were created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// update_trade_client_extensions documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTradeClientExtensionsRejection {
    /// The Transaction that rejects the modification of the Trade’s Client Extensions.
    #[serde(
        rename = "tradeClientExtensionsModifyRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify_reject_transaction:
        Option<TradeClientExtensionsModifyRejectTransaction>,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
    /// The code of the error that has occurred. This field may not be returned for some errors.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The human-readable description of the error that has occurred.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Update trade client extensions.
    pub async fn update_trade_client_extensions(
        &self,
        account_id: &AccountID,
        trade_specifier: &TradeSpecifier,
        body: &UpdateTradeClientExtensionsBody,
    ) -> std::result::Result<
        crate::ApiResponse<UpdateTradeClientExtensionsResponse>,
        crate::OperationError<UpdateTradeClientExtensionsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let trade_specifier_encoded = crate::client::segment(trade_specifier.as_str());
        let path = format!(
            "/v3/accounts/{account_id_encoded}/trades/{trade_specifier_encoded}/clientExtensions"
        );
        self.execute(
            Method::PUT,
            &path,
            None::<&()>,
            Some(body),
            Some(account_id),
            None,
        )
        .await
    }
}

/// set_trade_dependent_orders request body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTradeDependentOrdersBody {
    /// The specification of the Take Profit to create/modify/cancel. If takeProfit is set to null, the Take
    /// Profit Order will be cancelled if it exists. If takeProfit is not provided, the existing Take Profit
    /// Order will not be modified. If a sub-field of takeProfit is not specified, that field will be set to a
    /// default value on create, and be inherited by the replacing order on modify.
    #[serde(
        rename = "takeProfit",
        default,
        skip_serializing_if = "crate::Patch::is_unchanged"
    )]
    pub take_profit: crate::Patch<TakeProfitDetails>,
    /// The specification of the Stop Loss to create/modify/cancel. If stopLoss is set to null, the Stop Loss
    /// Order will be cancelled if it exists. If stopLoss is not provided, the existing Stop Loss Order will not
    /// be modified. If a sub-field of stopLoss is not specified, that field will be set to a default value on
    /// create, and be inherited by the replacing order on modify.
    #[serde(
        rename = "stopLoss",
        default,
        skip_serializing_if = "crate::Patch::is_unchanged"
    )]
    pub stop_loss: crate::Patch<StopLossDetails>,
    /// The specification of the Trailing Stop Loss to create/modify/cancel. If trailingStopLoss is set to null,
    /// the Trailing Stop Loss Order will be cancelled if it exists. If trailingStopLoss is not provided, the
    /// existing Trailing Stop Loss Order will not be modified. If a sub-field of trailingStopLoss is not
    /// specified, that field will be set to a default value on create, and be inherited by the replacing order
    /// on modify.
    #[serde(
        rename = "trailingStopLoss",
        default,
        skip_serializing_if = "crate::Patch::is_unchanged"
    )]
    pub trailing_stop_loss: crate::Patch<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss to create/modify/cancel. If guaranteedStopLoss is set to
    /// null, the Guaranteed Stop Loss Order will be cancelled if it exists. If guaranteedStopLoss is not
    /// provided, the existing Guaranteed Stop Loss Order will not be modified. If a sub-field of
    /// guaranteedStopLoss is not specified, that field will be set to a default value on create, and be
    /// inherited by the replacing order on modify.
    #[serde(
        rename = "guaranteedStopLoss",
        default,
        skip_serializing_if = "crate::Patch::is_unchanged"
    )]
    pub guaranteed_stop_loss: crate::Patch<GuaranteedStopLossDetails>,
}

/// set_trade_dependent_orders successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTradeDependentOrdersResponse {
    /// The Transaction created that cancels the Trade’s existing Take Profit Order.
    #[serde(
        rename = "takeProfitOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Take Profit Order for the Trade.
    #[serde(
        rename = "takeProfitOrderTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_transaction: Option<TakeProfitOrderTransaction>,
    /// The Transaction created that immediately fills the Trade’s new Take Profit Order. Only provided if the
    /// new Take Profit Order was immediately filled.
    #[serde(
        rename = "takeProfitOrderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction created that immediately cancels the Trade’s new Take Profit Order. Only provided if the
    /// new Take Profit Order was immediately cancelled.
    #[serde(
        rename = "takeProfitOrderCreatedCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that cancels the Trade’s existing Stop Loss Order.
    #[serde(
        rename = "stopLossOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Stop Loss Order for the Trade.
    #[serde(
        rename = "stopLossOrderTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_transaction: Option<StopLossOrderTransaction>,
    /// The Transaction created that immediately fills the Trade’s new Stop Order. Only provided if the new Stop
    /// Loss Order was immediately filled.
    #[serde(
        rename = "stopLossOrderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction created that immediately cancels the Trade’s new Stop Loss Order. Only provided if the
    /// new Stop Loss Order was immediately cancelled.
    #[serde(
        rename = "stopLossOrderCreatedCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that cancels the Trade’s existing Trailing Stop Loss Order.
    #[serde(
        rename = "trailingStopLossOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Trailing Stop Loss Order for the Trade.
    #[serde(
        rename = "trailingStopLossOrderTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_transaction: Option<TrailingStopLossOrderTransaction>,
    /// The Transaction created that cancels the Trade’s existing Guaranteed Stop Loss Order.
    #[serde(
        rename = "guaranteedStopLossOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Guaranteed Stop Loss Order for the Trade.
    #[serde(
        rename = "guaranteedStopLossOrderTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_transaction: Option<GuaranteedStopLossOrderTransaction>,
    /// The IDs of all Transactions that were created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// set_trade_dependent_orders documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetTradeDependentOrdersRejection {
    /// An OrderCancelRejectTransaction represents the rejection of the cancellation of an Order in the client’s
    /// Account.
    #[serde(
        rename = "takeProfitOrderCancelRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A TakeProfitOrderRejectTransaction represents the rejection of the creation of a TakeProfit Order.
    #[serde(
        rename = "takeProfitOrderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_reject_transaction: Option<TakeProfitOrderRejectTransaction>,
    /// An OrderCancelRejectTransaction represents the rejection of the cancellation of an Order in the client’s
    /// Account.
    #[serde(
        rename = "stopLossOrderCancelRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A StopLossOrderRejectTransaction represents the rejection of the creation of a StopLoss Order.
    #[serde(
        rename = "stopLossOrderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_reject_transaction: Option<StopLossOrderRejectTransaction>,
    /// An OrderCancelRejectTransaction represents the rejection of the cancellation of an Order in the client’s
    /// Account.
    #[serde(
        rename = "trailingStopLossOrderCancelRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A TrailingStopLossOrderRejectTransaction represents the rejection of the creation of a TrailingStopLoss
    /// Order.
    #[serde(
        rename = "trailingStopLossOrderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_reject_transaction: Option<TrailingStopLossOrderRejectTransaction>,
    /// An OrderCancelRejectTransaction represents the rejection of the cancellation of an Order in the client’s
    /// Account.
    #[serde(
        rename = "guaranteedStopLossOrderCancelRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A GuaranteedStopLossOrderRejectTransaction represents the rejection of the creation of a
    /// GuaranteedStopLoss Order.
    #[serde(
        rename = "guaranteedStopLossOrderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_reject_transaction:
        Option<GuaranteedStopLossOrderRejectTransaction>,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
    /// The code of the error that has occurred. This field may not be returned for some errors.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The human-readable description of the error that has occurred.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Set trade dependent orders.
    pub async fn set_trade_dependent_orders(
        &self,
        account_id: &AccountID,
        trade_specifier: &TradeSpecifier,
        body: &SetTradeDependentOrdersBody,
    ) -> std::result::Result<
        crate::ApiResponse<SetTradeDependentOrdersResponse>,
        crate::OperationError<SetTradeDependentOrdersRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let trade_specifier_encoded = crate::client::segment(trade_specifier.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/trades/{trade_specifier_encoded}/orders");
        self.execute(
            Method::PUT,
            &path,
            None::<&()>,
            Some(body),
            Some(account_id),
            None,
        )
        .await
    }
}
