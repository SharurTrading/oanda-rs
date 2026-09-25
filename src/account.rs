//! OANDA account endpoint contracts.
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

/// list_accounts successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsResponse {
    /// The list of Accounts the client is authorized to access and their associated properties.
    #[serde(rename = "accounts", default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountProperties>>,
}

/// list_accounts documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsRejection {
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
    /// List accounts.
    pub async fn list_accounts(
        &self,
    ) -> std::result::Result<
        crate::ApiResponse<ListAccountsResponse>,
        crate::OperationError<ListAccountsRejection>,
    > {
        let path = "/v3/accounts".to_owned();
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// account_details successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetailsResponse {
    /// The full details of the requested Account.
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// account_details documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetailsRejection {
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
    /// Account details.
    pub async fn account_details(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<AccountDetailsResponse>,
        crate::OperationError<AccountDetailsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// account_summary successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummaryResponse {
    /// The summary of the requested Account.
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountSummary>,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// account_summary documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummaryRejection {
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
    /// Account summary.
    pub async fn account_summary(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<AccountSummaryResponse>,
        crate::OperationError<AccountSummaryRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/summary");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// account_instruments query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountInstrumentsQuery {
    /// List of instruments to query specifically.
    #[serde(
        rename = "instruments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instruments: Option<Vec<InstrumentName>>,
}

/// account_instruments successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInstrumentsResponse {
    /// The requested list of instruments.
    #[serde(
        rename = "instruments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instruments: Option<Vec<Instrument>>,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// account_instruments documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountInstrumentsRejection {
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
    /// Account instruments.
    pub async fn account_instruments(
        &self,
        account_id: &AccountID,
        query: Option<&AccountInstrumentsQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<AccountInstrumentsResponse>,
        crate::OperationError<AccountInstrumentsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/instruments");
        self.execute(Method::GET, &path, query, None::<&()>, None, None)
            .await
    }
}

/// configure_account request body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureAccountBody {
    /// Client-defined alias (name) for the Account
    #[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// The string representation of a decimal number.
    #[serde(
        rename = "marginRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_rate: Option<Decimal>,
}

/// configure_account successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureAccountResponse {
    /// The transaction that configures the Account.
    #[serde(
        rename = "clientConfigureTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_configure_transaction: Option<ClientConfigureTransaction>,
    /// The ID of the last Transaction created for the Account.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// configure_account documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureAccountRejection {
    /// The transaction that rejects the configuration of the Account.
    #[serde(
        rename = "clientConfigureRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_configure_reject_transaction: Option<ClientConfigureRejectTransaction>,
    /// The ID of the last Transaction created for the Account.
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
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Configure account.
    pub async fn configure_account(
        &self,
        account_id: &AccountID,
        body: &ConfigureAccountBody,
    ) -> std::result::Result<
        crate::ApiResponse<ConfigureAccountResponse>,
        crate::OperationError<ConfigureAccountRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/configuration");
        self.execute(
            Method::PATCH,
            &path,
            None::<&()>,
            Some(body),
            Some(account_id),
            None,
        )
        .await
    }
}

/// account_changes query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountChangesQuery {
    /// ID of the Transaction to get Account changes since.
    #[serde(
        rename = "sinceTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub since_transaction_id: Option<TransactionID>,
}

/// account_changes successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountChangesResponse {
    /// The changes to the Account’s Orders, Trades and Positions since the specified Transaction ID. Only
    /// provided if the sinceTransactionID is supplied to the poll request.
    #[serde(rename = "changes", default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<AccountChanges>,
    /// The Account’s current price-dependent state.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<AccountChangesState>,
    /// The ID of the last Transaction created for the Account. This Transaction ID should be used for future
    /// poll requests, as the client has already observed all changes up to and including it.
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

/// account_changes documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountChangesRejection {
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
    /// Account changes.
    pub async fn account_changes(
        &self,
        account_id: &AccountID,
        query: Option<&AccountChangesQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<AccountChangesResponse>,
        crate::OperationError<AccountChangesRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/changes");
        self.execute(Method::GET, &path, query, None::<&()>, None, None)
            .await
    }
}
