# Contract and coverage

The current [OANDA v20 website](https://developer.oanda.com/rest-live-v20/introduction/) lists 32 operations across Account, Order, Trade, Position, Transaction, and Pricing. The repository's `docs/coverage.json` records each operation and the 168 named definitions visible on 2026-09-25.

The pinned [official OpenAPI](https://github.com/oanda/v20-openapi) revision is `70324cfee31ff0074ed0bf1f93e67d8ee6c84444` ([local copy](../spec/official/v20-openapi.json), SHA-256 `5856fab076e3bc6c40fb06ecaa78d85cc8a828e4f065a95806cace3ac1dea212`), retained under OANDA's [MIT license](../spec/official/LICENSE.txt). It has 40 operations and 156 definitions. The website adds `GET /v3/accounts/{accountID}/candles/latest` and 22 definitions including newer guaranteed-stop, dividend, and home-conversion models. The older OpenAPI file has nine endpoints and ten definitions absent from the website pages, including user and unscoped pricing endpoints. The website is authoritative for this crate; these differences are intentionally excluded or included as recorded here and checked offline.

The coverage ledger distinguishes inventoried, implemented, tested, and blocked capabilities. The offline `tools/check_coverage.py` requires all 32 operations and 168 definitions to be tested, verifies public methods and typed contracts, checks test markers, confirms the pinned OpenAPI checksum, and detects changes to the reviewed operation drift. The 30 REST operations each have a loopback success and rejection fixture; both streams have typed event fixtures. Every definition has a compile-time serde contract check. All 503 documented scalar enum values and 55 tagged order, order-request, and transaction variants have wire round-trip tests.

## Rejection evidence

A definitive provider rejection reports only what OANDA sent. `OperationError::Rejected::code` and
`::message` are `Option<String>` and are `None` when OANDA supplied no readable reason, and
`::body` is a `Supplied<R>` that separates a body OANDA did not send (`Absent`), one this client
decoded (`Decoded`), and one that arrived but did not match the documented shape (`Undecoded`).
`Error::Provider` uses the same `Option` fields for a refused stream. Neither path substitutes a
message OANDA did not write, and a caller reconciling a refused mutation can tell a reject
transaction it has already accounted for from one it is missing. `ApiResponse::request_id` and
`::next_page` carry the same three-state evidence: `Undecoded` reports a header OANDA sent that
this client could not read, never no header at all.

## Rejection typing

`docs/coverage.json` records each operation's rejection decision. The nine endpoints whose pages
document a rejection body — the mutations and `configure_account` — carry a schema-derived
`XRejection` struct (`"rejection": "endpoint"`). The remaining GET endpoints document no rejection
body, so their methods reject through the reviewed
[`GenericRejection`](https://docs.rs/oanda-rs) common error fields
(`"rejection": "generic"`) instead of a per-endpoint struct with fields OANDA never documented; the
`Supplied<R>` evidence contract and the `errorCode`/`errorMessage` read from the body apply
unchanged. The generator refuses to run without one of these two decisions: it no longer invents a
rejection pair, an empty success struct, a `String` alias, or a field description.

## Law-invariant audit

The [LAW-INVARIANT audit](law-invariant-audit.md) records every candidate inspected on 2026-09-30,
the owner and call-path invariant of each, the legitimate absence or no-op semantics where they
apply, and the disposition. It found no fabricated side, money, identity, account mode, or success
in the generated models or endpoint contracts. Outstanding repairs are tracked as their own issues
with the path and the required outcome.

## Release-note monitoring

The weekly CI schedule (Monday at 06:17 UTC) and manual CI dispatch run
`python3 tools/check_release_notes.py` against the public
[OANDA release notes](https://developer.oanda.com/rest-live-v20/release-notes/).
The initial baseline in [release-notes.json](release-notes.json) is `3.0.25`,
reviewed on 2026-09-27. This release-note review date is separate from the endpoint
and definition review date in the coverage ledger.

The job compares numeric version components and fails when any published version
advances past `last_reviewed_version`. It lists the newer versions in the job log,
an error annotation, and the job summary. Fetch failures, missing or malformed
release tables, unsupported version formats, and a latest version below the
baseline also fail the check; they do not count as an unchanged release.
PR and push checks run deterministic offline checker tests without fetching OANDA.

After a drift alert, review the new release notes and the six authoritative
endpoint pages and linked definitions. Update the client, coverage ledger, and
drift decisions as needed, then advance `last_reviewed_version` and `reviewed_at`
in a reviewed PR. The check never updates the baseline automatically. To replay
a downloaded page locally, use `python3 tools/check_release_notes.py --html PATH`.
This monitor detects published version advances; it does not detect edits within
an existing release or unversioned changes to endpoint documentation.

## Authenticated probes

The ignored `live-tests` feature contains read-only Practice probes. Run them only with `OANDA_READ_ONLY_PROBE=I_ACCEPT_READ_ONLY_PRACTICE`, `OANDA_TOKEN`, and `OANDA_ACCOUNT_ID` set, using `cargo test --features live-tests --test practice_read_only -- --ignored`. All three probes passed against an authenticated Practice account on 2026-09-25. A separate, temporary one-unit Practice limit-order probe also confirmed creation, pending state, cancellation, and transaction-stream updates; its source was removed after the run. These probes are not acceptance tests for live trading and are not run in normal CI. No mutation probe remains in the repository.
