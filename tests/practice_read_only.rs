//! Explicitly armed, ignored Practice-only probes. No mutation is sent.
#![cfg(feature = "live-tests")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use oanda_client::pricing::{PricingQuery, StreamPricingQuery};
use oanda_client::{AccountID, Client, Environment, InstrumentName};
use std::time::Duration;

fn armed() -> Option<(Client, AccountID)> {
    if std::env::var("OANDA_READ_ONLY_PROBE").ok().as_deref() != Some("I_ACCEPT_READ_ONLY_PRACTICE")
    {
        return None;
    }
    let token = std::env::var("OANDA_TOKEN").ok()?;
    let account = AccountID::new(std::env::var("OANDA_ACCOUNT_ID").ok()?).ok()?;
    let client = Client::builder(Environment::Practice, token).build().ok()?;
    Some((client, account))
}

#[tokio::test]
#[ignore = "requires explicit Practice credentials and OANDA_READ_ONLY_PROBE"]
async fn practice_rest_queries() {
    let (client, account) = armed().expect("set Practice probe variables");
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

#[tokio::test]
#[ignore = "requires explicit Practice credentials and OANDA_READ_ONLY_PROBE"]
async fn practice_price_stream() {
    let (client, account) = armed().expect("set Practice probe variables");
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

#[tokio::test]
#[ignore = "requires explicit Practice credentials and OANDA_READ_ONLY_PROBE"]
async fn practice_transaction_stream() {
    let (client, account) = armed().expect("set Practice probe variables");
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
