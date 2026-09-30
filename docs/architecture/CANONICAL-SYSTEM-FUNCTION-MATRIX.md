# A-TownChain Ecosystem — Canonical System Function Matrix

**Document ID:** ATC-ARCH-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT  
**Scope:** OS + A-TownChain L1 + Aurora AI  
**Rule:** Architecture presence does not imply implementation, testing, CI verification, integration, or E2E verification.

## 1. Canonical Traceability

Every architectural capability is traced as:

```text
Vision
  → Domain
  → Layer
  → Component
  → Function / API
  → Contract
  → Source
  → Test
  → Workflow
  → Exact-SHA Evidence
  → Verification
  → Residual
```

Evidence separation is mandatory:

```text
Error Evidence ≠ Finding Evidence ≠ Verification Evidence
Source vorhanden ≠ Implementiert ≠ Verifiziert
Architecture ≠ Implementation ≠ Test ≠ CI Evidence ≠ Verification
```

Status values:

```text
UNANALYZED → ANALYZED → FIXED → RERUNNING → VERIFIED / RESIDUAL
```

## 2. Operating System — 19 Canonical Domains

| ID | Domain | Canonical responsibility |
|---|---|---|
| OS-01 | Hardware / Firmware | CPU, GPU, NPU, devices, UEFI/BIOS, Secure Boot boundary |
| OS-02 | ShivaCore / TCB | trusted kernel, isolation, capabilities, security boundary |
| OS-03 | Kernel / Core | boot, scheduler, process/core services, kernel state |
| OS-04 | HAL / Drivers | hardware abstraction and device drivers |
| OS-05 | Memory / Process / Scheduler | memory management, processes, threads, scheduling |
| OS-06 | IPC / System Services | IPC, service lifecycle, system APIs |
| OS-07 | Storage / VFS | filesystem, VFS, block/storage services, persistence |
| OS-08 | Network | network stack, connectivity, transport and policy |
| OS-09 | Identity / AuthN / AuthZ | registration, login, sessions, identity, authorization |
| OS-10 | Security | isolation, secrets, policy enforcement, security monitoring |
| OS-11 | Graphics / Desktop | display, compositor, input, desktop/session |
| OS-12 | Application Runtime | application lifecycle, runtime APIs, sandboxing |
| OS-13 | Package / Software Lifecycle | package, install, dependency, lifecycle management |
| OS-14 | Update / Recovery | update, rollback, recovery, restart/resume |
| OS-15 | AI / Aurora Integration | Aurora/Kai ↔ OS APIs, IPC, capabilities, compute access |
| OS-16 | A-TownChain Integration | node/RPC/wallet/chain services and authorized chain access |
| OS-17 | Developer Platform | SDKs, CLI, development/runtime interfaces |
| OS-18 | Observability / Operations | logs, metrics, diagnostics, health, incidents |
| OS-19 | Testing / Verification | tests, CI, exact-SHA evidence, verification, residuals |

## 3. A-TownChain L1 — 24 Canonical Domains

