//! OANDA order endpoint contracts.

use crate::client::RequestParts;
use crate::ids::{
    AccountID, ClientRequestID, InstrumentName, OrderID, OrderSpecifier, TransactionID,
};
use crate::models::{
    ClientExtensions, Order, OrderCancelRejectTransaction, OrderCancelTransaction,
    OrderClientExtensionsModifyRejectTransaction, OrderClientExtensionsModifyTransaction,
    OrderFillTransaction, OrderRequest, OrderStateFilter, Transaction,
};
use crate::validation::Validate;
use crate::{Client, Error, Result};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Request body for `create_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderBody {
    /// Specification of the Order to create
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderRequest>,
}

impl Validate for CreateOrderBody {
    fn validate(&self) -> Result<()> {
        self.order
            .as_ref()
            .ok_or_else(|| Error::InvalidInput("order request is required".into()))?
            .validate()
    }
}

/// Successful response for `create_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    /// The Transaction that created the Order specified by the request.
    #[serde(
        rename = "orderCreateTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_create_transaction: Option<Transaction>,
    /// The Transaction that filled the newly created Order. Only provided when the Order was immediately filled.
    #[serde(
        rename = "orderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction that cancelled the newly created Order. Only provided when the Order was immediately
    /// cancelled.
    #[serde(
        rename = "orderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction that reissues the Order. Only provided when the Order is configured to be reissued for
    /// its remaining units after a partial fill and the reissue was successful.
    #[serde(
        rename = "orderReissueTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_transaction: Option<Transaction>,
    /// The Transaction that rejects the reissue of the Order. Only provided when the Order is configured to be
    /// reissued for its remaining units after a partial fill and the reissue was rejected.
    #[serde(
        rename = "orderReissueRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_reject_transaction: Option<Transaction>,
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

/// Documented rejection fields for `create_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRejection {
    /// The Transaction that rejected the creation of the Order as requested
    #[serde(
        rename = "orderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reject_transaction: Option<Transaction>,
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
    /// The code of the error that has occurred. This field may not be returned for some errors.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The human-readable description of the error that has occurred.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl Client {
    /// Create order.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    /// An uncertain sent mutation fences this account until caller-acknowledged reconciliation.
    pub async fn create_order(
        &self,
        account_id: &AccountID,
        body: &CreateOrderBody,
    ) -> std::result::Result<
        crate::ApiResponse<CreateOrderResponse>,
        crate::OperationError<CreateOrderRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/orders");
        self.execute(
            RequestParts::new(Method::POST, &path)
                .body(body)?
                .mutation(crate::client::MutationAdmission::Ordinary(account_id)),
        )
        .await
    }
}

/// Query parameters for `list_orders`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListOrdersQuery {
    /// List of Order IDs to retrieve
    #[serde(rename = "ids", default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<OrderID>>,
    /// The state to filter the requested Orders by \[default=PENDING\]
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderStateFilter>,
    /// The instrument to filter the requested orders by
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The maximum number of Orders to return \[default=50, maximum=500\]
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// The maximum Order ID to return. If not provided the most recent Orders in the Account are returned
    #[serde(rename = "beforeID", default, skip_serializing_if = "Option::is_none")]
    pub before_id: Option<OrderID>,
}

impl Validate for ListOrdersQuery {
    fn validate(&self) -> Result<()> {
        if let Some(ids) = &self.ids {
            crate::validation::nonempty(ids, "ids")?;
        }
        crate::validation::count(self.count, 500)
    }
}

/// Successful response for `list_orders`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOrdersResponse {
    /// The list of Order detail objects
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

impl Client {
    /// List orders.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn list_orders(
        &self,
        account_id: &AccountID,
        query: Option<&ListOrdersQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<ListOrdersResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/orders");
        let request = RequestParts::new(Method::GET, &path);
        let request = match query {
            Some(query) => request.query(query)?,
            None => request,
        };
        self.execute(request).await
    }
}

/// Successful response for `list_pending_orders`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPendingOrdersResponse {
    /// The list of pending Order details
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

impl Client {
    /// List pending orders.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn list_pending_orders(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<ListPendingOrdersResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/pendingOrders");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Successful response for `order_details`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailsResponse {
    /// The details of the Order requested
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

impl Client {
    /// Order details.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn order_details(
        &self,
        account_id: &AccountID,
        order_specifier: &OrderSpecifier,
    ) -> std::result::Result<
        crate::ApiResponse<OrderDetailsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let order_specifier_encoded = crate::client::segment(order_specifier.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/orders/{order_specifier_encoded}");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Request body for `replace_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOrderBody {
    /// Specification of the replacing Order
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderRequest>,
}

impl Validate for ReplaceOrderBody {
    fn validate(&self) -> Result<()> {
        self.order
            .as_ref()
            .ok_or_else(|| Error::InvalidInput("order request is required".into()))?
            .validate()
    }
}

/// Successful response for `replace_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOrderResponse {
    /// The Transaction that cancelled the Order to be replaced.
    #[serde(
        rename = "orderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction that created the replacing Order as requested.
    #[serde(
        rename = "orderCreateTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_create_transaction: Option<Transaction>,
    /// The Transaction that filled the replacing Order. This is only provided when the replacing Order was
    /// immediately filled.
    #[serde(
        rename = "orderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction that reissues the replacing Order. Only provided when the replacing Order was partially
    /// filled immediately and is configured to be reissued for its remaining units.
    #[serde(
        rename = "orderReissueTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_transaction: Option<Transaction>,
    /// The Transaction that rejects the reissue of the Order. Only provided when the replacing Order was
    /// partially filled immediately and was configured to be reissued, however the reissue was rejected.
    #[serde(
        rename = "orderReissueRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_reject_transaction: Option<Transaction>,
    /// The Transaction that cancelled the replacing Order. Only provided when the replacing Order was
    /// immediately cancelled.
    #[serde(
        rename = "replacingOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replacing_order_cancel_transaction: Option<OrderCancelTransaction>,
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

/// Documented rejection fields for `replace_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOrderRejection {
    /// The Transaction that rejected the creation of the replacing Order
    #[serde(
        rename = "orderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reject_transaction: Option<Transaction>,
    /// The IDs of all Transactions that were created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not be returned for some errors.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The human-readable description of the error that has occurred.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    /// The Transaction that rejected the cancellation of the Order to be replaced. Only present if the Account
    /// exists.
    #[serde(
        rename = "orderCancelRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_reject_transaction: Option<Transaction>,
}

