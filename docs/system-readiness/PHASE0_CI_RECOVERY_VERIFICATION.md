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

21 repositories have no run after recovery. **They are unverified, not faulty.** The same
applies to `workflow_dispatch`: zero observed runs in the measurement window is an absence of
evidence, not evidence of a defect. R-2 closes this section per event class — a successful
`push` does not prove that `schedule` or `workflow_dispatch` work, and vice versa.

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
| `docs/system-readiness/PHASE0_CI_ROOT_CAUSE.md` | `e8e27a9edea44e23e91fb8b110886f488b8536d926c8a52e5724046568527bcb` |
| `docs/system-readiness/PHASE0_CI_ROOT_CAUSE.yaml` | `a4af1b8939b3e417b5d98c8d37ff51136b4b848010bc1e73ab626f2827c11ddc` |
| `docs/system-readiness/BLOCKER_REGISTER.md` | `d8365da99450b77f51de9941afcf924e883f77ee87b61c57c0522d11460005e2` |
| `docs/system-readiness/BLOCKER_REGISTER.yaml` | `58848f5f1fa893cf7d641abeadf9f0cf092c9038946306c021d1ba33e78a7348` |
| `docs/system-readiness/PHASE0_CI_RECOVERY_VERIFICATION.yaml` | `9c146b027fba70e5b86d803a56d352f9a307fbec48994f532ca94570a84ba9e1` |

Hashes bind the RCA and register at the commit that introduced this document. They are historical anchors, not live checksums.

## 6.5 Repair progress — R-3 (policy markdown and pin chain)

### State

| Step | State | Evidence |
|---|---|---|
| 1. Policy markdown corrected | **DONE** — `.github` PR #35 | 44 → 0 findings, `markdownlint-cli2 0.18.1`, unchanged policy config; 40 whitespace fixes, 4 manual (MD025, MD001, MD040 ×2) |
| 2. Merge PR #35 (owner review) | **OPEN** — owner process | review pack: policy diff, config identity, rule strictness |
| 3. Reusable workflow pins corrected commit | **OPEN** — needs `workflow` scope | `github-ci-fix/reusable-markdownlint.yml` + `apply.sh` |
| 4. Seven consumer pins updated | **OPEN** — needs `workflow` scope | `github-ci-fix/CONSUMERS.md` (exact current pins) |
| 5. All seven consumers run green | **OPEN** | `evidence.sh` → `evidence_complete` per consumer |
| 6. Evidence recorded | **OPEN** | `EVIDENCE_REGISTRY.yaml` (`r3_consumer_evidence`), artifact hashes |

### Two separate defects (must not be merged into one)

1. **Policy content defect** — 44 markdownlint findings in the policy tree, fixed by PR #35.
2. **Integration defect** — the policy commit `1c198f3c…` pinned by the consumers is **not an
   ancestor of `main`**; the reusable workflow was never merged. A corrected `main` alone therefore
   cannot turn any consumer green.

### Mandatory guards for `apply.sh` (fail closed)

- `--policy-sha` must equal the reviewed merge commit of PR #35 and must be an ancestor of `main`;
  if not, abort before any write.
- The policy tree at that SHA must be proven clean with the policy config; abort otherwise.
- Exactly the seven documented consumers may be touched — any additional or missing repository aborts.
- Each consumer's current pin must match the recorded pin in `CONSUMERS.md`; drift aborts.
- Dry run is the default; `--apply` is required for writes. No `--force`.
- **TOCTOU:** the consumer HEAD is re-read immediately before the write; if it moved since planning,
  the run aborts. After the write the pushed branch head is read back and must carry the new pin.
- **Semantics (not just a successful call):** the pinned reusable workflow must contain
  `ref: <approved policy commit>`, and that commit must contain the approved policy file
  (SHA-256 match). A green run alone does not prove the corrected file was linted.

### Log-evidence specification (binding)

`evidence.sh` hashes the **gzip-decoded response bytes** of
`GET /repos/{org}/{repo}/actions/jobs/{job_id}/logs` exactly as received: no trimming, no line-ending
normalisation. Recorded per consumer: `http_status`, `log_bytes`, `log_sha256`, `retrieved_at`.
A 403/404/missing log yields `log_status=MISSING:<code>` and `evidence_complete=false` — a missing log
is never success evidence, whatever the run conclusion says.

`evidence_complete` is true only when **all ten** conditions hold (collector v3, machine-checked):

1. the run's repository identity equals the expected consumer;
2. `run.head_sha` equals the consumer HEAD read in the same collection;
3. the consumer workflow file at HEAD is byte-identical to the file at the approved workflow commit;
4. the executed reusable-workflow SHA (`run.referenced_workflows`) equals the approved workflow commit;
5. the consumer pin equals the approved workflow commit;
6. the policy ref inside the pinned workflow equals the approved policy commit;
7. the policy file at that ref matches the approved file hash;
8. the run was created strictly after the pin commit;
9. the markdownlint job conclusion is `success` (run level alone is not sufficient);
10. the lint job log was retrieved (HTTP 200) and hashed per the log specification.

A technically successful but historical, misattributed or log-less run cannot qualify. The archive
(JSONL) is append-only and its SHA-256 is recorded with the registry entry.

### Acceptance for R-3

R-3 is complete only when **7 of 7** consuming repositories reach `evidence_complete=true` against the
same approved policy and workflow commit, archived with run ID, consumer HEAD, executed workflow SHA,
job conclusion and log hash.

**Abort rule:** a single failed consumer blocks R-3. The failing consumer is isolated and investigated;
the six complete proofs are **not** discarded. No partial promotion, no "6 of 7 is good enough".

### Governance model

The separation requested by the owner is now normative for all blockers:
[`EVIDENCE_MODEL.md`](EVIDENCE_MODEL.md) defines the four objects (RCA immutable, Recovery operational,
register current state, evidence registry machine-checkable). Machine-readable proofs live in
[`EVIDENCE_REGISTRY.yaml`](EVIDENCE_REGISTRY.yaml); R-3 entries are appended there, never in the RCA.

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
| R-3 | **OPEN** — 7/7 consumer runs required |

A completed recovery verification requires: every governed repository with a started run on its
exact HEAD SHA, per event class, with run → job → step → log binding and artifact hashes.
