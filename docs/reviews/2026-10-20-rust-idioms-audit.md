<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Rust idioms and patterns audit — `oanda-client`

## Final implementation dispositions — 2026-10-20

This section records the final decisions for issue #78 and supersedes the suggested
fixes and delivery plan in the historical audit below. The operator authorized one
combined remediation PR and delegated final dispositions to the implementer. There is
no release in this change: source-breaking APIs are documented for the next major
release, whose separate reviewed change must execute the release gate.

| Item | Final disposition |
| --- | --- |
| T1.1 | Fixed: remove all 87 stale import allowances and use exact imports. |
| T1.2 | Fixed: remove seven module-wide allow blocks; compiler-suggested documentation markup and item-specific justified expectations replace suppression. |
| T1.3 | Fixed: document all 30 fallible REST methods, including typed definitive body-read failures and mutation ambiguity. |
| T1.4 | Fixed: construct only used fixture identities; remove unused_variables suppression. |
| T1.5 | Fixed: test lint exemptions have explicit fixture or function-size reasons. |
| T2.1 | Fixed: all nine endpoint rejection messages are required strings. Missing documented evidence is Undecoded, not decoded absence. No observed drift is asserted. |
| T2.2 | Fixed: standalone Unchanged serialization errors; all four existing dependent-order fields omit it, preserving the distinct Clear/null instruction. |
| T2.3 | Fixed: concrete DTO and union serialization reject contradictory explicit discriminator tags instead of overwriting them. |
| T2.4 | Revised: retain optional raw DTO tags because the schema specifies defaults rather than required fields. The union requires the tag; contradictory explicit tags fail serialization. Requiring a broad OrderType would not itself enforce a subtype. |
| T2.5 | Fixed at the typed request boundary: reject simultaneous price/distance and preserve borrowed input. Keep raw snapshots independently optional to retain provider evidence; permit both omitted on modifications, as the endpoint's inheritance contract requires. A mandatory trigger enum was declined because it rejects valid partial modifications. |
| T2.6 | Fixed: bind the real account with if-let; no placeholder identity branch. The former fallback was unreachable, not an observed fabricated account. |
| T2.7 | Fixed alongside T2.1: remove rejection Default because required provider evidence has no meaningful empty default. Internal absence of callers alone was not grounds for removing a public implementation. |
| T2.8 | Retained: all five query defaults express valid provider omission/filter behavior. Removing them would needlessly break public API. |
| T3.1 | Fixed: fence/reservation sets, mutation guard, and public mutation errors carry AccountID. Response RequestID metadata was also narrowed to its validated identity when this exposed the same rule. |
| T3.2 | Fixed: both inconsistent client trade fields use ClientID; provider-page spelling/type discrepancy is recorded in coverage notes. |
| T3.3 | Fixed: both fixed-price trade state fields use TradeState, preserving unknown strings. This strengthens the Rust API; the website itself uses string. |
| T3.4 | Fixed: symbolic/decimal closeout enums use exact parsing and serialization. Typed outgoing admission rejects nonpositive quantities, while raw response decoding preserves them as evidence. The old client already validated strings before sending; no floating-point loss was demonstrated. |
| T3.5 | Retained: provider aliases are public documentation vocabulary and part of the coverage contract, not distinct units. They make no false newtype claim; replacing or removing them offers no correctness improvement. |
| T3.6 | Documented intentional policy: scalar Unknown spellings remain serializable so provider evidence round-trips. Request-specific admission rejects unsupported constrained values; the datetime builder rejects unknown formats. AcceptDatetimeFormat's serializer itself never rejected Unknown. |
| T3.7 | Retained and documented: optional pricing tags are raw provider fields; stream consumers use PriceStreamEvent's validated classification. Another duplicated discriminator enum would add no invariant. |
| T3.8 | Fixed: InvalidId retains its consumed value and a typed Empty/ControlCharacter reason. |
| T4.1 | Fixed: typed Validate implementations beside DTOs replace URL/JSON dispatch; REST and stream query preparation invoke them before admission or network access. Preserve prior legitimate checks, correct the invalid position-empty-body rejection, and reject conflicting dependent triggers. |
| T4.2 | Fixed: Timestamp parsing returns InvalidTimestamp variants. |
| T4.3 | Fixed: library-owned token strings use Zeroizing on both builder and last client owner drop. zeroize 1.9.0 was already locked transitively; direct use requires human dependency/credential review. Caller and HTTP-stack copies are outside this erasure claim. |
| T4.4 | Fixed: cooldown installation returns typed invariant failure without modifying poisoned state. HTTP 429 remains an explicit definitive outcome and the mutation guard disarms before reporting cooldown failure. Connector poison recovery retains admitted Instant state with a specific rationale. |
| T4.5 | Fixed: remove response Deref; callers explicitly access .body. |
| T4.6 | Fixed: add Timestamp::new and From<DateTime<Utc>> without formatting/reparsing. |
| T4.7 | Fixed: invalid built-in endpoint literals report Error::Invariant, not caller input failure. |
| T4.8 | Fixed: prepare one serialized wire value after typed validation, then use it for URL/body construction. |
| T4.9 | Fixed: named RequestParts replaces six positional execute arguments and None placeholders. |
| T4.10 | Fixed: immutable stream-record boundary calculation. |
| T4.11 | Fixed: serializer mutation is contained before the immutable serialized value is published. |
| T4.12 | Fixed: discriminator errors distinguish missing, non-string, and unsupported request tag with the observed value. |
| T4.13 | Fixed: fractional timestamps use integer arithmetic and serialization uses collect_str/Display; signed Unix fractions are exact and overprecision fails explicitly. |
| T5.1 | Fixed: README included in rustdoc; credentialed example is explained no_run, credential-free helper examples are actually invoked. |
| T5.2 | Fixed: execute exit_reserve in normal and release CI and document the gate. |
| T5.3 | Fixed: close quantities explicitly document default ALL, including omission of either position side. |
| T5.4 | Fixed: models module documents absence, defaults, unknown preservation, and exact decimals. |
| T5.5 | Fixed: tagged-union docs distinguish preserving response unknowns from rejecting unsupported request tags. |
| T5.6 | Retained: o/h/l/c are conventional provider-native OHLC names in a provider-native library. Renaming them is optional cosmetic API breakage, not a missing money invariant. |
| T6.1 | Fixed: aborted stream opening must return a cancelled JoinError. |
| T6.2 | Fixed: runtime fixture tables feed both round-trip checks and pinned-spec tagged completeness; remove source-text scanning and fixed windows. |
| T6.3 | Fixed: shared request framing, responses, fixture clients, admission servers, and clock helpers live in tests/support; scenario-specific behavior stays explicit. |
| T6.4 | Fixed diagnostics: absent arming stays optional; explicitly armed malformed configuration reports its specific cause without credential values. Original tests already failed through expect rather than silently passing. |
| T6.5 | Fixed: negative assertion tests use should_panic(expected), without process-global panic-hook changes. |
| T7.1 | Fixed: a model-local macro emits each scalar enum and both wire mappings from one documented spelling table. Independent fixtures check all 503 values and pinned-spec differences. |
| T7.2 | Retained convention: camelCase is the intended wire default, with explicit renames for exact provider names/acronyms. Redundancy today is not a correctness defect; removing the convention makes future ordinary fields default to incorrect snake_case. |
| T8.1 | Fixed: decode each borrowed record before compaction; no record-sized Vec clone. Existing hostile-input and ordering checks remain. |

### Routed observations

- Correct the ledger-derived wording to 30 REST operations plus two streams in README
  and coverage notes; retain 32 total operations and 168 definitions.
- Add the six missing SPDX headers.
- Add scalar completeness against the pinned spec with explicit website drift, including
  the pinned UNITS_MIMIMUM_NOT_MET typo versus the website spelling.
- Retain scalar DelayedTradeClosureTransaction.tradeIDs because both schemas print it;
  the prose/schema conflict is recorded in coverage notes. No unobserved array shape is
  invented, and an array fails decoding explicitly.
- Name the local stream heartbeat timeout without asserting a provider guarantee.
- Retain cancellation-safe connector pacing without a FIFO promise. No FIFO contract
  or reproduced rate-limit violation supports a behavior change; fairness is not silently
  advertised.
- Fix conservative over-fencing for definitive rejection body failures and decoded
  mutation success with unusable pagination. The former retains status/request identity
  and a typed source; the latter retains the successful body and Undecoded metadata.
- Document the edit sites for adding an order family alongside the new scalar spelling table.

### Additional exposed boundary correction

MutationGuard::new formerly inserted an in-flight account before a second fallible fence
lock. A failure there could abandon the reservation without an owner. All fallible locks
and the fence check now precede insertion while the reservation lock prevents competing
starts. The regression asserts the observable invariant error and unchanged reservation
state. Internal poison errors use Error::Invariant rather than caller InvalidInput.

The website also describes a price/distance choice for StopLossOrderRequest and
GuaranteedStopLossOrderRequest while explicitly marking price required. Those required
fields remain schema-faithful; distance-only direct requests cannot be represented without
a separately evidenced requiredness decision. Both-set requests fail typed admission.
The independently optional *Details types still support distance-only and inherited
modifications. This contradiction is recorded, not disguised as observed provider drift.

Final disposition author: `gpt-6.1-sol`. Model/endpoint/test implementation also involved
`gpt-6-astra` and `gpt-6.1-sol` subagents. The original audit below retains its original
authorship and findings as historical evidence.

---

A whole-repository review of the hand-maintained Rust in `oanda-client` against
[docs/rust-idioms-and-patterns-review.md](../rust-idioms-and-patterns-review.md), whose review
obligation is carried by **OA-IDIOMS-01** in [AGENTS.md](../../AGENTS.md).

