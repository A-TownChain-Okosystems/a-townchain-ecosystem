---
document_id: ATC-BLOCKER-REGISTER-001
title: Org-wide Prioritized Blocker Register
version: 0.1.0
status: PROPOSED
owner: A-TownChain-Okosystems
updated: 2026-11-16
standard: ATC-STD-MD-001
related: [ATC-SYSTEM-READINESS-001, ATC-ARCH-001, ATC-BC-001]
---

# Org-wide Prioritized Blocker Register

> **Purpose.** Turns the 2026-11-16 ecosystem overview into an executable engineering plan:
> every blocker gets a priority, a gate, a repository (with exact HEAD SHA), dependencies,
> measurable acceptance criteria and an accountable owner.
> Machine-readable twin: [`BLOCKER_REGISTER.yaml`](BLOCKER_REGISTER.yaml).
> Gate model: [`END_TO_END_RUNTIME_ROADMAP.md`](END_TO_END_RUNTIME_ROADMAP.md) / [`system-readiness.yaml`](system-readiness.yaml).
> Tracking: [Issue #116](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/116).

**Baseline.** Org snapshot 2026-11-16, 33 repositories, all remotes verified. Every SHA below is the verified
HEAD of that snapshot. Nothing in this register claims verified runtime behavior; every status is `BLOCKED`
until the evidence contract of the readiness roadmap is satisfied.

## 1. Overall assessment

| Area | Rating | Critical finding |
|---|---|---|
| Architecture | Advanced | Three competing layer models |
| GlobusOS | Advanced | Hardware and security gaps |
| ShivaCore | Partially integrated | ABI decisions open (E-1..E-7) |
| ATCLang | Critical backlog | Contract lowering missing |
| ATC-VM | Critical backlog | Opcode incompatibility |
| A-TownChain | Development-ready | No E2E proof |
| Aurora AI | Active | Integration depth unproven |
| Genesis Engine | Maintenance candidate | Oldest development state (needs assessment) |
| Standards | Well organized | Implementation far behind specification |
| CI/Evidence | Critical | 19 repositories fail to start workflows |

## 2. The three systemic blockers

The blockers are **not equal rank**. Close them in this order:

1. **BLK-P0-01 CI and evidence infrastructure.** While workflows start with zero jobs, no gate can be
   evidence-bound. This is a precondition, not a parallel task.
2. **BLK-P0-02/03/04 ATCLang to bytecode to ATC-VM.** The largest technical integration gap: a normative
   bytecode specification (ATC-BC-001), real contract lowering, and one consensus-verified E2E path.
3. **BLK-P1-01 ShivaCore ABI and rights architecture.** E-1..E-7 must be decided by the owner; no parallel
   rights management in GlobusOS, Aurora or other subsystems. ShivaCore remains the single capability authority.

## 3. Architecture consolidation is a work item, not cosmetics

Three competing layer models and diverging wiki copies are a structural risk: agents implement the same
function in different places, repositories grow competing interfaces, standards drift from implementations.

**Rule:** `ATC-ARCH-001` is the sole normative architecture source. Any deviating representation must be
reduced to it: marked `DEPRECATED`, linked to the source, and removed from agent reading paths.

## 4. Blocker register

### P0

| ID | Blocker | Repos (HEAD) | Blocked by | Acceptance (measurable) | Owner |
|---|---|---|---|---|---|
| **BLK-P0-01** | CI `startup_failure` org-wide: 19 of 33 repos start zero jobs | org-wide (see Annex A) | — | 26/26 governed repos: workflow starts on every push to main on the exact SHA; 19 → 0 startup failures; common cause separated from repo-specific ones | RepositoryAgent + Owner (GitHub Support) |
| **BLK-P0-02** | ATC-BC-001: no normative bytecode spec; compiler has 19 instructions, VM has 16 opcodes, ~5 shared | atc-vm@21132a49, atclang@516482ab, a-townchain@2c918207 | — | Opcode IDs, semantics, operands, stack, types, gas, traps, versioning fixed; compiler and VM reference one table; compiler↔VM conformance test green | ArchitectAgent + CodingAgent |
| **BLK-P0-03** | Contract lowering missing: 2/27 contracts compile (7.4%); `contract` bodies are token streams only | atclang@516482ab, atc-contracts@876c03b2 | BLK-P0-02 | Defined contract scope 100% passes `atc check --profile consensus`; lowering covers state/require/events; ≥3 real contracts run `.atc → .ops → ATVM` with state assertions | CodingAgent + QAAgent |
| **BLK-P0-04** | No E2E proof of the chain: consensus-verified contract execution unproven | a-townchain@2c918207, atc-vm@21132a49, atclang@516482ab, atc-contracts@876c03b2 | BLK-P0-02, -03 | One transaction executes an ATCLang contract, verified by ≥2 nodes; identical state roots; replay determinism | QAAgent + SecurityAgent |

### P1

| ID | Blocker | Repos (HEAD) | Blocked by | Acceptance (measurable) | Owner |
|---|---|---|---|---|---|
| **BLK-P1-01** | E-1..E-7 undecided; four competing rights vocabularies; ABI 2.1 / SpawnProcess locked | globus-os@cb605552, atc-shivacore@d817de2d | — | 7 decisions recorded with date/owner; ABI 2.1 approved; exactly one rights vocabulary; grep proof: no second rights management outside ShivaCore | Owner decides, ArchitectAgent prepares |
| **BLK-P1-02** | Hardware chain and VMM conformance freeze open (IOMMU, SMP, PCIe, NVMe/NIC, TLB; residuals R-1..R-4) | globus-os@cb605552 | BLK-P0-01 | M1.2 freeze documented with CI acceptance matrix; hardware paths evidenced per SHA; residuals dispositioned | CodingAgent + SecurityAgent |
| **BLK-P1-03** | Six kernel blockers: service space not in image, entropy, one capability authority, one device authority, SMP | globus-os@cb605552 | BLK-P1-01 | Service space boots from image; deterministic entropy source; exactly one capability and one device authority (structural proof); SMP base with test | CodingAgent |
| **BLK-P1-04** | Architecture consolidation: three layer models, two diverging wikis, KAI-OS ambiguous | a-townchain-os@8dbbb5dd, a-townchain-os-docs@ce8534a5, a-townchain-ecosystem@e18bdb90, atc-standards@f5408351 | — | Exactly one layer model per ATC-ARCH-001 everywhere; deviations DEPRECATED and linked; one wiki SSOT; all counts generated, not hand-edited | ArchitectAgent + DocumentationAgent |

### P2

| ID | Blocker | Repos (HEAD) | Blocked by | Acceptance (measurable) | Owner |
|---|---|---|---|---|---|
| **BLK-P2-01** | Standards implementation: 36.8% (65 enforced + 131 implemented), 330 specification-only, 89/160 findings open | atc-standards@f5408351, atc-engineering@0f94b4f7 | BLK-P0-01 | Implementation waves with definition of done; findings 89 → 0 or dispositioned; fleet audit green | StandardsAgent + GovernanceAgent |
| **BLK-P2-02** | Evidence binding: `bound_commit: pending`, `tests: not_run`; prose numbers without runs | a-townchain-os@8dbbb5dd, a-townchain-os-docs@ce8534a5 | BLK-P0-01 | Every status claim carries bound_commit + run_id or is marked HISTORICAL; no hand-maintained numbers; evidence index verifiable | DocumentationAgent + RepositoryAgent |
| **BLK-P2-03** | Governance coverage: 6 ungoverned repos (incl. atc-engineering); admin-push bypass (F-045) | .github@1cf30d8c, atc-engineering@0f94b4f7 | — | 26/26 governed; bypass closed via enforced branch protection; mandatory gates without continue-on-error | GovernanceAgent |
| **BLK-P2-04** | Aurora AI integration depth unproven (intent → policy → capability → tool → audit) | aurora-ai@2855956b | BLK-P1-01, BLK-P1-02 | Model/runtime IDs recorded; inference, policy, denied-action and audit tests per G4 | CodingAgent + QAAgent |
| **BLK-P2-05** | Genesis Engine: last commit 2026-10-23 is an activity signal, not proof of backlog | genesis-engine@3c2be8c1, genesis-chronicles@734b2722, genesis-franchise-factory@673fa560 | — | Assessment with build/tests/dependencies/coverage metrics; outcome: gate assignment (G5) or maintenance plan | QAAgent + ResearchAgent |
| **BLK-P2-06** | Chain security architecture: independent deep analysis missing (consensus, signatures, replay, ZKP, upgrade) | a-townchain@2c918207, atc-algorithm@f25b4338, atc-zkp@a284eeab, atc-node@821d049e | BLK-P0-01 | Analysis report with findings, all dispositioned, no open P0 finding | SecurityAgent + ResearchAgent |
| **BLK-P2-07** | Release criteria incomplete: audit alone is not production readiness | a-townchain-ecosystem@e18bdb90 | BLK-P0-04, P2-01, P2-02 | Catalog approved covering audit, deterministic E2E, reproducibility, recovery, upgrade safety, operational evidence; each bound to a gate | Owner + GovernanceAgent |

## 5. Phase plan

Phases may be **prepared** in parallel; **implementation and release dependencies must not be skipped**.

| Phase | Work package | Blockers | Completion criterion |
|---|---|---|---|
| 0 | CI repair | BLK-P0-01 | Reproducible workflows on exact SHAs |
| 1 | Architecture consolidation | BLK-P1-04, BLK-P2-03 | One binding layer model (ATC-ARCH-001) |
| 2 | ATC-BC-001 | BLK-P0-02 | Shared compiler/VM specification approved |
| 3 | Contract lowering | BLK-P0-03 | Real compilable contracts |
| 4 | E-1..E-7 | BLK-P1-01 | ABI and capability decisions recorded |
| 5 | E2E integration | BLK-P0-04, BLK-P2-04 | End-to-end execution proven |
| 6 | Security audit | BLK-P2-06 | External security review complete |
| 7 | Release gate | BLK-P2-07, BLK-P2-02 | SHA-bound release evidence |

Gate mapping: Phase 0 is a precondition for G0-G5 evidence; Phase 1 closes G0; Phases 2-4 close G1/G3
findings; Phase 5 closes G3/G4/G5; Phases 6-7 close the release conditions of G5.

## 6. Precision notes on the 2026-11-16 overview

- **CI count corrected:** 19 unique repositories (not 20); 18 governed + `demo-repository`, which is
  excluded from governance by owner decision. `a-townchain` and `atc-oracle` run but fail; 12 repos are green.
- **Chain and services:** the direct measurement replaces the failed analysis agent for the collected metrics,
  but not for an independent deep analysis of the chain security architecture — see BLK-P2-06.
- **Genesis Engine:** the last commit date is an activity signal, not evidence of technical backlog — see BLK-P2-05.
- **Production readiness:** external audit is necessary but not sufficient; deterministic E2E, reproducibility,
  recovery, upgrade safety and operational evidence belong to the release gate — see BLK-P2-07.

## 7. Rules for working this register

1. Close the earliest blocking gate in its canonical repository. No replacement implementations in this repo.
2. Never weaken, skip or reinterpret a gate to obtain green.
3. Every status change requires exact-SHA evidence; otherwise it stays `BLOCKED`.
4. AI agents may propose, verify and report - they never approve a release.
5. Fewer new modules, no duplicate implementations, normative interfaces, automated tests, SHA-bound evidence.
6. Standalone First - Native Implementation - Ecosystem Second.

## Annex A - Repository responsibility matrix (snapshot 2026-11-16)

| Repository | HEAD | CI (latest) | Governance | Role | Key metric |
|---|---|---|---|---|---|
| `.github` | `1cf30d8c0b` | startup_failure | governance-hub | Org-Governance, Compliance-Scan | 25/25 COMPLIANT (11.09.) vs. 20/26 governed |
| `a-townchain` | `2c91820710` | failure | governed | L3 Chain-Monorepo (16 Komponenten) | 120.283 Rs + 157.587 .atc; Chain-ID 658467 |
| `a-townchain-ecosystem` | `e18bdb90db` | startup_failure | governed | System-of-Systems / Evidence-Control-Plane | 71 offene Issues, 29 PRs |
| `a-townchain-os` | `8dbbb5ddfa` | startup_failure | governed | L7 Integration (Rust+ATCLang) | STATUS: development / NOT_READY, bound_commit pending |
| `a-townchain-os-docs` | `ce8534a5b2` | startup_failure | governed | Docs/Wiki-Hub | 2.183 MD; Wiki-Fassung divergiert vom L7-Repo |
| `atc-algorithm` | `f25b433834` | startup_failure | governed | Konsens/Algorithmus | 541 Zeilen |
| `atc-compute` | `89d833edf3` | startup_failure | governed | L5 Service: Compute | 0,1 MB |
| `atc-contracts` | `876c03b2c8` | startup_failure | governed | ATCLang-Contracts | 48 .atc; 17/17 EXEC-CHAIN (Arithmetik); 2/27 compilierbar |
| `atc-engineering` | `0f94b4f7cc` | startup_failure | ungoverned | Engineering-Control-Plane | 13 Crates, 99 Rust + 97 Python-Tests; Fleet-Audit rot |
| `atc-explorer` | `c4da05304f` | success | governed | L5 Service (archiviert) |  konsolidiert |
| `atc-ide` | `84a233b4fb` | success | governed | IDE-Skelett (TS/Vite) | server.ts 18,9 KB, Tests leer |
| `atc-indexer` | `7f3da49f30` | success | governed | L5 Service (archiviert) | konsolidiert |
| `atc-interop` | `5905fca022` | success | governed | L5 Bridge (archiviert) | konsolidiert |
| `atc-launchpad` | `bd1a48c699` | startup_failure | governed | L5 Service: Launchpad | 0,1 MB |
| `atc-marketplace` | `66a1c93cc4` | startup_failure | governed | L5 Service: Marketplace | 0,4 MB |
| `atc-mining` | `227f2d7003` | success | governed | L5 Service (archiviert) | konsolidiert |
| `atc-node` | `821d049efc` | success | governed | Chain-Node | 0,1 MB |
| `atc-oracle` | `8087d34ddb` | failure | governed | L5 Service (archiviert) | konsolidiert, letzter Run rot |
| `atc-sdk` | `11e18587a4` | startup_failure | governed | SDK | 4 Demo-.atc, kein Rust-Tree |
| `atc-shivacore` | `d817de2dec` | startup_failure | governed | Kernel-Support/Governance (nicht mehr Kernel-SSOT) | canonical: false |
| `atc-standards` | `f54083518f` | success | governed | Standards-Registry (533 Standards) | 505/506 approved; 36,8% implementiert |
| `atc-storage` | `230746c1dc` | success | governed | L5 Service (archiviert) | konsolidiert |
| `atc-toolchain` | `aaed3daad2` | success | governed | Toolchain | kein Quellcode (nur README/docs) |
| `atc-vm` | `21132a4935` | startup_failure | governed | ATC-VM | 1.184 LOC, 52 Tests grün; ISA divergiert vom Compiler |
| `atc-wallet` | `812a31d292` | success | governed | L5 Service (archiviert) | konsolidiert |
| `atc-zkp` | `a284eeab3d` | startup_failure | governed | ZKP | 0,2 MB |
| `atclang` | `516482abbf` | startup_failure | governed | Sprache/Compiler | 105 Tests grün; Contract-Support = 0 |
| `aurora-ai` | `2855956b32` | startup_failure | governed | L2 AI-Plattform | 13.357 Rs + 8.822 .atc, 10 Crates |
| `demo-repository` | `58c6bf577c` | startup_failure | excluded | Demo (von Governance ausgeschlossen) | — |
| `genesis-chronicles` | `734b272245` | startup_failure | governed | Spiel/NFT-Anwendung | 0,1 MB |
| `genesis-engine` | `3c2be8c11c` | success | governed | L6 Game-Plattform | 6.212 Rs; letzter Commit 23.10. (kein Rückstandsbeweis) |
| `genesis-franchise-factory` | `673fa56032` | startup_failure | governed | Franchise-Fabrik | 0,1 MB |
| `globus-os` | `cb605552f6` | success | governed | L4 Betriebssystem + Kernel-SSOT | 1.877 Tests; Boot-to-Ring-3 OK; CI heute grün |

_CI column: `startup_failure` = workflow did not start (0 jobs); `success`/`failure` = workflow ran._
