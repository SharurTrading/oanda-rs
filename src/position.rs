//! OANDA position endpoint contracts.

use crate::client::RequestParts;
use crate::ids::{AccountID, InstrumentName, TransactionID};
use crate::models::{
    ClientExtensions, MarketOrderRejectTransaction, MarketOrderTransaction, OrderCancelTransaction,
    OrderFillTransaction, Position,
};
use crate::validation::Validate;
use crate::{Client, Error, Result};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Successful response for `list_positions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPositionsResponse {
    /// The list of Account Positions.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

impl Client {
    /// List positions.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn list_positions(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<ListPositionsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/positions");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Successful response for `list_open_positions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOpenPositionsResponse {
    /// The list of open Positions in the Account.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

impl Client {
    /// List open positions.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn list_open_positions(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<ListOpenPositionsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/openPositions");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Successful response for `position_details`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetailsResponse {
    /// The requested Position.
    #[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The ID of the most recent Transaction created for the Account
    #[serde(
        rename = "lastTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transaction_id: Option<TransactionID>,
}

impl Client {
    /// Position details.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn position_details(
        &self,
        account_id: &AccountID,
        instrument: &InstrumentName,
    ) -> std::result::Result<
        crate::ApiResponse<PositionDetailsResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let instrument_encoded = crate::client::segment(instrument.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/positions/{instrument_encoded}");
        self.execute(RequestParts::new(Method::GET, &path)).await
    }
}

/// Request body for `close_position`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionBody {
    /// Indication of how much of the long Position to closeout. Either the string “ALL”, the string “NONE”, or
    /// a `DecimalNumber` representing how many units of the long position to close using a `PositionCloseout`
    /// `MarketOrder`. The units specified must always be positive. Omitting this field closes all units
    /// on this side (the provider default is `ALL`); specify `NONE` to leave it open.
    #[serde(rename = "longUnits", default, skip_serializing_if = "Option::is_none")]
    pub long_units: Option<crate::PositionCloseoutUnits>,
    /// The client extensions to add to the `MarketOrder` used to close the long position.
    #[serde(
        rename = "longClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_client_extensions: Option<ClientExtensions>,
    /// Indication of how much of the short Position to closeout. Either the string “ALL”, the string “NONE”, or
    /// a `DecimalNumber` representing how many units of the short position to close using a `PositionCloseout`
    /// `MarketOrder`. The units specified must always be positive. Omitting this field closes all units
    /// on this side (the provider default is `ALL`); specify `NONE` to leave it open.
    #[serde(
        rename = "shortUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_units: Option<crate::PositionCloseoutUnits>,
    /// The client extensions to add to the `MarketOrder` used to close the short position.
    #[serde(
        rename = "shortClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_client_extensions: Option<ClientExtensions>,
}

impl Validate for ClosePositionBody {
    fn validate(&self) -> Result<()> {
        if self.long_units == Some(crate::PositionCloseoutUnits::None)
            && self.short_units == Some(crate::PositionCloseoutUnits::None)
        {
            return Err(Error::InvalidInput(
                "position close requests no action".into(),
            ));
        }
        for units in [self.long_units, self.short_units].into_iter().flatten() {
            units.validate()?;
        }
        Ok(())
    }
}

/// Successful response for `close_position`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionResponse {
    /// The `MarketOrderTransaction` created to close the long Position.
    #[serde(
        rename = "longOrderCreateTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_create_transaction: Option<MarketOrderTransaction>,
    /// `OrderFill` Transaction that closes the long Position
    #[serde(
        rename = "longOrderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_fill_transaction: Option<OrderFillTransaction>,
    /// `OrderCancel` Transaction that cancels the `MarketOrder` created to close the long Position
    #[serde(
        rename = "longOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The `MarketOrderTransaction` created to close the short Position.
    #[serde(
        rename = "shortOrderCreateTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_create_transaction: Option<MarketOrderTransaction>,
    /// `OrderFill` Transaction that closes the short Position
    #[serde(
        rename = "shortOrderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_fill_transaction: Option<OrderFillTransaction>,
    /// `OrderCancel` Transaction that cancels the `MarketOrder` created to close the short Position
    #[serde(
        rename = "shortOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_cancel_transaction: Option<OrderCancelTransaction>,
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

/// Documented rejection fields for `close_position`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionRejection {
    /// The Transaction created that rejects the creation of a `MarketOrder` to close the long Position.
    #[serde(
        rename = "longOrderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The Transaction created that rejects the creation of a `MarketOrder` to close the short Position.
    #[serde(
        rename = "shortOrderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_reject_transaction: Option<MarketOrderRejectTransaction>,
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
    /// Close position.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    /// An uncertain sent mutation fences this account until caller-acknowledged reconciliation.
    pub async fn close_position(
        &self,
        account_id: &AccountID,
        instrument: &InstrumentName,
        body: &ClosePositionBody,
    ) -> std::result::Result<
        crate::ApiResponse<ClosePositionResponse>,
        crate::OperationError<ClosePositionRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let instrument_encoded = crate::client::segment(instrument.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/positions/{instrument_encoded}/close");
        self.execute(
            RequestParts::new(Method::PUT, &path)
                .body(body)?
                .mutation(crate::client::MutationAdmission::Exit(account_id)),
        )
        .await
    }
}
