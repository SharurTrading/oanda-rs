//! Exact OANDA timestamp boundary.

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
    /// Leap seconds cannot be represented without changing the instant.
    #[error("OANDA leap-second timestamp cannot be represented")]
    LeapSecond,
    /// The instant falls outside the representable UTC range.
    #[error("OANDA timestamp out of range")]
    OutOfRange,
}

/// UTC instant accepted from RFC3339 or OANDA's fractional Unix-seconds form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(jiff::Timestamp);

impl Timestamp {
    /// Wrap an already represented UTC instant without formatting and reparsing.
    #[must_use]
    pub const fn new(value: jiff::Timestamp) -> Self {
        Self(value)
    }

    /// Return the parsed UTC instant.
    #[must_use]
    pub const fn into_inner(self) -> jiff::Timestamp {
        self.0
    }
}

impl From<jiff::Timestamp> for Timestamp {
    fn from(value: jiff::Timestamp) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.9}", self.0)
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
        if is_rfc3339(value) {
            let stamp = value
                .parse::<jiff::Timestamp>()
                .map_err(|_| InvalidTimestamp::Malformed)?;
            // Jiff otherwise maps a leap second to second 59.
            if value.as_bytes().get(17..19) == Some(b"60") {
                return Err(InvalidTimestamp::LeapSecond);
            }
            return Ok(Self(stamp));
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
        let fraction = i128::from(digits * 10_u32.pow(exponent));
        // Apply the sign from the wire even when the whole seconds are zero.
        let nanos = i128::from(seconds) * 1_000_000_000;
        let nanos = if value.starts_with('-') {
            nanos - fraction
        } else {
            nanos + fraction
        };
        jiff::Timestamp::from_nanosecond(nanos)
            .map(Self)
            .map_err(|_| InvalidTimestamp::OutOfRange)
    }
}

// The venue specifies RFC3339's full time and offset, a subset of Jiff's ISO syntax.
fn is_rfc3339(value: &str) -> bool {
    let [
        _,
        _,
        _,
        _,
        b'-',
        _,
        _,
        b'-',
        _,
        _,
        b'T' | b't' | b' ',
        _,
        _,
        b':',
        _,
        _,
        b':',
        _,
        _,
        tail @ ..,
    ] = value.as_bytes()
    else {
        return false;
    };
    let offset = if let [b'.', fraction @ ..] = tail {
        let digits = fraction.iter().take_while(|b| b.is_ascii_digit()).count();
        if digits == 0 {
            return false;
        }
        fraction.get(digits..).unwrap_or_default()
    } else {
        tail
    };
    matches!(
        offset,
        [b'Z' | b'z']
            | [b'+' | b'-', b'0'..=b'1', _, b':', _, _]
            | [b'+' | b'-', b'2', b'0'..=b'3', b':', _, _]
    )
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
    fn leap_seconds_are_refused_instead_of_aliasing_an_instant() {
        for value in ["2016-12-31T23:59:60Z", "2017-01-01T00:59:60.123+01:00"] {
            assert_eq!(
                value.parse::<Timestamp>(),
                Err(InvalidTimestamp::LeapSecond)
            );
            assert!(serde_json::from_str::<Timestamp>(&format!("\"{value}\"")).is_err());
        }
    }

    #[test]
    fn rfc3339_and_signed_unix_inputs_serialize_as_exact_utc() {
        for value in [
            "2026-10-08T11:00:00.123456789+11:00",
            "1791417600.123456789",
        ] {
            let instant = value.parse::<Timestamp>().expect("valid timestamp");
            assert_eq!(instant.to_string(), "2026-10-08T00:00:00.123456789Z");
        }
        for value in [
            "2026-10-08T11:00+11:00",
            "2026-10-08T11:00:00+1100",
            "2026-10-08T11:00:00+24:00",
        ] {
            assert_eq!(value.parse::<Timestamp>(), Err(InvalidTimestamp::Malformed));
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
