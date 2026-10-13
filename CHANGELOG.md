<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

All notable changes will be documented here. This project follows Semantic Versioning after its
first public release.

## [1.0.0] - 2026-10-13

First public release. The crate was previously developed at version 0.1.0 without ever being
published; the entries below describe the surface this release establishes.

- License the project under the MIT No Attribution License (MIT-0).
- Establish the standalone OANDA v20 client boundary: no trading-platform dependencies, a
  caller-owned Tokio runtime, and an injected bearer token that is never loaded, persisted, logged,
  or exposed, with `Debug` output redacted. Remote transport is HTTPS; plain HTTP is accepted only
  for exact loopback fixture hosts.
- Cover every operation the six OANDA v20 endpoint pages document: 30 REST methods plus the pricing
  and transaction streams, with typed success and rejection contracts for all 168 linked
  definitions, 503 scalar enum values, and 55 tagged order, request, and transaction variants. The
  [coverage ledger](docs/coverage.json) records each operation's source URL, public method or type,
  and local test; an offline CI check recomputes the claim from the ledger, and the generator
  pipeline fails on fabrication instead of writing fallback values.
- Parse financial values as exact `rust_decimal::Decimal` tokens from OANDA decimal strings without
  a floating-point round trip, accept documented numeric price-bucket liquidity exactly, and reject
  nonrepresentable coefficient/scale values during deserialization rather than rounding them.
- Use distinct validated typed identifiers for public provider identities, with dedicated types for
  timestamps and pagination.
- Bound every REST response body and each stream record individually. An HTTP chunk is a transport
  read, not a record boundary: stream records are bounded by a local hostile-input limit of 1 MiB,
  the buffered backlog by a 16 MiB cap that ends the stream generation explicitly rather than
  evicting accepted records, and malformed data, per-record overflow, and connection loss surface as
  gaps the caller must recover from. The library never silently resumes a sequence or claims every
  market tick arrived, and OANDA's at-most-four-prices-per-second pricing sampling is documented
  rather than hidden.
- Run both streams over HTTP/1.1 with incremental newline-delimited decoding and typed heartbeats,
  selected independently of any HTTP/2 feature a consumer enables on reqwest through Cargo feature
  unification; REST protocol negotiation is separate.
- Never automatically retry an account or trading mutation. An uncertain post-send outcome is
  explicitly ambiguous and fences further mutations for that account across cloned clients until the
  caller reads OANDA account and transaction state and calls `acknowledge_reconciliation`, which
  reports whether it released the fence.
- Report provider evidence faithfully: a definitive rejection carries only what OANDA sent — its
  `code` and `message` are `None` when no readable reason arrived — and `Supplied<T>` separates a
  body or header OANDA did not send from one that arrived and could not be decoded, including the
  `RequestID` and pagination `Link` headers. The client never invents a reason.
- Share conservative rate admission and provider cooldown across cloned clients, honouring both
  RFC 9110 `Retry-After` forms. The local 100-request, one-second window supports an optional
  reserved exit capacity for `cancel_order`, `close_trade`, and `close_position` that ordinary
  requests cannot consume; a mutation that cannot be admitted immediately fails locally without a
  wire attempt.
- Monitor the public OANDA release notes weekly against the last reviewed baseline (initially
  3.0.25) so provider contract drift surfaces as a failing CI job instead of a silent mismatch.
- Keep normal CI deterministic and credential-free: formatting, strict pedantic Clippy, rustdoc,
  tests, package verification, dependency and license checks (`cargo deny`, `cargo audit`), and
  full-history secret scanning. Live tests are ignored, explicitly armed, Practice-only, and
  read-only; three probes passed on 2026-09-25, and a separate temporary one-off probe validated
  limit-order creation and cancellation end to end before being removed. No Live environment has
  been tested.
- Record the [LAW-INVARIANT audit](docs/law-invariant-audit.md) with a disposition for every
  candidate, so no fabricated fallback or swallowed invariant failure ships in the typed surface.

[1.0.0]: https://github.com/SharurTrading/oanda-rs/releases/tag/v1.0.0
