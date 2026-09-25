# OANDA Rust Client Guide

This repository contains the standalone `oanda-client` library. Rules have stable IDs so reviews can cite them.

## Boundary and API

- **OA-BOUNDARY-01:** Own OANDA v20 transport, provider models, and protocol behavior. Do not depend on SharurPlatform or own its routing, risk, portfolio, persistence, or UI.
- **OA-CONTRACT-01:** The six current OANDA v20 endpoint pages and their linked definitions are the implementation authority. The pinned OANDA OpenAPI revision is a cross-check; document drift rather than silently using obsolete operations.
- **OA-COVERAGE-01:** Every operation in `docs/coverage.json` needs a public typed method, a typed success and rejection contract, and a deterministic test. A coverage claim must be computed from the ledger, not inferred from generated code.
- **OA-DECIMAL-01:** Financial values use exact `rust_decimal::Decimal` serialization from OANDA decimal strings. No floating-point round trip.
- **OA-ID-01:** Public provider identities are distinct validated types. No implicit identity guessing.

## Transport and safety

- **OA-SECRET-01:** Callers inject bearer tokens. Never load, persist, log, or expose tokens in public accessors; redact `Debug`. Remote transport is HTTPS; HTTP is accepted only for exact loopback test hosts.
- **OA-RUNTIME-01:** The caller owns the Tokio runtime. No hidden runtime or blocking network operation.
- **OA-HTTP-01:** Bound response bodies and stream records; return typed errors for provider status and malformed or oversized input. Never log raw requests or headers.
- **OA-MUTATION-01:** Never automatically retry account or trading mutations. An uncertain post-send outcome is explicitly ambiguous and fences further mutations for that account until caller-acknowledged reconciliation.
- **OA-RATE-01:** Rate admission and provider cooldown are shared by cloned clients. A mutation that cannot be admitted immediately fails locally.
- **OA-STREAM-01:** Pricing and transaction streams use incremental line-delimited JSON over HTTP. Surface malformed data, overflow, and connection loss as gaps; never silently resume a sequence or claim every market tick arrived.

## Development

- **OA-TEST-01:** Normal CI is deterministic and credential-free. Optional live tests are ignored, explicitly armed, Practice-only, and read-only.
- **OA-DOC-01:** Document public APIs and non-obvious venue behavior. Keep the coverage ledger, README, and code in sync.
- **OA-REVIEW-01:** Review money-moving behavior, cancellation, secrets, decimal precision, and coverage before release. A GitHub or crates.io release requires a separate reviewed change.

