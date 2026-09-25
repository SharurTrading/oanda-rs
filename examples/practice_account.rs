//! Read-only account snapshot; run with `OANDA_TOKEN` and `OANDA_ACCOUNT_ID` set.
use oanda_client::{AccountID, Client, Environment};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("OANDA_TOKEN")?;
    let account = AccountID::new(std::env::var("OANDA_ACCOUNT_ID")?)?;
    let client = Client::builder(Environment::Practice, token).build()?;
    let summary = client.account_summary(&account).await?;
    println!("request ID: {:?}", summary.request_id);
    println!("last transaction: {:?}", summary.last_transaction_id);
    Ok(())
}
