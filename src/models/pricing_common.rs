//! OANDA v20 pricing-common definitions.
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A `PriceBucket` represents a price available for an amount of liquidity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceBucket {
    /// The Price offered by the `PriceBucket`
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price: Option<Decimal>,
    /// The amount of liquidity offered by the `PriceBucket`
    #[serde(
        rename = "liquidity",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::decimal_wire::optional_number_or_string"
    )]
    pub liquidity: Option<Decimal>,
}