| ID | Domain | Canonical responsibility |
|---|---|---|
| L1-01 | Protocol & Chain Identity | chain/network identity, protocol versioning, genesis, domain separation |
| L1-02 | Transaction Layer | transaction format, nonce, encoding, hashing, signatures, validation, replay protection |
| L1-03 | Account & State Model | accounts, addresses, balances, nonces, state roots, state transitions |
| L1-04 | Wallet & Key Management | key lifecycle, signing, secp256k1, RFC6979, low-S, rotation/recovery |
| L1-05 | Cryptography | hashes, signatures, key encoding, Merkle structures, parameters, algorithm versions |
| L1-06 | Consensus | validator set, proposer selection, voting, finality, epochs, rotation, slashing, recovery |
| L1-07 | Block Layer | headers, bodies, parent hash, height, timestamp, roots, block validation/propagation |
| L1-08 | Economics & Monetary Policy | supply, rewards, halving, fees, treasury, staking, slashing, invariants |
| L1-09 | Mempool | admission, signature/nonce/fee checks, duplicates, ordering, limits, recovery |
| L1-10 | P2P / Network | discovery, identity, handshake, propagation, peer scoring, anti-Sybil, recovery |
| L1-11 | Storage | block/state/tx/validator stores, WAL, snapshots, pruning, indexes, crash recovery |
| L1-12 | State Transition Engine | deterministic validation → execution → transition → state-root pipeline |
| L1-13 | Smart Contracts / ATC-VM | ATCLang, AST/IR/ABI, bytecode, verifier, VM, typed values, state, determinism |
| L1-14 | Execution & Gas | instruction/resource accounting, memory/storage/call limits, deterministic failures |
| L1-15 | RPC / Node API | transaction, block, account, state, contract, event, mempool, validator and health APIs |
| L1-16 | Node Runtime | initialization, config, networking, mempool, consensus, execution, storage, RPC, restart |
| L1-17 | Synchronization | initial/block/state/snapshot sync, recovery, reorg/finality handling, resume |
| L1-18 | Indexer / Explorer | blocks, transactions, addresses, contracts, events, validators, balances, history, API |
| L1-19 | Governance / Protocol Upgrades | proposals, voting, parameters, upgrades, compatibility, migration, emergency procedures |
| L1-20 | Security | threat model, Sybil/DoS/replay/double-spend defense, contract/resource/state security |
| L1-21 | Observability / Operations | logs, metrics, tracing, health, diagnostics, alerts, crash reports |
| L1-22 | Testing & Determinism | unit/property/fuzz/vector/E2E, cross-language, consensus/VM/state determinism, recovery |
| L1-23 | Release & Evidence | reproducible build, hashes, SBOM, Exact-SHA CI, artifacts, RCA, rerun, verification |
| L1-24 | Developer Ecosystem | Rust/TypeScript/wallet SDKs, CLI, contract tooling, local node, devnet/testnet, docs |

### 3.1 Canonical A-TownChain Economic Contract

The architecture records the existing canonical values; implementation evidence remains repository-specific:

- Maximum supply: **360,000,000 ATC**
- Base denomination: **10^-18 ATC**
- Monetary values: **u128**
- Target block time: **360 seconds**
- Halving interval: **360,000 blocks**
- Halvings: **36**
- Chain ID: **658467**
- Transaction domain: **ATC-TX-DOMAIN-V2**
- Legacy transaction domain: **forbidden**

## 4. A-TownChain — 8 Architectural Layers

```text
A-TOWNCHAIN L1
│
├── 1. Protocol
│   Chain / Transaction / Account / State
│
├── 2. Economics
│   Supply / Fees / Rewards / Staking / Treasury
│
├── 3. Consensus
│   Validators / Proposal / Voting / Finality
│
├── 4. Execution
│   State Transition / ATC-VM / Contracts / Gas
│
├── 5. Network
│   P2P / Propagation / Synchronization
│
├── 6. Node / Storage
│   Runtime / Blocks / State / WAL / Snapshots
│
├── 7. Interfaces
│   RPC / SDK / Wallet / Explorer / Indexer
│
└── 8. Operations
    Security / Observability / Testing / Release / Evidence
```

## 5. Aurora AI — 23 Canonical Domains

