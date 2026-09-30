# LAW-INVARIANT audit

Every candidate below was read against the actual business contract of its call path, not against a
pattern list. `Option`, `Default`, an early `return`, a `continue`, and duplicate or stale
outcomes are legitimate where they express what the operation really guarantees; the question asked
of each one is whether a *failure* is being presented to the caller as absence, success, or a
harmless no-op. The audit was run on 2026-09-30 against commit `d6becff`.

`src/models/**` and the endpoint contracts in `src/{account,order,trade,position,transaction,pricing}.rs`
are generated from the six authoritative OANDA pages, and the read paths there were verified rather
than assumed: every documented variant has a typed arm, every unrecognized provider value keeps its
exact spelling in an `Unknown` arm that serializes back unchanged, a missing `type` discriminator is
a typed serde error, and every `Option` field is `#[serde(default)]` with a malformed value still
failing to decode. `Decimal` fields deserialize through `deserialize_str`, so a JSON number for a
price is a decode error rather than a float round trip. Those files needed no repair.

## Repaired in this change

| Candidate | Owner and call-path invariant | Legitimate semantics, if any | Disposition |
| --- | --- | --- | --- |
| `OperationError::Rejected::message` was `String`, filled with `"provider rejected request"` whenever OANDA's body carried no `errorMessage`. The provider's reason for refusing a money-moving request was therefore reported as a string OANDA never sent. | `src/client.rs` `Client::execute`, the definitive-rejection branch. The status proves OANDA refused; the message is OANDA's own attributable reason and must not be spoken for. | OANDA may genuinely send a rejection with no `errorMessage`; that is real absence and stays representable. | `message` is `Option<String>`, and the rendering says `OANDA sent no readable reason` rather than asserting one. `an_unreadable_rejection_body_is_reported_as_evidence_not_as_a_reason`, `a_rejection_with_no_body_is_absent_rather_than_guessed`. |
| `OperationError::Rejected::body` was `Option<R>`, filled by `serde_json::from_slice::<R>(&bytes).ok()`. A rejection body that failed to decode was reported as a body OANDA never sent, so a caller reconciling a refused mutation could not tell a reject transaction it has already accounted for from one it is missing. | `Client::execute`. The endpoint's documented rejection fields are evidence about account state, and OA-COVERAGE-01 requires the rejection contract to be observable. | A body that is absent, or present and empty, is real absence and stays representable. | New `Supplied<T>` (`Absent` / `Decoded(T)` / `Undecoded`) makes "arrived and unreadable" expressible. Same two tests. |
| `Error::Provider` for streams used `serde_json::from_slice(&body).unwrap_or_default()` and then `.unwrap_or("provider rejected stream")`, discarding OANDA's `errorCode` and inventing the message whenever the body was not a readable JSON object. | `src/stream.rs` `open_http_stream`. A refused stream still has to tell the caller what OANDA said. | None: an unreadable body is not a body without a reason. | `code` and `message` are read from the body only when it is readable, and the rendering names the absence. `a_readable_stream_rejection_keeps_its_reason`, `an_unreadable_stream_rejection_reports_no_reason`. |
| `install_cooldown` read `Retry-After` with `.parse::<u64>().ok().unwrap_or(1)`, so a present-but-unreadable header was read as no header and a one-second wait was presented as OANDA's instruction. It also accepted only delta-seconds, so the HTTP-date form RFC 9110 entitles a provider to send was itself treated as garbage. | `src/client.rs`. The cooldown is shared by every clone under OA-RATE-01 and is a value OANDA supplies. | An absent header is OANDA giving no instruction, and the documented local default of one second still applies. | `parse_retry_after` reads both RFC 9110 forms; an unreadable header takes the maximum local cooldown, which is local policy and is never attributed to OANDA. A date already past asks for no wait, which is a stated instruction. `retry_after_reads_both_rfc_9110_forms`, `an_unreadable_or_absent_retry_after_is_not_read_as_a_provider_instruction`. |
| `acknowledge_reconciliation` returned `()`, so a fence set it could not lock left the account fenced with the acknowledgement silently dropped and the caller's only option a new client. | `src/client.rs`. This is the caller's explicit acknowledgement, on the money path, and its outcome is not a benign no-op. | None. | Now returns `Result<()>` and reports the failure. `a_failed_acknowledgement_is_reported_and_leaves_the_account_fenced`. |
| `MutationGuard::drop` installed the ambiguity fence only `if let Ok(..)`, so a poisoned fence lock left an unresolved mutation unfenced: a later mutation would proceed with no evidence about the previous one. | `src/client.rs`. OA-MUTATION-01's fence is exactly what a dropped armed guard owes the caller, and `Drop` cannot report a failure. | None. A `Drop` impl has no owner to return an error to. | The guard takes the fence through `PoisonError::into_inner`; the critical sections hold plain `HashSet` values, so a poisoned lock is still readable. `an_ambiguous_mutation_is_fenced_even_after_a_poisoned_lock`. |
| `StreamLease::drop` released the connection slot only `if let Ok(..)`, so a poisoned lock reported a released stream as capacity still in use until the process restarted. | `src/stream.rs`. The slot counter is shared admission state. | None, for the same reason. | Same `into_inner` treatment. `a_cancelled_stream_releases_its_slot_even_after_a_poisoned_lock`. |
| `install_cooldown` installed the cooldown only `if let Ok(..)`, reporting a rate-limited account as one the client may keep calling. | `src/client.rs`. The rate window is two plain fields. | None. | Same `into_inner` treatment, exercised by the cooldown tests. |

