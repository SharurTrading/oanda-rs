//! OANDA transaction endpoint contracts.
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

/// list_transactions query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsQuery {
    /// The starting time (inclusive) of the time range for the Transactions being queried. \[default=Account
    /// Creation Time\] If the Account is created after the provided timestamp, the Account creation time will
    /// be used as the starting time.
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Timestamp>,
    /// The ending time (inclusive) of the time range for the Transactions being queried. \[default=Request
    /// Time\]
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Timestamp>,
    /// The number of Transactions to include in each page of the results. \[default=100, maximum=1000\]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// A filter for restricting the types of Transactions to retrieve.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<TransactionFilter>>,
}

/// list_transactions successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsResponse {
    /// The starting time provided in the request.
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Timestamp>,
    /// The ending time provided in the request.
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Timestamp>,
    /// The pageSize provided in the request
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// The Transaction-type filter provided in the request
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<TransactionFilter>>,
    /// The number of Transactions that are contained in the pages returned
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// The list of URLs that represent idrange queries providing the data for each page in the query results
    #[serde(rename = "pages", default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<String>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// list_transactions documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsRejection {
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
    /// List transactions.
    pub async fn list_transactions(
        &self,
        account_id: &AccountID,
        query: Option<&ListTransactionsQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<ListTransactionsResponse>,
        crate::OperationError<ListTransactionsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/transactions");
        self.execute(Method::GET, &path, query, None::<&()>, None, None)
            .await
    }
}

/// transaction_details successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetailsResponse {
    /// The details of the Transaction requested
    #[serde(
        rename = "transaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction: Option<Transaction>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// transaction_details documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetailsRejection {
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
    /// Transaction details.
    pub async fn transaction_details(
        &self,
        account_id: &AccountID,
        transaction_id: &TransactionID,
    ) -> std::result::Result<
        crate::ApiResponse<TransactionDetailsResponse>,
        crate::OperationError<TransactionDetailsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let transaction_id_encoded = crate::client::segment(transaction_id.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/transactions/{transaction_id_encoded}");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// transactions_by_id_range query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsByIdRangeQuery {
    /// The starting Transaction ID (inclusive) to fetch. \[required\]
    #[serde(rename = "from")]
    pub from: TransactionID,
    /// The ending Transaction ID (inclusive) to fetch. \[required\]
    #[serde(rename = "to")]
    pub to: TransactionID,
    /// The filter that restricts the types of Transactions to retrieve.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<TransactionFilter>>,
}

/// transactions_by_id_range successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsByIdRangeResponse {
    /// The list of Transactions that satisfy the request.
    #[serde(
        rename = "transactions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transactions: Option<Vec<Transaction>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// transactions_by_id_range documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsByIdRangeRejection {
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
    /// Transactions by id range.
    pub async fn transactions_by_id_range(
        &self,
        account_id: &AccountID,
        query: &TransactionsByIdRangeQuery,
    ) -> std::result::Result<
        crate::ApiResponse<TransactionsByIdRangeResponse>,
        crate::OperationError<TransactionsByIdRangeRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/transactions/idrange");
        self.execute(Method::GET, &path, Some(query), None::<&()>, None, None)
            .await
    }
}

/// transactions_since_id query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsSinceIdQuery {
    /// The ID of the last Transaction fetched. This query will return all Transactions newer than the
    /// TransactionID. \[required\]
    #[serde(rename = "id")]
    pub id: TransactionID,
    /// A filter for restricting the types of Transactions to retrieve.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<TransactionFilter>>,
}

/// transactions_since_id successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsSinceIdResponse {
    /// The list of Transactions that satisfy the request.
    #[serde(
        rename = "transactions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transactions: Option<Vec<Transaction>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// transactions_since_id documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsSinceIdRejection {
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
    /// Transactions since id.
    pub async fn transactions_since_id(
        &self,
        account_id: &AccountID,
        query: &TransactionsSinceIdQuery,
    ) -> std::result::Result<
        crate::ApiResponse<TransactionsSinceIdResponse>,
        crate::OperationError<TransactionsSinceIdRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/transactions/sinceid");
        self.execute(Method::GET, &path, Some(query), None::<&()>, None, None)
            .await
    }
}
