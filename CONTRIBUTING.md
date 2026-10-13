<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Contributing

Contributions are welcome. Changes must follow `AGENTS.md` and pass:

Optionally install the local pre-commit hook (fmt check + strict Clippy) with:

```text
git config core.hooksPath hooks
```

Hooks can be bypassed with `git commit --no-verify`; the CI run remains the authoritative gate.

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps --locked
cargo test --all-features --locked --no-fail-fast
cargo package --locked
cargo deny --locked check
cargo audit --file Cargo.lock --deny warnings
python3 tools/check_coverage.py
python3 -m unittest discover -s tools -p 'test_*.py'
gitleaks git --no-banner --redact --log-opts="--all" .
```

Never add live credentials, captured account data, private-platform dependencies, or generated
provider tokens. Tests must be deterministic and use synthetic fixtures unless they are explicitly
ignored read-only live probes.
