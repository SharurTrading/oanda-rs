<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

All notable changes will be documented here. This project follows Semantic Versioning after its
first public release.

## Unreleased — next major

The changes below require a major release because existing error variants gain a field.
The package version remains 3.0.0 until a separate reviewed release change runs the
release gate; this section is not a release.

- Breaking: an HTTP 3xx answer is the new `Error::Redirected { status, location,
  request_id, diagnostics }` on REST calls (as `OperationError::Client`) and stream opens,
  instead of `OperationError::Rejected`, `Error::AmbiguousMutation` or `Error::Provider`.
  Redirects are still never followed, so the bearer token is never sent to the target.
  `location` is a `Supplied<RedirectTarget>` holding only the scheme, host, non-default
  port and path of the `Location` header, resolved against the request URL when relative;
  its userinfo, query and fragment are discarded and never rendered. A 3xx answer to a
  mutation means the request was not executed at its URL, so it no longer fences the
  account.
- Breaking: `OperationError::Rejected`, `OperationError::RejectedUnreadable`,
  `Error::Provider`, `Error::Redirected`, `Error::AmbiguousMutation` and
  `Error::ProviderRateLimitedInvariant` carry `diagnostics: Box<ResponseDiagnostics>`,
  the `Server`, `CF-RAY` and `cf-mitigated` response headers as `Supplied<String>`. They
  let a caller tell an answer from an edge network, such as a bot-management challenge,
  from one by OANDA's servers. The client only reports them; it reads no cookies and does
  not act on a challenge. Patterns that list every field of these variants need `..` or
  the new field.
- Breaking: `Error::ProviderRateLimited` is now a struct variant,
  `ProviderRateLimited { diagnostics }`; match it as `ProviderRateLimited { .. }`.
- Breaking: `Error::AmbiguousMutation` also carries `status: Option<u16>`, the HTTP status
  of the answer that left the outcome uncertain, or `None` when no response arrived (its
  diagnostics are then all `Absent`). Its ambiguity is unchanged: the account is still
  fenced until the caller acknowledges reconciliation.

## [3.0.0] - 2026-10-20

