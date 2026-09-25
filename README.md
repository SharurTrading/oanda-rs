<!--
SPDX-License-Identifier: MIT-0
-->

# oanda-rs

[![CI](https://github.com/SharurTrading/oanda-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/oanda-rs/actions/workflows/ci.yml)
[![MSRV: Rust 1.95.0](https://img.shields.io/badge/MSRV-Rust%201.95.0-blue.svg?logo=rust)](rust-toolchain.toml)
[![license: MIT-0](https://img.shields.io/badge/license-MIT--0-blue.svg)](LICENSE)
[![v20 operations: 32/32 typed](https://img.shields.io/badge/v20%20operations-32%2F32%20typed-brightgreen.svg)](docs/coverage.json)
[![Practice probes: 3/3 passed](https://img.shields.io/badge/Practice%20probes-3%2F3%20passed-brightgreen.svg)](#testing-status)

> [!WARNING]
> **Pre-release:** three authenticated, read-only OANDA Practice probes passed on 2026-09-25.
> A separate, one-off Practice limit-order creation and cancellation probe also passed. No Live
> environment has been tested. Do not use this crate for live trading without independent
> validation.

An async, provider-native Rust client for the [OANDA REST-v20 API](https://developer.oanda.com/rest-live-v20/introduction/).
The Cargo package and import name are `oanda-client` and `oanda_client`; `-rs` belongs to the
repository name. This is an independent, unofficial client, unaffiliated with OANDA. It is designed
as an inner provider client that SharurPlatform can later consume through its own adapter. The
adapter is outside this repository.

The project is licensed under [MIT No Attribution (MIT-0)](LICENSE). The minimum supported Rust
version is **1.95.0**.

## Status

The six OANDA v20 endpoint pages and their linked definitions, reviewed on 2026-09-25, are the contract authority.
The pinned [official OpenAPI revision](docs/coverage.md) is a cross-check; documented differences
are recorded rather than silently adopting older operations.

| Capability | Current surface |
| --- | --- |
| Documented operations | 32/32 callable: 30 REST methods and 2 HTTP streams |
| Documented definitions | 168/168 typed contracts |
| Variants | 503 scalar enum values and 55 tagged order, request, and transaction variants tested locally |
| Environments | Paired Practice and Live REST and stream hosts |
| Runtime and credentials | Caller-owned Tokio runtime and injected bearer token |
| Authenticated validation | Three read-only Practice probes and one temporary limit-order creation/cancellation probe passed on 2026-09-25; no Live environment testing |

The exhaustive [coverage ledger](docs/coverage.json) names each operation and definition, its
source URL, public method or type, and local test. The [coverage notes](docs/coverage.md) explain
the website/OpenAPI differences and the offline CI check.

## Installation

Until the package is published, consume a local checkout:

```toml
[dependencies]
oanda-client = { path = "../oanda-rs" }
tokio = { version = "1", features = ["macros", "rt"] }
```

The crate uses Rust edition 2024 and exposes capability modules for accounts, orders, trades,
positions, transactions, pricing, instruments, transport, and primitives.

## Quick start

The caller obtains and stores the token. The library does not read `.env` files or discover
credentials from ambient process state.

```rust
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
```

This is the runnable [read-only Practice example](examples/practice_account.rs). With those two
environment variables set, run `cargo run --example practice_account`. Select
`Environment::Live` only after validating the application and its account reconciliation flow.

## API and transport

- Account details, summary, instruments, configuration, and changes are available alongside order,
  trade, position, transaction, candle, and pricing methods. Responses include typed provider
  fields, typed rejection bodies, request IDs, and pagination metadata where documented.
- Prices, units, balances, and other financial values use exact `rust_decimal::Decimal` values
  parsed from OANDA decimal strings. Price-bucket liquidity also accepts JSON numbers exactly.
  Provider IDs and timestamps have dedicated types.
- REST response bodies are bounded. Cloned clients share conservative rate admission and provider
  cooldown. Requests are single-attempt; a mutation is never automatically retried.
- An ambiguous mutation fences further mutations for its account across client clones. Read OANDA
  account and transaction state before calling `acknowledge_reconciliation`.
- Pricing and transaction streams are incremental, bounded, newline-delimited HTTP streams with
  typed heartbeats. Malformed records, oversized records, timeouts, and connection loss end that
  stream generation and require caller-owned recovery. The library does not reconnect or maintain
  account and price projections.
- OANDA's [pricing stream](https://developer.oanda.com/rest-live-v20/pricing-ep/) is sampled at most
  four prices per second per instrument. It does not carry every price update.

Read [SECURITY.md](SECURITY.md) before handling credentials or mutations.

## Testing status

Normal CI needs no OANDA credentials. It runs formatting, Clippy, tests, documentation, package
verification, and the offline coverage check on Rust 1.95. Clippy uses pedantic warnings as errors.
The manual release-readiness workflow checks the requested version and runs the same gates plus a
crates.io publish dry run; it does not publish a release. Dependabot checks Cargo and GitHub Actions
updates weekly. The 30 REST methods have deterministic loopback
success and rejection fixtures, including method, URL, query, header, and mutation-body checks.
Both streams have local framing, heartbeat, error, and gap tests. These tests establish the
documented wire surface; they do not establish behavior against an authenticated OANDA account.

Ignored, explicitly armed **read-only Practice** probes cover representative REST queries and
both streams:

```sh
OANDA_READ_ONLY_PROBE=I_ACCEPT_READ_ONLY_PRACTICE \
OANDA_TOKEN=... OANDA_ACCOUNT_ID=... \
cargo test --features live-tests --test practice_read_only -- --ignored
```

All three read-only Practice probes passed on 2026-09-25. A separate, temporary probe placed a
one-unit EUR/USD limit order well below the current bid, observed a price and heartbeat on the
pricing stream, received creation and cancellation updates on the transaction stream, and verified
the order's final state was `CANCELLED`. The temporary probe was removed; no mutation probe runs in
CI or remains in this repository. See [coverage and contract drift](docs/coverage.md) for the
reviewed source and verification details.
