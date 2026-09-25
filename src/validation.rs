//! Provider-specific request checks before a request can reach the network.

use crate::{Error, Result, Timestamp};
use rust_decimal::Decimal;
use serde_json::Value;
use std::str::FromStr;

pub(crate) fn validate(path: &str, query: Option<&Value>, body: Option<&Value>) -> Result<()> {
    if let Some(query) = query {
        for key in ["instruments", "candleSpecifications", "ids"] {
            if query
                .get(key)
                .and_then(Value::as_array)
                .is_some_and(Vec::is_empty)
            {
                return Err(Error::InvalidInput(format!("{key} must not be empty")));
            }
        }
        if let Some(specs) = query.get("candleSpecifications").and_then(Value::as_array) {
            for spec in specs {
                let text = spec
                    .as_str()
                    .ok_or_else(|| Error::InvalidInput("invalid candle specification".into()))?;
                let parts: Vec<_> = text.split(':').collect();
                if parts.len() != 3
                    || !valid_instrument(parts[0])
                    || parts[1].is_empty()
                    || !valid_components(parts[2])
                {
                    return Err(Error::InvalidInput("invalid candle specification".into()));
                }
            }
        }
        if let Some(price) = query.get("price").and_then(Value::as_str)
            && !valid_components(price)
        {
            return Err(Error::InvalidInput("invalid pricing component".into()));
        }
        if let Some(count) = query.get("count").and_then(Value::as_i64) {
            let maximum = if path.ends_with("/candles") {
                5000
            } else {
                500
            };
            if !(1..=maximum).contains(&count) {
                return Err(Error::InvalidInput(format!("count must be 1..={maximum}")));
            }
        }
        if let Some(size) = query.get("pageSize").and_then(Value::as_i64)
            && !(1..=1000).contains(&size)
        {
            return Err(Error::InvalidInput("pageSize must be 1..=1000".into()));
        }
        if let Some(hour) = query.get("dailyAlignment").and_then(Value::as_i64)
            && !(0..=23).contains(&hour)
        {
            return Err(Error::InvalidInput("dailyAlignment must be 0..=23".into()));
        }
        if path.ends_with("/transactions")
            && let (Some(from), Some(to)) = (
                query.get("from").and_then(Value::as_str),
                query.get("to").and_then(Value::as_str),
            )
        {
            let from = Timestamp::from_str(from)
                .map_err(Error::InvalidInput)?
                .into_inner();
            let to = Timestamp::from_str(to)
                .map_err(Error::InvalidInput)?
                .into_inner();
            if to < from || to.signed_duration_since(from) > chrono::Duration::days(365) {
                return Err(Error::InvalidInput(
                    "transaction range must be ordered and at most 365 days".into(),
                ));
            }
        }
        if path.ends_with("/transactions/idrange") {
            let from = query
                .get("from")
                .and_then(Value::as_str)
                .and_then(|x| x.parse::<u64>().ok());
            let to = query
                .get("to")
                .and_then(Value::as_str)
                .and_then(|x| x.parse::<u64>().ok());
            if !matches!((from,to), (Some(a),Some(b)) if a<=b) {
                return Err(Error::InvalidInput("invalid transaction ID range".into()));
            }
        }
    }
    if let Some(body) = body {
        let object = body
            .as_object()
            .ok_or_else(|| Error::InvalidInput("mutation body must be an object".into()))?;
        if object.is_empty() && !(path.contains("/trades/") && path.ends_with("/close")) {
            return Err(Error::InvalidInput(
                "mutation body must not be empty".into(),
            ));
        }
        if (path.ends_with("/orders") && !path.contains("/trades/"))
            || (path.contains("/orders/") && !path.ends_with("/clientExtensions"))
        {
            if let Some(order) = object.get("order") {
                validate_order(order)?;
            } else if path.ends_with("/orders") || !path.ends_with("/cancel") {
                return Err(Error::InvalidInput("order request is required".into()));
            }
        }
        if path.contains("/trades/")
            && path.ends_with("/close")
            && let Some(units) = object.get("units").and_then(Value::as_str)
        {
            validate_units(units, &["ALL"])?;
        }
        if path.contains("/positions/") && path.ends_with("/close") {
            let long = object.get("longUnits").and_then(Value::as_str);
            let short = object.get("shortUnits").and_then(Value::as_str);
            if long.is_none() && short.is_none() {
                return Err(Error::InvalidInput(
                    "position close needs longUnits or shortUnits".into(),
                ));
            }
            if let Some(units) = long {
                validate_units(units, &["ALL", "NONE"])?;
            }
            if let Some(units) = short {
                validate_units(units, &["ALL", "NONE"])?;
            }
            if long == Some("NONE") && short == Some("NONE") {
                return Err(Error::InvalidInput(
                    "position close requests no action".into(),
                ));
            }
        }
        if path.ends_with("/configuration")
            && let Some(rate) = object.get("marginRate")
        {
            let rate = rate
                .as_str()
                .ok_or_else(|| Error::InvalidInput("invalid marginRate".into()))?
                .parse::<Decimal>()
                .map_err(|_| Error::InvalidInput("invalid marginRate".into()))?;
            if rate <= Decimal::ZERO || rate > Decimal::ONE {
                return Err(Error::InvalidInput("marginRate must be in (0,1]".into()));
            }
        }
    }
    Ok(())
}

