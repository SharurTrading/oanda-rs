<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

All notable changes will be documented here. This project follows Semantic Versioning after its
first public release.

## Unreleased

- Add a credential-free admission owner that callers can share across clients using the same
  egress scope. It shares the local request budget, cooldown, exit reserve, stream leases, and
  completion-spaced connector pacing while keeping account mutation fences per client token.
