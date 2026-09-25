//! OANDA v20 instrument definitions.
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

/// The granularity of a candlestick
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CandlestickGranularity {
    /// 5 second candlesticks, minute alignment
    S5,
    /// 10 second candlesticks, minute alignment
    S10,
    /// 15 second candlesticks, minute alignment
    S15,
    /// 30 second candlesticks, minute alignment
    S30,
    /// 1 minute candlesticks, minute alignment
    M1,
    /// 2 minute candlesticks, hour alignment
    M2,
    /// 4 minute candlesticks, hour alignment
    M4,
    /// 5 minute candlesticks, hour alignment
    M5,
    /// 10 minute candlesticks, hour alignment
    M10,
    /// 15 minute candlesticks, hour alignment
    M15,
    /// 30 minute candlesticks, hour alignment
    M30,
    /// 1 hour candlesticks, hour alignment
    H1,
    /// 2 hour candlesticks, day alignment
    H2,
    /// 3 hour candlesticks, day alignment
    H3,
    /// 4 hour candlesticks, day alignment
    H4,
    /// 6 hour candlesticks, day alignment
    H6,
    /// 8 hour candlesticks, day alignment
    H8,
    /// 12 hour candlesticks, day alignment
    H12,
    /// 1 day candlesticks, day alignment
    D,
    /// 1 week candlesticks, aligned to start of week
    W,
    /// 1 month candlesticks, aligned to first day of the month
    M,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl CandlestickGranularity {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::S5 => "S5",
            Self::S10 => "S10",
            Self::S15 => "S15",
            Self::S30 => "S30",
            Self::M1 => "M1",
            Self::M2 => "M2",
            Self::M4 => "M4",
            Self::M5 => "M5",
            Self::M10 => "M10",
            Self::M15 => "M15",
            Self::M30 => "M30",
            Self::H1 => "H1",
            Self::H2 => "H2",
            Self::H3 => "H3",
            Self::H4 => "H4",
            Self::H6 => "H6",
            Self::H8 => "H8",
            Self::H12 => "H12",
            Self::D => "D",
            Self::W => "W",
            Self::M => "M",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for CandlestickGranularity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CandlestickGranularity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "S5" => Self::S5,
            "S10" => Self::S10,
            "S15" => Self::S15,
            "S30" => Self::S30,
            "M1" => Self::M1,
            "M2" => Self::M2,
            "M4" => Self::M4,
            "M5" => Self::M5,
            "M10" => Self::M10,
            "M15" => Self::M15,
            "M30" => Self::M30,
            "H1" => Self::H1,
            "H2" => Self::H2,
            "H3" => Self::H3,
            "H4" => Self::H4,
            "H6" => Self::H6,
            "H8" => Self::H8,
            "H12" => Self::H12,
            "D" => Self::D,
            "W" => Self::W,
            "M" => Self::M,
            _ => Self::Unknown(value),
        })
    }
}

/// The day of the week to use for candlestick granularities with weekly alignment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WeeklyAlignment {
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
    /// Sunday
    Sunday,
    /// An unrecognized provider value, preserved for forward compatibility.
    Unknown(String),
}

impl WeeklyAlignment {
    /// Return the exact provider spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for WeeklyAlignment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WeeklyAlignment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Monday" => Self::Monday,
            "Tuesday" => Self::Tuesday,
            "Wednesday" => Self::Wednesday,
            "Thursday" => Self::Thursday,
            "Friday" => Self::Friday,
            "Saturday" => Self::Saturday,
            "Sunday" => Self::Sunday,
            _ => Self::Unknown(value),
        })
    }
}

/// The Candlestick representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    /// The start time of the candlestick
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
    /// The candlestick data based on bids. Only provided if bid-based candles were requested.
    #[serde(rename = "bid", default, skip_serializing_if = "Option::is_none")]
    pub bid: Option<CandlestickData>,
    /// The candlestick data based on asks. Only provided if ask-based candles were requested.
    #[serde(rename = "ask", default, skip_serializing_if = "Option::is_none")]
    pub ask: Option<CandlestickData>,
    /// The candlestick data based on midpoints. Only provided if midpoint-based candles were requested.
    #[serde(rename = "mid", default, skip_serializing_if = "Option::is_none")]
    pub mid: Option<CandlestickData>,
    /// The number of prices created during the time-range represented by the candlestick.
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
    /// A flag indicating if the candlestick is complete. A complete candlestick is one whose ending time is not
    /// in the future.
    #[serde(rename = "complete", default, skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

/// The price data (open, high, low, close) for the Candlestick representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickData {
    /// The first (open) price in the time-range represented by the candlestick.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<Decimal>,
    /// The highest price in the time-range represented by the candlestick.
    #[serde(rename = "h", default, skip_serializing_if = "Option::is_none")]
    pub h: Option<Decimal>,
    /// The lowest price in the time-range represented by the candlestick.
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    pub l: Option<Decimal>,
    /// The last (closing) price in the time-range represented by the candlestick.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<Decimal>,
}

/// Response containing instrument, granularity, and list of candles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickResponse {
    /// The instrument whose Prices are represented by the candlesticks.
    #[serde(
        rename = "instrument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument: Option<InstrumentName>,
    /// The granularity of the candlesticks provided.
    #[serde(
        rename = "granularity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub granularity: Option<CandlestickGranularity>,
    /// The list of candlesticks that satisfy the request.
    #[serde(rename = "candles", default, skip_serializing_if = "Option::is_none")]
    pub candles: Option<Vec<Candlestick>>,
}
