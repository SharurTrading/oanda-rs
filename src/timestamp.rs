//! Exact OANDA timestamp boundary.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

/// UTC instant accepted from RFC3339 or OANDA's fractional Unix-seconds form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Return the parsed UTC instant.
    #[must_use]
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
        )
    }
}

impl FromStr for Timestamp {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
            return Ok(Self(parsed.with_timezone(&Utc)));
        }
        let (whole, fraction) = value.split_once('.').unwrap_or((value, ""));
        let seconds = whole
            .parse::<i64>()
            .map_err(|_| "invalid OANDA timestamp".to_owned())?;
        if fraction.len() > 9 || !fraction.bytes().all(|b| b.is_ascii_digit()) {
            return Err("invalid OANDA timestamp".to_owned());
        }
        let mut nanos = fraction.to_owned();
        nanos.extend(std::iter::repeat_n('0', 9 - fraction.len()));
        let nanos = nanos
            .parse::<u32>()
            .map_err(|_| "invalid OANDA timestamp".to_owned())?;
        DateTime::from_timestamp(seconds, nanos)
            .map(Self)
            .ok_or_else(|| "OANDA timestamp out of range".to_owned())
    }
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}
