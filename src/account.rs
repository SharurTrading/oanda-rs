//! OANDA account endpoint contracts.

use crate::client::RequestParts;
use crate::ids::{AccountID, InstrumentName, TransactionID};
use crate::models::{
    Account, AccountChanges, AccountChangesState, AccountProperties, AccountSummary,
    ClientConfigureRejectTransaction, ClientConfigureTransaction, Instrument,
};
use crate::validation::Validate;
use crate::{Client, Error, Result};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Successful response for `list_accounts`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsResponse {
    /// The list of Accounts the client is authorized to access and their associated properties.
    #[serde(rename = "accounts", default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountProperties>>,
}

impl Client {
    /// List accounts.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn list_accounts(
        &self,
    ) -> std::result::Result<
        crate::ApiResponse<ListAccountsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let path = "/v3/accounts".to_owned();
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Successful response for `account_details`.
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

impl Client {
    /// Account details.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn account_details(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<AccountDetailsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Successful response for `account_summary`.
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

impl Client {
    /// Account summary.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn account_summary(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<AccountSummaryResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/summary");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Query parameters for `account_instruments`.
///
/// The default selects all account instruments.
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

impl Validate for AccountInstrumentsQuery {
    fn validate(&self) -> Result<()> {
        if let Some(instruments) = &self.instruments {
            crate::validation::nonempty(instruments, "instruments")?;
        }
        Ok(())
    }
}

/// Successful response for `account_instruments`.
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

impl Client {
    /// Account instruments.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn account_instruments(
        &self,
        account_id: &AccountID,
        query: Option<&AccountInstrumentsQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<AccountInstrumentsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/instruments");
        let request = RequestParts::new(Method::GET, &path);
        let request = match query {
            Some(query) => request.query(query)?,
            None => request,
        };
        self.execute(request).await
    }
}

/// Request body for `configure_account`.
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub margin_rate: Option<Decimal>,
}

impl Validate for ConfigureAccountBody {
    fn validate(&self) -> Result<()> {
        if self.alias.is_none() && self.margin_rate.is_none() {
            return Err(Error::InvalidInput(
                "mutation body must not be empty".into(),
            ));
        }
        if self
            .margin_rate
            .is_some_and(|rate| rate <= Decimal::ZERO || rate > Decimal::ONE)
        {
            return Err(Error::InvalidInput("marginRate must be in (0,1]".into()));
        }
        Ok(())
    }
}

/// Successful response for `configure_account`.
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

/// Documented rejection fields for `configure_account`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl Client {
    /// Configure account.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    /// An uncertain sent mutation fences this account until caller-acknowledged reconciliation.
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
            RequestParts::new(Method::PATCH, &path)
                .body(body)?
                .mutation(crate::client::MutationAdmission::Ordinary(account_id)),
        )
        .await
    }
}

/// Query parameters for `account_changes`.
///
/// The default leaves the transaction cursor to the provider.
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

impl Validate for AccountChangesQuery {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Successful response for `account_changes`.
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

impl Client {
    /// Account changes.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn account_changes(
        &self,
        account_id: &AccountID,
        query: Option<&AccountChangesQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<AccountChangesResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/changes");
        let request = RequestParts::new(Method::GET, &path);
        let request = match query {
            Some(query) => request.query(query)?,
            None => request,
        };
        self.execute(request).await
    }
}
