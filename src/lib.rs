//! Async provider-native client for the OANDA v20 REST API.
//!
//! The caller owns credentials, their storage, and the Tokio runtime.

#[path = "account.rs"]
pub mod accounts;
/// Singular compatibility alias for account endpoint contracts.
pub use accounts as account;
mod client;
mod environment;
mod error;
mod ids;
pub mod models;
#[path = "order.rs"]
pub mod orders;
/// Singular compatibility alias for order endpoint contracts.
pub use orders as order;
mod patch;
#[path = "position.rs"]
pub mod positions;
/// Singular compatibility alias for position endpoint contracts.
pub use positions as position;
pub mod pricing;
pub mod stream;
mod timestamp;
#[path = "trade.rs"]
pub mod trades;
/// Singular compatibility alias for trade endpoint contracts.
pub use trades as trade;
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