| ID | Domain | Canonical responsibility |
|---|---|---|
| AI-01 | Vision & Governance | mission, principles, policy, risk, oversight, lifecycle |
| AI-02 | Identity & Access | user/service/model/agent identity, AuthN/AuthZ, RBAC/ABAC, secrets |
| AI-03 | Model Layer | foundation, LLM, multimodal, embedding, vision, speech, local/remote models |
| AI-04 | ModelHub | discovery, registry, metadata, routing, selection, health, compatibility, deprecation |
| AI-05 | AI Runtime | inference, tokenization, context, CPU/GPU/NPU, quantization, streaming, isolation |
| AI-06 | Agent System | lifecycle, state, planning, reasoning, decomposition, tools, coordination, termination |
| AI-07 | Capability / Policy | capabilities, scopes, permission checks, policy, approval, revocation, delegation |
| AI-08 | Tool & Action | tool registry, OS/network/files/DB/blockchain/developer tools, schemas, validation |
| AI-09 | Memory | conversation, working, long-term, semantic, episodic, preferences, retention/deletion |
| AI-10 | RAG / Knowledge | ingestion, parsing, chunking, embeddings, retrieval, ranking, attribution, freshness |
| AI-11 | Data | sources, contracts, schemas, lineage, classification, quality, privacy, datasets |
| AI-12 | AI Safety & Security | injection, tool abuse, exfiltration, isolation, sandboxing, output/policy controls |
| AI-13 | Evaluation | model/agent/tool/RAG evaluation, groundedness, robustness, safety, regression |
| AI-14 | Prompt / Context Engineering | prompts, developer instructions, context windows, versioning, injection testing |
| AI-15 | Multimodal AI | text, image, audio, STT, TTS, video, vision, document understanding |
| AI-16 | AI ↔ Operating System | OS API, IPC, processes, filesystem, network, devices, compute, quotas |
| AI-17 | AI ↔ Blockchain | wallet/RPC/state/contracts, agent identity, audit, approval-gated transactions |
| AI-18 | Developer Platform | AI/agent/tool/model/RAG/memory/evaluation SDKs, APIs, CLI, local runtime |
| AI-19 | Observability | inference/agent/tool traces, latency, usage, resources, errors, policy/security/audit |
| AI-20 | Operations | deployment, serving, scaling, scheduling, health, failover, rollback, updates |
| AI-21 | Supply Chain | model/data provenance, dependencies, signing, SBOM, integrity, pinning, reproducibility |
| AI-22 | Testing & Verification | unit/integration/agent/model/security/policy/tool/RAG/E2E, Exact-SHA evidence |
| AI-23 | Evidence & Governance | traceability, evidence separation, status, verification, residual management |

### 5.1 Aurora Authority Invariant

```text
Model
  ↓
Agent
  ↓
Capability
  ↓
Policy
  ↓
Approval
  ↓
Tool
  ↓
GlobusOS
  ↓
ShivaCore
  ↓
Hardware
```

Model output is **not** an authorization source. Blockchain actions require the applicable capability/policy/approval path.

## 6. Cross-System Integration Contracts

The three domains are connected through explicit contracts, not implicit authority:

```text
Aurora AI
   │
   ├── Capability / Policy / Approval
   │
   ├──────────────→ GlobusOS
   │                    │
   │                    └──→ ShivaCore / Hardware
   │
   └──────────────→ A-TownChain
                        │
                        ├── RPC / Wallet / State
                        ├── Transaction construction
                        └── Authorized signing / submission
```

For every cross-system connection:

```text
Interface
→ Contract
→ Source
→ Test
→ Workflow
→ Exact-SHA
→ Evidence
→ Verification
→ Residual
```

A connection listed in this document is an architectural requirement, not proof that the connection is currently implemented.

## 7. Repository Ownership

- **OS implementation SSOT:** `a-townchain-os` / relevant ShivaCore repositories
- **Blockchain implementation SSOT:** responsible L1 repositories such as `a-townchain`, `atc-algorithm`, `atc-vm`, `atc-wallet`, etc.
- **Aurora implementation SSOT:** `aurora-ai`
- **Standards/governance SSOT:** `atc-standards`
- **System architecture / integration aggregation:** `a-townchain-ecosystem`

This document must never replace component-level source, tests, release evidence, or repository contracts.

## 8. Verification Boundary

A domain/function may only move from architectural coverage to verified status when the evidence chain exists:

```text
Run
→ Job
→ Result
→ Step
→ Exit code
→ Log
→ Source
→ Root Cause (when applicable)
→ Minimal Fix
→ Commit
→ Rerun
→ Verification
```

Exact-SHA CI is commit-specific. Historical CI runs do not establish verification for a different source SHA.

## 9. Residuals

1. This matrix is an architecture/traceability contract; it does not claim all functions are implemented.
2. Full function coverage requires repository-specific AST/symbol inventory.
3. Hardware/UEFI/Secure Boot execution cannot be inferred from architecture or source presence.
4. Cross-repository integrations require their own source, test, CI and Exact-SHA evidence.
5. Blockchain consensus, VM, wallet/SDK, and state-transition claims require implementation-level evidence.
6. Aurora provider/runtime/RAG/multimodal/agent E2E capabilities remain subject to their own implementation gates.
7. Login/registration and identity claims require actual source-level identity evidence.
