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

## 7. Aurora AI — 23 Canonical Domains

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

## 8. Genesis Engine — Canonical Function Matrix

**Scope:** general-purpose Genesis Engine/editor/runtime/SDK and the Genesis Chronicles content platform. This matrix covers engine capabilities and their contracts; it does not claim that all gameplay, tooling or runtime functions are implemented.

| ID | Domain | Canonical responsibility |
|---|---|---|
| GEN-01 | Engine Core | engine lifecycle, initialization, shutdown, main loop, subsystem orchestration |
| GEN-02 | ECS / World Model | entities, components, systems, archetypes, queries, world state and deterministic updates |
| GEN-03 | Scene / World Management | scenes, worlds, loading, streaming, portals, world transitions and world persistence |
| GEN-04 | Rendering | renderer abstraction, materials, meshes, lighting, shadows, post-processing, cameras and render graph |
| GEN-05 | Physics | collision, rigid bodies, character controllers, constraints, raycasts, triggers and deterministic physics boundaries |
| GEN-06 | Animation | skeletal animation, state machines, blending, IK, facial animation and animation events |
| GEN-07 | Audio | music, ambience, SFX, spatial audio, mixing, buses, dialogue and adaptive audio |
| GEN-08 | Input / Interaction | keyboard, mouse, controller, touch, action mapping, interaction prompts and accessibility input |
| GEN-09 | UI / UX | HUD, menus, inventory, dialogue UI, settings, accessibility and UI framework |
| GEN-10 | Gameplay Framework | quests, dialogue, combat, abilities, progression, factions, items, crafting and gameplay state |
| GEN-11 | AI / NPC Systems | NPC behavior, navigation, perception, behavior trees/state machines, schedules and encounter logic |
| GEN-12 | Networking / Multiplayer | transport abstraction, replication, authority, prediction, synchronization, sessions and multiplayer state |
| GEN-13 | Save / Persistence | save slots, serialization, checkpoints, profiles, world state, migration and recovery |
| GEN-14 | Scripting / Modding | gameplay scripting, extension APIs, sandboxing, mod lifecycle, dependency/version compatibility |
| GEN-15 | Asset Pipeline | import, conversion, validation, dependency graphs, caching, packaging and asset versioning |
| GEN-16 | Resource / Package System | content manifests, bundles, virtual filesystem, streaming resources and runtime package resolution |
| GEN-17 | Editor / Tooling | world editor, scene editor, entity/component inspector, terrain, material, animation and quest tooling |
| GEN-18 | Build / Release Pipeline | deterministic builds, platform packaging, content cooking, artifact signing and release channels |
| GEN-19 | Platform / Hardware Abstraction | CPU/GPU, input, storage, audio, display, OS APIs and platform-specific adapters |
| GEN-20 | Simulation / Time | game clock, ticks, timers, scheduling, pause/slow-motion and deterministic simulation time |
| GEN-21 | Testing / Verification | unit, integration, gameplay, physics, rendering, networking, replay, determinism and regression tests |
| GEN-22 | Developer SDK / APIs | engine API, gameplay API, plugin API, asset API, editor API, runtime API and documentation |
| GEN-23 | Observability / Diagnostics | logging, profiling, telemetry, crash diagnostics, debug overlays and replay diagnostics |
| GEN-24 | Security / Trust Boundary | sandboxing, content validation, network trust, signed assets, anti-tamper boundaries and permission enforcement |
| GEN-25 | Genesis Chronicles | 50-world structure, Ur-Genesis content model, World 51 / Inner War progression, lore, species, quests and premium showcase integration |

### 8.1 Canonical Genesis Engine Pipeline

```text
Project / Assets
  ↓
Asset Import & Validation
  ↓
Content / Package System
  ↓
Editor / Tooling
  ↓
Build / Cook / Package
  ↓
Genesis Runtime
  ↓
ECS / World / Scene
  ↓
Gameplay / AI / Physics / Animation / Audio
  ↓
Rendering / UI / Input
  ↓
Persistence / Networking
  ↓
Platform / OS / Hardware
```

### 8.2 Genesis Engine Invariants