This major release contains source-breaking API corrections from the Rust idioms audit.
See the [2.x upgrade guide](https://github.com/SharurTrading/oanda-rs#upgrading-from-2x)
for downstream changes.

- Breaking: the nine endpoint rejection DTOs require `error_message: String` and no longer
  implement `Default`. A nonempty rejection body missing that documented required field is
  `Supplied::Undecoded`; the separately readable provider reason is still preserved.
- Breaking: close request quantities use `TradeCloseoutUnits` and `PositionCloseoutUnits`
  instead of strings. Use `All`, `Units(Decimal)`, or position-only `None`; omission retains
  OANDA's `ALL` default. Response closeout quantities use `TradeCloseoutUnits`, retaining
  negative provider evidence while outgoing admission rejects nonpositive quantities.
- Breaking: mutation fence errors carry `AccountID`, response metadata carries
  `Supplied<RequestID>`, and the two inconsistent transaction `client_trade_id` fields use
  `ClientID`. Fixed-price `trade_state` fields use the preserving `TradeState` enum.
- Breaking: `ApiResponse<T>` no longer dereferences to `T`; access response fields through
  `.body`. Timestamp parsing returns `InvalidTimestamp`; `InvalidId` retains the rejected
  spelling and exposes an `InvalidIdKind` through `kind()`.
- Add `Timestamp::new` and `From<DateTime<Utc>>`. Parse signed Unix fractions exactly,
  reject excess fractional precision rather than truncate, and avoid temporary timestamp
  strings during fractional parsing and serialization.
- Validate typed request DTOs before serialization, rate admission, account reservation,
  or network access. Request construction uses named parts and one prepared wire value,
  replacing URL-suffix and JSON-key dispatch. Invalid requests preserve input and admission
  state. Partial dependent-order modifications may omit both trigger fields to inherit the
  existing value, but cannot supply both price and distance.
- Fail serialization of standalone `Patch::Unchanged` instead of turning omission into
  cancellation. Existing dependent-order fields continue to omit it. Reject conflicting
  explicit order and transaction type discriminators instead of silently replacing them.
- Accept an omitted position-close body as the documented `ALL` defaults for both sides.
  To close only one side, explicitly use `PositionCloseoutUnits::None` on the other side;
  no live probe of omitted sides was performed.
- Preserve definitive rejections when body reading fails through
  `OperationError::RejectedUnreadable { status, request_id, source }`; such outcomes do not
  fence the account. A decoded mutation success retains its body when pagination metadata
  is unusable. Internal lock failures are typed invariant errors. A failed cooldown
  installation retains HTTP 429 through `Error::ProviderRateLimitedInvariant`, disarms the
  mutation guard, and leaves cooldown state unchanged.
- Check the mutation fence before inserting an in-flight reservation, so a lock failure
  cannot leave a reservation without an owning guard.
- Zeroize the library-owned bearer-token string on builder/client owner drop, using the
  existing pinned `zeroize` crate as a direct dependency. This does not erase copies owned
  by the caller or HTTP transport and requires human dependency/credential review.
- Declare scalar enum spellings once, preserving all existing documented variants and
  unknown spellings; add an independent pinned-spec scalar completeness check with explicit
  website drift. Replace source-text tagged-fixture scanning with shared fixture values.
- Decode stream records from borrowed buffers before compaction, retaining record limits,
  ordering, backlog handling, and explicit gaps. Name the local heartbeat timeout.
- Remove stale import and module-wide lint suppressions; document endpoint errors, unknown
  enum behavior, and close defaults. Compile README examples as doctests, run credential-free
  examples in CI, share integration-test transport helpers, and remove global panic-hook
  manipulation from negative tests.
- Commit the historical Rust idiom audit and record every final disposition, including
  retained provider-native naming, valid query defaults, and schema-faithful ambiguous fields.

## [2.0.0] - 2026-10-18

- Add consuming `HttpStream::close` and the move-only, fused `BufferedEvents` iterator.
  Closing releases the response and stream lease synchronously, then permits decoding
  exactly the already buffered prefix without any network reads or runtime.
- Add explicit `IncompleteStreamRecord { bytes }` and `StreamAlreadyEnded` errors.
  An unterminated tail, malformed or oversized record, or already-ended generation
  cannot be mistaken for a clean buffered drain. Existing stream bounds and decoder
  ordering remain unchanged. This local cut does not establish venue completeness.

- Breaking: `MarketOrderRejectTransaction::units` is now `Option<Decimal>`. OANDA omits `units`
  from the `CLOSEOUT_POSITION_DOESNT_EXIST` reject of a `close_position` on a side that is
  already flat, although the website marks it required. Every transaction read covering such a
  reject (by id, since id, id range and the transaction stream) and the `close_position`
  rejection body embedding it failed to decode; they now decode with the units absent.
- Add synthetic fixtures in the observed shape of that close and decode them through every
  read that carries them.
- Breaking: `OrderFillTransaction` gains `requested_units`, `pl_home_conversion_cost`,
  `base_financing_home_conversion_cost`, `guaranteed_execution_fee_home_conversion_cost` and
  `home_conversion_cost`, and `TradeReduce` gains the four conversion costs. OANDA sends these
  on fills but documents none of them, so they were dropped on decode. Each is an optional exact
  `Decimal`. Code building either struct with a literal must add the fields.
- Breaking: `Account` and `AccountSummary` gain `last_order_fill_timestamp`. The website omits
  `lastOrderFillTimestamp`, but the pinned OpenAPI spec documents it and OANDA's own v20
  clients read it, so it was dropped on decode. Code building either struct with a literal must
  add the field.
- Breaking: `TradeOpen` gains `guaranteed_execution_fee_home_conversion_cost`. OANDA sends it
  on the `tradeOpened` entry of an opening fill but does not document it, so it was dropped on
  decode. It is an optional exact `Decimal`: a stated zero is `Some(0)`, an absent field `None`.
  Code building `TradeOpen` with a literal must add the field.
- Expand the behaviour guide with the reconciliation fence, position-close and
  dependent-cancel behaviour, stream-open failures, and the read-ahead-of-stream
  comparison recorded on Practice in October 2026.
- Correct the behaviour guide: `homeConversionFactors` is not deprecated and was present on every
  recorded fill. The deprecated fields are `gainQuoteHomeConversionFactor` and
  `lossQuoteHomeConversionFactor`.

## [1.1.0] - 2026-10-15

- Add a plain-language behaviour guide covering connection, data and execution nuances for
  callers that wrap this client.
- Document the SDK surfaces the deterministic suites and the authenticated Practice probes
  actually verify.
- Add a credential-free admission owner that callers can share across clients using the same
  egress scope. It shares the 120 REST requests/s rolling admission window, cooldown, exit
  reserve, 20 active stream leases, and two connector starts/s while keeping account mutation
  fences per client and its clones.
- Update the pinned rustls lockfile to 0.23.46 for RUSTSEC-2026-0349 (unauthenticated
  ServerHello key-share prediction can downgrade later resumed connections to classical key
  exchange). Lockfile-only: the published crate never pins transitive versions, so consumers
  resolve the fixed rustls through their own lockfiles.

[3.0.0]: https://github.com/SharurTrading/oanda-rs/compare/v2.0.0...v3.0.0
