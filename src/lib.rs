//! Async provider-native client for the OANDA v20 REST API.
//!
//! The caller owns credentials, their storage, and the Tokio runtime.
#![doc = include_str!("../README.md")]

#[path = "account.rs"]
pub mod accounts;
/// Singular compatibility alias for account endpoint contracts.
pub use accounts as account;
mod admission;
mod client;
mod decimal_wire;
mod diagnostics;
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
mod redirect;
pub mod stream;
mod timestamp;
#[path = "trade.rs"]
pub mod trades;
mod units;
/// Singular compatibility alias for trade endpoint contracts.
pub use trades as trade;
#[path = "transaction.rs"]
pub mod transactions;
/// Singular compatibility alias for transaction endpoint contracts.
pub use transactions as transaction;
mod validation;

pub use admission::Admission;
pub use client::{ApiResponse, Client, ClientBuilder};
pub use diagnostics::ResponseDiagnostics;
pub use environment::Environment;
pub use error::{Error, GenericRejection, OperationError, Result, Supplied};
pub use ids::*;
pub use patch::Patch;
pub use redirect::RedirectTarget;
pub use stream::{BufferedEvents, HttpStream, PriceStreamEvent, TransactionStreamEvent};
pub use timestamp::{InvalidTimestamp, Timestamp};
pub use units::{PositionCloseoutUnits, TradeCloseoutUnits};

/// Instrument models and candle endpoint contracts.
pub mod instruments {
    pub use crate::models::{Instrument, InstrumentType};
    pub use crate::pricing::{InstrumentCandlesQuery, InstrumentCandlesResponse};
    pub use crate::{InstrumentName, PricingComponent};
}

/// HTTP client, streaming, and error contracts.
pub mod transport {
    pub use crate::{
        Admission, ApiResponse, BufferedEvents, Client, ClientBuilder, Environment, Error,
        GenericRejection, HttpStream, OperationError, PriceStreamEvent, RedirectTarget,
        ResponseDiagnostics, Supplied, TransactionStreamEvent,
    };
}

/// Exact values, timestamps, and validated provider identities.
pub mod primitives {
    pub use crate::Timestamp;
    pub use crate::ids::*;
    pub use crate::models::{AccountUnits, DecimalNumber, PriceValue};
}

#[cfg(test)]
extern crate self as oanda_client;
#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod test_support;