- Engine subsystems communicate through explicit contracts; hidden global state is not a canonical integration mechanism.
- Simulation state, render state and persistent state are distinct concerns.
- Deterministic simulation must not depend on wall-clock time, uncontrolled randomness or nondeterministic host behavior.
- Network authority and replication rules must be explicit before multiplayer state is treated as canonical.
- Asset identity, versioning and dependency resolution must be reproducible.
- Mods/extensions execute within explicit capability and trust boundaries.
- Save migration must be versioned and backwards/forwards compatibility must be defined where supported.
- Editor output must use the same canonical asset/content contracts consumed by runtime.
- AI-generated quests/dialogue/content remain content inputs; runtime authority stays within engine gameplay and security contracts.
- Genesis Chronicles is a product/content layer on the Genesis Engine and must not become an implicit replacement for the general engine architecture.

### 8.3 Genesis Engine Traceability

```text
Vision
→ Genesis Domain
→ Engine Component
→ Function / API
→ Contract
→ Source
→ Test / Replay / Vector
→ Workflow
→ Exact-SHA Evidence
→ Verification
→ Residual
```

Genesis Engine coverage here is architectural. Full function coverage requires repository-level AST/symbol inventory and implementation evidence.

## 9. Cross-System Integration Contracts

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

## 10. GateToHell Browser — Canonical Browser Function Matrix

**Existing-First baseline:** GateToHell Browser is already represented in the ecosystem/Aurora AI Studio sources as a browser component, including the `GateToHellBrowser` UI component, marketplace registration and ecosystem visualization. This matrix defines the canonical capability contract; existing UI presence does not establish production browser/runtime implementation.

| ID | Domain | Canonical responsibility |
|---|---|---|
| GTH-01 | Browser Shell | window lifecycle, tabs, navigation chrome, menus, browser state |
| GTH-02 | Navigation | back/forward, reload, history, navigation lifecycle and error pages |
| GTH-03 | Address / Search Bar | URL entry, search dispatch, normalization, validation and display |
| GTH-04 | Protocol Handler | HTTP(S), canonical `gth://` scheme, protocol registration and routing |
| GTH-05 | Page / Document Runtime | document lifecycle, DOM/page state, script execution boundary and page isolation |
| GTH-06 | Rendering Engine | HTML/CSS rendering, compositing, fonts, media and graphics integration |
| GTH-07 | JavaScript / Web Runtime | script runtime, Web APIs, workers and deterministic security boundaries |
| GTH-08 | Networking | DNS, HTTP(S), connections, proxy configuration, caching and network policy |
| GTH-09 | TLS / Certificate Trust | certificate validation, trust stores, secure transport policy and failure handling |
| GTH-10 | Origin / Site Isolation | origin model, process/site isolation, same-origin enforcement and cross-origin policy |
| GTH-11 | Permissions / Capabilities | camera, microphone, location, notifications, storage, clipboard and other capability grants |
| GTH-12 | Identity / Sessions | browser identity, login/session state, credential handling and session isolation |
| GTH-13 | Wallet / Blockchain | wallet integration, chain state queries, transaction construction, signing and approval boundaries |
| GTH-14 | Aurora AI Integration | AI-assisted navigation, page understanding, search, summarization and tool invocation through explicit capabilities |
| GTH-15 | GateToHell Service Discovery | `gth://` services, registered applications, system resources and service resolution |
| GTH-16 | Marketplace / Applications | app discovery, launch, metadata, lifecycle, package/install integration and trust metadata |
| GTH-17 | Downloads / Uploads | transfer lifecycle, destination policy, file validation, progress, cancellation and quarantine |
| GTH-18 | Storage / Cache | cookies, local/session storage, HTTP cache, indexed data and encrypted browser state |
| GTH-19 | Privacy / Tracking Protection | cookie policy, storage partitioning, tracker controls, permissions and privacy modes |
| GTH-20 | Content Security | sandboxing, CSP/security headers, content validation, malicious-content isolation and exploit boundaries |
| GTH-21 | Extensions / Web Apps | extension APIs, web-app lifecycle, permissions, signing and compatibility |
| GTH-22 | Developer Tools | inspector, console, network diagnostics, storage inspection, profiling and debugging |
| GTH-23 | Accessibility / UX | keyboard navigation, screen-reader semantics, scaling, contrast and accessible controls |
| GTH-24 | Observability / Audit | browser events, security decisions, navigation diagnostics, errors and auditable privileged actions |
| GTH-25 | Testing / Verification | protocol, navigation, rendering, security, permissions, wallet/AI integration, regression and Exact-SHA evidence |
| GTH-26 | Platform / OS Integration | GlobusOS APIs, filesystem, network, graphics, identity, capability enforcement and ShivaCore boundary |

