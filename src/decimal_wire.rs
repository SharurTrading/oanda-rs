//! Exact decoding for provider fields that may be JSON numbers or decimal strings.

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, de::Error as _};

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
    parse_exact(&raw).map(Some).map_err(D::Error::custom)
}

pub(crate) fn number_or_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Decimal, D::Error> {
    optional_number_or_string(deserializer)?
        .ok_or_else(|| D::Error::custom("required decimal is null"))
}

fn parse_exact(raw: &str) -> Result<Decimal, rust_decimal::Error> {
    if let Some((coefficient, _)) = raw.split_once(['e', 'E']) {
        Decimal::from_str_exact(coefficient)?;
        Decimal::from_scientific(raw)
    } else {
        Decimal::from_str_exact(raw)
    }
}
