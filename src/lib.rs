//! Async provider-native client for the OANDA v20 REST API.
//!
//! The caller owns credentials, their storage, and the Tokio runtime.

// These endpoint contracts and provider descriptions are generated from OANDA's published pages.
#[allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::wildcard_imports
)]
#[path = "account.rs"]
pub mod accounts;
/// Singular compatibility alias for account endpoint contracts.
pub use accounts as account;
mod client;
mod decimal_wire;
mod environment;
mod error;
mod ids;
#[allow(
    clippy::doc_markdown,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]
pub mod models;
#[allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::wildcard_imports
)]
#[path = "order.rs"]
pub mod orders;
/// Singular compatibility alias for order endpoint contracts.
pub use orders as order;
mod patch;
#[allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::wildcard_imports
)]
#[path = "position.rs"]
pub mod positions;
/// Singular compatibility alias for position endpoint contracts.
pub use positions as position;
#[allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::wildcard_imports
)]
pub mod pricing;
pub mod stream;
mod timestamp;
#[allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::wildcard_imports
)]
#[path = "trade.rs"]
pub mod trades;
/// Singular compatibility alias for trade endpoint contracts.
pub use trades as trade;
#[allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::wildcard_imports
)]
#[path = "transaction.rs"]
pub mod transactions;
/// Singular compatibility alias for transaction endpoint contracts.
pub use transactions as transaction;
mod validation;

pub use client::{ApiResponse, Client, ClientBuilder};
pub use environment::Environment;
pub use error::{Error, GenericRejection, OperationError, Result};
pub use ids::*;
pub use patch::Patch;
pub use stream::{HttpStream, PriceStreamEvent, TransactionStreamEvent};
pub use timestamp::Timestamp;

/// Instrument models and candle endpoint contracts.
pub mod instruments {
    pub use crate::models::{Instrument, InstrumentType};
    pub use crate::pricing::{InstrumentCandlesQuery, InstrumentCandlesResponse};
    pub use crate::{InstrumentName, PricingComponent};
}

/// HTTP client, streaming, and error contracts.
pub mod transport {
    pub use crate::{
        ApiResponse, Client, ClientBuilder, Environment, Error, GenericRejection, HttpStream,
        OperationError, PriceStreamEvent, TransactionStreamEvent,
    };
}

/// Exact values, timestamps, and validated provider identities.
pub mod primitives {
    pub use crate::Timestamp;
    pub use crate::ids::*;
    pub use crate::models::{AccountUnits, DecimalNumber, PriceValue};
}
