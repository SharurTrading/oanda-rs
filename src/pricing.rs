//! OANDA pricing endpoint contracts.
// Generated shared imports vary by capability; unused ones are deliberately allowed.
#[allow(unused_imports)]
use crate::Timestamp;
#[allow(unused_imports)]
use crate::ids::*;
#[allow(unused_imports)]
use crate::models::*;
#[allow(unused_imports)]
use crate::{Client, Result};
#[allow(unused_imports)]
use reqwest::Method;
#[allow(unused_imports)]
use rust_decimal::Decimal;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// latest_candles query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestCandlesQuery {
    /// List of candle specifications to get pricing for. \[required\]
    #[serde(rename = "candleSpecifications")]
    pub candle_specifications: Vec<CandleSpecification>,
    /// The number of units used to calculate the volume-weighted average bid and ask prices in the returned
    /// candles. \[default=1\]
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Decimal>,
    /// A flag that controls whether the candlestick is “smoothed” or not. A smoothed candlestick uses the
    /// previous candle’s close price as its open price, while an unsmoothed candlestick uses the first price
    /// from its time range as its open price. \[default=False\]
    #[serde(rename = "smooth", default, skip_serializing_if = "Option::is_none")]
    pub smooth: Option<bool>,
    /// The hour of the day (in the specified timezone) to use for granularities that have daily alignments.
    /// \[default=17, minimum=0, maximum=23\]
    #[serde(
        rename = "dailyAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_alignment: Option<i64>,
    /// The timezone to use for the dailyAlignment parameter. Candlesticks with daily alignment will be aligned
    /// to the dailyAlignment hour within the alignmentTimezone. Note that the returned times will still be
    /// represented in UTC. \[default=America/New_York\]
    #[serde(
        rename = "alignmentTimezone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub alignment_timezone: Option<String>,
    /// The day of the week used for granularities that have weekly alignment. \[default=Friday\]
    #[serde(
        rename = "weeklyAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub weekly_alignment: Option<WeeklyAlignment>,
}

/// latest_candles successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestCandlesResponse {
    /// The latest candle sticks.
    #[serde(
        rename = "latestCandles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_candles: Option<Vec<CandlestickResponse>>,
}

/// latest_candles documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LatestCandlesRejection {
    /// Provider error code.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Provider error message.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Latest candles.
    pub async fn latest_candles(
        &self,
        account_id: &AccountID,
        query: &LatestCandlesQuery,
    ) -> std::result::Result<
        crate::ApiResponse<LatestCandlesResponse>,
        crate::OperationError<LatestCandlesRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/candles/latest");
        self.execute(Method::GET, &path, Some(query), None::<&()>, None, None)
            .await
    }
}

/// pricing query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingQuery {
    /// List of Instruments to get pricing for. \[required\]
    #[serde(rename = "instruments")]
    pub instruments: Vec<InstrumentName>,
    /// Date/Time filter to apply to the response. Only prices and home conversions (if requested) with a time
    /// later than this filter (i.e. the price has changed after the since time) will be provided, and are
    /// filtered independently.
    #[serde(rename = "since", default, skip_serializing_if = "Option::is_none")]
    pub since: Option<Timestamp>,
    /// Flag that enables the inclusion of the unitsAvailable field in the returned Price objects.
    /// \[default=True\] Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "includeUnitsAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_units_available: Option<bool>,
    /// Flag that enables the inclusion of the homeConversions field in the returned response. An entry will be
    /// returned for each currency in the set of all base and quote currencies present in the requested
    /// instruments list. \[default=False\]
    #[serde(
        rename = "includeHomeConversions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_home_conversions: Option<bool>,
}

/// pricing successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingResponse {
    /// The list of Price objects requested.
    #[serde(rename = "prices")]
    pub prices: Vec<ClientPrice>,
    /// The list of home currency conversion factors requested. This field will only be present if
    /// includeHomeConversions was set to true in the request.
    #[serde(
        rename = "homeConversions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversions: Option<Vec<HomeConversions>>,
    /// The DateTime value to use for the “since” parameter in the next poll request.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
}

