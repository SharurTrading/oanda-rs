// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Wire financial tokens are never rounded during decoding.
use oanda_client::models::{CandlestickData, Instrument, PriceBucket};

#[test]
fn nonrepresentable_financial_tokens_are_refused() {
    for token in [
        "0.12345678901234567890123456789",
        "0.00000000000000000000000000001",
        "79228162514264337593543950336",
    ] {
        for key in ["price", "liquidity"] {
            let json = format!(r#"{{"{key}":"{token}"}}"#);
            assert!(serde_json::from_str::<PriceBucket>(&json).is_err());
        }
        assert!(serde_json::from_str::<CandlestickData>(&format!(r#"{{"o":"{token}"}}"#)).is_err());
        assert!(
            serde_json::from_str::<Instrument>(&format!(r#"{{"minimumTradeSize":"{token}"}}"#))
                .is_err()
        );
    }
}

#[test]
fn decimal_strings_and_numeric_liquidity_keep_their_exact_value() -> Result<(), serde_json::Error> {
    let value: PriceBucket =
        serde_json::from_str(r#"{"price":"-0.125","liquidity":0.0000000000000000000000000001}"#)?;
    assert_eq!(value.price, Some(rust_decimal::Decimal::new(-125, 3)));
    assert_eq!(value.liquidity, Some(rust_decimal::Decimal::new(1, 28)));
    let value: PriceBucket =
        serde_json::from_str(r#"{"price":"79228162514264337593543950335","liquidity":1e3}"#)?;
    assert_eq!(value.price, Some(rust_decimal::Decimal::MAX));
    assert_eq!(value.liquidity, Some(rust_decimal::Decimal::from(1000)));
    Ok(())
}
