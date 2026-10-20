<!-- SPDX-License-Identifier: MIT-0 -->

# Releasing oanda-client

Releases are cut from a pull request and published manually. The crate has no
credentialed release workflow; CI remains read-only. The release gate in
`AGENTS.md` governs the sequence — this guide is the operator checklist that
executes it. Track the 3.0.0 release in
[#80](https://github.com/SharurTrading/oanda-rs/issues/80) and leave that issue
open until publication and consumer verification are complete.

## Coverage gate

`docs/coverage.json` is the single source of truth for the operation and
definition set (OA-COVERAGE-01): every operation the six OANDA v20 endpoint
pages document, and every linked definition, carries a public typed method
or type, a typed success and rejection contract, and a deterministic test.
`python3 tools/check_coverage.py` recomputes that claim from the ledger and
must report every operation and definition tested before a release. There is
no code generator; provider types are hand-maintained, and a provider change
lands as a source edit plus a ledger update plus a wire fixture in one change.
The pinned OpenAPI revision in `spec/official/` is a cross-check only; the
website pages are authoritative (OA-CONTRACT-01), and drift between them is
documented in `docs/coverage.md`, never silently resolved.

Check both repository tags and crates.io before choosing the version.
Review the public API since the last published version and classify the
Semantic Versioning impact in the release PR: breaking changes to the typed
surface require a major version, additive typed surface a minor, and fixes
with no surface change a patch. Public provider identities are distinct
validated types (OA-ID-01), so a merged or renamed identity type is breaking
even when the wire format is unchanged.

## Release pull request

1. Start from an up-to-date `main` and create `release/X.Y.Z`.
2. Set the package version in `Cargo.toml` and `Cargo.lock`, and confirm the
   `publish` registry and `include` whitelist are unchanged. The whitelist
   ships only the release surface (`src/`, `examples/`, `tests/`, changelog,
   lockfile, license, README, security policy); repository tooling under
   `tools/`, the docs, the pinned spec, and CI never publish.
3. Write the `## [X.Y.Z] - YYYY-MM-DD` section in `CHANGELOG.md` from the
   changes since the last published version, dated with the actual release
   date, and update its comparison link. Before the first publication the
   changelog is intentionally empty; the 1.0.0 section is written in the
   1.0.0 release PR, not accumulated in advance.
4. Update the README installation, version, and assurance text, and confirm
   any badges point at the version about to be published.
5. Confirm OANDA's API terms permit the intended source distribution and
   branding, complete the secret and proprietary-content scan including Git
   history, and confirm all fixtures are synthetic and all documentation is
   public-safe (release gate steps 1–3).
6. Review and commit the intended release changes. Require a clean working
   tree before package validation so the inspected archive exactly matches
   a PR commit.
7. Run the full gates, then verify the publish archive:

   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
   python3 tools/check_coverage.py
   python3 -m unittest discover -s tools -p 'test_*.py'
   cargo deny check
   cargo audit
   cargo publish --dry-run --locked
   ```

8. Inspect `cargo package --list --locked`: no `tools/`, `docs/`, `spec/`,
   or `.github/` entries, and no missing source, example, or license file.
9. Open the release pull request. Start the configured review agent,
   address and resolve every review conversation, and obtain human
   (operator) review for any new dependency, credential handling, decimal
   precision, or CI action change (PROC-REVIEW). Do not tag or publish
   from the branch; any later commit requires the gates and archive checks
   to be rerun.

The `Release readiness` workflow (manual dispatch on `main`, with the
expected version as its input) reruns the same gates plus the registry dry
run against the merged commit; use it as the post-merge confirmation.

## Publish after merge

1. Confirm the release pull request's exact merge commit passed the required
   `quality`, supply-chain, and secret-scan checks on `main`.
2. Use a clean checkout of that merge commit and rerun the full gates plus
   `cargo publish --dry-run --locked`.
3. Before creating a public tag, verify crates.io owner authentication with
   `cargo owner --list oanda-client` and GitHub authentication with
   `gh auth status`. Bearer tokens for OANDA itself are never involved in
   publishing (OA-SECRET-01).
4. Publish the same clean tree with the crates.io owner credential:

   ```bash
   cargo publish --locked
   ```

5. Create and push an annotated tag on that exact published commit, without
   force:

   ```bash
   git tag -a vX.Y.Z -m "oanda-client X.Y.Z"
   git push origin vX.Y.Z
   ```

6. Create the GitHub release from the verified tag, marked Latest, using
   the matching changelog section as its notes.
7. Verify the new version resolves and compiles from crates.io in a fresh
   consumer project, that docs.rs builds it successfully, and that the
   README badges render against the published version. Only then close the
   release tracking issue.

Never move, replace, or reuse a published version tag, and never publish
from a dirty or unmerged worktree. If the crate is wrong after publication,
release a correction; yank only when the published version is unsafe to
select.