| | |
| --- | --- |
| Repository | `SharurTrading/oanda-rs`, package `oanda-client` 2.0.0 |
| Audited revision | `main` @ `6359149`, clean working tree |
| Date | 2026-10-20 |
| Toolchain | Rust 1.95.0, edition 2024, `unsafe_code = "forbid"`, Clippy pedantic denied |
| Scope | All 41 hand-maintained Rust files: `src/` 18,932 lines, `tests/` 5,569, `examples/` 39 |
| Guide check IDs | `RP-*` from the review guide; `OA-*`/`LAW-*` from AGENTS.md where the contract is involved |
| Method | Six reviewers with disjoint file scopes; the Lead independently re-verified every BLOCKER claim and a sample of every other severity class |

`cargo fmt --check`, strict Clippy (`-D warnings`, pedantic denied, `unwrap_used`/`expect_used`/`panic`
denied in `src`), tests, rustdoc (`RUSTDOCFLAGS='-D warnings'`) and the coverage-ledger gate are green
at this revision. Per the guide, compiler and Clippy diagnostics are **not** reproduced as findings;
everything below is a judgement a compiler cannot make.

---

## 1. Verdict

The crate is materially more disciplined than a typical provider client. There is **no `unsafe`**, no
FFI, no `f32`/`f64`, no `unwrap`/`expect`/`panic` in production paths, every financial value is an exact
`rust_decimal::Decimal` parsed through `decimal_wire`, and the money-movement machinery (mutation
fencing, stream leases, rate admission) is built from correct RAII guards and evidence-preserving
enums. Most of what this audit found is **API-shape and repository-hygiene debt**, not correctness of
money movement:

* **1 BLOCKER** — a documented-required provider rejection field is modelled optional, which lets a
  malformed 4xx body be reported as decoded evidence.
* **27 distinct FINDING outcomes** (30 raw, with the repeated lint-suppression and close-units patterns
  consolidated) — lint-suppression shims, unused `Default`s, unvalidated request values, a latent
  `Patch` serialization hazard, missing `# Errors` docs, unexecuted examples, and test-purpose
  duplication.
* **22 NITs** — naming, temporary `mut`, allocation style, doc gaps, and stale allowances.

Raw reviewer counts (before de-duplication; raw labels are kept in parentheses throughout):

| Area | Scope | BLOCKER | FINDING | NIT |
| --- | --- | --- | --- | --- |
| A — transport core | `lib`, `client`, `admission`, `error`, `environment`, `patch`, `timestamp`, `decimal_wire`, `ids` | 0 | 8 | 6 |
| B — endpoint operations | `account`, `order`, `position`, `pricing`, `trade`, `transaction`, `validation`, `stream` | 1 | 7 | 1 |
| C — transaction models | `src/models/transaction.rs` | 0 | 5 | 1 |
| D — order/variant models | `src/models/order.rs`, `variants.rs` | 0 | 3 | 4 |
| E — remaining models | `primitives`, `account`, `trade`, `instrument`, `position`, `pricing`, `mod` | 0 | 1 | 7 |
| F — tests, examples, manifests, CI, docs | `tests/**`, `examples/**`, Cargo/CI/README | 0 | 6 | 3 |
| **Total** | | **1** | **30** | **22** |

### Headline findings, ranked

| # | ID | Check | Severity | Where | One-line outcome |
| --- | --- | --- | --- | --- | --- |
| 1 | T2.1 (B-1) | RP-IDIOM-OPTION, LAW-INVARIANT | BLOCKER | 9 rejection DTOs | Required `errorMessage` is `Option`+`default`; malformed rejection bodies decode as evidence |
| 2 | T2.2 (A-2) | RP-IDIOM-CTOR, LAW-INVARIANT | FINDING (escalation candidate) | `src/patch.rs:24` | `Patch::Unchanged` and `Patch::Clear` both serialize to `null`; only field attributes prevent "omit" → "cancel TP/SL" |
| 3 | T1.2+T1.3 (A-1, B-7) | RP-ANTI-DENY-WARNINGS, RP-IDIOM-DOCS | FINDING | `src/lib.rs:5-72` | Seven module-scope lint exemptions; `missing_errors_doc` hides 30 public methods with no `# Errors` section |
| 4 | T1.1 (B-6, D-1, E-1, C-6) | RP-ANTI-DENY-WARNINGS | FINDING | 15 files, 87 sites | Generator-era per-import `#[allow(unused_imports)]` shim masks 14+ provably dead imports |
| 5 | T2.3 (D-2) | RP-IDIOM-OPTION, LAW-INVARIANT | FINDING | `src/models/order.rs:1514` | Request DTOs carry a second `type` discriminator that the union serializer silently overwrites |
| 6 | T3.1 (A-4) | RP-PAT-NEWTYPE, OA-ID-01 | FINDING | `src/error.rs:125` | `AccountID` is stringified in fence state and three public error payloads |
| 7 | T4.1 (B-3) | RP-IDIOM-CTOR, LAW-INVARIANT | FINDING | `src/validation.rs` | Request validation is a second, stringly-keyed schema dispatched by URL suffix and JSON key; streams bypass it |
| 8 | T4.3 (A-3) | RP-PAT-BUILDER | FINDING | `src/client.rs:92` | Bearer token is never zeroized on drop or builder error (redaction itself is correct) |
| 9 | T8.1 (B-4) | RP-ANTI-CLONE | FINDING | `src/stream.rs:93` | One allocation+copy per stream record purely to dodge a borrow |
| 10 | T7.1 (C-1, D-6, E-8) | RP-PAT-YAGNI | FINDING | all models | 503 provider spellings are hand-written twice on top of the variants (1,590 lines in `transaction.rs` alone) |

What is strong (fully recorded in §6): mutation-fence RAII, no blocking lock across `await`, exact
decimal handling end to end, evidence-preserving `Supplied`/`Unknown` enums, fallible constructors
and builders with typed errors, redacted `Debug` for credentials, deterministic credential-free tests,
and a lint table that already denies the panic family in `src`.

---

## 2. How to read this report

* Every finding carries the guide's check ID and one of its three severities: **BLOCKER** (repository
  non-negotiable, UB/unsafe, or a required failure hidden), **FINDING** (correct but non-idiomatic,
  over-abstracted, safer idiom available), **NIT** (naming/placement/readability).
* `file:line` references are current at `6359149`. Each raw area report is preserved under
  `target/idioms-review/` (gitignored) for per-finding evidence and confidence notes.
* Findings are grouped by theme, not by file, so one mechanical change can close a whole cluster. Raw
  reviewer labels are kept in parentheses for traceability.
* Several fixes change public field/impl shapes. The crate follows SemVer (`README.md`: breaking API
  changes need a new major); those are marked **major** and can be bundled with a 3.0.0 decision. Fixes
  that only reject previously-silent invalid input are marked **fail-closed** and are safe sooner.
* **No grandfathering** (guide, "Anti-patterns"): presence on `main` is not a disposition. Every item
  below needs a fix or a tracked issue that states the idiomatic outcome.

---

## 3. BLOCKER

### T2.1 (B-1) — Required provider `errorMessage` is optionalized, defeating the `Supplied` evidence contract

* **Check:** RP-IDIOM-OPTION (BLOCKER) with LAW-INVARIANT and OA-CONTRACT-01
* **Severity:** BLOCKER
* **Location:** `src/account.rs:213`; `src/order.rs:113`, `:373`, `:473`, `:587`; `src/trade.rs:209`,
  `:330`, `:607`; `src/position.rs:247` — all nine endpoint rejection DTOs.
* **Evidence:**

```rust
    /// The human-readable description of the error that has occurred.
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
```

* **Why:** the six authoritative endpoint pages (re-verified by the Lead on 2026-10-20 at
  `developer.oanda.com/rest-live-v20/order-ep/`) mark every endpoint rejection schema
  `errorMessage : (string, required)` while `errorCode` is unmarked. The repository's own policy in
  [`docs/coverage.md`](../coverage.md) is "Field requiredness follows the website's `required` marking,
  except where OANDA's own payload omits the field", and such an omission must be recorded as observed
  drift. There is **no recorded observation** that an endpoint rejection omits `errorMessage`; the one
  documented drift in this area is `MarketOrderRejectTransaction.units` (CHANGELOG 2.0.0). Because the
  field is `Option`+`default`, a 4xx body missing a documented-required field still deserializes as
  `Supplied::Decoded(XRejection { error_message: None })` — reporting "this body matched the documented
  shape" and "OANDA sent no reason". That is exactly the absent-vs-undecoded conflation the
  `Supplied<T>` contract exists to prevent (law audit row 22), and it hides a required failure from a
  caller reconciling a refused mutation. `OperationError::Rejected::message` itself is correctly
  `Option` (read key-by-key from the raw body, `src/client.rs:288`), so the caller-facing absence
  contract is preserved either way.
* **Idiomatic fix (choose one and record it):**
  1. Make `pub error_message: String` in all nine structs so omission fails to decode and the body
     becomes `Supplied::Undecoded`; add a rejection-body `missing_required_fails` test mirroring
     `tests/definition_wire.rs`'s required-field negatives, and keep the existing fixtures (all nine
     rejection fixtures already send `errorMessage`). **Break:** public field type → next major.
  2. Or rule the robustness reading deliberate and record the observed omission in
     `docs/coverage.md` + CHANGELOG exactly as the `units` drift was recorded. Today the deviation is
     silent, which is the actual contract breach.
* **Effort/risk:** medium; every in-tree fixture already sends the field, so the fix is contained.
* **Confidence:** certain (requiredness, code shape, policy); the choice in *how* to record it is the
  operator's.

