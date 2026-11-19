---
document_id: ATC-PHASE0-CI-RCA-001
title: Phase 0 — CI Root Cause Assessment
version: 1.0.0
status: FINAL (historical)
owner: A-TownChain-Okosystems
updated: 2026-11-16
standard: ATC-STD-MD-001
tracking_issue: https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/116
related: [ATC-BLOCKER-REGISTER-001, ATC-SYSTEM-READINESS-001, ATC-PHASE0-CI-RECOVERY-VERIFICATION-001]
---

# Phase 0 — CI Root Cause Assessment

> **Scope:** read-only investigation. No workflow was changed, no gate weakened, no run without jobs
> counted as success, no historical evidence overwritten.
> **Status decision (Owner, 2026-11-16): PARTIAL — 7 of 8 acceptance criteria MET. `CI_VERIFIED` = 0.**
> **IMMUTABLE (Owner decision 2026-11-16):** this document is the historical root-cause record.
> It is frozen; corrections and operational progress belong to the successor
> `ATC-PHASE0-CI-RECOVERY-VERIFICATION-001`.
> **Machine-readable twin:** [`PHASE0_CI_ROOT_CAUSE.yaml`](PHASE0_CI_ROOT_CAUSE.yaml).
> **Register entry:** BLK-P0-01 in [`BLOCKER_REGISTER.md`](BLOCKER_REGISTER.md).

## 1. Headline finding

The original 19 `startup_failure` reports were **not 19 independent repository defects**. They are
predominantly symptoms of a **time-bounded, organization-wide GitHub Actions initialization incident**,
plus four genuinely independent residual defects (§8).

The incident produced runs with **zero jobs** (`conclusion: startup_failure`, empty name,
`path: BuildFailed`) for user-initiated events (`push`, `workflow_dispatch`) and for `schedule` events.
`pull_request` events were **never** affected: 0 startup failures in **2,067** pull_request runs across
all 33 repositories.

**The incident recovered on 2026-11-16 without any workflow or configuration change in the
organization.** Evidence-bracketed recovery window:

| Event class | Last failing run (0 jobs) | First starting run | Recovery bracket |
|---|---|---|---|
| `push` | atc-node `43852994457`, 10:54:30Z | globus-os `43894367069` / atc-node `43891148071`, 14:56:52Z | (10:54:30Z, 14:56:52Z] |
| `schedule` | atc-launchpad, 14:52Z | atclang, 14:58Z | (14:52Z, 14:58Z) |
| `dynamic` | a-townchain, 2026-11-13T12:55Z | a-townchain, 2026-11-14T07:53Z | earlier, per repository |

Consequence: the "19 repositories with startup failures" describes the **historical** state. Of those,
some start again; the rest have **no run since recovery** and are therefore *unverified*, not *broken*.

## 2. Method and snapshot

- Source: GitHub REST API (runs, jobs, check-runs, commits) plus local org snapshot (33 repos).
- Snapshot 2026-11-16: 33 repositories, HEAD SHAs identical to Annex A of the blocker register.
- Corpus: **6,005 workflow runs** across all 33 repositories (paged, not sampled), job counts for the
  last 8 runs per repository, workflow files parsed locally for all 33 repositories.
- No workflow was triggered by this investigation; the only runs created were those caused by normal
  pushes to PR #247.

## 3. Event-class evidence (corpus 6,005 runs)

| Event | Runs | `startup_failure` | Share | Assessment |
|---|---:|---:|---:|---|
| `pull_request` | 2,067 | **0** | 0.0 % | never affected |
| `push` | 2,423 | 493 | 20.3 % | affected in cohorts, recovered 16.11. |
| `dynamic` (CodeQL / Code Quality) | 1,126 | 399 | 35.4 % | **also affected**, recovered earlier and per repository |
| `schedule` | 368 | 82 | 22.3 % | affected in a mass cohort 09.11., recovered 16.11. |
| `workflow_dispatch` | 20 | 10 | 50.0 % | affected; no post-recovery probe yet |

**Correction to v0.1.0:** the earlier statement "`dynamic` never affected" was wrong; it was affected
in a subset of repositories with an earlier recovery. The `pull_request` finding stands on 2,067 runs.

## 4. Onset cohorts (0.2/0.3) — the window is not uniform

The incident started at **different times per repository and per event class**. Applying
06.11.–16.11. to every workflow would be wrong.

| Onset | Event | Repositories (first `startup_failure`) |
|---|---|---|
| 05.11. 11:21 | push, dynamic | atc-ide, a-townchain-os-docs |
| 05.11. 12:47 | schedule | atc-standards |
| 06.11. 17:18–17:23 | push, dispatch, dynamic | atc-standards, demo-repository, atc-node, genesis-chronicles |
| 08.11. 09:05–18:35 | push | a-townchain-os, atc-node, atc-shivacore |
| **09.11. 13:26–13:45** | schedule | 17 repositories within one 19-minute block (mass cohort) |
| 12.11. 15:46–21:55 | push, dispatch | a-townchain, atclang |
| 13.11. 17:58–18:22 | push, dynamic, schedule | aurora-ai, .github |
| 14.11. 09:14–22:15 | push, dispatch | a-townchain-os, a-townchain-ecosystem |
| 15.11. 00:32–01:24 | push, dispatch | globus-os |
| 16.11. 12:22–13:22 | schedule | globus-os, a-townchain, aurora-ai (last tick before recovery) |

