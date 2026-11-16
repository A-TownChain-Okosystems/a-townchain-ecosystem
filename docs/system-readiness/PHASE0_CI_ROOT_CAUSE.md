---
document_id: ATC-PHASE0-CI-RCA-001
title: Phase 0 — CI Root Cause Assessment
version: 0.1.0
status: ASSESSED
owner: A-TownChain-Okosystems
updated: 2026-11-16
standard: ATC-STD-MD-001
tracking_issue: https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/116
related: [ATC-BLOCKER-REGISTER-001, ATC-SYSTEM-READINESS-001]
---

# Phase 0 — CI Root Cause Assessment

> **Scope:** read-only investigation. No workflow was changed, no gate weakened, no run without jobs
> counted as success, no historical evidence overwritten.
> **Machine-readable twin:** [`PHASE0_CI_ROOT_CAUSE.yaml`](PHASE0_CI_ROOT_CAUSE.yaml).
> **Register entry:** BLK-P0-01 in [`BLOCKER_REGISTER.md`](BLOCKER_REGISTER.md).

## 1. Headline finding

From **2026-11-06T17:23Z until 2026-11-16 ~14:56Z** the organization's user-initiated events
(`push`, `workflow_dispatch`) and most `schedule` events produced runs with **zero jobs**
(`conclusion: startup_failure`, empty name, `path: BuildFailed`), while `pull_request` and
GitHub-managed `dynamic` runs (CodeQL, Code Quality) started normally in the same repositories,
on the same branches, under the same permissions.

**The incident recovered during 2026-11-16 without any workflow or configuration change in the
organization.** The decisive boundary is between 10:54Z (last failing push) and 14:56Z (first
starting push); every observed run after 14:56Z starts and produces jobs.

Consequence for the register: the earlier count of "19 repositories with startup failures" describes
the **historical** state. Of those, several now start again; the rest have **no run since recovery**
and are therefore *unverified*, not *broken*.

## 2. Method and snapshot

- Source: GitHub REST API (runs, jobs, check-runs, commits) plus local org snapshot (33 repos).
- Snapshot 2026-11-16: 33 repositories, HEAD SHAs identical to Annex A of the blocker register.
- Sampling: last 40 runs per repository; job counts for the last 8 runs per repository; workflow
  files parsed locally for all 33 repositories.

## 3. Failure classification (0.2)

| Class | Description | Repositories | Evidence |
|---|---|---|---|
| **A** | Initialization failure, zero jobs (pre-recovery) | 19 historical; see §7 for the current split | globus-os run `43662920291` (15.11., 0 jobs), atc-standards `43830111537`, a-townchain-os-docs `43830162044`, atc-node `43852994457` (all 16.11. pre-recovery) |
| **B** | Content failures in *running* pipelines | a-townchain (5 gates on one PR), atc-oracle (historical) | §6, run `43893675443` etc. |
| **C** | Evidence access gap (job logs) | a-townchain | logs HTTP 403 vs. HTTP 200 for globus-os `43895261915` |
| **D** | Pending verification (no run since recovery) | 16 repos (9 active, 7 archived/inactive) | §7 |

## 4. Common causes (0.3)

**Confirmed:** the failure is event-class-specific and time-bounded, org-wide, and independent of
repository content. It affects `push`, `workflow_dispatch` and most `schedule` runs, but never
`pull_request` or `dynamic` runs in the same repositories during the same period.

**Excluded, with evidence:**

