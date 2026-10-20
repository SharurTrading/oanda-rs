//! Validated OANDA provider identifiers.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

/// Why an identifier was rejected at its public construction boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum InvalidIdKind {
    /// An identity must contain at least one character.
    #[error("empty identifier")]
    Empty,
    /// Provider identities cannot contain control characters.
    #[error("identifier contains a control character")]
    ControlCharacter,
}

/// A rejected provider identifier, retained for caller correction without cloning.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid OANDA identifier: {kind}")]
pub struct InvalidId {
    value: String,
    kind: InvalidIdKind,
}

impl InvalidId {
    /// The reason construction failed.
    #[must_use]
    pub const fn kind(&self) -> InvalidIdKind {
        self.kind
    }
    /// The rejected provider spelling.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Recover ownership of the rejected value for correction.
    #[must_use]
    pub fn into_value(self) -> String {
        self.value
    }
}

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
                if value.is_empty() {
                    return Err(InvalidId {
                        value,
                        kind: InvalidIdKind::Empty,
                    });
                }
                if value.chars().any(char::is_control) {
                    return Err(InvalidId {
                        value,
                        kind: InvalidIdKind::ControlCharacter,
                    });
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

#[cfg(test)]
// Invalid-input fixtures fail immediately if construction unexpectedly succeeds.
#[allow(clippy::expect_used)]
mod tests {
    use super::{AccountID, InvalidIdKind};

    #[test]
    fn rejected_identifiers_retain_the_consumed_spelling_and_reason() {
        for (value, kind) in [
            (String::new(), InvalidIdKind::Empty),
            ("bad\nidentity".to_owned(), InvalidIdKind::ControlCharacter),
        ] {
            let error = AccountID::new(value.clone()).expect_err("invalid identifier");
            assert_eq!(error.kind(), kind);
            assert_eq!(error.value(), value);
            assert_eq!(error.into_value(), value);
        }
    }
}
