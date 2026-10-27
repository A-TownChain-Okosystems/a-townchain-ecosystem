# Ecosystem Pipeline Coverage and Gap Baseline

**Assessment date:** 2026-10-27  
**Assessed repository:** `A-TownChain-Okosystems/a-townchain-ecosystem`  
**Base commit inspected:** `08df784788932df3f4392440b33b711a100c7a1f`  
**Decision:** `RESIDUAL` — this is a source/workflow-definition review, not a completed run-level verification.

## Purpose

Record the current pipeline surface before adding more workflows. Avoid duplicating existing gates, preserve SSOT ownership, and distinguish a workflow definition from successful evidence on an exact commit.

## Workflow inventory observed on `main`

The following 19 workflow files were present in `.github/workflows/` at the inspected revision:

- `atc-wallet-rust.yml`
- `blockchain-core.yml`
- `codeql.yml`
- `competitive-monitoring-validation.yml`
- `component-function-catalog.yml`
- `delivery-registry-validation.yml`
- `determinism-gate.yml`
- `ecosystem-integration.yml`
- `ecosystem-release-readiness.yml`
- `ecosystem-subtree-sync.yml`
- `governance-ci.yml`
- `knowledge-indexer.yml`
- `knowledge-system.yml`
- `l1-network-e2e.yml`
- `org-repository-integration.yml`
- `rust-format-branch.yml`
- `security-assurance.yml`
- `source-symbol-inventory.yml`
- `test-suite.yml`

This is a file inventory only. It does not establish that each workflow is enabled, required by branch protection, green, or complete for its intended scope.

## Confirmed coverage from workflow definitions

| Gate | Existing evidence in source | Assessment |
|---|---|---|
| Component build/test | `ecosystem-integration.yml` runs workspace check/test and `atc-ecosystem-integration` tests | Present; actual current-head results not verified in this review |
| Multi-language quality | `ecosystem-release-readiness.yml` scans Rust manifests, Python projects, JavaScript packages, and canonical component directories under `components/` | Present; does not by itself prove external canonical repository heads are integrated |
| Exact checkout SHA | `security-assurance.yml` and `component-function-catalog.yml` assert the expected checkout SHA | Present in these workflows; coverage is not universal by definition |
| Security inventory | `security-assurance.yml` records unsafe Rust, dependency inventory, fuzz-harness discovery, and configured Miri targets | Partial assurance; inventory/smoke checks are not proof that unsafe code is safe or fuzzing coverage is adequate |
| Catalog validation | `component-function-catalog.yml` runs taxonomy/spec validation and regression tests | Present; source-binding completeness and function-level verification remain separate acceptance requirements |
| Determinism | `determinism-gate.yml` covers selected component paths | Partial; the `a-townchain-rust` invocation uses `--test-cmd "true"`, so it does not execute a functional test command |
| E2E/network | `l1-network-e2e.yml` exists | Workflow definition present; exact-head run outcome and coverage require inspection |
| Governance | `governance-ci.yml` runs repository audit using `atc-standards` | Present; audit outcome must be checked per exact commit |
| Release readiness | `ecosystem-release-readiness.yml` exists | Not equivalent to a central release authorization gate that reconciles all cross-repository evidence |

## Gaps and required follow-up

### P0 — exact-SHA evidence reconciliation

1. Inventory all required checks and their trigger/event semantics.
2. For each candidate evidence record, bind PR head/source SHA, checkout SHA, run ID, job, terminal step/result, log and artifact digest.
3. Reject stale, queued, cancelled, skipped, incomplete, or wrong-event evidence where the contract requires a successful terminal run.
4. Reuse the existing engineering evidence handoff contract and schema; do not create a second competing evidence format.
5. Verify whether the cross-repository evidence consumer is present on the current default branch and determine its actual required-check coverage.

### P0 — source inventory and SSOT

1. Reconcile the component/function catalog with source census records at exact repository heads.
2. Require path, symbol, source commit and blob SHA for source-bound function claims.
3. Distinguish workspace packages from directory candidates and account for inline Rust modules, tests and feature gates.
4. Keep duplicate ownership and mirrored copies as explicit drift findings; the ecosystem repository remains the integration/evidence control plane, not a competing implementation SSOT.

### P0 — security and protocol contracts

1. Add/verify explicit negative regression tests for capability denial, unknown capabilities and revoked permissions.
2. Add cross-language golden fixtures for protocol bytes, transaction domain, signatures and SDK field widths where those implementations exist.
3. Treat any unresolved normative type decision (including `gas_price`) as blocked until the authoritative specification and implementations agree.
4. Verify exact-head status for the Aurora `wallet.export_key` denial regression before accepting its protection as current.

### P0 — composed E2E and release decision

1. Map E2E tests to real supported dependency edges; do not model the seven architecture blocks as a single linear chain.
2. Separate component tests, cross-repository contract tests and system E2E tests.
3. Create one central release decision that consumes evidence rather than re-running or duplicating every component implementation.
4. Require human approval after technical/governance gates; no auto-merge or release is implied by this baseline.

### P1/P2 — further assurance

- Supply-chain: lockfiles, dependency/license policy, vulnerability scanning and provenance verification.
- Recovery/fault injection: invalid inputs, timeouts, crashes, rollback and revocation.
- Platform: QEMU/x86-64 boot, memory and hardware abstraction paths for supported OS targets.
- Performance: explicit baselines and regression thresholds.
- Nightly: bounded fuzz campaigns, stress tests, feature matrices and long-running tests.
- Documentation/catalog drift: compare normative standards, inventory, catalog and implementation bindings.

## Immediate findings

1. **Do not create 16 independent workflows immediately.** First map the existing 19 workflows to a common gate contract and determine what is already reusable.
2. **The determinism workflow has a known weak test invocation:** the Rust scan for `components/a-townchain` passes `true` as the test command. A static scan may be useful, but it must not be represented as functional determinism verification.
3. **Release readiness is not global release authorization.** The inspected workflow scans the umbrella repository's `components/` tree; that alone does not prove compatibility with the current heads of all canonical repositories.
4. **Workflow presence is not evidence of success.** Run-level verification remains open for every gate until run/job/step results and exact SHAs have been inspected.
5. The catalog/build-loop work already represented by open PRs must be reconciled before introducing overlapping code. In particular, review the current status of the catalog/source-inventory work and the proposed catalog-driven ecosystem build loop before deciding whether to extend or replace anything.

## Status vocabulary

- `IMPLEMENTED`: implementation exists.
- `TESTED`: required functional tests were executed.
- `CI_VERIFIED`: required CI checks passed on the exact target commit.
- `E2E_VERIFIED`: required E2E tests passed.
- `VERIFIED`: all applicable evidence and governance requirements are satisfied.
- `RESIDUAL`: a known gap remains.
- `BLOCKED`: required evidence is missing, invalid, stale, or failed.
- `EXCLUDED`: explicitly excluded with a recorded rationale.

Unknown or uninspected status must never be promoted to success. This document does not assign `VERIFIED` to any pipeline because run-level evidence was not part of this source-definition review.
