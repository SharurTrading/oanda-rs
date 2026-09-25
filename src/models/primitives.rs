//! OANDA v20 primitives definitions.
// Generated shared imports vary by definition family.
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::ids::*;
#[allow(unused_imports)]
use crate::timestamp::Timestamp;
#[allow(unused_imports)]
use rust_decimal::Decimal;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// A tag associated with an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The type of the tag.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The name of the tag.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The type of an Instrument.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InstrumentType {
    /// Currency
    Currency,
    /// Contract For Difference
    Cfd,
    /// Metal
    Metal,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl InstrumentType {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Currency => "CURRENCY",
            Self::Cfd => "CFD",
            Self::Metal => "METAL",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for InstrumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for InstrumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CURRENCY" => Self::Currency,
            "CFD" => Self::Cfd,
            "METAL" => Self::Metal,
            _ => Self::Unknown(value),
        })
    }
}

/// The DayOfWeek provides a representation of the day of the week.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DayOfWeek {
    /// Sunday
    Sunday,
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl DayOfWeek {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Sunday => "SUNDAY",
            Self::Monday => "MONDAY",
            Self::Tuesday => "TUESDAY",
            Self::Wednesday => "WEDNESDAY",
            Self::Thursday => "THURSDAY",
            Self::Friday => "FRIDAY",
            Self::Saturday => "SATURDAY",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for DayOfWeek {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DayOfWeek {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "SUNDAY" => Self::Sunday,
            "MONDAY" => Self::Monday,
            "TUESDAY" => Self::Tuesday,
            "WEDNESDAY" => Self::Wednesday,
            "THURSDAY" => Self::Thursday,
            "FRIDAY" => Self::Friday,
            "SATURDAY" => Self::Saturday,
            _ => Self::Unknown(value),
        })
    }
}

/// A FinancingDayOfWeek message defines a day of the week when financing charges are debited or credited.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancingDayOfWeek {
    /// The day of the week to charge the financing.
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,
    /// The number of days worth of financing to be charged on dayOfWeek.
    #[serde(
        rename = "daysCharged",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_charged: Option<i64>,
}

/// Financing data for the instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentFinancing {
    /// The financing rate to be used for a long position for the instrument. The value is in decimal rather
    /// than percentage points, i.e. 5% is represented as 0.05.
    #[serde(rename = "longRate", default, skip_serializing_if = "Option::is_none")]
    pub long_rate: Option<Decimal>,
    /// The financing rate to be used for a short position for the instrument. The value is in decimal rather
    /// than percentage points, i.e. 5% is represented as 0.05.
    #[serde(rename = "shortRate", default, skip_serializing_if = "Option::is_none")]
    pub short_rate: Option<Decimal>,
    /// The days of the week to debit or credit financing charges; the exact time of day at which to charge the
    /// financing is set in the DivisionTradingGroup for the client’s account.
    #[serde(
        rename = "financingDaysOfWeek",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub financing_days_of_week: Option<Vec<FinancingDayOfWeek>>,
}

/// Full specification of an Instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    /// The name of the Instrument
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<InstrumentName>,
    /// The type of the Instrument
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<InstrumentType>,
    /// The display name of the Instrument
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    /// The location of the “pip” for this instrument. The decimal position of the pip in this Instrument’s
    /// price can be found at 10 ^ pipLocation (e.g. -4 pipLocation results in a decimal pip position of 10 ^ -4
    /// = 0.0001).
    #[serde(
        rename = "pipLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pip_location: Option<i64>,
    /// The number of decimal places that should be used to display prices for this instrument. (e.g. a
    /// displayPrecision of 5 would result in a price of “1” being displayed as “1.00000”)
    #[serde(
        rename = "displayPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_precision: Option<i64>,
    /// The amount of decimal places that may be provided when specifying the number of units traded for this
    /// instrument.
    #[serde(
        rename = "tradeUnitsPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_units_precision: Option<i64>,
    /// The smallest number of units allowed to be traded for this instrument.
    #[serde(
        rename = "minimumTradeSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_trade_size: Option<Decimal>,
    /// The maximum trailing stop distance allowed for a trailing stop loss created for this instrument.
    /// Specified in price units.
    #[serde(
        rename = "maximumTrailingStopDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_trailing_stop_distance: Option<Decimal>,
    /// The minimum distance allowed between the Trade’s fill price and the configured price for guaranteed Stop
    /// Loss Orders created for this instrument. Specified in price units.
    #[serde(
        rename = "minimumGuaranteedStopLossDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_guaranteed_stop_loss_distance: Option<Decimal>,
    /// The minimum trailing stop distance allowed for a trailing stop loss created for this instrument.
    /// Specified in price units.
    #[serde(
        rename = "minimumTrailingStopDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_trailing_stop_distance: Option<Decimal>,
    /// The maximum position size allowed for this instrument. Specified in units.
    #[serde(
        rename = "maximumPositionSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_position_size: Option<Decimal>,
    /// The maximum units allowed for an Order placed for this instrument. Specified in units.
    #[serde(
        rename = "maximumOrderUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_order_units: Option<Decimal>,
    /// The margin rate for this instrument.
    #[serde(
        rename = "marginRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_rate: Option<Decimal>,
    /// The commission structure for this instrument.
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<InstrumentCommission>,
    /// The current Guaranteed Stop Loss Order mode of the Account for this Instrument.
    #[serde(
        rename = "guaranteedStopLossOrderMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderModeForInstrument>,
    /// The amount that is charged to the account if a guaranteed Stop Loss Order is triggered and filled. The
    /// value is in price units and is charged for each unit of the Trade. This field will only be present if
    /// the Account’s guaranteedStopLossOrderMode for this Instrument is not ‘DISABLED’.
    #[serde(
        rename = "guaranteedStopLossOrderExecutionPremium",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_execution_premium: Option<Decimal>,
    /// The guaranteed Stop Loss Order level restriction for this instrument. This field will only be present if
    /// the Account’s guaranteedStopLossOrderMode for this Instrument is not ‘DISABLED’.
    #[serde(
        rename = "guaranteedStopLossOrderLevelRestriction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_level_restriction:
        Option<GuaranteedStopLossOrderLevelRestriction>,
    /// Financing data for this instrument.
    #[serde(rename = "financing", default, skip_serializing_if = "Option::is_none")]
    pub financing: Option<InstrumentFinancing>,
    /// The tags associated with this instrument.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// DateTime header
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AcceptDatetimeFormat {
    /// If “UNIX” is specified DateTime fields will be specified or returned in the “12345678.000000123” format.
    Unix,
    /// If “RFC3339” is specified DateTime will be specified or returned in “YYYY-MM-DDTHH:MM:SS.nnnnnnnnnZ”
    /// format.
    Rfc3339,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl AcceptDatetimeFormat {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unix => "UNIX",
            Self::Rfc3339 => "RFC3339",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for AcceptDatetimeFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AcceptDatetimeFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "UNIX" => Self::Unix,
            "RFC3339" => Self::Rfc3339,
            _ => Self::Unknown(value),
        })
    }
}