### 10.1 Canonical GateToHell Browser Pipeline

```text
User
  ↓
Browser Shell / Address Bar
  ↓
Navigation / Protocol Resolution
  ↓
Security / Identity / Capability Checks
  ↓
Network or gth:// Service Resolution
  ↓
Page / Application Runtime
  ↓
Rendering / Interaction
  ↓
Storage / Downloads / Extensions
  ↓
Aurora AI / Wallet / Blockchain (when explicitly authorized)
  ↓
GlobusOS
  ↓
ShivaCore / Hardware
```

### 10.2 GateToHell Security Invariants

- A page, script or model output is never an authorization source for privileged OS or blockchain actions.
- Browser capabilities are deny-by-default and granted through explicit identity, permission and policy contracts.
- `gth://` services require explicit registration, routing and trust semantics; the protocol itself does not imply privilege.
- Wallet signing requires the canonical wallet/transaction contract and an explicit user or policy approval boundary.
- Aurora AI actions follow `Model → Agent → Capability → Policy → Approval → Tool → GlobusOS → ShivaCore`; browser content cannot bypass this chain.
- Cross-origin and site-isolation rules are explicit security contracts, not UI behavior.
- Downloads, extensions and web applications require content/package validation and trust metadata before privileged integration.
- Browser storage and credentials are isolated by defined identity/origin boundaries.
- Browser implementation status is determined by source/test/CI evidence, not by the existence of the current `GateToHellBrowser` component.

### 10.3 GateToHell Traceability

```text
Vision
→ Browser Domain
→ Component
→ Function / API
→ Security / Runtime Contract
→ Source
→ Test
→ Workflow
→ Exact-SHA Evidence
→ Verification
→ Residual
```

## 12. ATC Mining — Canonical Function Matrix

**Existing-First baseline:** the ecosystem already contains `components/atc-mining` with execution, deterministic hashing, bounded reward primitives, architecture/status/roadmap documentation and ATC-MIN specifications. Mining explicitly does not own consensus; canonical consensus remains in `atc-algorithm`. Reward settlement remains separated from mining where the contract boundary requires it.

| ID | Domain | Canonical responsibility |
|---|---|---|
| MIN-01 | Mining Lifecycle | miner initialization, start/stop, lifecycle state and controlled execution |
| MIN-02 | Work / Candidate Generation | deterministic work generation, block candidate inputs and candidate lifecycle |
| MIN-03 | Hashing Engine | canonical mining hash function, input construction, digest calculation and deterministic hashing |
| MIN-04 | Nonce Search | bounded nonce iteration, overflow handling, search limits and deterministic candidate search |
| MIN-05 | Proof-of-Work | target comparison, proof validity and canonical PoW acceptance criteria |
| MIN-06 | Difficulty / Target | difficulty/target representation, adjustment inputs and boundary conditions |
| MIN-07 | Block Candidate | header/body assembly, commitment inputs, timestamp/height constraints and candidate validation |
| MIN-08 | Reward Calculation | block reward calculation, height-based emission schedule and checked arithmetic |
| MIN-09 | Monetary / Supply Bounds | hard supply ceiling, remaining emission, u128 monetary values and overflow/underflow protection |
| MIN-10 | Reward Claim / Settlement | reward claim construction, settlement interface and double-claim prevention |
| MIN-11 | Miner Identity / Payout | miner identity, payout destination, authorization and reward attribution |
| MIN-12 | Consensus Boundary | explicit separation from proposer selection, validator consensus and finality owned by `atc-algorithm` |
| MIN-13 | Chain / Economic Parameters | chain ID, block interval, target block time, halving schedule and canonical economic constants |
| MIN-14 | Execution Queue | deterministic FIFO work execution, concurrency boundaries and cancellation semantics |
| MIN-15 | Telemetry / Miner Events | structured mining, reward, difficulty and lifecycle events for observability/audit |
| MIN-16 | Security / Anti-Abuse | replay/double-claim protection, invalid-proof rejection, resource bounds and fail-closed behavior |
| MIN-17 | Storage / Evidence | mining evidence artifacts, run metadata, proof records and reproducible evidence references |
| MIN-18 | Testing / Determinism | hash vectors, reward vectors, boundary tests, property tests, deterministic replay and regression tests |
| MIN-19 | Integration / Node Interface | integration with block production, node runtime, mempool/consensus boundaries and downstream settlement |
| MIN-20 | Release / Verification | workflow validation, Exact-SHA evidence, release gates, verification and residual tracking |

