# A-TownChain Governance — Enforcement Baseline

**Date:** 2026-09-24  
**Scope:** `a-townchain-ecosystem`  
**Baseline:** 68/100 (provisional)

## 1. Purpose

This document records the current governance baseline and the controls that must be enforced by repository, CI, evidence, audit, and release mechanisms.

The score is an engineering assessment, not a certification or formal external audit result.

## 2. Current baseline

| Domain | Baseline |
|---|---:|
| Governance design | 80–85/100 |
| Governance implementation | 60–65/100 |
| Governance enforcement | 55–60/100 |
| Overall | **~68/100** |

The principal improvement target is enforcement of existing governance rules rather than adding more normative documents.

## 3. Enforcement chain

```
ATC Standards
    ↓
Registry / Lifecycle
    ↓
Roles / Approval
    ↓
Evidence
    ↓
CI Gates
    ↓
Repository Integrity
    ↓
Runtime / On-Chain Enforcement
    ↓
Audit / Release
```

## 4. Repository control anchors

The following artifacts are treated as required governance control anchors by CI:

- `components/atc-standards/registry/standards.yaml`
- `components/atc-standards/.atc/evidence/evidence.yaml`
- `components/atc-standards/.atc/lifecycle.yaml`
- `components/atc-standards/standards/governance-core/ATC-STD-CHANGE-001.md`
- `components/atc-standards/standards/audit/ATC-STD-AUDIT-001.md`
- `components/atc-standards/governance/ATC-STD-000.md`

The Release Readiness workflow now fails when these anchors are missing or empty.

## 5. Evidence enforcement

Rust Release Readiness diagnostics are retained as CI artifacts instead of being deleted after execution.

The workflow still prints the final diagnostic tail to the job log, but the complete gate output is preserved for audit and root-cause analysis.

This does **not** weaken any build, test, format, or clippy gate.

## 6. Remaining governance gaps

The following remain open until verified by implementation and evidence:

- complete enforcement of lifecycle transitions;
- complete repository-to-registry traceability;
- deterministic on-chain governance execution and restart reconstruction;
- validator/key persistence across restart;
- complete runtime enforcement of governance-critical policies;
- independent audit evidence for release decisions;
- resolution of the current ShivaCore compilation failure.

A documented rule is not treated as implemented merely because its specification exists.

## 7. Evidence rule

For future governance assessments:

**CLAIMED != PASS**

A control reaches PASS only when the repository contains the required implementation and the corresponding reproducible validation/evidence.

## 8. Next gates

1. Obtain the complete ShivaCore compiler diagnostics from CI artifacts.
2. Isolate and repair the first verified compiler failure without weakening the gate.
3. Re-run the complete Release Readiness workflow.
4. Verify governance evidence and repository integrity again.
5. Recalculate the governance baseline only from newly verified evidence.
