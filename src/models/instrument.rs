//! OANDA v20 instrument definitions.
use crate::ids::InstrumentName;
use crate::timestamp::Timestamp;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

provider_enum! {
    /// Unrecognized spellings are retained and serialized unchanged for provider
    /// compatibility; endpoint admission applies any operation-specific restrictions.
    /// The granularity of a candlestick
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum CandlestickGranularity {
        /// 5 second candlesticks, minute alignment
        S5 => "S5",
        /// 10 second candlesticks, minute alignment
        S10 => "S10",
        /// 15 second candlesticks, minute alignment
        S15 => "S15",
        /// 30 second candlesticks, minute alignment
        S30 => "S30",
        /// 1 minute candlesticks, minute alignment
        M1 => "M1",
        /// 2 minute candlesticks, hour alignment
        M2 => "M2",
        /// 4 minute candlesticks, hour alignment
        M4 => "M4",
        /// 5 minute candlesticks, hour alignment
        M5 => "M5",
        /// 10 minute candlesticks, hour alignment
        M10 => "M10",
        /// 15 minute candlesticks, hour alignment
        M15 => "M15",
        /// 30 minute candlesticks, hour alignment
        M30 => "M30",
        /// 1 hour candlesticks, hour alignment
        H1 => "H1",
        /// 2 hour candlesticks, day alignment
        H2 => "H2",
        /// 3 hour candlesticks, day alignment
        H3 => "H3",
        /// 4 hour candlesticks, day alignment
        H4 => "H4",
        /// 6 hour candlesticks, day alignment
        H6 => "H6",
        /// 8 hour candlesticks, day alignment
        H8 => "H8",
        /// 12 hour candlesticks, day alignment
        H12 => "H12",
        /// 1 day candlesticks, day alignment
        D => "D",
        /// 1 week candlesticks, aligned to start of week
        W => "W",
        /// 1 month candlesticks, aligned to first day of the month
        M => "M",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// Unrecognized spellings are retained and serialized unchanged for provider
    /// compatibility; endpoint admission applies any operation-specific restrictions.
    /// The day of the week to use for candlestick granularities with weekly alignment.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum WeeklyAlignment {
        /// Monday
        Monday => "Monday",
        /// Tuesday
        Tuesday => "Tuesday",
        /// Wednesday
        Wednesday => "Wednesday",
        /// Thursday
        Thursday => "Thursday",
        /// Friday
        Friday => "Friday",
        /// Saturday
        Saturday => "Saturday",
        /// Sunday
        Sunday => "Sunday",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub o: Option<Decimal>,
    /// The highest price in the time-range represented by the candlestick.
    #[serde(rename = "h", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub h: Option<Decimal>,
    /// The lowest price in the time-range represented by the candlestick.
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub l: Option<Decimal>,
    /// The last (closing) price in the time-range represented by the candlestick.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
