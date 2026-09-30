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

## 5. ATCLang — Canonical Language / Compiler Matrix

**Scope:** canonical smart-contract language from source text through lexical/syntactic analysis, semantics, ATC-IR, optimization, code generation and ATCA artifact production.

| ID | Domain | Canonical responsibility |
|---|---|---|
| LANG-01 | Language Specification | grammar, keywords, literals, operators, statements, declarations, types and normative semantics |
| LANG-02 | Lexer | source tokenization, literals, identifiers, comments, source spans and lexical diagnostics |
| LANG-03 | Parser / AST | grammar parsing, AST construction, source locations and syntax diagnostics |
| LANG-04 | Type System | primitive/compound types, U64/U128/U256, arrays, maps, structs, option/result semantics and type compatibility |
| LANG-05 | Name / Symbol Resolution | scopes, declarations, imports, namespaces, overload rules and symbol diagnostics |
| LANG-06 | Semantic Analysis | type checking, ownership/validity rules, control flow, unreachable code and contract invariants |
| LANG-07 | Contract Model | contract declarations, entry points, state, events, errors, interfaces and lifecycle semantics |
| LANG-08 | Standard Library | canonical math, crypto, encoding, collections, string, wallet, chain and I/O APIs with deterministic semantics |
| LANG-09 | Deterministic Semantics | forbidden nondeterminism, canonical integer behavior, evaluation order and deterministic host boundaries |
| LANG-10 | Security / Safety Rules | unsafe constructs, resource limits, recursion/depth constraints, capability boundaries and compile-time rejection |
| LANG-11 | ATC-IR Generation | typed canonical IR generation, normalization, validation and IR invariants |
| LANG-12 | Optimization | deterministic optimization passes that preserve observable contract semantics |
| LANG-13 | Code Generation | ATC-IR → canonical bytecode/ATCA encoding with reproducible output |
| LANG-14 | Diagnostics | structured errors, warnings, source spans, error codes and machine-readable diagnostics |
| LANG-15 | ABI Generation | function/interface schemas, argument/return types, events, errors and compatibility metadata |
| LANG-16 | Artifact Generation | canonical ATCA package/artifact, metadata, versioning, hashes and reproducible artifact layout |
| LANG-17 | Toolchain / CLI | compiler CLI, formatter/linter, build/check commands, artifact inspection and developer workflow |
| LANG-18 | Conformance / Compatibility | language-version compatibility, golden files, cross-implementation conformance and migration rules |
| LANG-19 | Testing / Verification | parser/type/semantic/compiler tests, negative tests, fuzzing, golden vectors, determinism and Exact-SHA evidence |

### 5.1 Canonical ATCLang Pipeline

```text
ATCLang Source
  ↓
Lexer / Tokens
  ↓
Parser / AST
  ↓
Name & Symbol Resolution
  ↓
Type / Semantic Analysis
  ↓
Contract & Safety Validation
  ↓
ATC-IR
  ↓
Deterministic Optimization
  ↓
Code Generation
  ↓
Bytecode / ATCA Artifact
  ↓
Independent VM Verifier
  ↓
ATC-VM Execution
  ↓
Canonical State Transition
```

### 5.2 ATCLang Canonical Invariants

- The language specification is normative; implementation behavior must conform to the canonical grammar and semantics.
- Numeric widths and conversions are explicit; implicit narrowing or lossy conversion is forbidden unless the language contract explicitly defines it.
- Contract compilation must be deterministic for identical source, compiler version and canonical inputs.
- Standard-library functions exposed to contracts must have deterministic, specified behavior and explicit host boundaries.
- Compiler acceptance does not imply VM execution validity; generated artifacts require independent verification.
- ABI output must be canonical and compatible with the transaction/contract interface contract.
- Diagnostics are evidence of compiler behavior, not VM verification evidence.
- Language changes require versioning, compatibility analysis, tests and Exact-SHA evidence.
- Python/reference implementations may support specification/conformance work but do not silently replace the canonical Rust-first compiler decision.

### 5.3 ATCLang Traceability

```text
Vision
→ Language Domain
→ Grammar / Compiler Component
→ Function / Syntax / Type / API
→ Language Contract
→ Source
→ Test / Golden Vector
→ Workflow
→ Exact-SHA Evidence
→ Verification
→ Residual
```

ATCLang architecture does not claim that every compiler stage is implemented or verified. Independent verifier acceptance, complete Rust-first compiler coverage, canonical ABI/artifact validation and cross-implementation determinism require repository-level evidence.

## 6. ATC-VM — Canonical Function Matrix

**Scope:** deterministic smart-contract execution from ATCLang source through ATC-IR/ABI, bytecode verification, ATCA artifacts, ATC-VM execution and canonical state transition.