/// An InstrumentCommission represents an instrument-specific commission
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentCommission {
    /// The commission amount (in the Account’s home currency) charged per unitsTraded of the instrument
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// The number of units traded that the commission amount is based on.
    #[serde(
        rename = "unitsTraded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub units_traded: Option<Decimal>,
    /// The minimum commission amount (in the Account’s home currency) that is charged when an Order is filled
    /// for this instrument.
    #[serde(
        rename = "minimumCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_commission: Option<Decimal>,
}

/// The overall behaviour of the Account regarding Guaranteed Stop Loss Orders for a specific Instrument.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GuaranteedStopLossOrderModeForInstrument {
    /// The Account is not permitted to create Guaranteed Stop Loss Orders for this Instrument.
    Disabled,
    /// The Account is able, but not required to have Guaranteed Stop Loss Orders for open Trades for this
    /// Instrument.
    Allowed,
    /// The Account is required to have Guaranteed Stop Loss Orders for all open Trades for this Instrument.
    Required,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl GuaranteedStopLossOrderModeForInstrument {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Disabled => "DISABLED",
            Self::Allowed => "ALLOWED",
            Self::Required => "REQUIRED",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for GuaranteedStopLossOrderModeForInstrument {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for GuaranteedStopLossOrderModeForInstrument {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "DISABLED" => Self::Disabled,
            "ALLOWED" => Self::Allowed,
            "REQUIRED" => Self::Required,
            _ => Self::Unknown(value),
        })
    }
}

/// A GuaranteedStopLossOrderLevelRestriction represents the total position size that can exist within a
/// given price window for Trades with guaranteed Stop Loss Orders attached for a specific Instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrderLevelRestriction {
    /// Applies to Trades with a guaranteed Stop Loss Order attached for the specified Instrument. This is the
    /// total allowed Trade volume that can exist within the priceRange based on the trigger prices of the
    /// guaranteed Stop Loss Orders.
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<Decimal>,
    /// The price range the volume applies to. This value is in price units.
    #[serde(
        rename = "priceRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_range: Option<Decimal>,
}

/// In the context of an Order or a Trade, defines whether the units are positive or negative.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Direction {
    /// A long Order is used to to buy units of an Instrument. A Trade is long when it has bought units of an
    /// Instrument.
    Long,
    /// A short Order is used to to sell units of an Instrument. A Trade is short when it has sold units of an
    /// Instrument.
    Short,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl Direction {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Long => "LONG",
            Self::Short => "SHORT",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for Direction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "LONG" => Self::Long,
            "SHORT" => Self::Short,
            _ => Self::Unknown(value),
        })
    }
}

/// A ConversionFactor contains information used to convert an amount, from an Instrument’s base or quote
/// currency, to the home currency of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionFactor {
    /// The factor by which to multiply the amount in the given currency to obtain the amount in the home
    /// currency of the Account.
    #[serde(rename = "factor", default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<Decimal>,
}

/// A HomeConversionFactors message contains information used to convert amounts, from an Instrument’s base
/// or quote currency, to the home currency of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeConversionFactors {
    /// The ConversionFactor in effect for the Account for converting any gains realized in Instrument quote
    /// units into units of the Account’s home currency.
    #[serde(
        rename = "gainQuoteHome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gain_quote_home: Option<ConversionFactor>,
    /// The ConversionFactor in effect for the Account for converting any losses realized in Instrument quote
    /// units into units of the Account’s home currency.
    #[serde(
        rename = "lossQuoteHome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loss_quote_home: Option<ConversionFactor>,
    /// The ConversionFactor in effect for the Account for converting any gains realized in Instrument base
    /// units into units of the Account’s home currency.
    #[serde(
        rename = "gainBaseHome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gain_base_home: Option<ConversionFactor>,
    /// The ConversionFactor in effect for the Account for converting any losses realized in Instrument base
    /// units into units of the Account’s home currency.
    #[serde(
        rename = "lossBaseHome",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loss_base_home: Option<ConversionFactor>,
}
