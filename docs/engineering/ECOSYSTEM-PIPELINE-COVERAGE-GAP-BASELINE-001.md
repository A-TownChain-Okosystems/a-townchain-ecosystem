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

## Run-level verification addendum — 2026-10-27

**PR head under test:** 987bf4ba779c7df1a9dddb4f31accb70e1d1fab5  
**PR:** [#237](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/237)  
**Observed event:** pull_request; run metadata confirms the head SHA above for the listed runs.

### Exact-SHA run outcomes

| Gate | Run ID | Observed outcome | Interpretation |
|---|---:|---|---|
| Repository Governance | [40557233764](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233764) | success; audit job terminal success | Run-level CI evidence for this SHA |
| Determinism Gate | [40557233515](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233515) | success; all five jobs terminal success | Still not functional determinism proof for a-townchain Rust because its test command is true |
| Ecosystem Integration | [40557233520](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233520) | success; workspace and contract-test jobs terminal success | Integration tests ran on this umbrella-repository revision |
| Blockchain Core Integration | [40557233717](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233717) | success; component checks and E2E jobs terminal success | Positive evidence for the tests this workflow actually invokes |
| Security Assurance | [40557233734](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233734) | success; exact-SHA assertions passed in applicable jobs | Partial assurance only: configured Miri targets were skipped and fuzz harness compile step was skipped |
| ATC Test Suite | [40557233618](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233618) | success; listed jobs terminal success | CI evidence for the umbrella repository's configured test suite |
| CodeQL | [40557233583](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233583) | in progress at inspection; Python and JavaScript/TypeScript jobs passed, Rust analysis still running | Not terminal; cannot mark complete |
| Ecosystem Release Readiness | [40557233616](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557233616) | in progress at inspection; Rust and JavaScript jobs still running, Python and repository-integrity jobs passed | Not terminal; cannot mark complete |

Security Assurance artifacts were bound to the same head SHA. Observed artifact digests:
- Unsafe inventory: sha256:d0352d709af3ba50e703f28671ca0c529e5765773d0c26e17864178cde307c16
- Dependency inventory: sha256:0443ebf98ae6a32f8c8e14415fed424b29f4d1d443d53a8666b1c64a14a12fbf
- Fuzz inventory: sha256:df7f31d01f3d1024f1bbe9b0d696b2c70759a69ed9bdc77b18e39f4cd411d48d
- Dependency graph: sha256:fa097054887f57f6dbfaea069cf5c08c8b029fcfa69decd952a7966ddab6592b

These digests identify the uploaded ZIP artifacts, not a claim that every finding inside them is clean. Step summaries were inspected; this addendum does not claim a full log-by-log security review.

### P-02/P-03 — source inventory, function catalog and SSOT

- The Source Symbol Inventory workflow is defined with exact-SHA checkout assertions, scanner tests, census generation, reconciliation, registry validation and artifact upload. It was not observed among the runs triggered for this documentation-only PR, so its current execution evidence remains RESIDUAL.
- The Component Function Catalog workflow has exact-SHA checkout assertions and taxonomy/spec validators. It was also not observed as a run for this PR.
- PR [#235](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/235) is merged and its recorded snapshot reported 54 existing catalog bindings: 42 blob matches, 10 blob drifts, and 2 unmapped/missing source paths. That snapshot explicitly did not establish symbol semantics or full public-function coverage; its source repository SHAs were pinned as of 2026-10-26 and require refresh before being treated as current.
- PR [#234](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/234) remains an open Draft. It reports a lexical binding correction and a two-file snapshot, not a complete exported-symbol census; its completeness remains RESIDUAL.
- The current source/function inventory and SSOT reconciliation are therefore not closed by the successful general test suite.

### P-04/P-05 — ABI and capability security

- The inspected umbrella workflows provide component builds/tests and security inventory, but the runs above do not alone establish cross-language ABI byte compatibility, transaction-domain agreement, signature compatibility, or the normative gas_price width.
- Capability denial, unknown-capability handling, revocation, and the Aurora wallet.export_key denial must be linked to current canonical source SHAs and explicit negative regression tests. No evidence in the listed umbrella runs closes those cross-repository assertions.
- Status: RESIDUAL; any disputed or unbound normative contract remains BLOCKED, not implicitly accepted.

### P-08/P-09 — E2E and central release authorization

- The Blockchain Core Integration run passed its configured SDK E2E, multi-node E2E, and multi-process network E2E jobs on this umbrella-repository SHA.
- The separate l1-network-e2e.yml workflow was not among the observed runs for this docs-only PR. Its presence must not be represented as a successful run for this SHA.
- The Release Readiness run was still in progress at inspection and is a quality/build gate, not by itself a central release authorization decision that consumes current cross-repository evidence.
- The catalog-driven build loop PR [#220](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/220) remains an open Draft; PR [#228](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/228), based on that branch, also remains an open Draft. Reconcile these existing changes before implementing overlapping orchestration or release logic.

### Updated decision

**P-01: PARTIAL** — several workflow runs and security artifact digests are verified against the exact PR head; two major runs were still in progress, and skipped security steps limit assurance.  
**P-02/P-03: RESIDUAL** — source census and catalog reconciliation require a fresh run and current canonical repository heads.  
**P-04/P-05: RESIDUAL / unresolved normative contracts BLOCKED** — explicit ABI and capability regression evidence remains to be reconciled.  
**P-08/P-09: RESIDUAL** — configured integration E2E jobs passed, but full architecture-level E2E closure and central evidence-consuming release authorization are not established.

No merge, release, or status promotion to VERIFIED is authorized by this addendum.


## Current-head follow-up — 2026-10-27

The PR head was re-read after the prior run snapshot. Current head: `4ea42fab2627dd4f78a62a941b7529377be68667`. PR #237 remains open, Draft, unmerged.

Current-head workflow runs observed:
- Governance run [40557844269](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844269): completed success; audit job terminal success.
- Determinism run [40557844316](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844316): completed success. The `a-townchain-rust` step is explicitly labeled a static scan; its success must not be described as functional Rust determinism testing. The previously inspected workflow command `--test-cmd "true"` remains a gap.
- ATC Test Suite run [40557844399](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844399): completed success at the workflow level; its observed component checks include tests for configured projects.
- Security Assurance run [40557844448](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844448): completed success, but Miri targets and fuzz-harness installation/compilation were skipped. Treat as partial coverage, not a complete security pass.
- Ecosystem Release Readiness [40557844414](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844414), CodeQL [40557844425](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844425), Ecosystem Integration [40557844355](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844355), and Blockchain Core Integration [40557844424](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/40557844424) were still in progress at the last inspection. Do not promote them to passed until their runs reach terminal conclusions.

Run-level job summaries for current head were inspected. This is not a complete raw-log review, and current workflow status can change after this snapshot.

### Updated P-01 disposition

**P-01 remains PARTIAL.** Governance, Determinism Gate, ATC Test Suite, and Security Assurance have terminal successful workflow results for current head `4ea42fab2627dd4f78a62a941b7529377be68667`. However, several relevant workflows were still running, and successful status does not close skipped steps or weak test commands. No merge or release authorization follows from this snapshot.
