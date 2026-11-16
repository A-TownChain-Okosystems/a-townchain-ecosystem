---
document_id: ATC-PHASE0-CI-RECOVERY-VERIFICATION-001
title: Phase 0 — CI Recovery Verification
version: 0.1.0
status: PENDING
owner: A-TownChain-Okosystems
updated: 2026-11-16
standard: ATC-STD-MD-001
tracking_issue: https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/116
related: [ATC-PHASE0-CI-RCA-001, ATC-BLOCKER-REGISTER-001]
---

# Phase 0 — CI Recovery Verification

> **Status: PENDING — deliberately not verified.** This document is the closing proof for Phase 0 and
> is completed only after R-3, R-4, R-2 and R-5. Sections marked OPEN are not evidence.
> **Gate decision: NOT VERIFIED.** `CI_VERIFIED` = 0, `VERIFIED` = 0.

## 1. Incident and window

| Property | Value |
|---|---|
| Symptom | runs with 0 jobs, `conclusion: startup_failure`, empty name, `path: BuildFailed` |
| Affected event classes | `push`, `workflow_dispatch`, `schedule`, `dynamic` (subset) |
| Unaffected | `pull_request` — 0 of 2,067 runs |
| Onset | cohorts 2026-11-05T11:21Z … 2026-11-15T00:32Z (see RCA §4) |
| Recovery (push) | between 2026-11-16T10:54:30Z and 14:56:52Z |
| Recovery (schedule) | between 2026-11-16T14:52Z and 14:58Z |
| Configuration changed by the organization | none |
| Support escalation | **OPEN** (Owner): ticket state not yet recorded |

## 2. Repository HEAD snapshot (33)

| Repository | HEAD | push | dispatch | schedule | PR | dynamic |
|---|---|---|---|---|---|---|
| .github | `1cf30d8c0b64` | startup_failure | - | startup_failure | no post-recovery run | no post-recovery run |
| a-townchain | `2c918207101a` | started | - | startup_failure | started | started |
| a-townchain-ecosystem | `e18bdb90dbdd` | started | - | started | started | started |
| a-townchain-os | `8dbbb5ddfad1` | startup_failure | startup_failure | - | no post-recovery run | no post-recovery run |
| a-townchain-os-docs | `ce8534a5b23e` | startup_failure | - | startup_failure | no post-recovery run | startup_failure |
| atc-algorithm | `f25b43383433` | no post-recovery run | - | started | no post-recovery run | no post-recovery run |
| atc-compute | `89d833edf338` | no post-recovery run | - | startup_failure | no post-recovery run | no post-recovery run |
| atc-contracts | `876c03b2c8ec` | no post-recovery run | - | startup_failure | no post-recovery run | no post-recovery run |
| atc-engineering | `0f94b4f7cc7f` | no post-recovery run | - | started | no post-recovery run | no post-recovery run |
| atc-explorer | `c4da05304f21` | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run |
| atc-ide | `84a233b4fbe5` | startup_failure | - | - | no post-recovery run | started |
| atc-indexer | `7f3da49f3046` | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run |
| atc-interop | `5905fca02274` | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run |
| atc-launchpad | `bd1a48c69983` | no post-recovery run | no post-recovery run | startup_failure | no post-recovery run | no post-recovery run |
| atc-marketplace | `66a1c93cc4dc` | no post-recovery run | no post-recovery run | startup_failure | no post-recovery run | no post-recovery run |
| atc-mining | `227f2d700307` | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run |
| atc-node | `821d049efc5b` | started | startup_failure | started | started | started |
| atc-oracle | `8087d34ddb8c` | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run |
| atc-sdk | `11e18587a4c2` | no post-recovery run | - | startup_failure | no post-recovery run | no post-recovery run |
| atc-shivacore | `d817de2deccb` | startup_failure | - | startup_failure | no post-recovery run | no post-recovery run |
| atc-standards | `f54083518fb8` | started | - | startup_failure | no post-recovery run | no post-recovery run |
| atc-storage | `230746c1dc5e` | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run | no post-recovery run |
| atc-toolchain | `aaed3daad2f6` | no post-recovery run | - | - | no post-recovery run | no post-recovery run |
| atc-vm | `21132a49355e` | no post-recovery run | - | started | no post-recovery run | no post-recovery run |
| atc-wallet | `812a31d29236` | no post-recovery run | - | no post-recovery run | no post-recovery run | no post-recovery run |
| atc-zkp | `a284eeab3dfe` | no post-recovery run | - | started | no post-recovery run | no post-recovery run |
| atclang | `516482abbfa9` | startup_failure | startup_failure | started | no post-recovery run | startup_failure |
| aurora-ai | `2855956b32fd` | startup_failure | startup_failure | startup_failure | - | startup_failure |
| demo-repository | `58c6bf577cee` | startup_failure | startup_failure | - | no post-recovery run | - |
| genesis-chronicles | `734b27224534` | startup_failure | no post-recovery run | startup_failure | no post-recovery run | no post-recovery run |
| genesis-engine | `3c2be8c11c00` | no post-recovery run | - | startup_failure | no post-recovery run | started |
| genesis-franchise-factory | `673fa560322b` | no post-recovery run | no post-recovery run | startup_failure | no post-recovery run | no post-recovery run |
| globus-os | `cb605552f623` | started | startup_failure | started | - | started |

