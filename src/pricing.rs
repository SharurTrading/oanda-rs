//! OANDA pricing endpoint contracts.

use crate::client::RequestParts;
use crate::ids::{AccountID, CandleSpecification, InstrumentName, PricingComponent};
use crate::models::{
    Candlestick, CandlestickGranularity, CandlestickResponse, ClientPrice, HomeConversions,
    WeeklyAlignment,
};
use crate::validation::Validate;
use crate::{Client, Error, Result, Timestamp};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Query parameters for `latest_candles`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestCandlesQuery {
    /// List of candle specifications to get pricing for. \[required\]
    #[serde(rename = "candleSpecifications")]
    pub candle_specifications: Vec<CandleSpecification>,
    /// The number of units used to calculate the volume-weighted average bid and ask prices in the returned
    /// candles. \[default=1\]
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
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
    /// represented in UTC. \[`default=America/New_York`\]
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

impl Validate for LatestCandlesQuery {
    fn validate(&self) -> Result<()> {
        crate::validation::nonempty(&self.candle_specifications, "candleSpecifications")?;
        for specification in &self.candle_specifications {
            let mut parts = specification.as_str().split(':');
            let valid = match (parts.next(), parts.next(), parts.next(), parts.next()) {
                (Some(instrument), Some(granularity), Some(components), None) => {
                    valid_instrument(instrument)
                        && !granularity.is_empty()
                        && valid_components(components)
                }
                _ => false,
            };
            if !valid {
                return Err(Error::InvalidInput("invalid candle specification".into()));
            }
        }
        crate::validation::daily_alignment(self.daily_alignment)
    }
}

/// Successful response for `latest_candles`.
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

impl Client {
    /// Latest candles.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn latest_candles(
        &self,
        account_id: &AccountID,
        query: &LatestCandlesQuery,
    ) -> std::result::Result<
        crate::ApiResponse<LatestCandlesResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/candles/latest");
        self.execute(RequestParts::new(Method::GET, &path).query(query)?)
            .await
    }
}

/// Query parameters for `pricing`.
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

impl Validate for PricingQuery {
    fn validate(&self) -> Result<()> {
        crate::validation::nonempty(&self.instruments, "instruments")
    }
}

/// Successful response for `pricing`.
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
    /// The `DateTime` value to use for the “since” parameter in the next poll request.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,
}

impl Client {
    /// Pricing.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn pricing(
        &self,
        account_id: &AccountID,
        query: &PricingQuery,
    ) -> std::result::Result<
        crate::ApiResponse<PricingResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let path = format!("/v3/accounts/{account_id_encoded}/pricing");
        self.execute(RequestParts::new(Method::GET, &path).query(query)?)
            .await
    }
}

/// `stream_pricing` stream query parameters.
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

impl Validate for StreamPricingQuery {
    fn validate(&self) -> Result<()> {
        crate::validation::nonempty(&self.instruments, "instruments")
    }
}

/// Query parameters for `instrument_candles`.
///
/// The default uses the provider’s candle range, granularity, and alignment.
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
    /// represented in UTC. \[`default=America/New_York`\]
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
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub units: Option<Decimal>,
}

impl Validate for InstrumentCandlesQuery {
    fn validate(&self) -> Result<()> {
        if self
            .price
            .as_ref()
            .is_some_and(|price| !valid_components(price.as_str()))
        {
            return Err(Error::InvalidInput("invalid pricing component".into()));
        }
        crate::validation::count(self.count, 5000)?;
        crate::validation::daily_alignment(self.daily_alignment)
    }
}

/// Successful response for `instrument_candles`.
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

impl Client {
    /// Instrument candles.
    ///
    /// # Errors
    ///
    /// Returns [`crate::OperationError::Client`] for invalid input, admission, transport, or
    /// malformed response failures. Provider refusals return [`crate::OperationError::Rejected`]
    /// or [`crate::OperationError::RejectedUnreadable`] when the rejection body cannot be read.
    pub async fn instrument_candles(
        &self,
        account_id: &AccountID,
        instrument: &InstrumentName,
        query: Option<&InstrumentCandlesQuery>,
    ) -> std::result::Result<
        crate::ApiResponse<InstrumentCandlesResponse>,
        crate::OperationError<crate::GenericRejection>,
    > {
        let account_id_encoded = crate::client::segment(account_id.as_str());
        let instrument_encoded = crate::client::segment(instrument.as_str());
        let path =
            format!("/v3/accounts/{account_id_encoded}/instruments/{instrument_encoded}/candles");
        let request = RequestParts::new(Method::GET, &path);
        let request = match query {
            Some(query) => request.query(query)?,
            None => request,
        };
        self.execute(request).await
    }
}

fn valid_instrument(text: &str) -> bool {
    let Some((base, quote)) = text.split_once('_') else {
        return false;
    };
    !base.is_empty()
        && !quote.is_empty()
        && base
            .bytes()
            .chain(quote.bytes())
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
}

fn valid_components(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 3
        && text.bytes().all(|byte| matches!(byte, b'B' | b'A' | b'M'))
        && text
            .as_bytes()
            .iter()
            .enumerate()
            .all(|(index, byte)| !text.as_bytes()[..index].contains(byte))
}
