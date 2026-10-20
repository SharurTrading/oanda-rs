<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Contract and coverage

The current [OANDA v20 website](https://developer.oanda.com/rest-live-v20/introduction/) lists 32 operations across Account, Order, Trade, Position, Transaction, and Pricing. The repository's `docs/coverage.json` records each operation and the 168 named definitions visible on 2026-09-25.

The pinned [official OpenAPI](https://github.com/oanda/v20-openapi) revision is `70324cfee31ff0074ed0bf1f93e67d8ee6c84444` ([local copy](../spec/official/v20-openapi.json), SHA-256 `5856fab076e3bc6c40fb06ecaa78d85cc8a828e4f065a95806cace3ac1dea212`), retained under OANDA's [MIT license](../spec/official/LICENSE.txt). It has 40 operations and 156 definitions. The website adds `GET /v3/accounts/{accountID}/candles/latest` and 22 definitions including newer guaranteed-stop, dividend, and home-conversion models. The older OpenAPI file has nine endpoints and ten definitions absent from the website pages, including user and unscoped pricing endpoints. The website is authoritative for this crate; these differences are intentionally excluded or included as recorded here and checked offline.

All provider types and endpoint methods are hand-maintained Rust; there is no
code generator. `docs/coverage.json` is the single source of truth for the
operation and definition set. The coverage ledger distinguishes inventoried,
implemented, tested, and blocked capabilities. The offline
`tools/check_coverage.py` requires all 32 operations and 168 definitions to be
tested, verifies public methods and typed contracts, checks test markers,
confirms the pinned OpenAPI checksum, and detects changes to the reviewed
operation drift. The 30 REST operations each have a loopback success and
rejection fixture; both streams have typed event fixtures. Every definition
has a decode fixture in `tests/definition_wire.rs`. All 503 documented scalar
enum values and 55 tagged order, order-request, and transaction variants have
wire round-trip tests. Scalar and tagged inventories are independently checked against the pinned spec, with explicit website drift fixtures.

To apply a provider change by hand: edit the DTO, enum, or method in `src/`,
update `docs/coverage.json`, and add or extend the wire fixture in the same
change. The round-trip manifests fail to compile when a hand edit drifts from
the documented values — a compile error beats a silent `Unknown` — and
`tagged_arms_match_the_pinned_spec` cross-checks the tagged set against the
pinned OpenAPI spec.

## Audit remediation contract decisions

The [2026-10-20 audit](reviews/2026-10-20-rust-idioms-audit.md) includes the final
per-item dispositions. Its original suggested outcomes are historical proposals; the
implementation decisions recorded at the top take precedence.

Endpoint rejection `errorMessage` remains required under the website contract. There is
no observed provider drift supporting an optional endpoint message. Generic rejection
contracts and the independently extracted reason stay optional because those describe
what was actually readable, not a promise that every rejection has an endpoint body.

Order DTO `type` fields retain absence and documented defaults: the website marks them
`default=…`, not `required`, despite saying each order's type is always its named variant.
The enclosing union requires a discriminator. Serialization rejects contradictory explicit
tags; it does not invent a subtype or overwrite an instruction.

Raw stop-loss detail snapshots retain independently optional `price` and `distance` so
provider rejection evidence can be read unchanged. Typed outbound admission rejects both
being set while permitting neither on modifications, where omitted subfields inherit
existing values. No mandatory trigger enum is imposed on partial modifications.

`MarketOrderDelayedTradeClose.clientTradeID` is documented as `TradeID`, although its
name and description identify a client-assigned trade identity. It uses `ClientID`, as
do the other client trade identities. `MarketOrderTradeClose.clientTradeID` is also
`ClientID`; the website's plain string is narrowed to the existing validated identity.
`FixedPriceOrder.tradeState` and its transaction counterpart are plain required strings
on the website; the crate uses `TradeState`, retaining future values in `Unknown`.

Position close omission retains the documented `ALL` default on each side, including
when both are omitted. The SDK does not translate omission into `NONE`. Explicitly
requesting `NONE` on both sides is locally rejected as no action. The behavior guide
records that omitted-side behavior has not been probed on Practice; these deterministic
checks enforce the published request shape, not authenticated venue behavior.

The direct `StopLossOrderRequest` and `GuaranteedStopLossOrderRequest` schemas also
say only one of price/distance may be supplied while explicitly marking price required.
The required price remains schema-faithful; a direct distance-only request is not exposed
without evidence resolving that requiredness contradiction. Both-set direct requests are
rejected locally. The optional Details types continue to support distance-only instructions.

Scalar spellings are cross-checked against the pinned spec and the explicit differences
in `tests/fixtures/scalar-enum-website-drift.json`. Adding a scalar requires updating its
single spelling table, tested value manifest, and any website/spec difference. Adding an
order family also requires its body, tagged union arms, and independent wire fixtures.

`DelayedTradeClosureTransaction.tradeIDs` remains scalar: both authoritative schemas
print `TradeID` while its prose says list. No array payload has been observed, so changing
it to an array would be speculative; an array currently fails decoding explicitly.
Connector admission guarantees rolling-window pacing, not FIFO order between competing
waiters. The cancellation-safe gate retains its existing pacing contract.

Definitive rejection headers disarm mutation ambiguity before bounded body reading. An
unreadable body retains its status, request ID, and typed read error. A decoded successful
mutation with unusable pagination metadata retains the body and marks the link
`Undecoded`; reads still reject malformed or off-origin pagination links.

## Spec fields the website omits

Where the pinned OpenAPI spec documents a field the website does not, the website wins and the
field is left out. One exception: `lastOrderFillTimestamp` on `Account` and `AccountSummary`,
"the date/time of the last order that was filled for this account". OANDA's own v20 clients
([Python](https://github.com/oanda/v20-python), [Java](https://github.com/oanda/v20-java),
[JavaScript](https://github.com/oanda/v20-javascript)) read it from both. The client holds it
as `Option<Timestamp>`, so absence stays absence. The other spec-only fields and definitions
belong to endpoints the website no longer lists (order and position books, user info, the
unscoped `Price`).

## API comparison page

The [API comparison page](https://developer.oanda.com/rest-live-v20/api-comparison/)
compares v20 REST against FIX and MT4; it is not an implementation authority
(OA-CONTRACT-01 makes the six endpoint pages and their linked definitions
authoritative). Audited on 2026-10-13: every v20 capability it lists maps to
the ledger — all order-request types and durations (`TimeInForce`, including
`GFD` for the page's DAY), account/position/trade/order/transaction queries,
candles and pricing, and both streams. Protocol traits (stateless HTTP/JSON),
application classes (web/desktop/mobile), partner/other-API rows (fxLabs,
Autochartist, custom indicators, backtesting), and provider usage quotas
(20 streams per IP in the [Development Guide](https://developer.oanda.com/rest-live-v20/development-guide/),
despite this comparison page's per-user wording, and polling rate) are not endpoint contracts; authentication
is caller-injected bearer tokens over HTTPS (OA-SECRET-01). One page value is
stale: it lists an `M3` candle granularity that neither the authoritative
instrument definition page nor the pinned OpenAPI spec documents, so there is
no `M3` variant; an unrecognized granularity from the provider is preserved
as `CandlestickGranularity::Unknown` rather than dropped.

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
unchanged. Every operation carries one of these two decisions, hand-maintained
in the ledger and the method signature: no rejection pair, empty success
struct, `String` alias, or field description is invented. The 22 removed
per-endpoint structs are a breaking change for callers naming them; stream operations record
`"rejection": "stream"` and reject through `Error::Provider`, whose `code`/`message` fields carry
the same evidence contract.

## Observed requiredness drift

Field requiredness follows the website's `required` marking, except where OANDA's own payload
omits the field. Such a field is an `Option`, so its absence stays absence, and a synthetic
fixture of the observed shape is decoded in `tests/close_flat_side.rs`.

| Definition | Field | Website | Client | Evidence |
| --- | --- | --- | --- | --- |
| `MarketOrderRejectTransaction` | `units` | required | `Option<Decimal>` | Practice, 2026-10-16: the `CLOSEOUT_POSITION_DOESNT_EXIST` reject of a `close_position` on a flat hedged side has no `units`; the size is only in `longPositionCloseout` |

The pinned OpenAPI spec declares no `required` list for `MarketOrderRejectTransaction` at all, so
only the website marks `units` required.

The other transactions a trade close, position closeout or margin closeout produces were
checked against the same evidence and left as they are. `MarketOrderTransaction` keeps
`units`, `instrument`, `timeInForce` and `positionFill` required: the position closeout market
order observed in the same close carries all four, as do the website's `close_position` and `close_trade`
examples. `MarketOrderRejectTransaction` keeps `instrument`, `timeInForce` and `positionFill`
required, since the observed reject carries them. `OrderFillTransaction` and
`OrderCancelTransaction` hold no required field. No refused `close_trade` or margin
closeout has been observed.

### Observed fields beyond the docs

OANDA also sends fields that neither the website nor the pinned OpenAPI spec lists. A field
observed in a provider payload is modelled when it carries money or size. Each is optional,
because nothing shows it is always sent, and each is an exact `Decimal` (OA-DECIMAL-01). The
synthetic fixtures `tests/fixtures/close-flat-side/order-fill.json` (a closing fill) and
`tests/fixtures/open-fill/order-fill.json` (an opening fill) carry the observed shapes, and
`tests/close_flat_side.rs` and `tests/open_fill.rs` decode them and re-encode them with nothing
dropped.

| Definition | Field | Website | Client | Evidence |
| --- | --- | --- | --- | --- |
| `OrderFillTransaction` | `requestedUnits` | absent | `Option<Decimal>` | Practice, 2026-10-16: the `MARKET_ORDER_POSITION_CLOSEOUT` fill of a `close_position` on a hedged EUR_USD account, as a decimal string |
| `OrderFillTransaction` | `plHomeConversionCost`, `baseFinancingHomeConversionCost`, `guaranteedExecutionFeeHomeConversionCost`, `homeConversionCost` | absent | `Option<Decimal>` | The same fill, each as a decimal string |
| `TradeReduce` | `plHomeConversionCost`, `baseFinancingHomeConversionCost`, `guaranteedExecutionFeeHomeConversionCost`, `homeConversionCost` | absent | `Option<Decimal>` | The same fill's one `tradesClosed` entry, each as a decimal string; also a `tradeReduced` entry and two `tradesClosed` entries in a Practice hedged-account recording of five `MARKET_ORDER` fills on 2026-10-12 |
| `TradeOpen` | `guaranteedExecutionFeeHomeConversionCost` | absent | `Option<Decimal>` | The two `tradeOpened` entries of that 2026-10-12 recording, each as a decimal string ([#68](https://github.com/SharurTrading/oanda-rs/issues/68)) |

Every fill in both recordings carried `requestedUnits` and the four fill-level costs. The
`tradeOpened` entries carried no cost other than `guaranteedExecutionFeeHomeConversionCost`, so
`TradeOpen` models only that one. Every cost in the 2026-10-12 recording was a stated zero, which
the client keeps as `Some(0)`, distinct from an absent field's `None`. A zero cannot show a sign, a formula, or how
the costs enter `pl`, `realizedPL` or `accountBalance`, so none of these is recorded; the
[behaviour guide](behaviour-guide.md#fills-money-and-pnl) says so. The fixtures' values are
synthetic.

## Stream record and transport bounds

Pricing and transaction streams bound each newline-delimited record at a local hostile-input limit
of 1 MiB; OANDA documents no record size, and realistic payloads are orders of magnitude smaller.
An HTTP chunk is a transport read, not a provider record boundary, so there is deliberately no
aggregate chunk-size rejection: a chunk carrying more than 8 MiB of individually valid records is
processed record-by-record in order (`a_large_transport_chunk_of_valid_records_invents_no_gap`).
Buffered records are never evicted for a slow caller, because dropping records would invent a
continuity gap the provider did not create. The buffered backlog is capped at a local 16 MiB: a
fill episode that parks more than that ends the generation with an explicit gap
(`a_backlog_beyond_the_local_bound_ends_the_generation`), because while undelivered records
remain no further reading — and therefore no per-record check — happens. A malformed or oversized
individual record still ends the generation explicitly.

## Law-invariant audit

The [LAW-INVARIANT audit](law-invariant-audit.md) records every candidate inspected on 2026-09-30,
the owner and call-path invariant of each, the legitimate absence or no-op semantics where they
apply, and the disposition. It found no fabricated side, money, identity, account mode, or success
in the models or endpoint contracts. Outstanding repairs are tracked as their own issues
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
