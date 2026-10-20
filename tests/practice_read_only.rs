// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicitly armed, ignored Practice-only probes. No mutation is sent.
#![cfg(feature = "live-tests")]
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]

use oanda_client::pricing::{PricingQuery, StreamPricingQuery};
use oanda_client::{AccountID, Client, Environment, InstrumentName};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
enum ArmError {
    MissingToken,
    InvalidTokenEncoding,
    MissingAccount,
    InvalidAccountEncoding,
    InvalidAccount,
    InvalidClient,
}

impl std::fmt::Display for ArmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MissingToken => "armed Practice probe requires OANDA_TOKEN",
            Self::InvalidTokenEncoding => "armed Practice probe requires a UTF-8 OANDA_TOKEN",
            Self::MissingAccount => "armed Practice probe requires OANDA_ACCOUNT_ID",
            Self::InvalidAccountEncoding => {
                "armed Practice probe requires a UTF-8 OANDA_ACCOUNT_ID"
            }
            Self::InvalidAccount => "armed Practice probe has an invalid OANDA_ACCOUNT_ID",
            Self::InvalidClient => "armed Practice probe client configuration is invalid",
        })
    }
}

fn armed_with(
    mut variable: impl FnMut(&str) -> Result<String, std::env::VarError>,
) -> Result<Option<(Client, AccountID)>, ArmError> {
    if variable("OANDA_READ_ONLY_PROBE").ok().as_deref() != Some("I_ACCEPT_READ_ONLY_PRACTICE") {
        return Ok(None);
    }
    let token = variable("OANDA_TOKEN").map_err(|error| match error {
        std::env::VarError::NotPresent => ArmError::MissingToken,
        std::env::VarError::NotUnicode(_) => ArmError::InvalidTokenEncoding,
    })?;
    let account = variable("OANDA_ACCOUNT_ID").map_err(|error| match error {
        std::env::VarError::NotPresent => ArmError::MissingAccount,
        std::env::VarError::NotUnicode(_) => ArmError::InvalidAccountEncoding,
    })?;
    let account = AccountID::new(account).map_err(|_| ArmError::InvalidAccount)?;
    let client = Client::builder(Environment::Practice, token)
        .build()
        .map_err(|_| ArmError::InvalidClient)?;
    Ok(Some((client, account)))
}

fn armed() -> (Client, AccountID) {
    armed_with(|name| std::env::var(name))
        .unwrap_or_else(|reason| panic!("{reason}"))
        .expect("set OANDA_READ_ONLY_PROBE=I_ACCEPT_READ_ONLY_PRACTICE to arm this probe")
}

#[test]
fn unarmed_probe_and_invalid_armed_configuration_are_distinct() {
    assert!(
        armed_with(|_| Err(std::env::VarError::NotPresent))
            .expect("unarmed")
            .is_none()
    );
    for (token, account, expected) in [
        (None, None, ArmError::MissingToken),
        (Some("fixture-secret"), None, ArmError::MissingAccount),
        (
            Some("fixture-secret"),
            Some("invalid-account-secret\n"),
            ArmError::InvalidAccount,
        ),
        (Some(""), Some("101-001-1-001"), ArmError::InvalidClient),
    ] {
        let error = armed_with(|name| match name {
            "OANDA_READ_ONLY_PROBE" => Ok("I_ACCEPT_READ_ONLY_PRACTICE".to_owned()),
            "OANDA_TOKEN" => token
                .map(str::to_owned)
                .ok_or(std::env::VarError::NotPresent),
            "OANDA_ACCOUNT_ID" => account
                .map(str::to_owned)
                .ok_or(std::env::VarError::NotPresent),
            _ => panic!("unexpected environment key"),
        })
        .expect_err("malformed armed probe must fail");
        assert_eq!(error, expected);
        let diagnostic = format!("{error}: {error:?}");
        assert!(!diagnostic.contains("fixture-secret"));
        assert!(!diagnostic.contains("invalid-account-secret"));
    }
}

#[ignore = "requires explicit Practice credentials and OANDA_READ_ONLY_PROBE"]
#[tokio::test]
async fn practice_rest_queries() {
    let (client, account) = armed();
    client.list_accounts().await.expect("account list");
    client.account_summary(&account).await.expect("summary");
    let query = PricingQuery {
        instruments: vec![InstrumentName::new("EUR_USD").expect("instrument")],
        since: None,
        include_units_available: None,
        include_home_conversions: None,
    };
    client.pricing(&account, &query).await.expect("pricing");
}

#[ignore = "requires explicit Practice credentials and OANDA_READ_ONLY_PROBE"]
#[tokio::test]
async fn practice_price_stream() {
    let (client, account) = armed();
    let query = StreamPricingQuery {
        instruments: vec![InstrumentName::new("EUR_USD").expect("instrument")],
        snapshot: Some(true),
        include_home_conversions: None,
    };
    let mut stream = client
        .stream_pricing(&account, &query)
        .await
        .expect("price stream");
    tokio::time::timeout(Duration::from_secs(15), stream.next_event())
        .await
        .expect("price stream timed out")
        .expect("stream ended")
        .expect("stream record");
}

#[ignore = "requires explicit Practice credentials and OANDA_READ_ONLY_PROBE"]
#[tokio::test]
async fn practice_transaction_stream() {
    let (client, account) = armed();
    let mut stream = client
        .stream_transactions(&account)
        .await
        .expect("transaction stream");
    tokio::time::timeout(Duration::from_secs(15), stream.next_event())
        .await
        .expect("transaction stream timed out")
        .expect("stream ended")
        .expect("stream record");
}
