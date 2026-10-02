# A-TownChain Project Status

> Canonical human-readable status. Machine-readable counterpart: `governance/status.json`.
> Both files MUST stay consistent. CI enforces the machine-readable contract and the
> claim/evidence/residual cross-references.

## Status Model

| Status | Requirement |
|---|---|
| PLANNED | Goal described, no implementation required yet. |
| SPECIFIED | Normative contract / specification exists. |
| IMPLEMENTED | Code exists at an exact SHA. |
| TESTED | Relevant tests pass at that exact SHA. |
| VERIFIED | Exact-SHA test evidence + public logs/artifacts + independent reproduction or external audit. |
| BLOCKED | Required evidence, build, or gate is missing. |
| RESIDUAL | Known open deviation or limitation. |

**Rule:** green CI alone does NOT produce `VERIFIED`.

## Evidence Chain

```
Claim → Repository → Source SHA → PR/Commit → Workflow Run → Job → Step → Log
→ Artifact → Result → independent reproduction / external audit → VERIFIED
```

Evidence must identify an exact 40-character source SHA. Failed or incomplete gates
remain evidence of their actual result; they are never converted into PASS by this
framework.

## Component Snapshot

See `governance/status.json` for the authoritative matrix.

| ID | Component | Status | Residuals |
|---|---|---|---|
| ipcbus | IPCBus | IMPLEMENTED | RES-SHIVACORE-BASELINE |
| eventbus | EventBus | IMPLEMENTED | RES-SHIVACORE-BASELINE |
| shivacore-baseline | ShivaCore Baseline Build | BLOCKED | RES-SHIVACORE-BASELINE |
| governance | Dependency Review Residual | RESIDUAL | RES-DEPREVIEW-GATE |

## Current Evidence State

- EventBus/IPCBus exact SHA: `fb90684fa8bca395d29177e68b235d80f45ca7f8`.
- Exact-SHA CodeQL, Repository Governance, RustSec, and SDK Build/Test runs were successful.
- Exact-SHA Rust CI, GlobusOS System CI, ATC Test Suite, and ShivaCore Kernel Boot remain failed because of the ShivaCore/kernel baseline.
- Dependency Review remains a documented non-blocking residual because GitHub Dependency Graph support is unavailable.
- No component is marked `VERIFIED`.

## What this document is NOT

This file is not a marketing surface. It does not assert production readiness.
It maps claims to evidence and exposes residuals.

## Governance Files

- `governance/status.json` — machine-readable source of truth.
- `governance/CLAIM_TO_EVIDENCE.md` — claim-to-evidence mapping.
- `governance/RESIDUALS.md` — open residual register.
- `governance/ATTRIBUTION.md` — external project attribution register.
- `governance/SECURITY.md` — ecosystem-governance security register.
