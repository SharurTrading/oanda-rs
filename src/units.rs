// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact closeout quantities and the provider's symbolic closeout instructions.

use crate::validation::Validate;
use crate::{Error, Result};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Trade closeout units, also used in executed position-closeout evidence.
///
/// Omitting a trade close request's units has the same effect as [`Self::All`].
/// Decimal evidence is preserved when decoded; outgoing requests require a positive quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeCloseoutUnits {
    /// Close all remaining units.
    All,
    /// An exact number of units. Outgoing requests require this value to be positive.
    Units(Decimal),
}

/// Units to close on one side of a position.
///
/// Omitting a position close request's side has the same effect as [`Self::All`].
/// Use [`Self::None`] explicitly to leave that side open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionCloseoutUnits {
    /// Close all units on this side.
    All,
    /// Leave this side open.
    None,
    /// An exact number of units. Outgoing requests require this value to be positive.
    Units(Decimal),
}

impl Serialize for TradeCloseoutUnits {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("ALL"),
            Self::Units(value) => serializer.collect_str(value),
        }
    }
}

impl<'de> Deserialize<'de> for TradeCloseoutUnits {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ALL" => Ok(Self::All),
            _ => Decimal::from_str_exact(&value)
                .map(Self::Units)
                .map_err(serde::de::Error::custom),
        }
    }
}

impl Serialize for PositionCloseoutUnits {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("ALL"),
            Self::None => serializer.serialize_str("NONE"),
            Self::Units(value) => serializer.collect_str(value),
        }
    }
}

impl<'de> Deserialize<'de> for PositionCloseoutUnits {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ALL" => Ok(Self::All),
            "NONE" => Ok(Self::None),
            _ => Decimal::from_str_exact(&value)
                .map(Self::Units)
                .map_err(serde::de::Error::custom),
        }
    }
}

impl Validate for TradeCloseoutUnits {
    fn validate(&self) -> Result<()> {
        match self {
            Self::All => Ok(()),
            Self::Units(value) => positive(*value),
        }
    }
}

impl Validate for PositionCloseoutUnits {
    fn validate(&self) -> Result<()> {
        match self {
            Self::All | Self::None => Ok(()),
            Self::Units(value) => positive(*value),
        }
    }
}

fn positive(value: Decimal) -> Result<()> {
    if value <= Decimal::ZERO {
        return Err(Error::InvalidInput("close units must be positive".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{PositionCloseoutUnits, TradeCloseoutUnits};
    use crate::validation::Validate;
    use rust_decimal::Decimal;

    #[test]
    fn exact_closeout_quantities_reject_rounding() -> Result<(), Box<dyn std::error::Error>> {
        let wire = "\"0.1234567890123456789012345678\"";
        let quantity: TradeCloseoutUnits = serde_json::from_str(wire)?;
        assert_eq!(serde_json::to_string(&quantity)?, wire);
        assert!(
            serde_json::from_str::<TradeCloseoutUnits>("\"0.12345678901234567890123456789\"")
                .is_err()
        );
        assert!(
            serde_json::from_str::<PositionCloseoutUnits>("\"0.12345678901234567890123456789\"")
                .is_err()
        );
        assert!(serde_json::from_str::<TradeCloseoutUnits>("\"NONE\"").is_err());
        Ok(())
    }

    #[test]
    fn invalid_outgoing_quantity_keeps_decoded_evidence() -> Result<(), Box<dyn std::error::Error>>
    {
        let quantity: TradeCloseoutUnits = serde_json::from_str("\"-1.25\"")?;
        assert!(quantity.validate().is_err());
        assert_eq!(serde_json::to_string(&quantity)?, "\"-1.25\"");
        assert!(
            PositionCloseoutUnits::Units(Decimal::ZERO)
                .validate()
                .is_err()
        );
        assert!(PositionCloseoutUnits::All.validate().is_ok());
        assert!(PositionCloseoutUnits::None.validate().is_ok());
        Ok(())
    }
}
