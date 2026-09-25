//! Exact decoding for provider fields that may be JSON numbers or decimal strings.

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, de::Error as _};
use std::str::FromStr;

pub(crate) fn optional_number_or_string<'de, D>(
    deserializer: D,
) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    let raw = match value {
        None => return Ok(None),
        Some(serde_json::Value::Number(number)) => number.to_string(),
        Some(serde_json::Value::String(string)) => string,
        Some(_) => return Err(D::Error::custom("expected a decimal number or string")),
    };
    Decimal::from_str(&raw).map(Some).map_err(D::Error::custom)
}
