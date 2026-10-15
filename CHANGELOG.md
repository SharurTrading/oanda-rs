<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

All notable changes will be documented here. This project follows Semantic Versioning after its
first public release.

## Unreleased

- Add a plain-language behaviour guide covering connection, data and execution nuances for
  callers that wrap this client.
- Add a credential-free admission owner that callers can share across clients using the same
  egress scope. It shares the 120 REST requests/s rolling admission window, cooldown, exit
  reserve, 20 active stream leases, and two connector starts/s while keeping account mutation
  fences per client and its clones.