### 12.1 Canonical ATC Mining Pipeline

```text
Economic / Consensus Parameters
  ↓
Mining Work Input
  ↓
Candidate Generation
  ↓
Deterministic Hashing
  ↓
Nonce Search
  ↓
Proof-of-Work Validation
  ↓
Valid Block Candidate
  ↓
Consensus / Proposer Boundary
  ↓
Reward Calculation
  ↓
Reward Settlement / State Transition
  ↓
Canonical Blockchain State
```

### 12.2 ATC Mining Invariants

- Mining does **not** redefine consensus; proposer selection, validator voting and finality remain owned by the consensus layer.
- Monetary values use the canonical economic width (`u128`) where applicable; counters such as height/nonce/timestamp remain explicitly typed.
- Reward calculation is deterministic and height-based; no wall-clock-dependent emission logic.
- Supply limits are hard bounds; checked arithmetic must reject overflow/underflow rather than wrap.
- Mining work, nonce search and proof validation must be deterministic for identical canonical inputs.
- Invalid proofs never create valid reward entitlement.
- Reward claims require explicit settlement semantics and protection against duplicate/replay claims.
- Difficulty/target changes must be governed by the canonical consensus/economic contract rather than an independent mining interpretation.
- Mining telemetry is observability/evidence; telemetry does not authorize state transitions.
- Historical documentation mentioning legacy economics (for example 21M ATC) is not treated as current canonical economics; the current canonical architecture uses the established 360M ATC / 18-decimal economics.
- Proof-of-AI concepts remain separate from the canonical mining validity path unless and until a normative protocol contract explicitly integrates them.

### 12.3 ATC Mining Traceability

```text
Vision
→ Mining Domain
→ Mining Component
→ Function / API
→ Economic / Consensus Contract
→ Source
→ Test / Vector
→ Workflow
→ Exact-SHA Evidence
→ Verification
→ Residual
```

The matrix is architectural. Existing mining source/status evidence does not by itself establish full protocol conformance or current-main verification.

## 14. Staking — Canonical Function Matrix

**Existing-First baseline:** the master architecture already defines the validator lifecycle boundary as **Identity → Registration → Stake/Bond → Eligibility → Active Set → Proposal/Voting → Rewards → Performance Accounting → Slashing/Restriction → Exit/Unbond → Withdrawal**. The staking matrix makes these responsibilities explicit without conflating staking with consensus itself.

| ID | Domain | Canonical responsibility |
|---|---|---|
| STK-01 | Staking Lifecycle | stake creation, lifecycle state, activation, active, restricted and exit states |
| STK-02 | Validator Identity | validator identity, consensus key binding, operator identity and key-rotation boundaries |
| STK-03 | Registration | validator registration, metadata, commission parameters and registration validation |
| STK-04 | Stake / Bond | self-bond, delegated stake, minimum stake, bond creation and canonical accounting |
| STK-05 | Delegation | delegation creation, increase, decrease, redelegation and delegation ownership |
| STK-06 | Eligibility | stake thresholds, validator eligibility, eligibility transitions and admission rules |
| STK-07 | Active Set | active validator set construction, set membership, effective stake and activation delays |
| STK-08 | Proposal / Voting Weight | stake-weighted proposal/voting inputs and explicit boundary to consensus |
| STK-09 | Rewards | validator rewards, delegator rewards, commission, reward accrual and deterministic distribution |
| STK-10 | Performance Accounting | participation, uptime/downtime accounting, missed duties and performance records |
| STK-11 | Slashing | equivocation/double-sign evidence, slash calculation, penalties and deterministic application |
| STK-12 | Restriction / Jail | validator restriction, suspension, jail state, reactivation conditions and safety limits |
| STK-13 | Unbonding | unbond requests, unbonding period, pending exits and deterministic state transitions |
| STK-14 | Withdrawal | matured stake withdrawal, payout destination, authorization and settlement |
| STK-15 | Lockups / Escrow | lock periods, escrowed stake, withdrawal constraints and lifecycle invariants |
| STK-16 | Delegation Accounting | per-validator/per-delegator balances, shares, reward debt and accounting invariants |
| STK-17 | Economic Parameters | minimum stake, reward parameters, commission bounds, unbonding period and penalty parameters |
| STK-18 | Security / Evidence | signed evidence, equivocation proofs, replay protection, authorization and audit records |
| STK-19 | Validator / Node Integration | integration with node runtime, validator services, consensus inputs and state transition |
| STK-20 | Governance Boundary | governance-controlled parameter changes, validator-set policy and explicit authority separation |
| STK-21 | Storage / State | canonical staking state, indexes, persistence, recovery and deterministic state reconstruction |
| STK-22 | Testing / Determinism | staking vectors, reward/slashing vectors, lifecycle tests, property tests and deterministic replay |
| STK-23 | Observability | staking events, validator performance, reward/slash events, metrics and audit telemetry |
| STK-24 | Release / Verification | workflow gates, Exact-SHA evidence, verification status and residual tracking |