fn validate_order(order: &Value) -> Result<()> {
    let object = order
        .as_object()
        .ok_or_else(|| Error::InvalidInput("order must be an object".into()))?;
    let kind = object
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::InvalidInput("order type is required".into()))?;
    if !matches!(
        kind,
        "MARKET"
            | "LIMIT"
            | "STOP"
            | "MARKET_IF_TOUCHED"
            | "TAKE_PROFIT"
            | "STOP_LOSS"
            | "GUARANTEED_STOP_LOSS"
            | "TRAILING_STOP_LOSS"
    ) {
        return Err(Error::InvalidInput("unsupported order request type".into()));
    }
    if let Some(units) = object.get("units").and_then(Value::as_str) {
        let amount = units
            .parse::<Decimal>()
            .map_err(|_| Error::InvalidInput("invalid order units".into()))?;
        if amount.is_zero() {
            return Err(Error::InvalidInput("order units must not be zero".into()));
        }
    }
    let tif = object.get("timeInForce").and_then(Value::as_str);
    if let Some(tif) = tif {
        if !matches!(tif, "GTC" | "GTD" | "GFD" | "FOK" | "IOC") {
            return Err(Error::InvalidInput("unsupported timeInForce".into()));
        }
        if kind == "MARKET" && !matches!(tif, "FOK" | "IOC") {
            return Err(Error::InvalidInput(
                "market order requires FOK or IOC".into(),
            ));
        }
        if kind != "MARKET" && matches!(tif, "FOK" | "IOC") {
            return Err(Error::InvalidInput(
                "non-market order cannot use FOK or IOC".into(),
            ));
        }
        if tif == "GTD" && object.get("gtdTime").is_none() {
            return Err(Error::InvalidInput("GTD order requires gtdTime".into()));
        }
    }
    for (field, values) in [
        (
            "positionFill",
            &["OPEN_ONLY", "REDUCE_FIRST", "REDUCE_ONLY", "DEFAULT"][..],
        ),
        (
            "triggerCondition",
            &["DEFAULT", "INVERSE", "BID", "ASK", "MID"][..],
        ),
    ] {
        if let Some(value) = object.get(field).and_then(Value::as_str)
            && !values.contains(&value)
        {
            return Err(Error::InvalidInput(format!("unsupported {field}")));
        }
    }
    Ok(())
}

fn validate_units(units: &str, special: &[&str]) -> Result<()> {
    if special.contains(&units) {
        return Ok(());
    }
    let value = units
        .parse::<Decimal>()
        .map_err(|_| Error::InvalidInput("invalid close units".into()))?;
    if value <= Decimal::ZERO {
        return Err(Error::InvalidInput("close units must be positive".into()));
    }
    Ok(())
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
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
}

fn valid_components(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 3
        && text.bytes().all(|b| matches!(b, b'B' | b'A' | b'M'))
        && {
            let bytes = text.as_bytes();
            bytes
                .iter()
                .enumerate()
                .all(|(i, b)| !bytes[..i].contains(b))
        }
}
