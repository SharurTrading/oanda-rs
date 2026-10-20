//! Typed request admission before serialization, rate admission, or network access.

use crate::{Error, Result};

/// Validate provider constraints on an outgoing request without changing its evidence.
pub(crate) trait Validate {
    fn validate(&self) -> Result<()>;
}

// Bodyless operations and streams have no payload constraints.
impl Validate for () {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn nonempty<T>(values: &[T], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(Error::InvalidInput(format!("{field} must not be empty")));
    }
    Ok(())
}

pub(crate) fn count(value: Option<i64>, maximum: i64) -> Result<()> {
    if value.is_some_and(|value| !(1..=maximum).contains(&value)) {
        return Err(Error::InvalidInput(format!("count must be 1..={maximum}")));
    }
    Ok(())
}

pub(crate) fn daily_alignment(value: Option<i64>) -> Result<()> {
    if value.is_some_and(|value| !(0..=23).contains(&value)) {
        return Err(Error::InvalidInput("dailyAlignment must be 0..=23".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::account::ConfigureAccountBody;
    use crate::positions::ClosePositionBody;
    use crate::{
        AccountID, Client, Environment, Error, InstrumentName, OperationError,
        PositionCloseoutUnits,
    };
    use rust_decimal::Decimal;

    #[tokio::test(start_paused = true)]
    async fn invalid_typed_mutations_preserve_input_and_admission_capacity()
    -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = reqwest::Url::parse("http://127.0.0.1:1/")?;
        let client = Client::builder(Environment::Practice, "synthetic")
            .endpoints(endpoint.clone(), endpoint)
            .build()?;
        let account = AccountID::new("synthetic-account")?;
        let instrument = InstrumentName::new("EUR_USD")?;
        let configuration = ConfigureAccountBody {
            alias: None,
            margin_rate: Some(Decimal::TWO),
        };
        let position = ClosePositionBody {
            long_units: Some(PositionCloseoutUnits::Units(Decimal::ZERO)),
            short_units: Some(PositionCloseoutUnits::None),
            long_client_extensions: None,
            short_client_extensions: None,
        };
        let original_configuration = configuration.clone();
        let original_position = position.clone();
        assert!(matches!(
            client.configure_account(&account, &configuration).await,
            Err(OperationError::Client(Error::InvalidInput(_)))
        ));
        assert!(matches!(
            client
                .close_position(&account, &instrument, &position)
                .await,
            Err(OperationError::Client(Error::InvalidInput(_)))
        ));
        assert_eq!(configuration, original_configuration);
        assert_eq!(position, original_position);
        for _ in 0..crate::admission::REQUESTS_PER_SECOND {
            client.admit(true, false).await?;
        }
        assert!(matches!(
            client.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        Ok(())
    }
}
