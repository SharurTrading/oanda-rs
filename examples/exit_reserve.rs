// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Enables, changes and releases shared exit capacity without making requests.

use oanda_client::{Client, Environment, Result};

fn configured_client(token: String, reserve_requests: u32) -> Result<Client> {
    Client::builder(Environment::Practice, token)
        .exit_reserve(reserve_requests)
        .build()
}

fn main() -> Result<()> {
    let client = configured_client("fixture-token".into(), 20)?;
    let reads = client.clone();
    reads.set_exit_reserve(40)?;
    client.set_exit_reserve(0)?;
    client.set_exit_reserve(20)?;
    match client.set_exit_reserve(120) {
        Err(oanda_client::Error::InvalidInput(_)) => Ok(()),
        _ => Err(oanda_client::Error::InvalidInput(
            "full-window reserve was accepted".into(),
        )),
    }
}
