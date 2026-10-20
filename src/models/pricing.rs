//! OANDA v20 pricing definitions.
use super::{PriceBucket, UnitsAvailable};
use crate::ids::{Currency, InstrumentName};
use crate::timestamp::Timestamp;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// The specification of an Account-specific Price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientPrice {
    /// The string “PRICE”. Used to identify the a Price object when found in a stream.
    /// This optional wire field preserves provider evidence. Stream consumers use
    /// [`crate::PriceStreamEvent`] for typed event dispatch.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The Price’s Instrument.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The date/time when the Price was created
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The status of the Price. Deprecated: Will be removed in a future API update.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PriceStatus>,
    /// Flag indicating if the Price is tradeable or not
    #[serde(rename = "tradeable", default, skip_serializing_if = "Option::is_none")]
    pub tradeable: Option<bool>,
    /// The list of prices and liquidity available on the Instrument’s bid side. It is possible for this list to
    /// be empty if there is no bid liquidity currently available for the Instrument in the Account.
    #[serde(rename = "bids", default, skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<PriceBucket>>,
    /// The list of prices and liquidity available on the Instrument’s ask side. It is possible for this list to
    /// be empty if there is no ask liquidity currently available for the Instrument in the Account.
    #[serde(rename = "asks", default, skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<PriceBucket>>,
    /// The closeout bid Price. This Price is used when a bid is required to closeout a Position (margin
    /// closeout or manual) yet there is no bid liquidity. The closeout bid is never used to open a new position.
    #[serde(
        rename = "closeoutBid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub closeout_bid: Option<Decimal>,
    /// The closeout ask Price. This Price is used when a ask is required to closeout a Position (margin
    /// closeout or manual) yet there is no ask liquidity. The closeout ask is never used to open a new position.
    #[serde(
        rename = "closeoutAsk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub closeout_ask: Option<Decimal>,
    /// The factors used to convert quantities of this price’s Instrument’s quote currency into a quantity of
    /// the Account’s home currency. When the includeHomeConversions is present in the pricing request
    /// (regardless of its value), this field will not be present. Deprecated: Will be removed in a future API
    /// update.
    #[serde(
        rename = "quoteHomeConversionFactors",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_home_conversion_factors: Option<QuoteHomeConversionFactors>,
    /// Representation of how many units of an Instrument are available to be traded by an Order depending on
    /// its positionFill option. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "unitsAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub units_available: Option<UnitsAvailable>,
}

provider_enum! {
    /// The status of the Price.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum PriceStatus {
        /// The Instrument’s price is tradeable.
        Tradeable => "tradeable",
        /// The Instrument’s price is not tradeable.
        NonTradeable => "non-tradeable",
        /// The Instrument of the price is invalid or there is no valid Price for the Instrument.
        Invalid => "invalid",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

/// `QuoteHomeConversionFactors` represents the factors that can be used to convert quantities of a Price’s
/// Instrument’s quote currency into the Account’s home currency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteHomeConversionFactors {
    /// The factor used to convert a positive amount of the Price’s Instrument’s quote currency into a positive
    /// amount of the Account’s home currency. Conversion is performed by multiplying the quote units by the
    /// conversion factor.
    #[serde(
        rename = "positiveUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub positive_units: Option<Decimal>,
    /// The factor used to convert a negative amount of the Price’s Instrument’s quote currency into a negative
    /// amount of the Account’s home currency. Conversion is performed by multiplying the quote units by the
    /// conversion factor.
    #[serde(
        rename = "negativeUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub negative_units: Option<Decimal>,
}

/// `HomeConversions` represents the factors to use to convert quantities of a given currency into the
/// Account’s home currency. The conversion factor depends on the scenario the conversion is required for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeConversions {
    /// The currency to be converted into the home currency.
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// The factor used to convert any gains for an Account in the specified currency into the Account’s home
    /// currency. This would include positive realized P/L and positive financing amounts. Conversion is
    /// performed by multiplying the positive P/L by the conversion factor.
    #[serde(
        rename = "accountGain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub account_gain: Option<Decimal>,
    /// The factor used to convert any losses for an Account in the specified currency into the Account’s home
    /// currency. This would include negative realized P/L and negative financing amounts. Conversion is
    /// performed by multiplying the positive P/L by the conversion factor.
    #[serde(
        rename = "accountLoss",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub account_loss: Option<Decimal>,
    /// The factor used to convert a Position or Trade Value in the specified currency into the Account’s home
    /// currency. Conversion is performed by multiplying the Position or Trade Value by the conversion factor.
    #[serde(
        rename = "positionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub position_value: Option<Decimal>,
}

/// A `PricingHeartbeat` object is injected into the Pricing stream to ensure that the HTTP connection remains
/// active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingHeartbeat {
    /// The string “HEARTBEAT”
    /// This optional wire field preserves provider evidence; use
    /// [`crate::PriceStreamEvent`] for typed stream dispatch.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The date/time when the Heartbeat was created.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
}