| Candidate | Verdict | Evidence |
|---|---|---|
| Workflow YAML syntax | excluded | all workflow files in all 33 repos parse; a syntax error would hit one workflow, not CodeQL and not every repo |
| Actions disabled / allowed_actions | excluded | `actions/permissions`: `enabled: true`, `allowed_actions: all`; recovery without policy change |
| Billing / minutes | excluded | 805 included minutes, `netAmount 0.0`, all repos public (globus-os forensics 15.11.) |
| Runner availability | excluded | zero jobs are created; no runner is requested; no self-hosted labels used |
| Workflow definition / dispatch resolution | excluded | dispatch accepted (204), run correctly named, still zero jobs |
| Reusable-workflow references | excluded | all pinned org SHAs exist and contain the referenced file (verified at `f7656d5b`, `2783846f`, `a41d9a93`, `b4ec342a`, `05607bf5`); `review-conflict-free.yml@main` is a floating ref (risk noted, not the cause) |
| Commit signature / verification | **excluded (hypothesis refuted)** | post-recovery pushes started with `verified: false, reason: unsigned` (globus-os `43894367069`, atc-standards `43898277268`) |
| Repository-specific configuration | excluded | affected and unaffected repositories share the same workflow structure; `pull_request` started everywhere |
| Organization/repository policy | not supported | same repos, same rights, `pull_request` runs started |

**Open:** the GitHub-internal cause. The incident is consistent with a platform-side initialization
fault affecting user-initiated event classes; a support ticket text exists in
`globus-os/.atc/ci-forensics/2026-11-15-startup-failure.md`. Ticket status is an owner action (R-1).

## 5. Reproducibility (0.7)

| Time (UTC) | Repo | Event | Result | Run |
|---|---|---|---|---|
| 2026-11-15 01:24 | globus-os | push | startup_failure, 0 jobs | 43662920291 |
| 2026-11-16 08:18 | atc-standards | push | startup_failure, 0 jobs | 43830111537 |
| 2026-11-16 08:18 | a-townchain-os-docs | push | startup_failure, 0 jobs | 43830162044 |
| 2026-11-16 10:54 | atc-node | push | startup_failure, 0 jobs | 43852994457 |
| **2026-11-16 14:56** | atc-node | push | started (`success`) | 43891148071 |
| 2026-11-16 15:15 | globus-os | push | started (`success`) | 43894367069 |
| 2026-11-16 15:37 | atc-standards | push | started (`failure`, 2 jobs) | 43898277268 |
| 2026-11-16 17:05 | a-townchain-ecosystem | push | started (`success`) | 43914636413 |

Control: `pull_request` runs started continuously throughout the incident (e.g. a-townchain
`43893673792`, a-townchain-ecosystem `43914664210`), proving the workflows, permissions and runners
themselves were intact. The recovery happened without any change in this organization.

## 6. Repository-specific findings (0.4) and red-but-running pipelines (0.5)

### 6.1 Organization policy markdown defect (org-wide, single fix location)

`reusable-markdownlint.yml` (referenced by 7 repositories) checks out `.github@1c198f3c` into
`.org-github-policy` and lints it with the caller's markdown set. That policy file itself violates
the configured rules:

- `.org-github-policy/AGENT_MANIFEST.md:53-58` — 11 findings: MD056 (table column count, lines 50-58),
  MD058 (blanks around tables), MD022 (blanks around headings).

Result: **Markdown Lint fails in every consuming repository** — a-townchain, atc-contracts,
atc-engineering, atc-ide, atc-node, atc-shivacore, aurora-ai. Evidence: check-run annotations of
a-townchain run `43893675443` (step "Lint Markdown").

### 6.2 a-townchain — one Dependabot PR blocks five gates

All five red runs (15:11Z) belong to a single `pull_request` from
`dependabot/npm_and_yarn/modules/atc-blockchain/lucide-react-1.72.0` (sha `acce944f9`):

| Workflow | Failing step | Evidence |
|---|---|---|
| Markdown Lint | Lint Markdown | 11 annotations, policy defect (§6.1) |
| Ruff Code Quality | Ruff | exit 1, no annotation detail — logs HTTP 403 |
| ATC Code Quality (STD-081/082) | quality | exit 1 — logs HTTP 403 |
| Determinism Gate | determinism | exit 1 — logs HTTP 403 |
| ATC Test Suite | pytest, npm-tests | exit 1 — logs HTTP 403 |