> **Escalation candidates to decide with the BLOCKER.** The Lead endorses raw A-2 (`Patch`, below) as
> the highest-value FINDING and would accept a BLOCKER reading if "a future field can silently cancel
> a take-profit/stop-loss" is treated as a money/account-state risk. OA-ID-01 identity questions
> (T3.1, T3.2) are BLOCKER-shaped only if that rule is read to cover error payloads and client-assigned
> ids, not just request/response field types.

---

## 4. FINDINGS by theme

### Theme 1 — Lint and suppression hygiene (RP-ANTI-DENY-WARNINGS)

#### T1.1 (B-6, D-1, E-1, C-6) — 87 per-import `#[allow(unused_imports)]` sites across 15 files

* **Check:** RP-ANTI-DENY-WARNINGS — FINDING (OA-IDIOMS-01)
* **Locations / counts:** endpoint modules `account.rs`, `order.rs`, `position.rs`, `pricing.rs`,
  `trade.rs`, `transaction.rs` each carry 7 = **42** sites (B-6); `src/models/{order}.rs` 5 (D-1),
  `src/models/{primitives,account,trade,instrument,position,pricing,pricing_common}.rs` 5 each = **35**
  (E-1); `src/models/transaction.rs` 5 (C-6). Representative `src/order.rs:2-16`:

```rust
// Shared imports vary by capability; unused ones are deliberately allowed.
#[allow(unused_imports)]
use crate::Timestamp;
...
```

* **Why:** a single template comment is not the one-line per-item justification the guide requires,
  and the code is no longer generated (`docs/plans/2026-10-13-remove-autogen.md`), so "imports vary by
  capability" describes nothing real. The allowances actively defeat `-D warnings`: 14 imports are
  provably dead in the endpoint modules (`Timestamp` ×4, `rust_decimal::Decimal` ×4,
  `crate::Result` ×6), `Timestamp` is dead in `primitives.rs`/`position.rs`/`pricing_common.rs`, and
  `pricing_common.rs` uses only 2 of its 5 imports. `src/stream.rs` and `src/validation.rs` already
  show the idiomatic shape: exact import lists, no suppression.
* **Idiomatic fix:** delete all 87 attributes and the template comment; prune dead imports and replace
  globs with explicit lists. `models/order.rs`'s `use crate::ids::*` is additionally redundant
  (`src/models/mod.rs:2` re-exports it through `use super::*`). One repo-wide sweep; the exact-import
  style already present in `stream.rs` makes this mechanical. Removing `wildcard_imports` exemptions
  then depends on T1.2.
* **Effort/risk:** small; imports only.
* **Confidence:** certain.

#### T1.2 (A-1) — Seven module-scope `#[allow]` blocks silence denied pedantic lints for seven modules

* **Check:** RP-ANTI-DENY-WARNINGS — FINDING
* **Location:** `src/lib.rs:5-10`, `:21-26`, `:28-32`, `:38-42`, `:47-51`, `:55-59`, `:64-68`
* **Evidence:**

```rust
// These endpoint contracts and provider descriptions are hand-maintained from OANDA's published pages.
#[allow(clippy::doc_markdown, clippy::missing_errors_doc, clippy::wildcard_imports)]
#[path = "account.rs"]
pub mod accounts;
```

* **Why:** the crate denies `clippy::pedantic`; these attributes hand seven modules an exemption from
  that table. Only the first block has a comment, and it justifies `doc_markdown` for
  hand-maintained provider text — it says nothing about `missing_errors_doc` (which hides T1.3) or
  `wildcard_imports` (which hides T1.1). The later six blocks have no comment at all, including
  `must_use_candidate`/`too_many_lines` for `models`. OA-IDIOMS-01 forbids grandfathering the shape
  simply because the de-generated code inherited it. A scoped multi-lint list is not a blanket
  allow — hence FINDING rather than the guide's blanket-allow BLOCKER — but each exemption still needs
  an item-level, justified outcome.
* **Idiomatic fix:** after T1.1/T1.3, delete these blocks entirely; backtick provider terms for
  `doc_markdown`, add `# Errors` sections for endpoint methods, import explicitly. If one lint is
  genuinely unattainable for a specific item, use a per-item `#[allow(..., reason = "...")]`-style
  justification, not a module-wide list.
* **Effort/risk:** medium (documentation + import churn, no behaviour change).
* **Confidence:** certain.

#### T1.3 (B-7) — Thirty public fallible endpoint methods have no `# Errors` section

* **Check:** RP-IDIOM-DOCS — FINDING (OA-DOC-01); root cause is T1.2's `missing_errors_doc` exemption.
* **Location:** all 30 `Client` methods in the six endpoint modules — `src/account.rs` (6),
  `src/order.rs` (7), `src/position.rs` (4), `src/pricing.rs` (3), `src/trade.rs` (6),
  `src/transaction.rs` (4); representative `src/account.rs:28-38` (`list_accounts` documents only
  "List accounts.").
* **Why:** each returns a two-layer contract — `OperationError::Client(Error)` and
  `OperationError::Rejected { status, code, message, body: Supplied<R> }` — plus mutation ambiguity and
  reconciliation errors for the nine mutations. The crate's own convention exists elsewhere
  (`src/stream.rs:311-314`, `ClientBuilder::build`), and `missing_errors_doc` would surface all 30 once
  T1.2 is removed. The guide lists missing `# Errors`/`# Panics` on public fallible items as a red flag.
* **Idiomatic fix:** one to two sentences per method (client-side error vs definitive rejection with the
  endpoint rejection body vs ambiguous mutation), then delete the module exemption so the compiler
  enforces it.
* **Effort/risk:** medium (30 mechanical sections); docs only.
* **Confidence:** certain.

#### T1.4 (F-3) — `unused_variables` is crate-root allowed to excuse pre-created fixture ids

* **Check:** RP-ANTI-DENY-WARNINGS — FINDING
* **Location:** `tests/operation_matrix.rs:2-8` (allow list), `:164-171` (representative closure)
* **Why:** `unused_variables` is the one rustc-level lint in that allow, and it exists only because all
  30 `check(...)` closures pre-create the same five ids and use at most two. The panic-family
  exemptions are the legitimate test-fixture allowance; a correctness-adjacent rustc lint is not, and
  it makes a genuinely unused binding invisible in the file.
* **Idiomatic fix:** module-level `fn account()/order_specifier()/trade_specifier()/transaction_id()/
  instrument()` helpers (or construct only what each closure calls), then remove `unused_variables`
  from the allow list.
* **Effort/risk:** small–medium (mechanical 30-closure edit, compile-checked).
* **Confidence:** certain.

#### T1.5 (F-7) — Six test allow blocks carry no justification comment — NIT

`tests/admission_scope.rs:6`, `enum_variants.rs:2`, `tagged_variants.rs:2`,
`definition_wire.rs:16`, `operation_matrix.rs:2`, `practice_read_only.rs:3` lack the one-line
fail-fast rationale that `client_contract.rs:5`, `close_flat_side.rs:11`, `open_fill.rs:10` and
`support/mod.rs:7` carry; the extra `too_many_lines`/`needless_pass_by_value` exemptions have no
recorded reason. Add the standard comment and name each extra lint's reason. (Covered in the NIT
table.)

### Theme 2 — Required/absent evidence and representable invalid state

#### T2.2 (A-2) — `Patch::Unchanged` and `Patch::Clear` serialize identically

* **Check:** RP-IDIOM-CTOR — FINDING with LAW-INVARIANT; **escalation candidate** (money path).
* **Location:** `src/patch.rs:24-31`; four dependent-order fields at `src/trade.rs:371-412`.
* **Evidence:**

```rust
impl<T: Serialize> Serialize for Patch<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unchanged | Self::Clear => serializer.serialize_none(),
            Self::Set(value) => value.serialize(serializer),
        }
    }
}
```

* **Why:** the three-variant API promises "omit" / "send null to cancel" / "set", but the type's only
  `Serialize` renders the first two as `null`. Four `skip_serializing_if = "crate::Patch::is_unchanged"`
  attributes are the only thing keeping "leave the take-profit/stop-loss unchanged" from becoming
  "cancel it". A fifth `Patch` field without the attribute, or a standalone
  `serde_json::to_value(patch)`, silently flips the money instruction; the type also fails to
  round-trip (`Unchanged` decodes back as `Clear`). This is exactly the "impossible branch is not
  permission to invent a business value" shape in LAW-INVARIANT.
* **Idiomatic fix:** make the omission explicit — `Unchanged => Err(serde::ser::Error::custom("Patch::Unchanged must be skipped by the enclosing field"))`,
  keep `Clear => serialize_none()`, `Set` unchanged; every current caller already skips before
  `serialize`, so shipped bodies are unaffected and a future missing annotation fails at serialization
  instead of clearing provider state. Add a regression test that `SetTradeDependentOrdersBody` with
  `Unchanged` fields omits those keys and a standalone `Unchanged` now errors. **Fail-closed**, no wire
  change for valid callers.
* **Effort/risk:** small.
* **Confidence:** certain (code); severity judgement recorded.

#### T2.3 (D-2) — Request order DTOs expose a second `type` authority that the serializer silently overwrites

* **Check:** RP-IDIOM-OPTION / LAW-INVARIANT — FINDING
* **Location:** `src/models/order.rs:1514-1515` (+ 7 more request structs: `:1597`, `:1694`, `:1802`,
  `:1910`, `:1964`, `:2033`, `:2094`); overwrite at `src/models/variants.rs:123-126` and `:229-232`.
* **Evidence:**

```rust
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OrderType>,
```

