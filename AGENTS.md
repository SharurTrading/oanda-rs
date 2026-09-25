# OANDA Rust Client Guide

This repository contains the standalone `oanda-client` library. Rules have stable IDs so reviews can cite them.

## Boundary and API

- **OA-BOUNDARY-01:** Own OANDA v20 transport, provider models, and protocol behavior. Do not depend on SharurPlatform or own its routing, risk, portfolio, persistence, or UI.
- **OA-CONTRACT-01:** The six current OANDA v20 endpoint pages and their linked definitions are the implementation authority. The pinned OANDA OpenAPI revision is a cross-check; document drift rather than silently using obsolete operations.
- **OA-COVERAGE-01:** Every operation in `docs/coverage.json` needs a public typed method, a typed success and rejection contract, and a deterministic test. A coverage claim must be computed from the ledger, not inferred from generated code.
- **OA-DECIMAL-01:** Financial values use exact `rust_decimal::Decimal` parsing and serialization. Accept OANDA decimal strings and documented numeric liquidity without a floating-point round trip.
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

## Procedure rules

- **PROC-ATTRIB:** An AI agent posting to GitHub under the operator's login — a PR description, an
  issue, a comment or review reply, an inline comment, a release, any other publication — states the
  EXACT MODEL that authored it in the artifact's own body; a footer line naming the model is the
  usual shape. The post carries the operator's identity while speaking with the agent's judgment,
  and a reader — the operator's future self, a reviewer, an auditor — is owed the distinction
  between the operator's voice and the machine's. The attribution names the model identifier the
  harness reports (e.g. `GLM-5.3`), never a generic "an AI" and never the harness or client
  standing in the model's place, and it lives in the text every reader sees: a machine-readable
  trailer the GitHub UI hides is not disclosure.
- **PROC-ISSUE-TRIAGE:** Every issue is classified when it is created, and a mis-classified one is
  corrected whenever it is touched. Four things, all mandatory: the native issue type (exactly one
  of Bug, Feature, Task), the kind label that spells it (bug / enhancement / task — a fixed 1:1
  mapping onto the type, so a reader filtering by label and a reader filtering by type see the same
  set of work), and the Priority ISSUE FIELD on the issue itself (Urgent / High / Medium / Low),
  defined once at the ORGANIZATION level. It is an issue field — not a project field, and never a
  label: the value travels with the issue instead of living on one board's item, it holds exactly
  one value that is re-set as urgency changes rather than accumulating stale ones, and a label
  would be free to disagree with it. It is read and written through the issue under ordinary repo
  scope — GET/POST on the issue's issue-field-values, with the field id from the organization's
  issue-fields — so a lane that can read the issue can read its priority, and no project scope is
  involved. The priority ladder is: Urgent — the platform is wrong about money, orders, or account
  state right now, or a live session is blocked; other work stops for it. High — it blocks the next
  live session, or the next step of an active plan. Medium — ordinary work, and the DEFAULT: an
  issue nobody has argued is urgent, high, or low is Medium, never unset. Low — polish, nits, and
  anything deferrable without loss. The fourth is the difficulty label — exactly one of
  `difficulty: hard` / `difficulty: medium` / `difficulty: easy` (operator direction 2026-09-18) —
  and it routes the issue to the class of agent that should take it, which is why it is a label
  where priority is a field: it is working state the repository's own issue list filters by, not a
  fact that must travel with the issue. The difficulty ladder is: hard — a frontier agent:
  architecture or identity refactors, money-path and reconciliation semantics, concurrency or
  lifecycle decisions, research-heavy evidence work, wide cross-crate changes. medium — a strong
  coding agent: real engineering on a bounded surface the issue itself already specifies. easy — a
  basic coding agent: mechanical, well-scoped work with a clear acceptance check; operator-only
  trackers awaiting credentials or a decision are easy when no implementation work is required.
  Difficulty is judged from the issue's own scope at creation and re-set whenever understanding
  changes. A documentation label sits beside the kind label when the work is docs, and any other
  repository label is welcome; none of those is required. An issue missing one of the four is a
  finding on the next PR that touches it, and in the tracker it is a gap the operator is asked to
  fill rather than a state to leave standing.
- **PROC-PR-TRIAGE:** A PR carries the same classification as the issue it closes, so the tracker
  and the PR list read as one body of work: exactly one kind label (bug / enhancement / task,
  matching what the diff changes), and the same Priority issue field set to its linked issue's
  value — a PR is an issue to the field API, so a reviewer reads the value from the PR itself
  rather than taking it on trust. The label is not redundant with the issue's type — native issue
  types exist on issues only, so on a PR the kind label IS the type, and a PR that omits it is
  unclassified however well its issue is labelled. The priority is INHERITED, never re-argued: a PR
  whose value differs from its issue's is a finding, and a genuine disagreement is settled on the
  issue, where the ladder lives. A PR closing no issue sets its own priority by that same ladder,
  and says in its description why it closes none. The value is never restated as prose in the
  description: one field, one home, and a copy could only be free to disagree with it.
- **PROC-REVIEW:** All work lands via PR into main. The operator manually starts the configured
  review agent for every PR; its findings cite rule IDs from this file. Every review conversation —
  inline, including outdated threads, and recommendations outside the diff in review-summary
  comments — receives an explicit disposition and is resolved before merge, whether relevant or
  adopted or not: relevant adopted findings are fixed, while irrelevant or declined findings are
  resolved with the recorded reason and require no unnecessary code. Human (operator) review is
  required for mutation and account reconciliation behavior, credential handling, decimal
  precision, and any new dependency. One FEATURE per PR (operator ruling 2026-09-02): a PR is split
  only when its verification needs separate diffs — a pure-move proof, a red-first pin that must
  land before the change it guards — or when parallel lanes need disjoint files; never by step count
  or description length. A plan's steps are the implementer's checklist, not PR boundaries. A plan
  whose groups only make sense together lands as ONE PR when the operator rules so; the split
  criteria above govern everything else.