The 09.11. schedule cohort (17 repositories failing within 19 minutes) is the strongest single
pointer to a platform-side change rather than repository configuration.

## 5. Common causes (0.3)

**Confirmed:** event-class-specific, time-bounded initialization failure, independent of repository
content, branch, permissions and workflow definition.

**Excluded, with evidence:**

| Candidate | Verdict | Evidence |
|---|---|---|
| YAML syntax | excluded | all workflow files in all 33 repos parse; 0 syntax errors; a syntax error would affect one workflow, not all workflows plus CodeQL |
| Actions permissions | excluded | `actions/permissions`: `enabled: true`, `allowed_actions: all` |
| Billing / spending limit | excluded | 805 of 2,000 included minutes; `net_amount_due: 0.00` |
| Runners | excluded | no workflow requires a self-hosted runner; all `ubuntu-latest` |
| Workflow definition / naming | excluded | runs keep correct names on `workflow_dispatch` (accepted with 204) yet produce 0 jobs |
| Reusable workflow references | excluded | all pinned refs resolve; file existence verified at every pinned SHA (`reusable-markdownlint.yml` @ `f7656d5b`, `2783846f`, `a41d9a93`; `reusable-ruff.yml` @ `b4ec342a`, `05607bf5`) |
| Commit signature / verification | **refuted** | post-recovery pushes that started are unsigned (`verified: false`): globus-os `43894367069` @ `91fc29e9c2`, atc-standards `43898277268` @ `3fce0b37b3` |
| Repository-specific configuration | excluded | affected and unaffected repositories share the same workflow structure (`schedule`/`reusable`/`environment` feature comparison in §7) |
| Pull request events | unaffected | 0 of 2,067 |

**Open:** the GitHub-internal cause and the status of the support escalation (Owner action, R-1).

## 6. Reproducibility (0.7)

- Pre-recovery: 0 jobs for `push`, `workflow_dispatch` and `schedule`; reproducible across four
  repositories at four different times (globus-os 15.11. 01:24, atc-standards 16.11. 08:18,
  a-townchain-os-docs 16.11. 08:18, atc-node 16.11. 10:54).
- Control: `pull_request` runs started throughout the same period in the same repositories.
- Post-recovery: every observed run after 14:56:52Z started and produced jobs.
- No configuration changed in the organization between the failing and the starting runs.

## 7. Repository-specific findings (0.4) and red-but-running pipelines (0.5)

### 7.1 Organization policy markdown defect (org-wide, single fix location)
`reusable-markdownlint.yml` checks out `.github` at `1c198f3c6a73…` into `.org-github-policy` and lints
`**/*.md`, including the policy file itself. `.org-github-policy/AGENT_MANIFEST.md` has **11
markdownlint findings** (MD056 table-column-count ×3, MD058 blanks-around-tables, MD022
blanks-around-headings, and further MD056 entries). Therefore **Markdown Lint fails in all 7 consuming
repositories**: a-townchain, atc-contracts, atc-engineering, atc-ide, atc-node, atc-shivacore,
aurora-ai. Evidence: a-townchain run `43893675443`, job `markdownlint / markdownlint-cli2`, 12
annotations.

Fix is single-location, but the pin chain must be moved: policy file → internal ref in
`reusable-markdownlint.yml` → pinned reusable-workflow SHA in each of the 7 consumers.

**Pin-chain finding (2026-11-16):** the policy commit `1c198f3c` that the consumers pin is **not an
ancestor of `main`** — the reusable workflow was never merged. The consumers' pinned reusable versions
bind the policy to `1c198f3c` (6 repos) and `daf4b6e` (aurora-ai), so the policy tree is linted at an
old, unclean commit regardless of `main`.

Operational repair progress is tracked exclusively in
`ATC-PHASE0-CI-RECOVERY-VERIFICATION-001`.
### 7.2 a-townchain — one Dependabot PR blocks five gates
All five red workflows belong to **one** pull request
(`dependabot/npm_and_yarn/modules/atc-blockchain/lucide-react-1.72.0`, sha `acce944f9`):
Markdown Lint (policy defect, §7.1), Ruff Code Quality, ATC Code Quality (STD-081/082), Determinism
Gate, ATC Test Suite. Runs: `43893675443`, `43893675171`, `43893674072`, `43893673988`, `43893673930`.
Markdown failure is fully explained by §7.1; the other four exit code 1 with logs unavailable (§7.3).

