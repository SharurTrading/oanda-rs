<!--
SPDX-License-Identifier: MIT-0
-->

# oanda-rs

[![CI](https://github.com/SharurTrading/oanda-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/oanda-rs/actions/workflows/ci.yml)
[![MSRV: Rust 1.95.0](https://img.shields.io/badge/MSRV-Rust%201.95.0-blue.svg?logo=rust)](rust-toolchain.toml)
[![license: MIT-0](https://img.shields.io/badge/license-MIT--0-blue.svg)](LICENSE)
[![v20 operations: 32/32 typed](https://img.shields.io/badge/v20%20operations-32%2F32%20typed-brightgreen.svg)](docs/coverage.json)
[![live testing: not run](https://img.shields.io/badge/live%20testing-not%20run-orange.svg)](#testing-status)

> [!WARNING]
> **Pre-release:** no authenticated OANDA Practice or Live environment testing has been conducted.
> Validation currently consists of deterministic synthetic fixtures, loopback transport tests,
> and reviewed documentation and schema checks. Do not use this crate for live trading without
> independent validation.

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
| Live validation | Not run; three ignored, explicitly armed, read-only Practice probes are available |

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

async fn account_summary(token: String, account_id: &str)
    -> Result<(), Box<dyn std::error::Error>>
{
    let account = AccountID::new(account_id)?;
    let client = Client::builder(Environment::Practice, token).build()?;
    let snapshot = client.account_summary(&account).await?;
    println!("last transaction: {:?}", snapshot.last_transaction_id);
    Ok(())
}
```

See the runnable [read-only Practice example](examples/practice_account.rs). Select
`Environment::Live` only after validating the application and its account reconciliation flow.

## API and transport

- Account details, summary, instruments, configuration, and changes are available alongside order,
  trade, position, transaction, candle, and pricing methods. Responses include typed provider
  fields, typed rejection bodies, request IDs, and pagination metadata where documented.
- Prices, units, balances, and other financial values use exact `rust_decimal::Decimal` values
  parsed from OANDA decimal strings. Provider IDs and timestamps have dedicated types.
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
verification, and the offline coverage check. The 30 REST methods have deterministic loopback
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

No live mutation probe is part of acceptance. The current repository has **not** run these
Practice probes. See [coverage and contract drift](docs/coverage.md) for the reviewed source and
verification details.
