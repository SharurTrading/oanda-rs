//! Validated OANDA provider identifiers.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

/// Error returned when a provider identifier is empty or contains control characters.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid OANDA identifier")]
pub struct InvalidId;

macro_rules! id_type {
    ($name:ident) => {
        #[doc = concat!("Validated OANDA ", stringify!($name), ".")]
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            /// Validate and construct an identifier.
            ///
            /// # Errors
            ///
            /// Returns [`InvalidId`] for an empty identifier or one containing control characters.
            pub fn new(value: impl Into<String>) -> Result<Self, InvalidId> {
                let value = value.into();
                if value.is_empty() || value.chars().any(char::is_control) {
                    return Err(InvalidId);
                }
                Ok(Self(value))
            }

            /// Return the provider spelling.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
        impl FromStr for $name {
            type Err = InvalidId;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new(s)
            }
        }
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.0)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

id_type!(AccountID);
id_type!(OrderID);
id_type!(TradeID);
id_type!(TransactionID);
id_type!(InstrumentName);
id_type!(OrderSpecifier);
id_type!(TradeSpecifier);
id_type!(ClientID);
id_type!(ClientTag);
id_type!(ClientComment);
id_type!(RequestID);
id_type!(ClientRequestID);
id_type!(Currency);
id_type!(CandleSpecification);
id_type!(PricingComponent);
