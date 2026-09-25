//! Provider-native OANDA v20 models.
pub use crate::ids::*;
pub use crate::timestamp::Timestamp as DateTime;
/// Exact OANDA decimal number.
pub type DecimalNumber = rust_decimal::Decimal;
/// Exact account-currency units.
pub type AccountUnits = rust_decimal::Decimal;
/// Exact provider price.
pub type PriceValue = rust_decimal::Decimal;
mod variants;
pub use variants::*;

mod account;
pub use account::*;
mod instrument;
pub use instrument::*;
mod order;
pub use order::*;
mod trade;
pub use trade::*;
mod position;
pub use position::*;
mod transaction;
pub use transaction::*;
mod pricing;
pub use pricing::*;
mod pricing_common;
pub use pricing_common::*;
mod primitives;
pub use primitives::*;
