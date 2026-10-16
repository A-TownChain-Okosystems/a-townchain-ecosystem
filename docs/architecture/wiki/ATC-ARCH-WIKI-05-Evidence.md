# Chapter 05 — Verification and evidence

**Status:** `PROPOSED / NOT VERIFIED`

## Independent status axes

- Architecture: `PROPOSED`, `ACCEPTED`, `DEPRECATED`, `REJECTED`.
- Mapping: `MAPPED`, `UNRESOLVED`, `EXCLUDED`.
- Implementation: `IMPLEMENTED`, `NOT_IMPLEMENTED`, `UNKNOWN`.
- Verification: `VERIFIED`, `NOT_VERIFIED`, `BLOCKED`, `RESIDUAL`.
- Reconciliation: `MATCHED`, `MISSING`, `DUPLICATE`, `STALE`, `UNRESOLVED`, `EXCLUDED`.

Never collapse these into one status. A mapped or implemented function is not necessarily verified.

## G0–G9 acceptance gates

| Gate | Requirement |
|---|---|
| G0 | Scope, owner, boundaries and applicability |
| G1 | Inventory completeness and explicit dispositions |
| G2 | Unique evidenced canonical SSOT |
| G3 | Complete behavioral/interface contracts |
| G4 | Exact source mapping for implementation claims |
| G5 | Required tests and evidence pass on exact revision |
| G6 | Security boundaries, threat model and negative tests |
| G7 | Dependency and end-to-end integration evidence |
| G8 | Operations, resource limits, lifecycle and recovery |
| G9 | Independent review and explicit acceptance/residual decision |

Hard gates include unresolved security invariants, normative MUST requirements, consensus/protocol ambiguity, ambiguous production ownership and missing mandatory evidence.

## Evidence packet

A verification claim must include stable evidence/requirement/layer/sublayer/component/contract IDs; repository; full commit SHA; path and symbol; blob SHA when available; workflow Run, Job and Step; exit code; logs; artifact references and hashes; reviewer; ISO-8601 review time; result and rationale.

Evidence must match the exact tested source revision. Missing logs, skipped tests, absent exit codes or artifacts without verifiable hashes do not prove a pass.

## Review decisions

Use `PASS`, `PASS_WITH_ACCEPTED_RESIDUALS` or `BLOCKED` with rationale. Decisions apply only to the reviewed IDs and exact source/catalog revisions. Green CI alone is not verification.
