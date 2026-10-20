//! Provider-native OANDA v20 models.
//!
//! Required fields follow the provider definitions; optional fields preserve absence,
//! including documented provider defaults and partial account-change snapshots.
//! Scalar enums retain unrecognized spellings in `Unknown` for forward compatibility.
//! Financial values use exact decimals, accepting documented strings and numeric
//! liquidity without a floating-point conversion. Typed endpoint admission validates
//! outbound instructions separately, so decoding retains provider evidence.

// Each spelling is declared once, alongside its public variant documentation.
macro_rules! provider_enum {
    (
        $(#[$meta:meta])* $visibility:vis enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident => $wire:literal,)*
            ;
            $(#[$unknown_meta:meta])* Unknown(String),
        }
    ) => {
        $(#[$meta])*
        $visibility enum $name {
            $($(#[$variant_meta])* $variant,)*
            $(#[$unknown_meta])* Unknown(String),
        }

        impl $name {
            /// Return the exact provider spelling.
            #[must_use]
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $wire,)*
                    Self::Unknown(value) => value,
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                Ok(match value.as_str() {
                    $($wire => Self::$variant,)*
                    _ => Self::Unknown(value),
                })
            }
        }
    };
}
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