### 7.3 Evidence access gap (Class C)
Job logs for a-townchain return HTTP 403 (Azure SAS signature error) while the same call for globus-os
returns HTTP 200 (`43895261915`). **A 403 is an evidence-procurement problem, not proof that a step
failed.** For a-townchain the binding is therefore step-level only, not log-level. This is a hard gap
for §11 criterion "each finding bound to run, job, step and log".

### 7.4 atc-oracle — historical, not current
`ATC Test Suite` fails since 2026-10-07 in the evidence-binding step (SCR-0086); archived repository,
no activity since. Not part of the incident.

### 7.5 a-townchain-ecosystem — integration gates red after recovery (Class B)
First post-recovery run on `849186604` started **10 workflows**: 5 success (Repository Governance ×2,
Security Assurance Pipeline, CodeQL, Code Quality PR #247), 5 failure (ATC Test Suite, Blockchain Core
Integration, Determinism Gate, Ecosystem Integration, Ecosystem Release Readiness). The 10th workflow
(`CodeQL`) completed **success**; the earlier "1 in progress" was a snapshot, now resolved — final
count 5 success / 5 failure / 0 open. All failures are content gates over imported component
snapshots (fmt/clippy/tests/determinism/integration crate), **not** caused by the documentation commit
under test. Confirmed again on `2c3824d4c` with the identical 5/5 split.

## 8. Historical vs. current state

- Historical: 19 repositories with `startup_failure` in their last runs.
- Current verified: `push` started in 5 repositories (a-townchain, a-townchain-ecosystem, atc-node,
  atc-standards, globus-os); `schedule` started in 8 (a-townchain-ecosystem, atc-algorithm,
  atc-engineering, atc-node, atc-vm, atc-zkp, atclang, globus-os); `pull_request` in 3; `dynamic` in 6;
  `workflow_dispatch` in **0** — no dispatch probe exists after recovery.
- **21 repositories have no run at all after 14:56Z** and are unverified (R-2 sweep target).
- Verified-broken at this moment: **0**. Verified-green: only the runs listed above, on their SHAs.

## 9. Repair plan and dependencies (Owner order 16.11.)

| ID | Work | Depends on | Acceptance |
|---|---|---|---|
| **R-3** | Organization policy markdown (single authoritative fix + pin chain) | — | **PARTIAL**: PR #35 (44→0, no rule weakened). Remaining: workflow pin + 7 consumer bumps — Owner action, token has no `workflow` scope |
| **R-4** | Job-log access (a-townchain 403) | — | complete job/step/log evidence retrievable |
| **R-2** | Verification sweep, per event class | R-3, R-4 | every active repository assessed; `push`, `workflow_dispatch`, `schedule` judged separately |
| **R-5** | Dependabot PR triage | R-3 (markdown), R-4 (others) | each of the 5 gates judged separately |
| **R-6** | Evidence binding | R-2, R-5 | all proofs SHA-bound; `CI_VERIFIED` assigned per repository only on a green exact-SHA run |
| **R-1** | GitHub support escalation status | Owner | ticket state recorded |

## 10. Phase 0 acceptance status

| # | Criterion | Status | Note |
|---|---|---|---|
| 1 | All 33 HEAD SHAs captured | **MET** | API snapshot 2026-11-16 |
| 2 | All 19 startup failures individually examined | **MET** | §4 cohorts, per repo and event |
| 3 | Common causes proven or excluded | **MET** | §5 |
| 4 | Repository-specific causes documented | **MET** | §7 |
| 5 | Red-but-running pipelines assessed separately | **MET** | §7.2, §7.4, §7.5 |
| 6 | Each finding bound to run, job, step and log | **PARTIAL** | logs unavailable for a-townchain (403) |
| 7 | Repair plan with dependencies | **MET** | §9 |
| 8 | Register and roadmap updated consistently | **MET** | BLK-P0-01 with 01a–01e |

**7 of 8 MET → PARTIAL.** `CI_VERIFIED` = 0, `VERIFIED` = 0. BLK-P0-01 remains `BLOCKED`.

## 11. Owner acceptance criteria (recorded verbatim, 2026-11-16)

- **A. Policy file:** fix only the authoritative org policy file; all 11 violations removed; no
  weakening of the markdownlint rules; reusable workflow re-executed in all seven affected
  repositories; results documented with commit SHA and run ID.
- **B. Log access:** HTTP 403 is an access/permission problem in evidence procurement; missing job logs
  must not be interpreted as success or failure of any step.
- **C. Imported snapshots:** no blanket formatting of the monorepo, no automatic changes to imported
  snapshots; establish origin and freshness per affected snapshot; trace each error back to its source
  repository; only then decide whether to update the snapshot or fix the source.
- **D. Verification sweep:** the nine unverified active repositories need their own sweep; `push`,
  `workflow_dispatch` and `schedule` must be judged separately — a successful push does not prove that
  scheduled workflows work.

## 12. Governance note

The platform recovery must not be equated with restored CI and evidence infrastructure. A workflow that
starts is not green; a green run is not verified until it is bound to an exact SHA and its evidence is
complete. No release gate inherits evidence from this assessment.