## Inspected and not violations

| Candidate | Why it is the operation's real contract |
| --- | --- |
| Every `Option<Decimal>` price, unit, balance, and P/L field in `src/models/**`. | OANDA's own schema marks these optional, and `#[serde(default)]` applies to absence only: a malformed value still fails to decode, and no fallback ever supplies a number. |
| `Unknown(String)` and `Unknown { kind, raw }` catch-all arms on 503 scalar enums and 55 tagged variants. | The unrecognized provider value keeps its exact spelling and serializes back unchanged, so forward compatibility loses no evidence. `OrderRequest` deliberately does the opposite and rejects an unknown type, because an unrecognized *request* variant must not be sent. |
| `decode_transaction` treating every non-`HEARTBEAT` record as a transaction. | `Transaction`'s deserializer requires a `type` discriminator, so a record without one is a typed decode error that ends the generation and creates a continuity gap. |
| `decode_price`'s `_` arm, which also covers a missing `type`. | It is an error either way, so a priced record is never reported as a heartbeat. |
| Empty-line `continue` and the `strip_suffix(b"\r")` `unwrap_or` in `HttpStream::next_event`. | An empty line is the documented record separator and a missing carriage return is the normal case; both re-enter the read loop and neither is a failure. |
| `Patch<T>`'s `#[default] Unchanged` on `SetTradeDependentOrdersBody` fields. | OANDA documents an omitted `takeProfit` as "the existing Take Profit Order will not be modified", so `Unchanged` is the provider's contract, distinct from an explicit `null`, and a malformed field still errors. |
| `Timestamp::from_str`'s `split_once('.')` and the `and_then(Value::as_str)` guards in `validation::validate` and `Client::admit`. | Both values arrive from typed query structs, so the type boundary already makes the mismatched shape unrepresentable; nothing is defaulted, and the value is forwarded unchanged for OANDA to judge. |
| `ids.rs`'s `id_type!` validation. | An empty or control-character identifier returns `InvalidId` on construction and on deserialization; no path guesses an identity. |
| `decimal_wire::optional_number_or_string`. | A JSON number is converted through its exact `arbitrary_precision` literal and then parsed as `Decimal`; a boolean, array, or object is a typed error rather than a default. |
| Definitive-rejection statuses that disarm the mutation guard, and the statuses that instead report `AmbiguousMutation`. | A 400/401/403/404/405 is OANDA's own refusal, and everything else after a send is reported as ambiguous and fenced. The fence is never released on a path that did not observe a definite outcome. |