Legend: `started` = a run after 14:56Z produced jobs; `startup_failure` = last run had 0 jobs;
`no post-recovery run` = unverified; `-` = event never used in this repository.

## 3. Event coverage — OPEN

| Event class | Repositories with post-recovery evidence | Required |
|---|---|---|
| `push` | 5 | 26 governed |
| `schedule` | 8 | every repository with cron workflows |
| `workflow_dispatch` | **0** | at least one probe per dispatch-capable workflow |
| `pull_request` | 3 | not incident-relevant, but needed for content gates |
| `dynamic` | 6 | not incident-relevant |

21 repositories have no run after recovery. R-2 closes this section.

## 4. Error classification

| Class | Content | Status |
|---|---|---|
| A | platform initialization failure (0 jobs) | assessed pre-recovery, recovery evidenced |
| B | content failures in running pipelines | assessed (RCA §7.2/7.4/7.5), repair open |
| C | evidence access gap (a-townchain logs HTTP 403) | **OPEN** — R-4 |
| D | unverified repositories (no post-recovery run) | **OPEN** — R-2 |

## 5. Run → Job → Step → Log binding

| Finding | Run | Job | Step | Log |
|---|---|---|---|---|
| pre-recovery, 0 jobs (push) | `43852994457` (atc-node) | none (0 jobs) | none | n/a — no job exists |
| pre-recovery, 0 jobs (push) | `43894367069` is post-recovery | — | — | — |
| post-recovery start (push) | `43894367069` (globus-os, `91fc29e9c2`) | 1 job | build ok | HTTP 200 |
| post-recovery start (push) | `43891148071` (atc-node, `46333478db`) | jobs present | — | OPEN |
| policy markdown defect | `43893675443` (a-townchain) | `markdownlint / markdownlint-cli2` | `Lint Markdown` | **HTTP 403 — OPEN** |
| a-townchain gates | `43893675171`, `43893674072`, `43893673988`, `43893673930` | per workflow | exit code 1 | **HTTP 403 — OPEN** |
| integration gates | `43916656415`, `43916656205`, `43916656345` (a-townchain-ecosystem) | per component | format/clippy/tests | OPEN |

## 6. Artifact integrity (SHA-256 at commit time)

| Artifact | SHA-256 |
|---|---|
| `docs/system-readiness/PHASE0_CI_ROOT_CAUSE.md` | `35d4fa0f3eda2ecb74e4b1ce6c7e2add0e1ff0322b1666830f15a57b93041dec` |
| `docs/system-readiness/PHASE0_CI_ROOT_CAUSE.yaml` | `a4af1b8939b3e417b5d98c8d37ff51136b4b848010bc1e73ab626f2827c11ddc` |
| `docs/system-readiness/BLOCKER_REGISTER.md` | `d8365da99450b77f51de9941afcf924e883f77ee87b61c57c0522d11460005e2` |
| `docs/system-readiness/BLOCKER_REGISTER.yaml` | `7aa03e6438340ca763e6d06d540f1d4b036989ff33aa176575da3b308c24b193` |

## 7. Remaining blockers

| ID | Blocker | Owner | Status |
|---|---|---|---|
| BLK-P0-01a | verification sweep, per event class | RepositoryAgent | OPEN (R-2) |
| BLK-P0-01b | org policy markdown, pin chain | DocumentationAgent | OPEN (R-3) |
| BLK-P0-01c | job-log access a-townchain | RepositoryAgent | OPEN (R-4) |
| BLK-P0-01d | content failures (a-townchain PR, integration snapshots) | QAAgent | OPEN (R-5) |
| BLK-P0-01e | GitHub support escalation state | Owner | OPEN (R-1) |

## 8. Gate decision

| Decision | Value |
|---|---|
| Phase 0 | **PARTIAL** (7 of 8 acceptance criteria) |
| Recovery verification | **PENDING** — not completed |
| `CI_VERIFIED` | **0** repositories |
| `VERIFIED` | **0** |
| BLK-P0-01 | **BLOCKED** |

A completed recovery verification requires: every governed repository with a started run on its
exact HEAD SHA, per event class, with run → job → step → log binding and artifact hashes.
