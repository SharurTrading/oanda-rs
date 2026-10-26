<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# oanda-rs

[![CI](https://github.com/SharurTrading/oanda-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/oanda-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/oanda-client.svg)](https://crates.io/crates/oanda-client)
[![docs.rs](https://img.shields.io/docsrs/oanda-client)](https://docs.rs/oanda-client)
[![MSRV: Rust 1.95.0](https://img.shields.io/badge/MSRV-Rust%201.95.0-blue.svg?logo=rust)](https://github.com/SharurTrading/oanda-rs/blob/main/rust-toolchain.toml)
[![license: MIT-0](https://img.shields.io/badge/license-MIT--0-blue.svg)](https://github.com/SharurTrading/oanda-rs/blob/main/LICENSE)
[![v20 operations: 32/32 typed](https://img.shields.io/badge/v20%20operations-32%2F32%20typed-brightgreen.svg)](https://github.com/SharurTrading/oanda-rs/blob/main/docs/coverage.json)
[![Offline tests: 124 passed locally](https://img.shields.io/badge/offline%20tests-124%20passed%20locally-brightgreen.svg)](#testing-status)
[![Practice probes: 3 passed on 2026-09-25](https://img.shields.io/badge/Practice%20probes-3%20passed%20%7C%202026--09--25-brightgreen.svg)](#testing-status)
[![Practice observations: recorded October 2026](https://img.shields.io/badge/Practice%20observations-recorded%20October%202026-blue.svg)](https://github.com/SharurTrading/oanda-rs/blob/main/docs/behaviour-guide.md)

> [!IMPORTANT]
> Read the [behaviour guide](https://github.com/SharurTrading/oanda-rs/blob/main/docs/behaviour-guide.md) before wrapping this client. It explains
> OANDA and SDK behaviour that matters to a wrapper: account modes, limits, stream recovery,
> closes and stated PnL.

An async, provider-native Rust client for the [OANDA REST-v20 API](https://developer.oanda.com/rest-live-v20/introduction/).
The Cargo package and import name are `oanda-client` and `oanda_client`; `-rs` belongs to the
repository name. This is an independent, unofficial client, unaffiliated with OANDA. It is designed
as an inner provider client that `SharurPlatform` can later consume through its own adapter. The
adapter is outside this repository.

The project is licensed under [MIT No Attribution (MIT-0)](https://github.com/SharurTrading/oanda-rs/blob/main/LICENSE). The minimum supported Rust
version is **1.95.0**.

Version **3.0.0** follows Semantic Versioning and contains source-breaking API corrections
from the Rust idioms audit; see [Upgrading from 2.x](#upgrading-from-2x). Public API changes
that require downstream source changes use a new major version; additive APIs and fixes use
minor and patch releases.
Provider contract changes can still require callers to update operational behavior, so review the
[changelog](https://github.com/SharurTrading/oanda-rs/blob/main/CHANGELOG.md) before upgrading and keep recovery around ambiguous money-moving
outcomes.

## Status

The six OANDA v20 endpoint pages and their linked definitions, reviewed on 2026-09-25, are the contract authority.
The pinned [official `OpenAPI` revision](https://github.com/SharurTrading/oanda-rs/blob/main/docs/coverage.md) is a cross-check; documented differences
are recorded rather than silently adopting older operations.

| Capability | Current surface |
| --- | --- |
| Documented operations | 32/32 callable: 30 REST methods and 2 HTTP streams |
| Documented definitions | 168/168 typed contracts |
| Variants | 503 scalar enum values and 55 tagged order, request, and transaction variants tested locally |
| Environments | Paired Practice and Live REST and stream hosts |
| Runtime and credentials | Caller-owned Tokio runtime and injected bearer token |
| Authenticated validation | Three read-only Practice probes and one temporary limit-order creation/cancellation probe passed on 2026-09-25; no Live environment testing |

The exhaustive [coverage ledger](https://github.com/SharurTrading/oanda-rs/blob/main/docs/coverage.json) names each operation and definition, its
source URL, public method or type, and local test. The [coverage notes](https://github.com/SharurTrading/oanda-rs/blob/main/docs/coverage.md) explain
the website/`OpenAPI` differences, the offline CI check, and the rejection-evidence contract. The
[LAW-INVARIANT audit](https://github.com/SharurTrading/oanda-rs/blob/main/docs/law-invariant-audit.md) records the semantic review of fabricated
fallbacks and swallowed failures, with a disposition for every candidate.

## Installation

```sh
cargo add oanda-client@3
```

Or add the current major release directly:

```toml
[dependencies]
oanda-client = "3"
```

The complete public API is available on [docs.rs](https://docs.rs/oanda-client).

The crate uses Rust edition 2024 and exposes capability modules for accounts, orders, trades,
positions, transactions, pricing, instruments, transport, and primitives.

## Upgrading from 2.x

Version 3 requires downstream source changes. Review the full
[3.0.0 changelog](https://github.com/SharurTrading/oanda-rs/blob/main/CHANGELOG.md#300---2026-10-20)
before upgrading, especially request validation and definitive rejection handling.

| 2.x usage | 3.0.0 migration |
| --- | --- |
| Access response fields through `ApiResponse<T>` dereferencing | Access the decoded value through `.body` |
| String close quantities | Use `TradeCloseoutUnits::{All, Units}` or `PositionCloseoutUnits::{All, None, Units}` with exact `Decimal` quantities |
| Position close request with an omitted side | Set that side explicitly to `PositionCloseoutUnits::None`; omission means `ALL` for each side |
| Construct an endpoint rejection with `Default` or an optional message | Supply its required `error_message: String`; generic rejection reasons remain optional |
| Match mutation errors with string accounts or response metadata with string request IDs | Handle `AccountID` and `Supplied<RequestID>`; use `.as_str()` when a borrowed spelling is needed |
| Construct the affected transaction client trade IDs or fixed-price trade states as strings | Use `ClientID` and the preserving `TradeState` enum |
| Construct or pattern-match the `InvalidId` unit struct | Match `error.kind()` against `InvalidIdKind`; recover the rejected spelling with `value()` or `into_value()` |
| Treat a timestamp parse error as `String` | Handle the typed `InvalidTimestamp`; more than nine fractional digits now fail explicitly |
| Exhaustively match `OperationError` | Handle `RejectedUnreadable { status, request_id, source }` as a definitive refusal with unreadable body evidence |

Outgoing typed requests are validated before consuming admission capacity or sending data.
Contradictory type tags and price/distance instructions fail explicitly. Serialize
`Patch::Unchanged` only through an enclosing field that omits it; `Patch::Clear` remains the
explicit cancellation instruction. Nonpositive outgoing close quantities are rejected while
signed response quantities remain preserved. Definitive rejection body-read failures no
longer fence an account, and decoded mutation successes retain their bodies when pagination
metadata cannot be read. The recovery procedure for ambiguous mutations still applies.

## Quick start

The caller obtains and stores the token. The library does not read `.env` files or discover
credentials from ambient process state.

This example needs Practice credentials and network access, so its doctest compiles without running.

```rust,no_run
use oanda_client::{AccountID, Client, Environment};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("OANDA_TOKEN")?;
    let account = AccountID::new(std::env::var("OANDA_ACCOUNT_ID")?)?;
    let client = Client::builder(Environment::Practice, token).build()?;
    let summary = client.account_summary(&account).await?;
    println!("request ID: {:?}", summary.request_id);
    println!("last transaction: {:?}", summary.body.last_transaction_id);
    Ok(())
}
```

This is the runnable [read-only Practice example](https://github.com/SharurTrading/oanda-rs/blob/main/examples/practice_account.rs). With those two
environment variables set, run `cargo run --example practice_account`. Select
`Environment::Live` only after validating the application and its account reconciliation flow.

## API and transport

- Account details, summary, instruments, configuration, and changes are available alongside order,
  trade, position, transaction, candle, and pricing methods. Responses include typed provider
  fields, typed rejection bodies, request IDs, and pagination metadata where documented.
- A definitive rejection reports only what OANDA sent. Its `code` and `message` are `None` when
  OANDA supplied no readable reason, and its typed `body` is a `Supplied<T>` that separates a body
  OANDA did not send from one that arrived and could not be decoded. The client never invents a
  reason. Endpoint rejection bodies require their documented `errorMessage`; an unreadable or
  oversized definitive rejection body returns `RejectedUnreadable` with its status, observed
  request ID, and typed read error. These definitive outcomes do not fence the account.
  Upgrading from an earlier build: `Rejected::message` and `Error::Provider::message` are
  now `Option<String>`, `Rejected::body` is `Supplied<R>`, and `acknowledge_reconciliation`
  returns `Result` — match arms and acknowledgement call sites need updating. A response's
  `request_id` and `next_page` are likewise `Supplied`: `Undecoded` means OANDA offered a value
  this client could not read, which is a different fact from no value being offered.
- Redirects are never followed, so the bearer token is only sent to the selected environment's
  hosts. An HTTP 3xx answer to a REST call or stream open is `Error::Redirected`, carrying its
  status, `RequestID` and a `RedirectTarget` reduced to the `Location` scheme, host, port and
  path; userinfo, query and fragment are discarded. A redirected mutation was not executed, so it
  does not fence the account.
- Every HTTP answer reported as an error (`Rejected`, `RejectedUnreadable`, `Error::Provider`,
  `Error::Redirected`, `Error::AmbiguousMutation`, `Error::ProviderRateLimited` and its invariant
  variant) carries `ResponseDiagnostics`: the `Server`, `CF-RAY` and `cf-mitigated` headers as
  received, so a caller can tell an edge network's answer, such as a bot-management challenge, from
  OANDA's own. `AmbiguousMutation` also carries the status that left it uncertain, or `None` when no
  response arrived. The client only reports these; it reads no cookies and never acts on a
  challenge.
- Prices, units, balances, and other financial values use exact `rust_decimal::Decimal` values
  parsed from OANDA decimal strings. Price-bucket liquidity also accepts JSON numbers exactly.
  Required and optional financial fields reject nonrepresentable coefficient/scale values
  during deserialization rather than rounding them.
  Provider IDs and timestamps have dedicated types.
- REST response bodies are bounded. Clients built with the same `Admission` owner share
  local rate admission and provider cooldown, which honours both `Retry-After` forms RFC
  9110 defines. Clones also share that owner. Requests are single-attempt; a mutation is never
  automatically retried.
- Caller-injected token strings are redacted and zeroized when their builder or last client
  owner drops. This covers the library-owned string, not copies owned by the caller or transport.
- An ambiguous mutation fences further mutations for its account across client clones. Read OANDA
  account and transaction state before calling `acknowledge_reconciliation`, which reports whether
  it released the fence.
- Pricing and transaction streams use HTTP/1.1, including when a consumer enables HTTP/2
  on reqwest through Cargo feature unification. REST protocol negotiation is independent.
  The streams are incremental and newline-delimited, with typed heartbeats. Records are bounded individually by a local hostile-input limit of 1 MiB, and the
  buffered backlog by a local cap of 16 MiB that ends the generation explicitly rather than evicting
  records already accepted. HTTP chunk segmentation is a transport read, not a provider record
  boundary, so a large chunk carrying many valid records is processed in order rather than
  rejected, and buffered records are never evicted for a slow caller. Malformed or oversized records, timeouts, and connection loss end that stream
  generation and require caller-owned recovery. The library does not reconnect or maintain account
  and price projections.
- `HttpStream::close` consumes one stream, closes its response and admission lease
  synchronously, and returns `BufferedEvents`, a finite synchronous iterator over the
  already buffered prefix. Handle every result before treating it as drained. Partial
  tails and already-ended generations are explicit errors; malformed and oversized
  records keep their existing errors. The iterator reads no further network data and
  does not establish a complete venue sequence. Dropping a stream still discards its
  undelivered buffer; use `close` when that prefix must be consumed.
- OANDA's [pricing stream](https://developer.oanda.com/rest-live-v20/pricing-ep/) is sampled at most
  four prices per second per instrument. It does not carry every price update.

Read [SECURITY.md](https://github.com/SharurTrading/oanda-rs/blob/main/SECURITY.md) before handling credentials or mutations.

## Optional exit reserve

The outer wrapper opts in and sizes the reserve for its workload. The default is zero.
`ClientBuilder::exit_reserve(R)` sets the initial policy for a private owner;
`Client::set_exit_reserve(R)` or `Admission::set_exit_reserve(R)`
replaces it at runtime, and zero releases the reserve. It keeps `R` requests inside the local
120-request rolling one-second REST window: ordinary requests enter below `120 - R`, and eligible exits
enter below 120. `R` must be less than 120. OANDA's [Development Guide](https://developer.oanda.com/rest-live-v20/development-guide/)
states a hard 120 REST requests/s limit per requesting IP; its [Best Practices](https://developer.oanda.com/rest-live-v20/best-practices/)
separately recommend 100 requests/s on an established connection. The owner admits the full
documented limit; a caller can choose additional pacing when following that recommendation.

| Operation | Admission |
| --- | --- |
| `cancel_order`, `close_trade`, `close_position` | Can consume reserved request capacity |
| Reads, placement, replacement, account configuration and other REST mutations | Ordinary capacity |
| All REST requests | Same aggregate window and provider cooldown across clients sharing an owner |
| Stream starts | Independent of the REST window; require an active stream lease and connector admission |

For example, a wrapper can pass its chosen reserve when constructing its shared client:

```rust
use oanda_client::{Client, Environment, Result};

fn configured_client(token: String, reserve_requests: u32) -> Result<Client> {
    Client::builder(Environment::Practice, token)
        .exit_reserve(reserve_requests)
        .build()
}

fn update_reserve(client: &Client) -> Result<()> {
    let reads = client.clone();
    reads.set_exit_reserve(40)?;
    client.set_exit_reserve(0)?;
    Ok(())
}
# fn main() -> Result<()> {
# let client = configured_client("fixture-token".into(), 20)?;
# update_reserve(&client)?;
# assert!(client.set_exit_reserve(120).is_err());
# Ok(())
# }
```

The configuration and runtime changes are exercised by the credential-free
[configuration example](https://github.com/SharurTrading/oanda-rs/blob/main/examples/exit_reserve.rs): `cargo run --example exit_reserve`.
The example's number is illustrative, not a sizing recommendation. Share clones for reads and
exits. To share admission across separately built credentials using the same caller-selected egress
scope, pass one credential-free owner to both builders:

```rust
use oanda_client::{Admission, Client, Environment, Result};

fn two_clients(first_token: String, second_token: String) -> Result<(Client, Client, Admission)> {
    let owner = Admission::with_exit_reserve(40)?;
    let first = Client::builder(Environment::Practice, first_token)
        .admission(owner.clone())
        .build()?;
    let second = Client::builder(Environment::Practice, second_token)
        .admission(owner.clone())
        .build()?;
    Ok((first, second, owner))
}
# fn main() -> Result<()> {
# let (first, second, owner) = two_clients("fixture-a".into(), "fixture-b".into())?;
# owner.set_exit_reserve(20)?;
# first.set_exit_reserve(0)?;
# second.set_exit_reserve(40)?;
# assert!(owner.set_exit_reserve(120).is_err());
# Ok(())
# }
```

An explicit builder reserve must equal the injected owner's current reserve or construction fails;
omitting it preserves that policy and all spent capacity. One outer coordinator owns this shared
policy: serialized updates replace the previous value, with the last update winning;
they are not additive leases or promised permits. Every REST admission attempt reads the current
policy. A change wakes waiting reads
so they can reconsider capacity immediately. Changes never refund requests, reset timestamps,
clear cooldowns or revoke admitted attempts. Raising the reserve after the aggregate budget is
spent creates no new exit capacity; releasing it restores ordinary access only to unspent capacity.
Each owner also shares 20 active or opening stream leases, following the Development Guide's
per-IP stream limit. There is no extra stream-start gap and streams do not consume REST request
capacity.
Dropping or canceling a stream releases its lease. Independently constructed owners and separate
processes do not share these counters; the caller chooses which traffic shares an egress scope.

Both REST and stream transports use one owner-scoped connector-start budget: at most two connector
invocations begin in any rolling second. The budget records each start without waiting for the
connection to complete; failure or cancellation after a start does not refund it. Cancellation
before a start consumes none. Reusing an established HTTP connection does not enter the gate.
OANDA's Development Guide limits new connections to 2/s; the Best Practices page also recommends
2/s. This is local pacing of connector operations, not a hard limit on raw TCP attempts: DNS
address fallback and connection racing inside a connector can make more than one attempt. REST
admission occurs before any connector wait, so it does not claim exact wire-arrival pacing or
prevent every possible HTTP 429. The caller remains responsible for coordinating other processes
and traffic on the same egress address.

> [!IMPORTANT]
> This is finite shared capacity, not an exclusive or unlimited Flatten allowance. Provider
> cooldown, the aggregate limit, account reconciliation fences and in-flight mutation exclusion
> still apply to exits. A mutation refused by local request-budget admission returns
> `Error::RateLimited` immediately without a wire attempt. An admitted mutation may wait for
> bounded transport establishment, then makes one send with no automatic retry. An uncertain
> post-send result fences that account until caller-acknowledged reconciliation. Reads and stream
> starts can wait for connector capacity. Stream leases and connector pacing are shared only within one owner.
> Trading state, reserve sizing, account selection and recovery remain the outer wrapper's work.

## Testing status

The badges distinguish deterministic test results, documented API coverage, and dated
Practice observations. The [successful CI run on 2026-10-18](https://github.com/SharurTrading/oanda-rs/actions/runs/39095523070)
at [commit `0f8c1e2`](https://github.com/SharurTrading/oanda-rs/commit/0f8c1e2990b5f5750a69a6c227b6d6b2cdd0613b)
passed 102 Rust tests and left 3 live probes ignored. The current
local suite passes **124 credential-free Rust tests, including three README doctests** with **3 live probes ignored**. The static
test-count badge records the local suite; it is not a count of operations or authenticated venue
tests.

| Evidence | Verified scope | Result and date |
| --- | --- | --- |
| Credential-free Rust tests | Unit and synthetic loopback tests, including the checks below | 121 unit/integration tests and 3 README doctests passed locally on 2026-10-20; 3 live probes ignored. The linked historical 2026-10-18 CI run passed 102 tests. |
| Coverage ledger | 30 REST operations, 2 streams and 168 named definitions | 32/32 operations and 168/168 definitions marked tested by the offline ledger check in that run |
| Authenticated read-only Practice probes | Account list and summary, EUR/USD pricing, and one record from each stream | 3 passed on 2026-09-25, as recorded in the [probe notes](https://github.com/SharurTrading/oanda-rs/blob/main/docs/coverage.md#authenticated-probes) |
| Practice observations while building an integration | Order fills in both account modes, stream open behaviour and timing, transaction-stream fill and replace events, P/L precision | Recorded October 2026 with per-fact source tags in the [behaviour guide](https://github.com/SharurTrading/oanda-rs/blob/main/docs/behaviour-guide.md); small-sample observations, untested on Live |
| One-off Practice mutation probe | One-unit EUR/USD limit-order creation, pending state and cancellation, with stream updates | Passed on 2026-09-25; temporary probe source removed |
| Live environment | Authenticated Live requests and trading | Not tested |

The deterministic tests exercise these SDK behaviors without credentials:

| Behavior | Evidence |
| --- | --- |
| REST success and rejection contracts | [Operation matrix](https://github.com/SharurTrading/oanda-rs/blob/main/tests/operation_matrix.rs): method, path, query, headers and mutation bodies for all 30 REST operations |
| Provider definitions and variants | [Definition fixtures](https://github.com/SharurTrading/oanda-rs/blob/main/tests/definition_wire.rs), [scalar enums](https://github.com/SharurTrading/oanda-rs/blob/main/tests/enum_variants.rs) and [tagged variants](https://github.com/SharurTrading/oanda-rs/blob/main/tests/tagged_variants.rs): 168 definitions, 503 enum values and 55 tagged variants |
| Observed transaction shapes | [Close of a flat side](https://github.com/SharurTrading/oanda-rs/blob/main/tests/close_flat_side.rs): synthetic transactions in the shape a 2026-10-16 Practice close of a flat hedged side produced, decoded by id, since id, id range, from the transaction stream and inside the `close_position` rejection; the closeout fill re-encodes with its undocumented requested units and home-conversion costs intact. [Opening fill](https://github.com/SharurTrading/oanda-rs/blob/main/tests/open_fill.rs): a synthetic fill in the shape of 2026-10-12 Practice opening fills keeps the undocumented cost on `tradeOpened`, a stated zero apart from absence |
| Financial precision | [Decimal fixtures](https://github.com/SharurTrading/oanda-rs/blob/main/tests/decimal_precision.rs): exact strings and numeric liquidity, with nonrepresentable values refused |
| Stream delivery and failure | [Client contract tests](https://github.com/SharurTrading/oanda-rs/blob/main/tests/client_contract.rs) and stream unit tests: fragmentation, heartbeats, malformed and oversized records, backlog bounds and HTTP/1.1; synchronous close/drain preserves the finite buffered prefix, explicit tails/errors, cancellation, slot reuse and sibling stream/REST access |
| Exit reserve and runtime changes | [Client contract tests](https://github.com/SharurTrading/oanda-rs/blob/main/tests/client_contract.rs) and client unit tests: shared-clone admission, cancel/trade-close/position-close capacity, runtime replacement and release, spent capacity and provider cooldown |
| Cross-client transport admission | [Admission scope tests](https://github.com/SharurTrading/oanda-rs/blob/main/tests/admission_scope.rs) and admission unit tests: different credentials share stream leases, connector pacing and cooldown; established socket reuse, cancellation and independent owners |
| Redirects and response diagnostics | [Redirect tests](https://github.com/SharurTrading/oanda-rs/blob/main/tests/redirects.rs): REST reads, mutations and stream opens answered with 3xx return `Error::Redirected` with a reduced target and request ID, never contact the target, and leave the account unfenced; rejections, refused stream opens, 429s and ambiguous mutations keep `Server`, `CF-RAY` and `cf-mitigated`, absence included, and an ambiguous mutation keeps its status and its fence |
| Mutation uncertainty and rejection evidence | [Client contract tests](https://github.com/SharurTrading/oanda-rs/blob/main/tests/client_contract.rs): single-attempt exits, account fences after ambiguous replies or timeouts, and supplied versus absent or undecodable provider evidence |

These local checks do not extend the dated Practice results to all operations, runtime reserve
changes, or Live trading. The SDK leaves account projections, reconciliation decisions and
stream recovery to its caller.

Normal CI needs no OANDA credentials. It runs formatting, Clippy, tests, documentation, package
verification, dependency and license checks, full-history secret scanning, and the offline coverage
check on Rust 1.95. Clippy uses pedantic warnings as errors. The manual release-readiness workflow
repeats the Rust, coverage, and supply-chain checks, checks the requested version, and runs a
crates.io publish dry run; it does not publish a release. Dependabot checks Cargo and GitHub
Actions updates weekly. The 30 REST methods have deterministic loopback success and rejection
fixtures, including method, URL, query, header, and mutation-body checks.
Both streams have local framing, heartbeat, error, and gap tests. These tests establish the
documented wire surface; they do not establish behavior against an authenticated OANDA account.

Weekly and manually dispatched CI also check the public OANDA release notes for versions beyond
the [last reviewed baseline](https://github.com/SharurTrading/oanda-rs/blob/main/docs/release-notes.json), initially `3.0.25`. Newer versions and
fetch/parsing failures fail the dedicated job with an actionable summary. PR and push runs test
the checker offline. See [release-note monitoring](https://github.com/SharurTrading/oanda-rs/blob/main/docs/coverage.md#release-note-monitoring)
for the review and baseline-update procedure.

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
CI or remains in this repository. See [coverage and contract drift](https://github.com/SharurTrading/oanda-rs/blob/main/docs/coverage.md) for the
reviewed source and verification details.

## License

Licensed under the [MIT No Attribution License (MIT-0)](https://github.com/SharurTrading/oanda-rs/blob/main/LICENSE). Notable changes between releases
are recorded in the [changelog](https://github.com/SharurTrading/oanda-rs/blob/main/CHANGELOG.md).