impl Client {
    /// Replace order.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    /// An uncertain sent mutation fences this account until caller-acknowledged reconciliation.
    pub async fn replace_order(
        &self,
        account_id: &AccountID,
        order_specifier: &OrderSpecifier,
        body: &ReplaceOrderBody,
        client_request_id: Option<&ClientRequestID>,
    ) -> std::result::Result<
        crate::ApiResponse<ReplaceOrderResponse>,
        crate::OperationError<ReplaceOrderRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let order_specifier_encoded = crate::client::segment(order_specifier.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/orders/{order_specifier_encoded}");
        self.execute(
            RequestParts::new(Method::PUT, &path)
                .body(body)?
                .mutation(crate::client::MutationAdmission::Ordinary(account_id))
                .client_request_id(client_request_id),
        )
        .await
    }
}

/// Successful response for `cancel_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// The Transaction that cancelled the Order
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

/// Documented rejection fields for `cancel_order`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRejection {
    /// The Transaction that rejected the cancellation of the Order. Only present if the Account exists.
    #[serde(
        rename = "orderCancelRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// The IDs of all Transactions that were created while satisfying the request. Only present if the Account
    /// exists.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_transaction_ids: Option<Vec<TransactionID>>,
    /// The ID of the most recent Transaction created for the Account. Only present if the Account exists.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not be returned for some errors.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The human-readable description of the error that has occurred.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl Client {
    /// Cancel order.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    /// An uncertain sent mutation fences this account until caller-acknowledged reconciliation.
    pub async fn cancel_order(
        &self,
        account_id: &AccountID,
        order_specifier: &OrderSpecifier,
        client_request_id: Option<&ClientRequestID>,
    ) -> std::result::Result<
        crate::ApiResponse<CancelOrderResponse>,
        crate::OperationError<CancelOrderRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let order_specifier_encoded = crate::client::segment(order_specifier.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/orders/{order_specifier_encoded}/cancel");
        self.execute(
            RequestParts::new(Method::PUT, &path)
                .mutation(crate::client::MutationAdmission::Exit(account_id))
                .client_request_id(client_request_id),
        )
        .await
    }
}

/// Request body for `update_order_client_extensions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderClientExtensionsBody {
    /// The Client Extensions to update for the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The Client Extensions to update for the Trade created when the Order is filled. Do not set, modify, or
    /// delete clientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

impl Validate for UpdateOrderClientExtensionsBody {
    fn validate(&self) -> Result<()> {
        if self.client_extensions.is_none() && self.trade_client_extensions.is_none() {
            return Err(Error::InvalidInput(
                "mutation body must not be empty".into(),
            ));
        }
        Ok(())
    }
}

/// Successful response for `update_order_client_extensions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderClientExtensionsResponse {
    /// The Transaction that modified the Client Extensions for the Order
    #[serde(
        rename = "orderClientExtensionsModifyTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_client_extensions_modify_transaction: Option<OrderClientExtensionsModifyTransaction>,
    /// The ID of the most recent Transaction created for the Account
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
}

/// Documented rejection fields for `update_order_client_extensions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderClientExtensionsRejection {
    /// The Transaction that rejected the modification of the Client Extensions for the Order
    #[serde(
        rename = "orderClientExtensionsModifyRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_client_extensions_modify_reject_transaction:
        Option<OrderClientExtensionsModifyRejectTransaction>,
    /// The ID of the most recent Transaction created for the Account
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
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl Client {
    /// Update order client extensions.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    /// An uncertain sent mutation fences this account until caller-acknowledged reconciliation.
    pub async fn update_order_client_extensions(
        &self,
        account_id: &AccountID,
        order_specifier: &OrderSpecifier,
        body: &UpdateOrderClientExtensionsBody,
    ) -> std::result::Result<
        crate::ApiResponse<UpdateOrderClientExtensionsResponse>,
        crate::OperationError<UpdateOrderClientExtensionsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let order_specifier_encoded = crate::client::segment(order_specifier.as_str());
        let path = format!(
            "/v3/accounts/{account_id_encoded}/orders/{order_specifier_encoded}/clientExtensions"
        );
        self.execute(
            RequestParts::new(Method::PUT, &path)
                .body(body)?
                .mutation(crate::client::MutationAdmission::Ordinary(account_id)),
        )
        .await
    }
}
