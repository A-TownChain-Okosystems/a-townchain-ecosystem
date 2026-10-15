# ATC File Format Registry 001

**Registry state:** `BASELINE_RESIDUAL`  
**Release decision:** `BLOCKED`  
**Machine-readable registry:** [ATC-FILE-FORMAT-REGISTRY-001.json](ATC-FILE-FORMAT-REGISTRY-001.json)  
**Catalog SSOT:** [ATC-COMPONENT-FUNCTION-CATALOG-001.json](ATC-COMPONENT-FUNCTION-CATALOG-001.json)

## Purpose and authority

This registry inventories required file formats and their processing, security, ownership, and evidence requirements. It is control-plane metadata, not an implementation repository. Format parsers, writers, loaders, installers, and cryptographic operations must remain in their canonical component repositories. Normative cross-repository encoding and protocol rules belong in `atc-standards`.

The registry does **not** assert that listed capabilities exist. Format-specific records remain `RESIDUAL` unless source and exact-SHA evidence proves otherwise. The ATC-VM bytecode record is `BLOCKED` while the ATCB-versus-ATC1 conflict remains unresolved. Proposed extensions such as `.atctx`, `.atcblk`, and `.atcstate` are candidates, not normative formats.

## Required processing chain

For each format, reconcile the actual implementation and catalog contracts for:

1. Detection and safe file opening.
2. Parsing / decoding and strict structural validation.
3. Serialization / encoding and canonical representation where required.
4. Deserialization / loading with explicit size, depth, allocation, and resource bounds.
5. Integrity verification and signature verification where applicable.
6. Version compatibility, migration, and explicit rejection of unsupported versions.
7. Installation, update, rollback, or restore for executable, OS, package, and state-image formats.
8. Positive, negative, malformed-input, boundary, fuzz, compatibility, and regression tests.
9. Evidence capture bound to the exact source commit and tested scope.

Not every format needs every operation. Mark inapplicable capabilities `EXCLUDED` with a reason rather than inventing a placeholder implementation.

## Required record contract

Each format record must carry a stable `format_id`, name, extensions, family, maturity, canonical owner, schema/version policy, capability set, security profile, status, and evidence or an explicit residual reason. Binary formats additionally require normative magic bytes, version fields, length semantics, endianness, maximum sizes, resource bounds, and invalid-input behavior where applicable.

Each implementation function must be added to the existing `ATC-COMPONENT-FUNCTION-CATALOG-001.json` using that catalog's established IDs and schema. This registry is not a second function catalog. The function contract must identify the canonical repository, full 40-character commit SHA, source path, symbol, blob SHA, inputs/outputs, preconditions, failure semantics, resource limits, security boundary, and test evidence.

## Evidence and status rules

- A source symbol mapping proves location only; it does not prove functional correctness.
- `IMPLEMENTED` is not equivalent to `VERIFIED`.
- `VERIFIED` requires actual terminal evidence for the exact commit: workflow run, job, step, exit code 0, relevant logs/artifact digest, and scope match.
- Pending, skipped, inaccessible, or mismatched checks remain residual.
- Structural schema validation is not functional verification.
- Do not weaken CI gates, auto-merge, or mark release ready based on registry presence.
- Release remains `BLOCKED` until catalog reconciliation and all required implementation/security/conformance gates are independently evidenced.

## Immediate blockers

- Resolve ATCB vs. ATC1 in the normative bytecode specification before treating a binary decoder or encoder as canonical.
- Reconcile each listed format with the current source census and existing catalog to identify actual reader/writer/parser/validator/serializer/deserializer/signature-verifier/installer/migration symbols.
- Record absent capabilities as `RESIDUAL` or `BLOCKED`; record duplicate implementations as an ownership conflict until resolved by evidence.
- Validate the registry and catalog on the same pull-request head, then inspect exact-SHA workflow logs and artifacts. No validation result is claimed by this document.