and then the union serializer re-inserts `"type"` from the matched variant.
* **Why:** each body's own doc says the field "Must be set to MARKET", yet the only send path wraps the
  body in `OrderRequest`, whose serializer discards that value and writes the variant's kind. A
  contradictory construction — `OrderRequest::LimitOrderRequest(LimitOrderRequest { r#type: Some(Market), .. })`
  — is silently normalized to `LIMIT` rather than reporting the disagreement, and every caller literal
  must spell a field with no effect.
* **Idiomatic fix:** one authority. If `body.r#type` is `Some(t)` and `t.as_str() != kind`, return a
  typed serde error naming both; or stop modelling the tag on request bodies and let the union own it.
  **Fail-closed**, valid requests unchanged.
* **Effort/risk:** medium; low risk.
* **Confidence:** certain (code); severity judgement.

#### T2.4 (D-3) — Response order DTOs model the "Always set" provider discriminator as optional

* **Check:** RP-IDIOM-OPTION — FINDING
* **Location:** `src/models/order.rs:39-41` (+ 8 more response structs: `:225`, `:371`, `:550`, `:738`,
  `:937`, `:1073`, `:1236`, `:1390`).
* **Evidence:** `/// The type of the Order. Always set to "MARKET" for Market Orders.` immediately above
  `#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")] pub r#type: Option<OrderType>,`
* **Why:** every checked-in fixture and every `Order`-union decode carries the field, and the provider
  text says it is always set, but the model accepts absence (decode yields `None`) and omits it again
  on standalone serialize. Provider-required data reported as absence is the RP-IDIOM-OPTION red flag.
* **Idiomatic fix:** make it `pub r#type: OrderType` for the nine response structs and drop `default`;
  a payload missing it then fails to decode. **Break:** public field type → next major.
* **Effort/risk:** medium; low runtime risk (strictness only for payloads the provider documents as
  impossible).
* **Confidence:** judgement — requiredness inferred from the in-file "Always set" wording and
  fixtures, not a fetched definition page.

#### T2.5 (C-2) — The documented one-of `price`/`distance` pair is two independent public fields

* **Check:** RP-IDIOM-CTOR — FINDING
* **Location:** `src/models/transaction.rs:4479-4488` (`StopLossDetails`) and `:4523-4532`
  (`GuaranteedStopLossDetails`), reused by order requests in `src/models/order.rs`.
* **Why:** both fields document "Only one of the price and distance fields may be specified", yet both
  are public `Option<Decimal>` with no constructor or builder, so
  `StopLossDetails { price: Some(..), distance: Some(..), .. }` is constructible and serialized
  straight into an order request with no local failure. `Default` is *not* the fix (all-`None` is not
  a valid stop-loss specification).
* **Idiomatic fix:** `enum TriggerPrice { Price(Decimal), Distance(Decimal) }` with a custom serde
  emitting exactly one key (wire shape unchanged), or fallible `at_price`/`at_distance` constructors
  with the raw fields private. **Break:** public field/type → next major, or open a tracked issue with
  that outcome.
* **Effort/risk:** medium; zero wire change, medium API risk.
* **Confidence:** judgement.

#### T2.6 (A-7) — A dead `map_or` arm can fabricate an empty account identity

* **Check:** RP-IDIOM-OPTION — FINDING with LAW-INVARIANT
* **Location:** `src/client.rs:262-270`
* **Evidence:**

```rust
        if mutation_account.is_some()
            && (status.is_server_error() || ...)
        {
            return Err(Error::AmbiguousMutation {
                account: mutation_account.map_or(String::new(), ToString::to_string),
            }
            .into());
        }
```

* **Why:** the enclosing condition already proves `is_some()`; the `None` arm exists only to satisfy
  the type checker and would put `""` into a public ambiguity error — an invented identity where the
  real one is known. The `is_some()`/`map_or` pair also hides the fact that the account is already
  available.
* **Idiomatic fix:** bind once — `if let Some(account) = mutation_account && (status.is_server_error() || ...) { ... }`
  — so the error carries the real id and no placeholder is expressible.
* **Effort/risk:** small; intended behaviour unchanged (dead arm only).
* **Confidence:** certain.

#### T2.7 (B-2) — Nine rejection DTOs derive `Default`, fabricating a wire-impossible rejection body

* **Check:** RP-IDIOM-DEFAULT — FINDING (plus RP-PAT-YAGNI)
* **Location:** `src/account.rs:193`; `src/order.rs:86`, `:346`, `:445`, `:559`; `src/trade.rs:196`,
  `:302`, `:524`; `src/position.rs:213`
