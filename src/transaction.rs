//! OANDA transaction endpoint contracts.

use crate::client::RequestParts;
use crate::ids::{AccountID, TransactionID};
use crate::models::{Transaction, TransactionFilter};
use crate::validation::Validate;
use crate::{Client, Error, Result, Timestamp};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Query parameters for `list_transactions`.
///
/// The default uses the provider’s time range, page size, and transaction filters.
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

impl Validate for ListTransactionsQuery {
    fn validate(&self) -> Result<()> {
        if self
            .page_size
            .is_some_and(|size| !(1..=1000).contains(&size))
        {
            return Err(Error::InvalidInput("pageSize must be 1..=1000".into()));
        }
        if let (Some(from), Some(to)) = (self.from, self.to)
            && (to < from
                || to.into_inner().signed_duration_since(from.into_inner())
                    > chrono::Duration::days(365))
        {
            return Err(Error::InvalidInput(
                "transaction range must be ordered and at most 365 days".into(),
            ));
        }
        Ok(())
    }
}

/// Successful response for `list_transactions`.
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

impl Client {
    /// List transactions.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn list_transactions(
        &self,
        account_id: &AccountID,
        query: Option<&ListTransactionsQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<ListTransactionsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/transactions");
        let request = RequestParts::new(Method::GET, &path);
        let request = match query {
            Some(query) => request.query(query)?,
            None => request,
        };
        self.execute(request).await
    }
}

/// Successful response for `transaction_details`.
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

impl Client {
    /// Transaction details.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn transaction_details(
        &self,
        account_id: &AccountID,
        transaction_id: &TransactionID,
    ) -> std::result::Result<
        crate::ApiResponse<TransactionDetailsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let transaction_id_encoded = crate::client::segment(transaction_id.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/transactions/{transaction_id_encoded}");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Query parameters for `transactions_by_id_range`.
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

impl Validate for TransactionsByIdRangeQuery {
    fn validate(&self) -> Result<()> {
        let from = self.from.as_str().parse::<u64>();
        let to = self.to.as_str().parse::<u64>();
        if !matches!((from, to), (Ok(from), Ok(to)) if from <= to) {
            return Err(Error::InvalidInput("invalid transaction ID range".into()));
        }
        Ok(())
    }
}

/// Successful response for `transactions_by_id_range`.
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

impl Client {
    /// Transactions by id range.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn transactions_by_id_range(
        &self,
        account_id: &AccountID,
        query: &TransactionsByIdRangeQuery,
    ) -> std::result::Result<
        crate::ApiResponse<TransactionsByIdRangeResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/transactions/idrange");
        self.execute(RequestParts::new(Method::GET, &path).query(query)?)
            .await
    }
}

/// Query parameters for `transactions_since_id`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsSinceIdQuery {
    /// The ID of the last Transaction fetched. This query will return all Transactions newer than the
    /// `TransactionID`. \[required\]
    #[serde(rename = "id")]
    pub id: TransactionID,
    /// A filter for restricting the types of Transactions to retrieve.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<TransactionFilter>>,
}

impl Validate for TransactionsSinceIdQuery {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Successful response for `transactions_since_id`.
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

impl Client {
    /// Transactions since id.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn transactions_since_id(
        &self,
        account_id: &AccountID,
        query: &TransactionsSinceIdQuery,
    ) -> std::result::Result<
        crate::ApiResponse<TransactionsSinceIdResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/transactions/sinceid");
        self.execute(RequestParts::new(Method::GET, &path).query(query)?)
            .await
    }
}
