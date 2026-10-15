<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

All notable changes will be documented here. This project follows Semantic Versioning after its
first public release.

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