### 6.3 Evidence access gap (Class C)

Job logs for a-townchain return **HTTP 403** ("Server failed to authenticate the request… signature")
while globus-os logs return HTTP 200 (7,597 bytes). Findings for four of the five gates can therefore
only be bound to run/job/step level, not to log content. This is itself an evidence defect (R-4).

### 6.4 atc-oracle — historical, not current

The last run (2026-10-07, push) fails in "Evidence-Bindung (SCR-0086 idempotentes test_run-Update)".
The repository is archived; the failure is historical and does not affect current readiness.

## 7. Historical vs. current state

| Status | Repositories |
|---|---|
| **Started after recovery (verified)** | globus-os, atc-node, a-townchain, atc-standards, a-townchain-ecosystem (push 17:05Z), atc-algorithm, atc-engineering, atc-vm, atc-zkp, atclang (schedule), globus-os (schedule) |
| **Unverified — no run since 14:56Z** | .github, a-townchain-os, a-townchain-os-docs, atc-contracts, atc-marketplace, aurora-ai, genesis-chronicles, genesis-franchise-factory, demo-repository (governance-excluded) |
| **Inactive / archived** | atc-explorer, atc-indexer, atc-interop, atc-mining, atc-oracle, atc-storage, atc-toolchain, atc-wallet |
| **Red but running (content)** | a-townchain (5 gates, §6.2) |

No repository is currently **proven** broken by the platform incident. `CI_VERIFIED` is still not
awarded anywhere: that requires a green run on the exact HEAD (registry: ATC-STD-003).

## 8. Repair plan and dependencies

| ID | Action | Depends on | Owner |
|---|---|---|---|
| **R-1** | Record the GitHub Support ticket status; confirm the incident window (06.11.–16.11.) or obtain the official cause | — | Owner (Michael) |
| **R-2** | Verification sweep: obtain one fresh run per unverified repo (natural push or approved dispatch); bind run IDs | R-1 (or next natural push) | RepositoryAgent |
| **R-3** | Fix markdownlint findings in `.github@1c198f3c` policy file (AGENT_MANIFEST.md) — unblocks Markdown Lint in 7 repos | — | DocumentationAgent + GovernanceAgent |
| **R-4** | Investigate job-log HTTP 403 for a-townchain (evidence infrastructure) | — | RepositoryAgent |
| **R-5** | Triage the Dependabot PR: reproduce the 4 remaining gate failures locally or via logs once R-4 is resolved | R-3, R-4 | QAAgent + CodingAgent |
| **R-6** | Update BLK-P0-01 evidence; award `CI_VERIFIED` per repo only on a green exact-SHA run | R-2, R-3, R-5 | RepositoryAgent |

R-3 and R-4 can run in parallel immediately. No bulk workflow repair is required or permitted.

## 9. Phase 0 acceptance status

| Criterion | Status | Note |
|---|---|---|
| All 33 HEAD SHAs current | **MET** | API snapshot 2026-11-16 |
| 19 startup failures individually examined | **MET** | last 40 runs + job counts per repo |
| Common causes proven or excluded | **MET** | §4, each candidate with evidence |
| Repo-specific causes documented | **MET** | §6.1-6.4 |
| Running-but-red pipelines separately assessed | **MET** | a-townchain, atc-oracle |
| Each finding bound to run/job/step/log | **PARTIAL** | log access 403 for a-townchain (§6.3) |
| Repair plan with dependencies | **MET** | §8 |
| Register and roadmap consistent | **MET** | BLK-P0-01 updated, cross-linked |

## 10. Governance note

A started run is not a green run, and a green run is not `CI_VERIFIED`. `CI_VERIFIED` requires a
successful run on the exact HEAD SHA with traceable evidence (registry ATC-STD-003). Historical
evidence is not overwritten; the incident window 2026-11-06T17:23Z–2026-11-16T14:56Z is recorded as
`INCIDENT` and any run from that window is marked historical.
