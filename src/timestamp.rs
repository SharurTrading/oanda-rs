//! Exact OANDA timestamp boundary.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

/// Why a timestamp cannot be represented at OANDA's nanosecond precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum InvalidTimestamp {
    /// The input is neither RFC3339 nor fractional Unix seconds.
    #[error("invalid OANDA timestamp")]
    Malformed,
    /// More than nine fractional digits would require rounding.
    #[error("OANDA timestamp has more than nine fractional digits")]
    TooManyFractionalDigits,
    /// The instant falls outside the representable UTC range.
    #[error("OANDA timestamp out of range")]
    OutOfRange,
}

/// UTC instant accepted from RFC3339 or OANDA's fractional Unix-seconds form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Wrap an already represented UTC instant without formatting and reparsing.
    #[must_use]
    pub const fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    /// Return the parsed UTC instant.
    #[must_use]
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S%.9fZ"))
    }
}

impl FromStr for Timestamp {
    type Err = InvalidTimestamp;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some((_, fraction)) = value.split_once('.')
            && fraction.bytes().take_while(u8::is_ascii_digit).count() > 9
        {
            return Err(InvalidTimestamp::TooManyFractionalDigits);
        }
        if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
            return Ok(Self(parsed.with_timezone(&Utc)));
        }
        let (whole, fraction) = value.split_once('.').unwrap_or((value, ""));
        let seconds = whole.parse::<i64>().map_err(|error| match error.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                InvalidTimestamp::OutOfRange
            }
            _ => InvalidTimestamp::Malformed,
        })?;
        if fraction.len() > 9 {
            return Err(InvalidTimestamp::TooManyFractionalDigits);
        }
        if !fraction.bytes().all(|byte| byte.is_ascii_digit())
            || (value.contains('.') && fraction.is_empty())
        {
            return Err(InvalidTimestamp::Malformed);
        }
        let digits = fraction
            .bytes()
            .fold(0_u32, |nanos, byte| nanos * 10 + u32::from(byte - b'0'));
        let exponent =
            u32::try_from(9 - fraction.len()).map_err(|_| InvalidTimestamp::Malformed)?;
        let nanos = digits * 10_u32.pow(exponent);
        // Unix decimals are signed quantities, while chrono's fractional part is positive.
        let (seconds, nanos) = if value.starts_with('-') && nanos != 0 {
            (
                seconds.checked_sub(1).ok_or(InvalidTimestamp::OutOfRange)?,
                1_000_000_000 - nanos,
            )
        } else {
            (seconds, nanos)
        };
        DateTime::from_timestamp(seconds, nanos)
            .map(Self)
            .ok_or(InvalidTimestamp::OutOfRange)
    }
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
// Test setup fails fast; production timestamp parsing returns typed errors.
#[allow(clippy::expect_used)]
mod tests {
    use super::{InvalidTimestamp, Timestamp};

    #[test]
    fn fractions_are_exact_and_signed_before_the_epoch() {
        for (unix, rfc) in [
            ("0.000000001", "1970-01-01T00:00:00.000000001Z"),
            ("-0.000000001", "1969-12-31T23:59:59.999999999Z"),
            ("-1.25", "1969-12-31T23:59:58.750000000Z"),
        ] {
            let parsed: Timestamp = unix.parse().expect("valid timestamp");
            assert_eq!(parsed.to_string(), rfc);
            assert_eq!(Timestamp::from(parsed.into_inner()), parsed);
            assert_eq!(
                serde_json::to_string(&parsed).expect("serialize"),
                format!("\"{rfc}\"")
            );
        }
    }

    #[test]
    fn unrepresentable_precision_is_an_error_in_both_formats() {
        for value in ["1.1234567890", "2024-01-02T03:04:05.1234567890Z"] {
            assert_eq!(
                value.parse::<Timestamp>(),
                Err(InvalidTimestamp::TooManyFractionalDigits)
            );
        }
        assert_eq!(
            "garbage".parse::<Timestamp>(),
            Err(InvalidTimestamp::Malformed)
        );
        assert_eq!(
            "9223372036854775808".parse::<Timestamp>(),
            Err(InvalidTimestamp::OutOfRange)
        );
    }
}
