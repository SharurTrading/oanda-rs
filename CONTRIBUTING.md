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
cargo run --example exit_reserve --locked
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

Integration tests that need a loopback server, an HTTP response or a fixture client use the helpers
in `tests/support/mod.rs` (`mod support;`) rather than defining their own.

## Local build cache

Kache is optional and speeds up only local builds of this repository. Crates that
depend on this one never use it, and CI does not use it (it caches with
`Swatinem/rust-cache`). To opt in, install it and run its one-time setup, which
covers every repository on the machine:

```sh
cargo install kache --locked
kache init
kache doctor
```

[.kache.toml](.kache.toml) keeps the cache local-only, with no remote backends or
planner, and isolates this repository's entries under its own key salt;
`kache doctor` lists it as the config file. The
[Kache quick start](https://ninja.kunobi.com/docs/kache/getting-started/quick-start)
covers the rest.