/// pricing documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PricingRejection {
    /// Provider error code.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Provider error message.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Pricing.
    pub async fn pricing(
        &self,
        account_id: &AccountID,
        query: &PricingQuery,
    ) -> std::result::Result<
        crate::ApiResponse<PricingResponse>,
        crate::OperationError<PricingRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/pricing");
        self.execute(Method::GET, &path, Some(query), None::<&()>, None, None)
            .await
    }
}

/// stream_pricing stream query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamPricingQuery {
    /// List of Instruments to stream Prices for. \[required\]
    #[serde(rename = "instruments")]
    pub instruments: Vec<InstrumentName>,
    /// Flag that enables/disables the sending of a pricing snapshot when initially connecting to the stream.
    /// \[default=True\]
    #[serde(rename = "snapshot", default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    /// Flag that enables the inclusion of the homeConversions field in the returned response. An entry will be
    /// returned for each currency in the set of all base and quote currencies present in the requested
    /// instruments list. \[default=False\]
    #[serde(
        rename = "includeHomeConversions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_home_conversions: Option<bool>,
}

/// instrument_candles query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentCandlesQuery {
    /// The Price component(s) to get candlestick data for. \[default=M\]
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<PricingComponent>,
    /// The granularity of the candlesticks to fetch \[default=S5\]
    #[serde(
        rename = "granularity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub granularity: Option<CandlestickGranularity>,
    /// The number of candlesticks to return in the response. Count should not be specified if both the start
    /// and end parameters are provided, as the time range combined with the granularity will determine the
    /// number of candlesticks to return. \[default=500, maximum=5000\]
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// The start of the time range to fetch candlesticks for.
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Timestamp>,
    /// The end of the time range to fetch candlesticks for.
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Timestamp>,
    /// A flag that controls whether the candlestick is “smoothed” or not. A smoothed candlestick uses the
    /// previous candle’s close price as its open price, while an unsmoothed candlestick uses the first price
    /// from its time range as its open price. \[default=False\]
    #[serde(rename = "smooth", default, skip_serializing_if = "Option::is_none")]
    pub smooth: Option<bool>,
    /// A flag that controls whether the candlestick that is covered by the from time should be included in the
    /// results. This flag enables clients to use the timestamp of the last completed candlestick received to
    /// poll for future candlesticks but avoid receiving the previous candlestick repeatedly. \[default=True\]
    #[serde(
        rename = "includeFirst",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_first: Option<bool>,
    /// The hour of the day (in the specified timezone) to use for granularities that have daily alignments.
    /// \[default=17, minimum=0, maximum=23\]
    #[serde(
        rename = "dailyAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_alignment: Option<i64>,
    /// The timezone to use for the dailyAlignment parameter. Candlesticks with daily alignment will be aligned
    /// to the dailyAlignment hour within the alignmentTimezone. Note that the returned times will still be
    /// represented in UTC. \[default=America/New_York\]
    #[serde(
        rename = "alignmentTimezone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub alignment_timezone: Option<String>,
    /// The day of the week used for granularities that have weekly alignment. \[default=Friday\]
    #[serde(
        rename = "weeklyAlignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub weekly_alignment: Option<WeeklyAlignment>,
    /// The number of units used to calculate the volume-weighted average bid and ask prices in the returned
    /// candles. \[default=1\]
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Decimal>,
}

/// instrument_candles successful response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentCandlesResponse {
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

/// instrument_candles documented rejection fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentCandlesRejection {
    /// Provider error code.
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Provider error message.
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
}

impl Client {
    /// Instrument candles.
    pub async fn instrument_candles(
        &self,
        account_id: &AccountID,
        instrument: &InstrumentName,
        query: Option<&InstrumentCandlesQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<InstrumentCandlesResponse>,
        crate::OperationError<InstrumentCandlesRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let instrument_encoded = crate::client::segment(instrument.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/instruments/{instrument_encoded}/candles");
        self.execute(Method::GET, &path, query, None::<&()>, None, None)
            .await
    }
}
