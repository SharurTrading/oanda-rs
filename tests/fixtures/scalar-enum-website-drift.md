<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Scalar enum website drift

`scalar-enum-website-drift.json` records the explicit differences between the
existing website-derived scalar fixtures and `spec/official/v20-openapi.json`.
The website remains authoritative under OA-CONTRACT-01. The `add` lists contain
website spellings absent from the pinned schema; `remove` lists contain pinned
spellings absent from the website. For a definition absent from the pinned
schema, `add` contains the complete website set.

The sole removed spelling is the pinned `UNITS_MIMIMUM_NOT_MET` typo; the website
spells it `UNITS_MINIMUM_NOT_MET`. Guaranteed-stop definitions, later closeout
reasons, financing days and instrument-specific guaranteed-stop modes account
for the remaining additions. The definition-page sources and pinned revision
are listed in `docs/coverage.md`.

This is independent expected data for `enum_variants.rs`. Changing model code,
removing a fixture, or removing an entire fixture group must not update this
file automatically. A contract change needs explicit website evidence and a
reviewed update to both the fixture and this drift record. The test compares
actual executed fixture values with the pinned schema plus these explicit
differences; duplicate fixtures also fail.