### 14.1 Canonical Staking Pipeline

```text
Validator Identity
  ↓
Registration
  ↓
Stake / Bond
  ↓
Eligibility
  ↓
Active Validator Set
  ↓
Proposal / Voting Inputs
  ↓
Rewards / Performance Accounting
  ↓
Slashing / Restriction
  ↓
Exit / Unbond
  ↓
Withdrawal
  ↓
Canonical Blockchain State
```

### 14.2 Staking Invariants

- Staking state is canonical blockchain state; wallets, explorers and UI views are not authoritative.
- Validator identity, operator identity, consensus key and payout identity remain explicitly bound rather than implicitly interchangeable.
- Stake amounts use the canonical monetary width (`u128`) where applicable.
- Validator eligibility and active-set transitions are deterministic and height/epoch governed.
- Consensus owns proposal/voting/finality rules; staking supplies explicitly defined economic/eligibility inputs and does not redefine consensus.
- Rewards and commissions are deterministic and use checked arithmetic.
- Slashing requires verifiable evidence and deterministic penalty rules; telemetry alone is never slash authority.
- Unbonding and withdrawal are distinct states; immature stake cannot be withdrawn.
- Delegation accounting must preserve conservation and prevent double-crediting, double-withdrawal and replay.
- Key rotation, revocation and validator exit must have explicit lifecycle semantics.
- Governance may control parameters only through the canonical governance authority; local validator configuration cannot silently override protocol rules.
- Error Evidence ≠ Finding Evidence ≠ Verification Evidence.
- Architecture ≠ Implementation ≠ Test ≠ CI Evidence ≠ Verification.

### 14.3 Staking Traceability

```text
Vision
→ Staking Domain
→ Validator / Delegation Component
→ Function / API
→ Economic / Consensus / Governance Contract
→ Source
→ Test / Vector
→ Workflow
→ Exact-SHA Evidence
→ Verification
→ Residual
```

The matrix is architectural. Existing validator/staking architecture does not by itself establish complete implementation or current-main verification.

## 16. Repository Ownership

- **OS implementation SSOT:** `a-townchain-os` / relevant ShivaCore repositories
- **Blockchain implementation SSOT:** responsible L1 repositories such as `a-townchain`, `atc-algorithm`, `atc-vm`, `atc-wallet`, etc.
- **Aurora implementation SSOT:** `aurora-ai`
- **Standards/governance SSOT:** `atc-standards`
- **System architecture / integration aggregation:** `a-townchain-ecosystem`

This document must never replace component-level source, tests, release evidence, or repository contracts.

## 11. Verification Boundary

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

## 12. Residuals

1. This matrix is an architecture/traceability contract; it does not claim all functions are implemented.
2. Full function coverage requires repository-specific AST/symbol inventory.
3. Hardware/UEFI/Secure Boot execution cannot be inferred from architecture or source presence.
4. Cross-repository integrations require their own source, test, CI and Exact-SHA evidence.
5. Blockchain consensus, VM, wallet/SDK, and state-transition claims require implementation-level evidence.
6. Aurora provider/runtime/RAG/multimodal/agent E2E capabilities remain subject to their own implementation gates.
7. Login/registration and identity claims require actual source-level identity evidence.
