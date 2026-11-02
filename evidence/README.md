# COMPONENT-001 — Component Implementation & Evidence Matrix

This directory contains the machine-readable audit matrix for the A-TownChain ecosystem.

## Files

- `component-implementation-matrix.json` — one row per canonical component function.
- `component-implementation-matrix.schema.json` — JSON Schema for validation.

## Status contract

`PRESENT → SPECIFIED → IMPLEMENTED → UNIT_TESTED → INTEGRATION_TESTED → EXACT_SHA_CI_VERIFIED → E2E_VERIFIED → RELEASE_VERIFIED`

A row is **not** promoted by documentation, filenames, issue text, or historical CI. Evidence must point to the actual source and the exact relevant test/CI/E2E artifact.

`BLOCKED` is used when a required verification layer cannot currently be established (for example an unavailable external CI dependency). `UNASSESSED` means the audit has not yet established the state.

## Evidence requirements

- **SPECIFIED:** canonical specification/contract reference.
- **IMPLEMENTED:** executable source reference at the audited commit.
- **UNIT_TESTED:** relevant passing unit/property/negative/fuzz evidence.
- **INTEGRATION_TESTED:** passing cross-component test evidence.
- **EXACT_SHA_CI_VERIFIED:** CI run/job/log tied to the exact audited commit SHA.
- **E2E_VERIFIED:** real end-to-end execution path and artifact/log evidence.
- **RELEASE_VERIFIED:** release artifact plus reproducibility/validation evidence.

The matrix is an audit instrument, not an implementation-status claim.
