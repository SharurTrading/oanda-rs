//! OANDA position endpoint contracts.
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

/// list_positions successful response.
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

/// list_positions documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListPositionsRejection {
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
    /// List positions.
    pub async fn list_positions(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<ListPositionsResponse>,
        crate::OperationError<ListPositionsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/positions");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// list_open_positions successful response.
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

/// list_open_positions documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListOpenPositionsRejection {
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
    /// List open positions.
    pub async fn list_open_positions(
        &self,
        account_id: &AccountID,
    ) -> std::result::Result<
        crate::ApiResponse<ListOpenPositionsResponse>,
        crate::OperationError<ListOpenPositionsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/openPositions");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// position_details successful response.
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

/// position_details documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetailsRejection {
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
    /// Position details.
    pub async fn position_details(
        &self,
        account_id: &AccountID,
        instrument: &InstrumentName,
    ) -> std::result::Result<
        crate::ApiResponse<PositionDetailsResponse>,
        crate::OperationError<PositionDetailsRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let instrument_encoded = crate::client::segment(instrument.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/positions/{instrument_encoded}");
        self.execute(Method::GET, &path, None::<&()>, None::<&()>, None, None)
            .await
    }
}

/// close_position request body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionBody {
    /// Indication of how much of the long Position to closeout. Either the string “ALL”, the string “NONE”, or
    /// a DecimalNumber representing how many units of the long position to close using a PositionCloseout
    /// MarketOrder. The units specified must always be positive.
    #[serde(rename = "longUnits", default, skip_serializing_if = "Option::is_none")]
    pub long_units: Option<String>,
    /// The client extensions to add to the MarketOrder used to close the long position.
    #[serde(
        rename = "longClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_client_extensions: Option<ClientExtensions>,
    /// Indication of how much of the short Position to closeout. Either the string “ALL”, the string “NONE”, or
    /// a DecimalNumber representing how many units of the short position to close using a PositionCloseout
    /// MarketOrder. The units specified must always be positive.
    #[serde(
        rename = "shortUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_units: Option<String>,
    /// The client extensions to add to the MarketOrder used to close the short position.
    #[serde(
        rename = "shortClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_client_extensions: Option<ClientExtensions>,
}

/// close_position successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionResponse {
    /// The MarketOrderTransaction created to close the long Position.
    #[serde(
        rename = "longOrderCreateTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_create_transaction: Option<MarketOrderTransaction>,
    /// OrderFill Transaction that closes the long Position
    #[serde(
        rename = "longOrderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_fill_transaction: Option<OrderFillTransaction>,
    /// OrderCancel Transaction that cancels the MarketOrder created to close the long Position
    #[serde(
        rename = "longOrderCancelTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The MarketOrderTransaction created to close the short Position.
    #[serde(
        rename = "shortOrderCreateTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_create_transaction: Option<MarketOrderTransaction>,
    /// OrderFill Transaction that closes the short Position
    #[serde(
        rename = "shortOrderFillTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_fill_transaction: Option<OrderFillTransaction>,
    /// OrderCancel Transaction that cancels the MarketOrder created to close the short Position
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

/// close_position documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionRejection {
    /// The Transaction created that rejects the creation of a MarketOrder to close the long Position.
    #[serde(
        rename = "longOrderRejectTransaction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The Transaction created that rejects the creation of a MarketOrder to close the short Position.
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
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Close position.
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