## Repaired in follow-up changes

| Candidate | Owner and call-path invariant | Legitimate semantics, if any | Disposition |
| --- | --- | --- | --- |
| `ApiResponse::request_id` read a present-but-unreadable `RequestID` as `None`, the same value as an absent header, while the sibling Link header failed loudly on the same class of failure ([#7](https://github.com/SharurTrading/oanda-rs/issues/7)). | `src/client.rs` `Client::execute`. The `RequestID` is a caller's only correlation with OANDA's own record of the request. | An absent header is real absence. | `request_id` is `Supplied<String>` and `next_page` is `Supplied<Url>`, so both headers agree: absent, decoded, or sent and unreadable. A Link header whose bytes cannot be read as text is reported `Undecoded` while the decoded body is kept; a *readable* `rel="next"` item that does not match the documented shape stays a hard `Error::Decode` — it decoded fine and was refused, exactly like a readable link that leaves the client's API origin — and the lossy `into_decoded()` accessor is removed so no one-liner turns "offered but unreadable" into "not offered". A readable next link that leaves the client's API origin is still a hard error: that value decoded fine and was refused, not lost. `an_unreadable_request_id_is_reported_as_sent_not_absent`, `an_absent_request_id_is_absent`, `an_unreadable_link_header_is_not_read_as_no_next_page`, `a_malformed_next_link_is_undecoded_not_an_error`, `a_link_header_offering_no_next_page_is_absent`. |

| The `tools/generate_*.py` generators substituted fallbacks for provider content they failed to parse: an invented `errorCode`/`errorMessage` pair for endpoints with no documented rejection schema (materialized in 22 checked-in structs), an empty success struct for a missing response schema, `pub type X = String` for an unparsed definition, and a fabricated field description ([#8](https://github.com/SharurTrading/oanda-rs/issues/8)). | The generators write the checked-in contracts; a default that survives into `src/` is provider content this repository never reviewed. | None: a parse miss is not a documented absence. | Every fabrication path is a hard failure with a non-zero exit, nothing written, and the endpoint or definition named on stderr; both generators now parse every page before writing any file. The 22 invented pairs are removed — those endpoints reject through the reviewed `GenericRejection`, and `docs/coverage.json` records each operation's decision as `"rejection": "endpoint" | "generic"`. The schema-token grammar is validated (`required`, `deprecated`, `default=…`), so a requiredness spelling drift fails instead of demoting a field to `Option`; a parameter table with no `[required]` row fails, since OANDA documents `Authorization` on every endpoint. `tools/test_generate_tools.py` covers each failure path offline. |

## Outstanding, tracked separately

These are confirmed candidates whose repair is a separate, bounded change. Each has its own issue
with the path and the required outcome, as the law's final clause requires.

- [#9](https://github.com/SharurTrading/oanda-rs/issues/9) — the generators bind page content to
  output by position (the *i*-th HTML chunk to the *i*-th hardcoded method name) and silently drop
  parameters, fields, definitions, and variants they cannot match.
- [#10](https://github.com/SharurTrading/oanda-rs/issues/10) — `tools/check_coverage.py` cannot
  distinguish a real typed contract from a generator's string fallback, counts a definition as tested
  on a name mention rather than a decode, and only checks stream contracts for the two hardcoded
  stream method names.
- [#11](https://github.com/SharurTrading/oanda-rs/issues/11) — `tools/check_release_notes.py` drops a
  release row whose version cell is empty and can then report "current" while a newer version went
  uncompared.
- [#12](https://github.com/SharurTrading/oanda-rs/issues/12) — the verification tests lock in
  fallback behaviour: `tests/tagged_variants.rs` asserts a `type` value the serializer wrote rather
  than one the provider sent, and `tests/enum_variants.rs` round-trips any string through the
  `Unknown` arm, so a variant the generator lost is invisible.
