//! Provider-native OANDA v20 models.
//!
//! Required fields follow the provider definitions; optional fields preserve absence,
//! including documented provider defaults and partial account-change snapshots.
//! Scalar enums retain unrecognized spellings in `Unknown` for forward compatibility.
//! Financial values use exact decimals, accepting documented strings and numeric
//! liquidity without a floating-point conversion. Typed endpoint admission validates
//! outbound instructions separately, so decoding retains provider evidence.

pub use crate::ids::*;
pub use crate::timestamp::Timestamp as DateTime;
mod provider_enum;
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