* **Why:** with the required `errorMessage` (T2.1), an all-`None` rejection is provider evidence the
  schema cannot produce; and there are **zero call sites** for any `Default` anywhere in
  `src/`/`tests/`/`examples/` (`grep -rnE '(\.\.Default::default|::default\()'` returns nothing), so the
  derived impl is unused public surface that lets downstream construct a fake OANDA refusal. The five
  all-optional query/request `Default` derives are a different, defensible case (documented "no
  filter" contract) and are listed in the NIT table as unused-but-valid.
* **Idiomatic fix:** remove `Default` from the nine rejection structs; keep `Deserialize`/`Serialize`.
  Removing a public trait impl is technically breaking → next major, or an operator-approved minor.
* **Effort/risk:** small; compile-time only.
* **Confidence:** certain.

### Theme 3 — Identity and newtype discipline

#### T3.1 (A-4) — `AccountID` is degraded to `String` in mutation-fence state and public errors

* **Check:** RP-PAT-NEWTYPE (+ OA-ID-01) — FINDING
* **Location:** `src/client.rs:73-74` (`fenced`/`in_flight` are `HashSet<String>`; `MutationGuard.account: String`),
  `src/error.rs:125-141` (`AmbiguousMutation`, `ReconciliationRequired`, `MutationInFlight` store
  `account: String`).
* **Why:** the validated newtype exists so identity cannot be guessed, yet it is stringified at every
  fence/error boundary. Consequences: a public error is constructible with an unvalidated identity; a
  caller routing reconciliation by identity must re-validate or string-compare; each mutation pays
  several `to_string()` allocations. The in-repo tests match these variants with `..`, so the internal
  change is contained.
* **Idiomatic fix:** store `AccountID` in the fence sets, the guard, and the three error payloads (its
  `Display` keeps the same text). Internal half is non-breaking; the public error-field change is
  **major** — land it with the next major or open a tracked issue with that outcome.
* **Effort/risk:** small (internal) / medium (semver).
* **Confidence:** certain (code); semver handling is judgement.

#### T3.2 (C-4) — `clientTradeID` has two shapes: raw `String` and the provider `TradeID`

* **Check:** RP-PAT-NEWTYPE (+ OA-ID-01) — FINDING
* **Location:** `src/models/transaction.rs:4803` (`Option<String>`) and `:4880` (`Option<TradeID>`);
  the field doc says "Client ID" and the other **10** `client_trade_id` fields are `Option<ClientID>`
  (verified counts).
* **Why:** the authority page itself prints `string` for one definition and `TradeID` for the other, so
  the current shape is faithful to a provider-document inconsistency — but OA-ID-01 wants one
  validated identity type for identity, and the fix is wire-identical (`ClientID` and `TradeID`
  serialize/validate the same way). Record the page discrepancy as drift in `docs/coverage.md`.
* **Idiomatic fix:** type both `Option<ClientID>`. **Break:** public field type → next major; small.
* **Confidence:** fact certain; `ClientID` choice is judgement.

#### T3.3 (C-5) — `tradeState` is `String` where the crate already has `TradeState`

* **Check:** RP-PAT-NEWTYPE — FINDING
* **Location:** `src/models/transaction.rs:654` (`FixedPriceOrderTransaction.trade_state: String`;
  same defect at `src/models/order.rs:246`).
* **Why:** the doc describes a trade state, the crate models exactly that value as `TradeState`
  (`src/models/trade.rs:130`) with `Unknown(String)` preservation, and typing it is wire-identical. The
  authority page prints plain `string`, so this is a type-quality fix, not schema correction.
* **Idiomatic fix:** `pub trade_state: TradeState`. **Break:** public field type → next major; small.

#### T3.4 (B-5 + C-3) — "ALL or decimal quantity" close-out units are unvalidated `String`s

* **Check:** RP-PAT-NEWTYPE (+ OA-DECIMAL-01) — FINDING
* **Location:** endpoints/dto fields `src/trade.rs:150-151` (`CloseTradeBody.units`),
  `src/position.rs:123-124`, `:135-140` (`longUnits`/`shortUnits`); model copies
  `src/models/transaction.rs:4807`, `:4906`.
* **Evidence:**

```rust
    /// Indication of how much of the Trade to close. Either "ALL" or a DecimalNumber ...
    #[serde(rename = "units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
```

* **Why:** a closed set — `ALL` (and `NONE` for position close) or an exact positive quantity — modelled
  as any string, so `"abc"`, `"-1"`, `"1.2.3"` are representable and only rejected by the path-keyed
  validator (T4.1). The decimal branch never reaches exact `Decimal` parsing (OA-DECIMAL-01), and
  callers get no help formatting. The same concept is already precise elsewhere:
  `MarketOrderRequest.units: Decimal` with `decimal_wire`.
* **Idiomatic fix:** `enum CloseoutUnits { All, Units(Decimal) }` (add `None` for the position type)
  with a serde codec emitting the exact spellings; omission keeps the documented `default=ALL`
  contract; delete the string parsing from `validate_units`. **Break:** public field type → next major;
  wire-identical.
* **Effort/risk:** medium.
* **Confidence:** judgement.

#### Theme 3 NITs (detail in §5): E-3, E-4, E-7, A-9.

### Theme 4 — Constructor, validation-boundary, and guard design

#### T4.1 (B-3) — Request admission is a second stringly-keyed schema over serialized JSON

* **Check:** RP-IDIOM-CTOR — FINDING (plus RP-PAT-YAGNI, LAW-INVARIANT); largest design item.
* **Location:** `src/validation.rs:10` (signature), `:41-50`, `:97`, `:102-116`, `:137-148`,
  `:174-183`; bypass at `src/stream.rs:320-324`.
* **Evidence:**

```rust
pub(crate) fn validate(path: &str, query: Option<&Value>, body: Option<&Value>) -> Result<()> {
...
        if let Some(count) = query.get("count").and_then(Value::as_i64) {
            let maximum = if path.ends_with("/candles") {
```

* **Why:** validation does not operate on the typed request. `Client::execute` serializes query/body to
  `serde_json::Value`, then re-derives the endpoint identity from URL suffixes
  (`path.ends_with("/candles")`, `path.contains("/trades/")`) and field identity from wire spellings
  (`"pageSize"`, `"dailyAlignment"`, `"longUnits"`). Every `and_then(Value::as_str)` is a silent skip if
  a `Serialize` shape changes; e.g. `marginRate` is range-checked only because `Decimal` currently
  serializes as a string. The types themselves promise no invariant: `ClosePositionBody` permits both
  sides `None` or both `"NONE"`, `ConfigureAccountBody` permits `marginRate: Some(2)`, and each is only
  rejected after send-preparation. `stream_pricing` hand-rolls the same empty-instruments rule because
  `open_http_stream` never calls `validate`. The guide's constructor default is that the type boundary,
  not a JSON reparse, validates what the type promises.
* **Idiomatic fix:** put each request type's admission rule beside the type — fallible constructors
  (`ClosePositionBody::try_new` enforcing one live side) or an internal
  `trait ValidatedRequest { fn validate(&self) -> Result<()> }` called before serialization — and delete
  the path-suffix/JSON-key dispatch. Minimum step: a typed operation discriminator with an exhaustive
  `match` per body type so a new endpoint cannot silently inherit or skip a rule.
* **Effort/risk:** large; preserve every existing `Error::InvalidInput` outcome (tests pin them) and do
  not move provider-owned checks client-side. Sequence with T4.8 (double serialization) and T4.9
  (`execute` shape), which disappear with this fix.
* **Confidence:** judgement.

#### T4.2 (A-8) — `Timestamp`'s public parser returns an untyped `String` error

* **Check:** RP-IDIOM-CTOR — FINDING
* **Location:** `src/timestamp.rs:29-51` — `impl FromStr for Timestamp { type Err = String; ... }`
* **Why:** a public fallible constructor returns `"invalid OANDA timestamp"` / `"OANDA timestamp out
  of range"` strings, so callers cannot distinguish malformed input from out-of-range without string
  matching; the crate's own identity type models this correctly with a typed `InvalidId`. It is on the
  request path (`src/validation.rs:67,70`) and reached by every price/transaction record.
* **Idiomatic fix:** `#[derive(Debug, thiserror::Error)] pub enum InvalidTimestamp { Malformed, TooManyFractionalDigits, OutOfRange }`
  (thiserror already a dependency). **Break:** public error type → next major; small.

#### T4.3 (A-3) — The bearer token is never zeroized

* **Check:** RP-PAT-BUILDER — FINDING
* **Location:** `src/client.rs:92-94` (builder field), `:69` and `:526` (`Inner.token`); no `Drop` for
  `ClientBuilder`/`Inner`.
* **Why:** the guide's builder review says a credential must be kept out of `Debug`/`Display` **and
  zeroized**; redaction and move-only handling are done well (`Debug` impls at `:55-61`, `:104-111`;
  token moved, never cloned), but every `build` failure path drops `self.token` as ordinary heap memory
  and `Inner.token` is freed unwiped. `zeroize 1.9.0` is already resolved in `Cargo.lock` via the
  rustls/aws-lc-rs stack, so using `zeroize::Zeroizing<String>` is a direct-dependency declaration, not
  a new crate in the graph — still a dependency decision under PROC-DEP-DECIDE/PROC-REVIEW.
* **Idiomatic fix:** `zeroize::Zeroizing<String>` in `ClientBuilder` and `Inner`
  (`bearer_auth(self.inner.token.as_str())`); do not add a public token accessor. If the dependency
  declaration is declined, open a tracked issue stating the accepted outcome.
* **Effort/risk:** small; no API/wire change.
* **Confidence:** certain (no zeroization); severity judgement.

#### T4.4 (A-5) — Two production mutex-poison paths silently continue

* **Check:** RP-PAT-RAII — FINDING with LAW-INVARIANT
* **Location:** `src/admission.rs:185-196` (`install_cooldown`), `:302-311` (`admit_start`)
* **Why:** every sibling path reports poisoning as a typed error (`:113-124`, `:127-133`, `:140-145`,
  `:198-203`) and `src/client.rs:744-747` documents why `Drop` must instead keep going. `install_cooldown`
  is not `Drop` and its caller `execute` can propagate a typed error, yet it installs cooldown on
  poisoned state; `admit_start` swallows poisoning with no comment. The guarded values are plain
  `Instant`/`VecDeque`, so this is not memory unsafety — it is an inconsistent "report internal
  inconsistency" contract inside the rate-admission mechanism (OA-RATE-01 machinery).
* **Idiomatic fix:** make `install_cooldown` return `Result<()>` and map poison to
  `Error::InvalidInput("rate lock poisoned")` from `execute`. For `admit_start`, a typed error cannot
  be synthesized inside `Service::poll_ready`; keep `into_inner` but add the same one-line rationale as
  `MutationGuard::drop`.
* **Effort/risk:** small; safe direction (poison now refuses locally).
* **Confidence:** certain (code); impact judgement.

#### T4.5 (A-6) — `ApiResponse<T>` implements `Deref` to its body for ergonomics

* **Check:** RP-IDIOM-DEREF / RP-ANTI-DEREF — FINDING
* **Location:** `src/client.rs:42-47`
* **Evidence:**

```rust
impl<T> Deref for ApiResponse<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.body }
}
```

* **Why:** `ApiResponse` is a response envelope carrying `body` plus `request_id`/`next_page` evidence,
  not a pointer-like wrapper over borrowed data. The `body` field is public; the impl only lets field
  access fall through implicitly. All but three in-repo call sites already spell `.body`; only three
  test lines rely on the deref. The guide forbids a `Deref` that exists to shorten call sites on a
  non-pointer wrapper.
* **Idiomatic fix:** delete the impl and write `.body` at those test lines. **Break:** removal of a
  public trait impl → next major; small. Do not add `DerefMut`.

#### Theme 4 NITs (detail in §5): A-10, A-11, A-12, A-13, A-14, B-9, D-4, D-5.

### Theme 5 — Documentation and examples (RP-IDIOM-DOCS / OA-DOC-01)

#### T5.1 (F-1) — README's Rust examples are never compiled or run

* **Check:** RP-IDIOM-DOCS — FINDING (OA-DOC-01)
* **Location:** `README.md:82`, `:168`, `:191`; no `include_str!`/`#[doc = ...]` in `src/lib.rs`; zero
  fenced code blocks in `src/**` (verified).
* **Why:** the three fenced `rust` blocks are the crate's only executable teaching material, yet nothing
  wires them into a doctest or CI step: `Cargo.toml:11`'s `readme = "README.md"` is metadata, not a
  doctest target. `README.md:97` calls the quick-start block "the runnable read-only Practice example",
  and it is byte-identical to `examples/practice_account.rs`, so the two copies can drift silently
  (OA-DOC-01 requires README and code stay in sync).
* **Idiomatic fix:** `#![doc = include_str!("../README.md")]` on the crate root so `cargo test` compiles
  every fence; mark the credentialed quick start `no_run` with the stated reason; make the
  exit-reserve blocks call their helper functions so the described path executes; verify
  `RUSTDOCFLAGS='-D warnings'`. Small; docs presentation changes need a rustdoc check.

#### T5.2 (F-2) — The credential-free example is compiled but never executed

* **Check:** RP-IDIOM-DOCS — FINDING (OA-DOC-01, testing gap)
* **Location:** `.github/workflows/ci.yml:36-39` and `.github/workflows/release-readiness.yml:47-50`
  (only `cargo clippy --all-targets` and `cargo test`); `examples/exit_reserve.rs:20-25`.
* **Why:** `exit_reserve.rs` makes no network call and asserts behaviour in-process (rejects
  `set_exit_reserve(120)`, shares policy across clones), but CI only compiles examples. `README.md:185-186`
  states the configuration and runtime changes "are exercised by the credential-free configuration
  example", which no check verifies, so the example's assertions can rot silently.
* **Idiomatic fix:** add `cargo run --example exit_reserve --locked` to CI (deterministic, offline) and
  the matching CONTRIBUTING gate line; repeat in release readiness.

#### T5.3 (B-8) — The `default=ALL` close-body contract is absent from rustdoc

* **Check:** RP-IDIOM-DOCS — FINDING (OA-DOC-01, money path)
* **Location:** `src/trade.rs:146-151`, `src/position.rs:120-124`, `:132-140`.
* **Why:** the authoritative pages document `units`/`longUnits`/`shortUnits` as `default=ALL`: omitting
  the field closes **everything**, not nothing. The public rustdoc never says so, while sibling query
  DTOs do record provider defaults (`src/order.rs:153`, `src/pricing.rs:197`, `src/transaction.rs:31`).
  A caller reading only rustdoc can build `CloseTradeBody { units: None }` believing it inert.
* **Idiomatic fix:** add the default to the three field docs (and to T3.4's enum if it lands). Small,
  docs only.

#### Theme 5 NITs (detail in §5): E-5, D-7, E-6.

### Theme 6 — Test idioms

#### T6.1 (F-4) — An aborted stream-open task's join result is discarded

* **Check:** RP-PAT-COMMAND — FINDING
* **Location:** `tests/admission_scope.rs:342-345`
* **Evidence:** `let opening = tokio::spawn(...); ... opening.abort(); let _ = opening.await;`
* **Why:** the test claims cancellation releases the shared lease, but discarding the `JoinError` hides
  a panic in the spawned task, so the 19-lease assertions can pass for the wrong reason. The repo
  already uses the correct shape in the same situation (`tests/client_contract.rs:873`, `:923`, `:1055`:
  `stationary.await.expect_err("stationary task cancelled")`).
* **Idiomatic fix:** match the join result and require cancellation (assert `is_cancelled()`); else fail
  loudly. Small; test only.

#### T6.2 (F-5) — The tagged-variant completeness test greps its own source text

* **Check:** RP-PAT-YAGNI — FINDING
* **Location:** `tests/tagged_variants.rs:261-264` (call sites), `:274-294` (`fixture_kinds` scanner;
  `include_str!("tagged_variants.rs")` + a fixed 700-byte window around `"type":`).
* **Why:** reformatting, a marker in a comment, JSON longer than the window, or key reordering changes
  the derived fixture list without a behaviour change — the completeness check is coupled to source
  formatting rather than data.
* **Idiomatic fix:** one data table (`const ORDER_FIXTURES: &[(&str, Value)]`, or a `macro_rules!`
  fixture list) consumed by both the round-trip test and the pinned-spec membership test; delete the
  scanner. Medium; test-only.

#### T6.3 (F-6) — Integration tests bypass `tests/support/mod.rs`

* **Check:** RP-PAT-COMPOSE — FINDING
* **Location:** `tests/client_contract.rs:27-116`; `tests/admission_scope.rs:20-117`, `:177-183`;
  `tests/operation_matrix.rs:44-85`; compared with `tests/support/mod.rs:21-63`, `:75-81`.
* **Why:** `CONTRIBUTING.md:35-36` says integration tests needing a loopback server, HTTP response or
  fixture client use the shared helpers; instead the request framing is copied at least five times and
  the paused-clock spawn three times (while `admission_scope.rs:195-201` already isolates
  `keep_clock_frozen()`). Each copy is a place for framing/timeout behaviour to drift.
* **Idiomatic fix:** move `spawn_loopback`, `read_request`, `serve_responses`, `fixture_client_with`,
  `keep_clock_frozen` into `tests/support/mod.rs`; keep only genuinely special stubs as thin wrappers.
  Medium; test-only.

#### Theme 6 NITs (detail in §5): F-8, F-9.

### Theme 7 — Maintainability duplication (RP-PAT-YAGNI)

#### T7.1 (C-1, D-6, E-8) — Provider enum spelling tables are hand-written twice on top of the variants

* **Check:** RP-PAT-YAGNI — FINDING
* **Location:** representative `src/models/transaction.rs:3316` (`TransactionType`), `:4164`
  (`OrderCancelReason`), `:5509` (`TransactionRejectReason`): **16 enums / 393 variants / 1,590 lines**
  (`as_str` 755 + `Deserialize` 755 + `Serialize` 80 = 24% of the file). Same shape 7 enums in
  `src/models/order.rs:2167-2550` (D-6) and 15 enums in `src/models/primitives.rs`,
  `instrument.rs`, `pricing.rs`, `trade.rs`, `account.rs` (E-8). 393 of the crate's 503 scalar values
  live in `transaction.rs`.
* **Why:** every provider spelling is a literal twice (variant declaration on top, then `as_str`, then
  `Deserialize`), so adding or correcting one value means editing up to four places. The tables are
  currently consistent (C verified a variant↔`as_str`↔`Deserialize` bijection over all 393 values), so
  this is maintainability, not a defect; RP-PAT-YAGNI's test is met — a present problem, real
  implementations, and a simpler Rust form. The de-generation plan itself records
  "503 enum values … painful to edit by hand."
* **Idiomatic fix:** one file-local `macro_rules!` taking `Variant => "SPELLING",` rows and emitting the
  `#[non_exhaustive]` enum with `Unknown(String)`, `as_str`, `Serialize` and the preserving
  `Deserialize` from that single table. Public API/rustdoc unchanged; guarded by the 503-value
  round-trip suite (`tests/enum_variants.rs`) and wire fixtures. A macro for the 55 structs would be
  YAGNI — those are per-field provider data.
* **Effort/risk:** medium; low behavioural risk, non-breaking.
* **Confidence:** judgement (duplication certain; single-sourcing worth it is the judgement).

#### T7.2 (E-2) — `#[serde(rename_all = "camelCase")]` is dead on 31 model structs — NIT

All 287 fields in those 31 structs carry an explicit `rename`, so the container attribute never decides
a wire name; a new field added without `rename` would silently inherit camelCase instead of failing a
fixture. Removing the attribute is byte-identical today (explicit `rename` wins) and makes a forgotten
rename fail the wire suite. See §5.

### Theme 8 — Hot-path clone/allocation

#### T8.1 (B-4) — The stream decoder copies every record into a `Vec` to dodge a borrow

* **Check:** RP-ANTI-CLONE — FINDING
* **Location:** `src/stream.rs:93`
* **Evidence:**

```rust
            let line = self.bytes[self.start..end].to_vec();
            self.start = newline + 1;
```

* **Why:** the copy exists only because the cursor is advanced before the decoder call, but
  `decoder: fn(&[u8]) -> Result<E>` returns an owned `E`, so with NLL the decode can run on the
  borrowed slice first and the cursor mutations follow. On a live pricing stream this is one
  allocation+memcpy per tick (and per transaction record) — the "clone to satisfy the borrow checker"
  shape, with no ordering or limit change.
* **Idiomatic fix:** `let decoded = (self.decoder)(&self.bytes[self.start..end]);` before the cursor
  update; keep the empty-line branch separate so no empty slice is decoded. Behaviour-identical,
  covered by existing record/order/limit tests. Small.

---

## 5. NIT findings (complete list)

The guide's NIT disposition is optional; each item still needs either the small fix or an explicit
"not adopted" note in the review thread. Raw reviewer IDs in parentheses. The raw reviewer NIT total is
22; three raw NITs (C-6, D-6, E-8) are folded into T1.1/T7.1 and are listed below for traceability, and
T2.8 combines notes from two areas, so the unique outcomes below are 20.

| ID | Check | Location | Outcome |
| --- | --- | --- | --- |
| T1.5 (F-7) | RP-ANTI-DENY-WARNINGS | tests: 6 files | Six test allow blocks lack the one-line fail-fast justification and per-lint reasons the four sibling files carry; add them |
| T2.8 | RP-IDIOM-DEFAULT | `src/account.rs:106,247`, `trade.rs:19`, `pricing.rs:182`, `transaction.rs:19` | Five all-optional query/request `Default` derives are semantically valid but unused; keep (justified) or drop with the rejection `Default`s |
| T3.5 (E-3) | RP-PAT-NEWTYPE | `src/models/mod.rs:3-9`, `src/lib.rs:99-105` | `DecimalNumber`/`AccountUnits`/`PriceValue`/`DateTime` are 1:1 aliases used in no field or signature, so they do not create a unit boundary; apply them to field types (cost-free documentation) or drop them from the public API/ledger |
| T3.6 (E-4) | RP-PAT-YAGNI | `src/models/trade.rs:61-72`, `primitives.rs:315-323`, `instrument.rs:133-150` | Request-only enums carry an `Unknown` arm that can be serialized outbound with no local rejection; reject outbound `Unknown` (as `AcceptDatetimeFormat` already does) or document the forwarding policy |
| T3.7 (E-7) | RP-PAT-NEWTYPE | `src/models/pricing.rs:18-20`, `:198-200` | Pricing stream tag is a free-form `Option<String>` while every other provider tag is a typed enum and the stream re-parses literals; single-source a typed tag (or document wire-only intent) |
| T3.8 (A-9) | RP-IDIOM-ERROR-ARG | `src/ids.rs:6-9`, `:23-29` | `InvalidId` discards the rejected spelling and conflates empty vs control-character; carry the value/kind in the error (public error change) |
| T4.6 (A-10) | RP-IDIOM-CTOR | `src/timestamp.rs:11-17` | No infallible `Timestamp::new(DateTime<Utc>)`/`From`; callers must format-and-reparse for query fields; add the additive constructor |
| T4.7 (A-11) | RP-IDIOM-CTOR | `src/environment.rs:16-30` | Hard-coded endpoint URL parse failure is reported as caller `InvalidInput`; use the invariant error (or validate once) |
| T4.8 (A-13) | RP-IDIOM-MUT | `src/client.rs:180-188`, `:203-205`, `:345-348` | Request/query payloads are serialized twice and validation checks a different derivation than the one sent; send the prepared `Value` |
| T4.9 (A-14) | RP-IDIOM-CTOR | `src/client.rs:164-177` | Internal `execute` takes six positional params (four optional), forcing 44 `None::<&()>` placeholders at call sites; introduce a small `RequestParts`/split helper |
| T4.10 (B-9) | RP-IDIOM-MUT | `src/stream.rs:86-89` | Temporary `mut` on `end` outlives its single mutation; rebind immutably |
| T4.11 (D-4) | RP-IDIOM-MUT | `src/models/variants.rs:81`, `:192`, `:462` | Temporary `mut` outlives the one mutation in the three union serializers; scope and rebind |
| T4.12 (D-5) | RP-IDIOM-ERROR-ARG | `src/models/variants.rs:42`, `:159`, `:336`, `:185` | Discriminator decode errors conflate absent vs non-string and drop the rejected tag; report the observed name |
| T4.13 (A-12) | RP-IDIOM-FORMAT | `src/timestamp.rs:42-46`, `:53-57` | Two `String` allocations per fractional-unix parse and a `String` per serialize on a per-record path; digit arithmetic + `collect_str` |
| T5.4 (E-5) | RP-IDIOM-DOCS | `src/models/mod.rs:1` | Public `models` module documents none of the provider-native evidence contract (Option requiredness, unknown preservation, exact decimals); add a paragraph |
| T5.5 (D-7) | RP-IDIOM-DOCS | `src/models/variants.rs:5-8`, `:131-134` | Public docs omit the deliberate asymmetry — `Order`/`Transaction` preserve unknown payloads, `OrderRequest` rejects unknown types; say so |
| T5.6 (E-6) | RP-PAT-YAGNI (naming) | `src/models/instrument.rs:218-235` | Public candlestick fields are `o`/`h`/`l`/`c` while the wire letter is already in `rename`; `open`/`high`/`low`/`close` (next major) |
| T6.4 (F-8) | RP-IDIOM-OPTION | `tests/practice_read_only.rs:14-16` | Live-probe helper uses `Option` for both "not armed" and "armed but invalid", then panics with the wrong diagnosis; keep `Option` only for absence of env vars |
| T6.5 (F-9) | RP-PAT-YAGNI | `tests/tagged_variants.rs:200-221`, `tests/enum_variants.rs:1666-1681` | Hand-rolled `catch_unwind` + global panic-hook harness duplicates `#[should_panic(expected = ...)]` and mutates process-global state; use the attribute |
| T7.2 (E-2) | RP-PAT-YAGNI | 31 model structs, e.g. `src/models/primitives.rs:16` | `#[serde(rename_all = "camelCase")]` is dead because all 287 fields carry explicit `rename`; remove it so a forgotten rename fails the wire suite (byte-identical today) |
| T1.1 (C-6) | RP-ANTI-DENY-WARNINGS | `src/models/transaction.rs:3-11` | Five stale `unused_imports` allowances (all imports used today) — folded into T1.1 |
| T7.1 (D-6) | RP-IDIOM-CTOR | `src/models/order.rs:2167-2550` | Seven enums repeat spelling tables — folded into T7.1 |
| T7.1 (E-8) | RP-IDIOM-CTOR | 15 enums across model files | Same duplication — folded into T7.1 |

---

## 6. Checks that pass — justified deviations recorded

These were examined rather than assumed, and are deliberately **not** findings. They are recorded so
the next reviewer does not relitigate them, and so the report shows what the codebase gets right.

* **No `unsafe`, no FFI, no `unsafe extern`** anywhere (`unsafe_code = "forbid"`); the FFI checks
  (RP-IDIOM-FFI, RP-PAT-UNSAFE-MODS, RP-PAT-OBJECT-API, RP-PAT-WRAPPERS) are not applicable.
* **OA-DECIMAL-01 holds end to end.** Zero `f32`/`f64` in `src/`, `tests/`, `examples/`; all 104
  `Decimal` fields in `models/transaction.rs`, all 44 in `models/order.rs`, and the endpoint/model
  copies route through `decimal_wire`; `serde_json` `arbitrary_precision` preserves literal text and
  `Decimal::from_str_exact` precedes `from_scientific`, so numeric liquidity is exact with no float
  round trip (verified against `rust_decimal` 1.43.0 source).
* **`MutationGuard` is a correct inverted RAII fence** (RP-IDIOM-DROP / RP-PAT-RAII): armed on
  construction before the first await, dropped-armed fences the account, only success/429/definitive
  rejection disarm, `Drop` is panic-free (poison recovery justified by a comment and plain guarded
  data), it fences *before* releasing the in-flight reservation, and no `?`/early return can bypass it.
  No `let _ =` of a guard; no guard cloning.
* **No blocking `std::sync::Mutex` guard crosses an `.await`.** The rate lock is confined to the
  capacity-computation block, the connection-start lock is released before `sleep_until`, and
  `poll_ready` takes no lock.
* **`StreamLease` is a correct RAII guard** (private construction, `Drop` decrements with
  `saturating_sub`, held in `HttpStream.lease`, released by `close`/drop; cancellation tests cover it).
* **Cancellation-safe `Option::take` in `HttpStream::finish`** does not violate RP-IDIOM-MEM: no `?`/await
  while the placeholder is live, buffered bytes survive cancellation, and the generation ends
  explicitly on every error (`close()` is move-only and fused).
* **`Supplied<T>` and the raw one-key `ProviderError::read` are a well-designed evidence contract**
  (unique in this domain): absent vs decoded vs undecoded stay distinguishable, and `RequestID`/
  `Link`/rejection body all preserve that distinction. T2.1 is the one requiredness hole in it.
* **Open provider enums are justified** (RP-IDIOM-PRIVACY): provider enums use `Unknown(String)` with
  `#[non_exhaustive]` (roughly 50 such attributes in `src/`) and preserve the exact provider spelling,
  re-serializing it unchanged; `OrderRequest` rejects
  unknown *request* tags with a typed serde error; no crate path treats `Unknown` as absence, success
  or a default. Disposition recorded in [`docs/law-invariant-audit.md`](../law-invariant-audit.md).
* **No `Default` fabrication inside the models** (models contain zero derives); no `Default` on
  `Environment` or `Timestamp` (a default venue/instant would be a money hazard). `Default for
  Admission` correctly delegates to `new()`.
* **Clone inventory is clean:** 32 clones in transport (19 `client.rs`, 13 `admission.rs`), of which
  only six are production — `Arc` handles, immutable endpoint snapshots, and one Drop-ordering `String`
  clone justified by the fence-before-release ordering. No duplicated authoritative account/mutation
  state anywhere in the crate; response DTO `Clone` is a snapshot, not a second live copy.
* **No `DerefMut`, no infallible `From<Inner>`, no mutable accessor** on any validated wrapper; the
  only `Deref` is T4.5.
* **`ids.rs` is the right newtype shape** (RP-PAT-NEWTYPE/OA-ID-01 at the definition site): private
  field, fallible `new`, validating `FromStr`/`Deserialize`, no identity bypass; 15 distinct types.
* **`ClientBuilder` shape satisfies RP-PAT-BUILDER:** required fields enforced by the constructor,
  fallible validating `build`, no alternate public constructor, redacted `Debug`, token moved not
  cloned; `Admission` injection and exit-reserve validation are explicit.
* **Constructor/panic policy holds:** `src/` has zero `unwrap`/`expect`/`panic!` (lint-enforced and
  grep-confirmed); test-module allowances are cfg(test)-scoped with reasons.
* **Test suite is deterministic and credential-free** (OA-TEST-01): all HTTP tests bind
  `127.0.0.1:0`; no ambient credentials, real DNS, fixed ports or real sleeps; the three live probes
  are `#![cfg(feature = "live-tests")]` + `#[ignore = "..."]` + runtime `OANDA_READ_ONLY_PROBE` arming,
  Practice-only and read-only.
* **Test crate-root panic-family allows are legitimate** fail-fast fixture exemptions (the package lint
  table applies to all targets); only the extra `unused_variables` exemption (T1.4) and the missing
  justifications (T1.5) are non-idiomatic.
* **Lint configuration is the correct single-package equivalent** of the workspace table: no
  `#![deny(warnings)]`, no blanket `#![allow(warnings)]`, no `unsafe` allowance; `pedantic = deny` with
  `priority = -1`, `unwrap_used`/`expect_used`/`panic` denied, `missing_docs` warn, `unsafe_code`
  forbid; CI and `hooks/pre-commit` both run `-D warnings`.
* **Examples teach no forbidden idiom:** no `unwrap`/`expect`/`panic`/unsafe/floating-point money;
  `Box<dyn std::error::Error>` in example `main` is the conventional std shape; tokens are read from
  the caller's environment and never printed.
* **Docs are otherwise strong:** every public item documented; `# Errors` convention exists and is used
  by the builder/stream/transport items (T1.3 is the endpoint-module gap, not a crate-wide habit);
  `# Panics` sections are unnecessary because no public path can panic.
* **`#[expect(clippy::large_enum_variant, reason = "…")]`** on `Transaction` is the self-removing
  per-item form with a reason — the idiomatic allowance shape.

---

## 7. Observations outside the guide's checks (routed, not scored)

These are real, verified at this revision, and need an owner lane other than OA-IDIOMS-01. None of them
is an idiom finding.

1. **Coverage wording drift (OA-DOC-01/OA-COVERAGE-01).** `README.md:46` says "32 REST methods and 2
   HTTP streams" and `:254`/`:264`/`:282` say "32 REST operations", but
   [`docs/coverage.json`](../coverage.json) has 32 operations total = **30 REST (21 query + 9
   mutation) + 2 streams**, and `tests/operation_matrix.rs:1` itself says "30 documented REST
   operations". README should say 30 REST + 2 streams; a coverage claim is computed from the ledger,
   not inferred.
2. **SPDX headers missing** on `tests/definition_wire.rs`, `tests/enum_variants.rs`,
   `tests/operation_matrix.rs`, `tests/practice_read_only.rs`, `tests/tagged_variants.rs`, and
   `examples/practice_account.rs`; every other file in scope carries them.
3. **`tests/enum_variants.rs` has no pinned-spec completeness cross-check** (unlike
   `tests/tagged_variants.rs:224-272`), so the "503 documented scalar values" claim rests on the
   hand-maintained table alone.
4. **`DelayedTradeClosureTransaction.tradeIDs`** is typed scalar `TradeID` while its own description
   says "List of Trade ID's"; the authority page and pinned spec both print scalar, so the model is
   schema-faithful — but if OANDA sends an array, every such record fails to decode (loudly). Needs a
   live-fixture probe or tracked issue, not a speculative change.
5. **Stream heartbeat timeout is a hard-coded 30 s literal** (`src/stream.rs:216`) while the other
   hostile-input bounds are named constants; no guide check covers magic numbers.
6. **`ConnectionGate` has no FIFO fairness** — a queued waiter can be overtaken by a fresh connector
   start (`src/admission.rs:270-299`); concurrency review, not idioms.
7. **Over-fencing on defensible outcomes.** `src/client.rs:238-245` maps *any* body-read failure on a
   mutation to `AmbiguousMutation`, including after a definitive 4xx status, and a 2xx mutation with a
   malformed `Link` header is fenced at `:219-227`. Conservative in the safe direction, but
   OA-MUTATION-01 reviewers may want the fence to require genuine uncertainty.
8. **Hand-maintenance cost of provider order types.** Adding one order type is now a four-place edit
   (body struct, `OrderType` + impls, union variant + two match arms, fixtures); T7.1's macro fixes the
   enum half, but the coverage-ledger workflow should record the expected edit sites.

---

## 8. Recommended backlog

### Decision 0 — operator decisions before code (do these first)

1. **T2.1 requiredness policy.** Either make `errorMessage` required in the nine rejection DTOs
   (breaking → 3.0.0) or record the robustness reading in `docs/coverage.md` + CHANGELOG the way the
   `MarketOrderRejectTransaction.units` drift was recorded. Silence is the only unacceptable outcome.
2. **OA-ID-01 scope.** Decide whether error payloads (T3.1) and client-assigned ids (T3.2) fall under
   "public provider identities are distinct validated types". If yes, T3.1/T3.2 become BLOCKER-shaped
   and belong in the next major.
3. **SemVer plan.** Many fixes change public field/impl shapes (marked **major** above). Group them into
   a 3.0.0 plan, or approve individual exceptions; `README.md` commits to a major for source-breaking
   changes.
4. **`A-2`/`Patch` risk acceptance.** Decide whether the latent "omit becomes cancel" hazard is treated
   as a money-path BLOCKER; the fix is small either way.

### PR 1 — non-breaking hygiene (mechanical, one PR)

* Delete all 87 `#[allow(unused_imports)]` attributes + template comments; prune the 14+ dead imports;
  replace globs with explicit imports (T1.1).
* Add `# Errors` to the 30 public endpoint methods, then remove the module-scope lint exemptions from
  `src/lib.rs` (T1.2, T1.3).
* Remove `Default` from the nine rejection DTOs once the SemVer decision allows it (T2.7); keep the five
  query `Default`s or drop them as unused (T2.8).
* Delete dead `rename_all` from the 31 model structs (T7.2).
* `tests/operation_matrix.rs`: remove `unused_variables` allow and the pre-created ids (T1.4).
* Add the one-line justification comments to the six test allow blocks (T1.5).
* **Acceptance:** `cargo fmt --check`, strict Clippy, tests, rustdoc and coverage ledger green; no
  behavioural change.

### PR 2 — fail-closed correctness hardening (small, high value)

* `Patch::Unchanged` → typed serde error + regression test (T2.2).
* Stream decoder: decode before advancing the cursor (T8.1).
* `install_cooldown`/`admit_start`: typed error / documented rationale (T4.4).
* Mutation request DTO union: reject a mismatched explicit `type` instead of overwriting (T2.3).
* `tests/admission_scope.rs:345`: require `is_cancelled()` (T6.1).
* Token zeroization via `Zeroizing<String>` if the dependency decision approves (T4.3).
* **Acceptance:** all existing wire fixtures unchanged; new negative tests assert the typed failures and
  state preservation; `cargo test --all-features` green.

### PR 3 — docs and teaching material

* Wire README fences into doctests (`#![doc = include_str!("../README.md")]`, `no_run` + reason for the
  credentialed quick start); add `cargo run --example exit_reserve --locked` to CI and release
  readiness; document `default=ALL` and the `models` evidence contract; document the
  unknown-vs-reject asymmetry (T5.1–T5.4, T5.5).
* **Acceptance:** `RUSTDOCFLAGS='-D warnings' cargo doc` clean; README fences compile; the
  credential-free example's assertions run in CI.

### PR 4 — next major (public API shape)

* Required `r#type` on the nine response order structs (T2.4); `error_message: String` if Decision 0
  chooses it (T2.1); one-of `price`/`distance` (T2.5); `CloseoutUnits` enum (T3.4); `ClientID` for both
  `clientTradeID` fields (T3.2); `TradeState` for `trade_state` (T3.3); `AccountID` in fence/error
  payloads (T3.1); remove `ApiResponse` `Deref` (T4.5); `InvalidTimestamp` typed error (T4.2);
  `open`/`high`/`low`/`close` field names (T5.6).
* **Acceptance:** wire bytes unchanged for valid inputs; CHANGELOG documents each breaking change;
  README/upgrade notes updated; all 168 definition fixtures and 503 enum round-trips green.

### PR 5 — maintainability refactors (tracked issues acceptable)

* Enum single-table `macro_rules!` sweep across 503 scalar values (T7.1).
* Typed request-validation boundary replacing the path/JSON-key validator (T4.1); sequence with T4.8
  and T4.9, which disappear.
* Test infrastructure consolidation and fixture-table replacement (T6.2, T6.3).
* Tracked issues for the remaining NITs that are not taken immediately, each stating the idiomatic
  outcome — including the README coverage wording, SPDX headers, and the `tradeIDs` probe from §7.

---

## 9. Method, coverage, and limitations

**Execution.** Six teammates with disjoint file scopes audited the repository against the guide; each
wrote a raw report with check ID, severity, exact `file:line`, verbatim evidence, required outcome,
effort/risk and confidence. The Lead (i) read the guide and repository contract, (ii) defined the
scopes and cross-cutting questions (requiredness, `Default` classification, `Unknown` dispositions),
(iii) independently re-verified the BLOCKER (including fetching the authoritative order endpoint page
and the `docs/coverage.md` requiredness policy), the `Patch` mechanism, the dead `map_or`, the
`Default` derives, the stream copy, the admission poison paths, the `clientTradeID` counts, the
`Default` inventory, and the CI/example gaps, and (iv) consolidated duplicate patterns
across areas into single outcomes. No tracked file was modified; the raw reports are under
`target/idioms-review/` (gitignored).

**Coverage.** All 41 Rust files were assigned and read (18,932 `src/` lines, 5,569 test lines, 39
example lines), plus `Cargo.toml`, `rust-toolchain.toml`, `deny.toml`, both CI workflows,
`hooks/pre-commit`, `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, `docs/coverage.md`,
`docs/law-invariant-audit.md`, the de-generation plan, the pinned OpenAPI spec and the saved provider
definition snapshot. Model requiredness was checked against the recorded authority policy and fixtures
(the website pages are not offline); the transaction model was additionally cross-checked field-by-field
against a saved copy of the authoritative transaction definition page (0 type mismatches, 1 documented
drift, 10 documented observed-only fields).

**Limitations.**

* No `cargo`/CI command was run during the audit (the brief was review-only); compiler/lint statements
  are source-level, and the clean-Clippy premise is taken from the task facts.
* No live provider payloads were captured; the live endpoint pages were fetched for the BLOCKER check
  only, and a page change since the saved 2026-10-16 snapshot would not be visible offline.
* Judgement findings (marked "Confidence: judgement") estimate severity, not existence; the evidence
  was verified in every case.
* Refactor effort estimates are static; no benchmark was run for the hot-path allocation NITs.
* `docs/reviews/` is a new directory; the report is not part of the crates.io package surface
  (`Cargo.toml` `include`).

**Reproduction commands** (from the repository root):

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
RUSTDOCFLAGS='-D warnings' cargo doc --all-features --no-deps --locked
python3 tools/check_coverage.py
grep -rn --include='*.rs' '#\[allow(unused_imports)\]' src | wc -l        # 87
grep -rn --include='*.rs' -E '\b(f32|f64)\b' src tests examples          # no output
grep -rn --include='*.rs' '#!\[deny' src tests examples                  # no output
```

## Appendix — raw area reports

| Area | Raw report | Scope |
| --- | --- | --- |
| A — transport core | `target/idioms-review/A-transport-core.md` | `src/lib.rs`, `client.rs`, `admission.rs`, `error.rs`, `environment.rs`, `patch.rs`, `timestamp.rs`, `decimal_wire.rs`, `ids.rs` |
| B — endpoint operations | `target/idioms-review/B-endpoint-ops.md` | `src/account.rs`, `order.rs`, `position.rs`, `pricing.rs`, `trade.rs`, `transaction.rs`, `validation.rs`, `stream.rs` |
| C — transaction models | `target/idioms-review/C-model-transactions.md` | `src/models/transaction.rs` |
| D — order/variant models | `target/idioms-review/D-model-orders-variants.md` | `src/models/order.rs`, `variants.rs` |
| E — remaining models | `target/idioms-review/E-model-primitives.md` | `src/models/{primitives,account,trade,instrument,position,pricing,pricing_common,mod}.rs` |
| F — tests, examples, meta | `target/idioms-review/F-tests-meta.md` | `tests/**`, `examples/**`, Cargo/CI/README/CONTRIBUTING |