| ID | Domain | Canonical responsibility |
|---|---|---|
| VM-01 | Language Frontend | ATCLang lexer, parser, AST, syntax diagnostics, source locations |
| VM-02 | Semantic Analysis | type checking, symbol resolution, control-flow rules, determinism constraints |
| VM-03 | ATC-IR | canonical intermediate representation, typed operations, validation invariants |
| VM-04 | ABI / Contract Interface | function signatures, argument/return types, serialization, events, compatibility |
| VM-05 | Bytecode Format | instruction set, operand encoding, constants, metadata, canonical encoding |
| VM-06 | Compiler / Codegen | AST/semantics → ATC-IR → bytecode/ATCA artifact, deterministic code generation |
| VM-07 | Independent Verifier | bytecode validity, instruction safety, type/stack rules, control-flow validity, resource bounds |
| VM-08 | Typed Value System | U64, U128, U256 and other canonical runtime types, conversions, overflow/underflow rules |
| VM-09 | Execution Engine | deterministic instruction dispatch, stack/memory semantics, calls, returns, traps |
| VM-10 | State / Storage Interface | contract state reads/writes, storage isolation, canonical state-transition interface |
| VM-11 | Gas / Resource Accounting | instruction cost, memory/storage/call limits, deterministic exhaustion and failure |
| VM-12 | Security / Sandbox | isolation, forbidden operations, capability boundary, recursion/depth limits, DoS resistance |
| VM-13 | Determinism | canonical arithmetic, encoding, execution order, no nondeterministic host dependencies |
| VM-14 | Runtime / Host Boundary | explicit host functions, environment inputs, chain context, syscall/API boundary |
| VM-15 | Artifact / Deployment Lifecycle | ATCA artifact validation, deployment, compatibility, versioning, upgrade constraints |
| VM-16 | Testing / Vectors | unit, property, fuzz, conformance, cross-language vectors, negative/security tests |
| VM-17 | Evidence / Verification | exact-SHA CI, verifier evidence, execution evidence, RCA, rerun and residual tracking |

### 5.1 Canonical ATC-VM Pipeline

```text
ATCLang
  ↓
Lexer / Parser / AST
  ↓
Semantic Analysis
  ↓
ATC-IR
  ↓
ABI / Type Validation
  ↓
Optimizer / Codegen
  ↓
Bytecode / ATCA Artifact
  ↓
Independent Verifier
  ↓
ATC-VM
  ↓
Host / State Interface
  ↓
Deterministic State Transition
  ↓
Canonical Blockchain State
```

### 5.2 VM Type and Safety Invariants

- Monetary/state-economic values must support the canonical **u128** representation where required by the L1 contract.
- VM arithmetic must define overflow, underflow, division-by-zero, conversion and comparison semantics explicitly.
- Typed execution must not silently reinterpret one integer width as another.
- Stack, memory, storage, call-depth and program-operation limits are consensus-relevant when they affect execution validity.
- Verifier acceptance is a prerequisite for execution; source presence or compiler output alone is not proof of executable validity.
- Host/environment access must be explicit and deterministic; hidden filesystem, clock, randomness or network dependencies are forbidden in deterministic execution.
- VM failure/trap semantics must be deterministic and distinguish validation failure from runtime failure.
- Cross-language vectors must prove byte-identical encoding and execution semantics where Rust/TypeScript/wallet tooling participates.

### 5.3 Canonical VM Traceability

Every VM function follows:

```text
Vision
→ VM Domain
→ Component
→ Function / Opcode / API
→ Contract
→ Source
→ Test / Vector
→ Workflow
→ Exact-SHA Evidence
→ Verification
→ Residual
```

VM-specific evidence separation remains mandatory:

```text
Compiler Error Evidence ≠ Verifier Finding Evidence ≠ Execution Verification Evidence
```

The matrix does not claim that every VM domain is implemented. In particular, **Independent Verifier**, complete typed-value coverage, canonical bytecode/ATCA validation and full cross-language determinism require source-level and test-level evidence before they can become VERIFIED.

## 6. Aurora AI — 23 Canonical Domains

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

## 7. Cross-System Integration Contracts

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

## 8. Repository Ownership

- **OS implementation SSOT:** `a-townchain-os` / relevant ShivaCore repositories
- **Blockchain implementation SSOT:** responsible L1 repositories such as `a-townchain`, `atc-algorithm`, `atc-vm`, `atc-wallet`, etc.
- **Aurora implementation SSOT:** `aurora-ai`
- **Standards/governance SSOT:** `atc-standards`
- **System architecture / integration aggregation:** `a-townchain-ecosystem`

This document must never replace component-level source, tests, release evidence, or repository contracts.

## 9. Verification Boundary

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

## 10. Residuals

1. This matrix is an architecture/traceability contract; it does not claim all functions are implemented.
2. Full function coverage requires repository-specific AST/symbol inventory.
3. Hardware/UEFI/Secure Boot execution cannot be inferred from architecture or source presence.
4. Cross-repository integrations require their own source, test, CI and Exact-SHA evidence.
5. Blockchain consensus, VM, wallet/SDK, and state-transition claims require implementation-level evidence.
6. Aurora provider/runtime/RAG/multimodal/agent E2E capabilities remain subject to their own implementation gates.
7. Login/registration and identity claims require actual source-level identity evidence.
