# MASTER-ARCH-001 — System Completeness & Authority Contract

**Status:** ACTIVE / CANONICAL ARCHITECTURE CONTRACT  
**Scope:** A-TownChain Ecosystem System-of-Systems  
**Canonical integration repository:** A-TownChain-Okosystems/a-townchain-ecosystem  
**Component SSOT rule:** this contract does not replace component repository Source of Truth.

---

## 1. Purpose

MASTER-ARCH-001 defines the system-wide architecture contract for the A-TownChain ecosystem.

It defines:

- system boundaries
- architectural layers
- subsystem responsibilities
- authority boundaries
- state ownership
- canonical interfaces
- event contracts
- security boundaries
- failure and recovery obligations
- compatibility and migration obligations
- observability and evidence requirements
- repository ownership and SSOT relationships
- lifecycle and completeness requirements

The contract separates:

> **Component ≠ Function ≠ Subsystem ≠ Authority ≠ Interface ≠ Evidence**

The master architecture describes relationships and obligations. Implementation remains in the canonical repository for each domain.

---

## 2. Normative Terms

The terms **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative.

A master-architecture statement is not implementation evidence.

Status levels are distinct:

~~~text
PRESENT
  ↓
SPECIFIED
  ↓
IMPLEMENTED
  ↓
TESTED
  ↓
CI_VERIFIED
  ↓
INTEGRATED
  ↓
E2E_VERIFIED
  ↓
RELEASED
  ↓
DEPLOYED
~~~

---

## 3. System Boundaries

### 3.1 In Scope

The system-of-systems architecture includes:

1. A-TownChain L1
2. ATCLang
3. ATC-IR / ABI / bytecode
4. ATC-VM
5. Node / consensus / mempool / state
6. Network / P2P / synchronization
7. Wallet / SDK
8. Identity / credentials / trust
9. Memory Federation
10. Aurora AI
11. World Intelligence
12. Dialogue AI
13. Quest AI
14. Genesis Engine / Runtime
15. GlobusOS
16. ShivaCore
17. Storage / indexing / explorer
18. Oracle / compute / interoperability
19. Governance
20. Security / evidence / lifecycle

### 3.2 Out of Scope

This contract does not itself define:

- the complete implementation of any component
- a replacement for repository-local API specifications
- a replacement for ATCLang language specification
- a replacement for VM implementation specifications
- a replacement for OS/kernel specifications
- a replacement for game/content canon
- a replacement for governance standards maintained by atc-standards

---

## 4. Canonical Layer Architecture

~~~text
APPLICATION
  Genesis / Wallet / SDK / Applications
        ↓
INTELLIGENCE
  Aurora / Agents / Dialogue / Quest / World AI / RAG
        ↓
WORLD / DATA
  Memory / Lore / World / Player / NPC / Economy / Index
        ↓
PROTOCOL / API
  APIs / Events / IPC / Interop / ATP
        ↓
BLOCKCHAIN
  Node / Mempool / Validation / Consensus / Finality / State
        ↓
VM
  ATCLang / IR / ABI / Bytecode / Verifier / ATC-VM
        ↓
NETWORK
  P2P / Discovery / Gossip / Sync / Transport
        ↓
TRUST / SECURITY
  Identity / Crypto / Capability / Policy / Authorization
        ↓
OS / TCB
  GlobusOS / ShivaCore / HAL / Secure Boot
        ↓
HARDWARE
  CPU / GPU / NPU / TPM / TEE / Storage / NIC
~~~

Cross-cutting domains:

- Security
- Identity
- Governance
- Observability
- Evidence
- Compatibility
- Lifecycle

---

## 5. System-of-Systems Map

~~~text
A-TOWNCHAIN ECOSYSTEM
│
├── TRUST
│   ├── Identity
│   ├── Credentials
│   ├── Capability
│   ├── Policy
│   └── Authorization
│
├── PROTOCOL
│   ├── Network
│   ├── Blockchain
│   ├── Consensus
│   ├── VM
│   ├── Storage
│   └── Interoperability
│
├── INTELLIGENCE
│   ├── Aurora
│   ├── Agents
│   ├── RAG
│   ├── Dialogue AI
│   ├── Quest AI
│   └── World Intelligence
│
├── DATA / STATE
│   ├── Consensus State
│   ├── Account State
│   ├── Contract State
│   ├── Identity State
│   ├── Memory State
│   ├── World State
│   ├── Player State
│   └── Economy State
│
├── APPLICATION
│   ├── Genesis
│   ├── Wallet
│   ├── SDK
│   ├── Marketplace
│   └── Explorer
│
└── OS / TCB
    ├── GlobusOS
    ├── ShivaCore
    ├── HAL
    └── Hardware / TEE / TPM
~~~

---

## 6. Repository and SSOT Contract

Every subsystem MUST define:

| Field | Requirement |
|---|---|
| Canonical Repository | MUST be explicit |
| Source of Truth | MUST be explicit |
| Owner | MUST be explicit |
| API/ABI Owner | MUST be explicit where applicable |
| Test Owner | MUST be explicit |
| CI Owner | MUST be explicit |
| Security Owner | MUST be explicit |
| Release Owner | MUST be explicit |
| Evidence Owner | MUST be explicit |
| Integration Repository | MUST be explicit |
| Dependency Direction | MUST be explicit |

### Standalone First

~~~text
CORE REPOSITORY
 ├── Source
 ├── Tests
 ├── Build
 ├── API / ABI
 ├── Release
 └── Evidence
          ↓
ECOSYSTEM INTEGRATION
~~~

The ecosystem repository MUST NOT become a hidden implementation dependency of a core repository.

---

# 7. MASTER-ARCH-001 Authority Contract

## 7.1 Authority Matrix

| Subsystem | Read | Propose | Execute | State Mutation | Final Authority |
|---|---:|---:|---:|---:|---|
| Consensus | ✓ | ✓ | ✓ | ✓ | Consensus protocol |
| VM | ✓ | — | ✓ | ✓* | VM/state rules |
| State | ✓ | — | ✓ | ✓ | State transition rules |
| Network | ✓ | ✓ | ✓ | network state | Network protocol |
| Identity | ✓ | ✓ | authorized | identity state | Identity protocol |
| Memory | ✓ | ✓ | replicated | memory state | Memory contract |
| Aurora | ✓ | ✓ | policy-bound | policy-bound | None over consensus |
| Genesis | ✓ | ✓ | ✓ | world/game state | World authority |
| GlobusOS | ✓ | ✓ | ✓ | OS state | OS policy |
| ShivaCore | ✓ | — | TCB | TCB state | Kernel boundary |
| Governance | ✓ | ✓ | policy-bound | protocol changes | Governance protocol |
| Wallet | ✓ | ✓ | user-authorized | authorized account actions | User/key authority |
| Oracle | ✓ | ✓ | attestation | oracle state | Oracle protocol |

VM state mutation is valid only through the canonical deterministic state-transition contract.

## 7.2 Authority Invariants

**AUTH-001:** A subsystem MUST NOT exercise authority not explicitly assigned to it.

**AUTH-002:** Capability grants execution ability; it does not grant ownership or final authority.

**AUTH-003:** Policy grants conditional permission; it does not redefine protocol authority.

**AUTH-004:** Aurora MUST NOT acquire implicit consensus, governance, kernel, or canonical-state authority.

**AUTH-005:** Genesis world authority MUST NOT redefine blockchain consensus state.

**AUTH-006:** Memory MUST NOT become consensus state, world canon, or AI authority by implication.

**AUTH-007:** Integration repositories MUST NOT acquire authority over Core repository Source of Truth.

---

## 8. Identity and Trust

Identity domains:

- Human Identity
- User Identity
- Account
- Wallet
- Device Identity
- Node Identity
- Validator Identity
- Service Identity
- Agent Identity
- AI Model Identity
- Game Identity
- NPC Identity
- World Identity
- Organization Identity

Trust chain:

~~~text
Identity
  ↓
Credential
  ↓
Capability
  ↓
Policy
  ↓
Authorization
  ↓
Execution
  ↓
Audit
  ↓
Evidence
~~~

Required capabilities:

- public-key identity
- key binding
- key rotation
- revocation
- attestation
- device binding
- node registration
- agent authorization

---

## 9. Network / P2P Contract

Required functions:

- peer discovery
- peer identity
- DHT / discovery mechanism
- authenticated handshake
- transport
- NAT traversal where applicable
- connection management
- peer scoring
- Sybil resistance
- gossip
- transaction propagation
- block propagation
- consensus messaging
- state synchronization
- snapshot synchronization
- fast synchronization
- chunk transfer
- bandwidth management
- backpressure
- connection recovery
- partition detection
- network rejoin

Canonical ATP protocol domains:

~~~text
ATP
├── Peer Protocol
├── Discovery Protocol
├── Gossip Protocol
├── Block Protocol
├── Transaction Protocol
├── Consensus Protocol
├── State Sync Protocol
└── Snapshot Protocol
~~~

Each protocol MUST define schema, authentication, authorization, replay protection, versioning, error semantics and recovery.

---

## 10. Blockchain / Consensus Contract

Canonical transaction lifecycle:

~~~text
SDK
 ↓
Transaction
 ↓
Signature
 ↓
Mempool
 ↓
Validation
 ↓
Consensus
 ↓
Block
 ↓
Finality
 ↓
State Transition
 ↓
Storage
 ↓
Indexer
~~~

The contract covers:

- transaction validation
- signatures
- nonce
- fees
- block construction
- block validation
- validator set
- validator rotation
- finality
- fork handling
- reorganization policy
- chain selection
- checkpoints
- state commitments
- rewards
- halving
- slashing
- treasury
- epochs
- upgrade activation

---

## 11. State and Data Ownership

Every state domain MUST define:

- owner
- authority
- schema
- canonical serialization
- hashing
- version
- persistence
- replication
- consistency
- recovery
- read API
- write API
- audit
- migration

Canonical domains:

~~~text
Consensus State
Blockchain State
Account State
Wallet State
Contract State
VM State
Node State
Identity State
Governance State
Memory State
World State
Player State
NPC State
Character State
Quest State
Dialogue State
Lore State
Item State
Economy State
Marketplace State
AI State
Model State
~~~

Mandatory boundaries:

~~~text
Memory State     ≠ Consensus State
Memory State     ≠ World Canon
Memory State     ≠ AI Authority

Application Data ≠ Consensus Data
Private Memory   ≠ AI Context
~~~

---

## 12. Memory Federation Contract — MEMORY-001 Integration Boundary

~~~text
Canonical Memory Record
 ↓
Canonical Serialization
 ↓
Content Hash
 ↓
Authorization
 ↓
Replication
 ↓
Federation Handshake
 ↓
Peer Verification
 ↓
Persistence
 ↓
Audit
 ↓
Evidence
~~~

Required contract fields:

- record ID
- schema version
- canonical serialization
- content hash
- provenance
- ownership
- authorization
- encryption metadata
- replication metadata
- version
- conflict policy
- merge rules
- retention
- expiration
- revocation
- deletion semantics
- audit data

Federation MUST be deterministic at the serialization/hash/authorization boundary.

---

## 13. Canonical Interface Architecture

Every canonical interface MUST define:

~~~text
Identity
Authentication
Authorization
Schema
Serialization
Version
Request
Response
Error Model
Timeout
Retry
Idempotency
Ordering
Replay Protection
Consistency
Rate Limits
Audit
Evidence
~~~

API domains include:

- Identity API
- Capability API
- Policy API
- Memory API
- AI API
- Model API
- Blockchain API
- Transaction API
- VM API
- Storage API
- Indexer API
- Event API
- World API
- Game API
- Dialogue API
- Quest API
- Oracle API
- Compute API
- Governance API
- Evidence API

---

## 14. Canonical Event Architecture

Canonical event:

~~~text
event_id
source
subject
type
version
timestamp
sequence
causality
payload
authorization
signature
provenance
replay_policy
~~~

Event domains:

- Blockchain
- VM
- Node
- Identity
- Memory
- AI
- World
- Quest
- Dialogue
- Governance
- Security
- Evidence

Event producers and consumers MUST define schema version compatibility.

---

## 15. Security Architecture

Canonical security path:

~~~text
Hardware Root of Trust
 ↓
Secure Boot
 ↓
ShivaCore
 ↓
GlobusOS
 ↓
Identity
 ↓
Cryptography
 ↓
Capability
 ↓
Policy
 ↓
Authorization
 ↓
Sandbox
 ↓
Execution
 ↓
Audit
 ↓
Evidence
~~~

Security domains:

- secure boot
- measured boot
- attestation
- key management
- key rotation
- secrets
- encryption
- secure storage
- capability security
- policy engine
- sandboxing
- process isolation
- VM isolation
- AI tool isolation
- supply-chain security
- dependency security
- anti-replay
- anti-Sybil
- rate limiting
- abuse detection
- incident response
- recovery

---

## 16. ATCLang / VM Architecture

Canonical execution pipeline:

~~~text
ATCLang
 ↓
Parser
 ↓
AST
 ↓
ATC-IR
 ↓
ABI
 ↓
Bytecode
 ↓
Verifier
 ↓
ATCA
 ↓
ATC-VM
 ↓
Runtime
 ↓
State Transition
~~~

Required specifications:

- grammar
- type system
- compiler
- IR
- ABI
- bytecode
- verifier
- resource/gas model
- determinism
- sandbox
- host interface
- storage interface
- cryptographic interface
- error semantics
- versioning
- compatibility

**VM-001:** ShivaCore MUST NOT be treated as the canonical contract VM unless an explicit architecture contract assigns that role.

---

## 17. Aurora AI Architecture

Aurora is the AI control/intelligence plane, not the kernel.

Canonical planes:

1. Model Plane
2. Runtime Plane
3. Agent Plane
4. Authority Plane
5. Tool Plane
6. Memory / Knowledge Plane
7. Multimodal Plane
8. Domain Intelligence
9. Governance / Evidence
10. Integration Plane

Authority path:

~~~text
Model
 ↓ proposes
Agent
 ↓ requests
Capability
 ↓ checked by
Policy
 ↓ may require
Approval
 ↓ authorizes
Tool
 ↓ executes
GlobusOS
 ↓
ShivaCore
~~~

Invariants:

- models have no implicit execution authority
- agents cannot bypass capability checks
- capabilities are policy-bound
- sensitive actions MAY require approval
- tools execute only through authorized interfaces
- Aurora has no direct kernel authority
- Aurora has no direct consensus authority
- privileged execution MUST be auditable

AURORA-001 remains the component-level completeness/evidence gate.

---

## 18. World Intelligence

Canonical capabilities:

~~~text
World AI
Character AI
Creature AI
Dialogue AI
Quest AI
Lore AI
Event AI
Economy AI
Simulation AI
Narrative AI
Relationship AI
Director AI
~~~

AI output MUST be treated as proposal/context unless the receiving runtime explicitly authorizes execution.

### Dialogue boundary

~~~text
Context
 ↓
Plan
 ↓
Generate
 ↓
Validate
 ↓
Authorize
 ↓
Genesis Runtime
 ↓
Deterministic State Change
~~~

### Quest boundary

~~~text
World State
 ↓
Event Detection
 ↓
Quest Proposal
 ↓
Schema / Canon / Economy / Security Validation
 ↓
Quest Runtime
 ↓
Deterministic State Change
~~~

AI MUST NOT directly mutate authoritative game state.

---

## 19. Genesis Architecture

Required domains:

- Engine
- Editor
- Runtime
- World
- Scene
- Entity / ECS
- Physics
- Rendering
- Audio
- Animation
- AI
- Dialogue
- Quest
- Inventory
- Item
- Weapon
- Mod System
- Economy
- Multiplayer
- Networking
- Persistence
- Save State
- World State
- Scripting
- Asset Pipeline
- Plugin System
- SDK

Mod lifecycle:

~~~text
Mod
 ↓
Manifest
 ↓
Identity
 ↓
Capability Declaration
 ↓
Policy Validation
 ↓
Sandbox
 ↓
Load
 ↓
Runtime
 ↓
Audit
~~~

---

## 20. GlobusOS / ShivaCore

### ShivaCore

- kernel
- scheduler
- memory management
- IPC
- capability system
- process isolation
- device abstraction
- cryptography
- secure-boot boundary
- attestation boundary

### GlobusOS

- HAL
- drivers
- process runtime
- package system
- .gpkg
- service manager
- IPC
- security policy
- A/B update
- rollback
- update verification
- TPM/TEE integration
- network stack
- storage stack
- Aurora runtime integration

The OS layer and AI layer MUST remain authority-separated.

---

## 21. Interoperability

Interop domains:

- chain-to-chain
- chain-to-service
- wallet interoperability
- SDK interoperability
- VM interoperability
- oracle interoperability
- external AI providers
- external storage
- external identity

Every adapter MUST specify:

- identity
- protocol
- message schema
- proof
- finality assumption
- trust model
- replay protection
- failure model
- timeout
- recovery
- audit

---

## 22. Storage Architecture

Required storage domains:

- blockchain storage
- state storage
- VM storage
- object/blob storage
- memory storage
- vector storage
- world storage
- asset storage
- cache
- snapshot storage
- archive storage

Required controls:

- persistence
- replication
- encryption
- integrity
- versioning
- garbage collection
- backup
- restore
- snapshot
- pruning
- archival

---

## 23. Indexing and Query

Required index domains:

- block
- transaction
- account
- contract
- event
- memory
- world
- quest
- dialogue
- asset

Required query surfaces:

- search
- query API
- graph API

Indexes MUST NOT become an alternate authority for canonical state.

---

## 24. Oracle Architecture

Oracle systems MUST define:

- data sources
- oracle nodes
- aggregation
- attestation
- signatures
- freshness
- confidence
- dispute
- failure handling
- economic incentives
- slashing where applicable
- audit

---

## 25. Compute Architecture

Canonical flow:

~~~text
Resource
 ↓
Identity
 ↓
Capability
 ↓
Policy
 ↓
Quota
 ↓
Allocation
 ↓
Execution
 ↓
Metering
 ↓
Settlement
 ↓
Audit
~~~

Domains:

- compute node
- scheduler
- resource registry
- workload manager
- GPU/NPU runtime
- AI inference
- distributed compute
- metering
- marketplace
- settlement

---

## 26. Governance

Canonical lifecycle:

~~~text
IDEA
 ↓
DRAFT
 ↓
REVIEW
 ↓
TECHNICAL
 ↓
SECURITY
 ↓
ARCHITECTURE
 ↓
GOVERNANCE
 ↓
APPROVED
 ↓
IMPLEMENTED
 ↓
TESTED
 ↓
CI VERIFIED
 ↓
E2E VERIFIED
 ↓
STABLE
~~~

Governance authority MUST remain separate from implementation authority.

atc-standards remains the standards/governance SSOT where applicable.

---

## 27. Upgrade and Migration

Required migration domains:

- protocol
- state
- schema
- ABI
- VM
- network
- OS
- AI model
- memory schema
- world
- database

Every migration MUST define:

- compatibility window
- activation condition/height where applicable
- migration procedure
- validation
- rollback/recovery
- evidence

---

## 28. Failure and Recovery

Canonical recovery pattern:

~~~text
Detect
 ↓
Classify
 ↓
Isolate
 ↓
Validate
 ↓
Recover
 ↓
Resync
 ↓
Verify
 ↓
Resume
 ↓
Evidence
~~~

Failure domains:

- transaction
- mempool
- node
- validator
- consensus
- network
- storage
- state
- VM
- AI
- agent
- tool
- memory
- database
- OS
- device
- game runtime

---

## 29. Observability and Evidence

Canonical evidence chain:

~~~text
Runtime
 ↓
Telemetry
 ↓
Logs / Metrics / Traces
 ↓
Events
 ↓
Evidence
 ↓
Artifact
 ↓
Commit SHA
 ↓
CI Run
 ↓
Audit
~~~

Evidence classes:

- source evidence
- build evidence
- test evidence
- CI evidence
- security evidence
- runtime evidence
- E2E evidence
- deployment evidence
- governance evidence

**EVIDENCE-001:** Evidence MUST identify the exact source revision being evaluated.

**EVIDENCE-002:** A successful CI run on another commit MUST NOT be used as evidence for the current commit.

**EVIDENCE-003:** File existence MUST NOT be interpreted as implementation evidence.

---

## 30. Deployment and Topology

Canonical node topology:

~~~text
Hardware
 ↓
ShivaCore
 ↓
GlobusOS
 ↓
Runtime Services
 ├── A-TownChain Node
 ├── Aurora
 ├── Genesis
 ├── Storage
 ├── Indexer
 ├── Oracle
 └── Compute
       ↓
    P2P Network
       ↓
Distributed Ecosystem
~~~

Node roles:

- full node
- validator
- RPC node
- indexer node
- storage node
- oracle node
- compute node
- AI node
- game server
- edge node
- mobile/device node
- offline node

A deployment role MUST NOT implicitly imply consensus authority.

---

## 31. Compatibility

Compatibility MUST be defined for:

| Domain | Required |
|---|---|
| Protocol | Version + compatibility |
| API | Version + compatibility |
| ABI | Version + compatibility |
| ATCLang | Version + compatibility |
| IR | Version + compatibility |
| Bytecode | Version + compatibility |
| VM | Version + compatibility |
| State schema | Version + migration |
| Memory schema | Version + migration |
| Event schema | Version + compatibility |
| Identity schema | Version + compatibility |
| Network protocol | Version + compatibility |
| OS ABI | Version + compatibility |

Each version MUST declare:

- minimum
- maximum
- compatible
- migration required
- deprecated
- removed

---

## 32. Performance and Capacity

The architecture MUST provide target fields for:

- block time
- throughput
- transaction latency
- finality latency
- P2P bandwidth
- storage growth
- memory growth
- VM execution limits
- AI inference latency
- GPU/NPU capacity
- concurrent agents
- concurrent players
- world simulation capacity
- recovery time objective
- recovery point objective

Values are targets until implementation and benchmark evidence exists.

---

## 33. Privacy and Data Governance

Data classes:

- public
- private
- sensitive
- encrypted
- personal
- AI context
- memory

Required controls:

- ownership
- access
- export
- revocation
- deletion
- retention
- provenance
- audit

Mandatory separation:

~~~text
Consensus Data
    ≠
Application Data
    ≠
Private Memory
    ≠
AI Context
~~~

---

## 34. Resource and Economic Architecture

Economic/resource domains:

- ATC monetary layer
- compute economy
- storage economy
- bandwidth economy
- AI inference economy
- marketplace
- fees
- rewards
- staking
- slashing
- treasury
- resource metering
- settlement

Economic state MUST have an explicit owner and authoritative mutation path.

---

## 35. Lifecycle Architecture

Every architectural artifact MUST support:

~~~text
Idea
 ↓
Specification
 ↓
Implementation
 ↓
Test
 ↓
CI
 ↓
Security
 ↓
Integration
 ↓
E2E
 ↓
Release
 ↓
Deployment
 ↓
Monitoring
 ↓
Maintenance
 ↓
Deprecation
 ↓
Migration
 ↓
Retirement
~~~

---

## 36. Subsystem Completeness Contract

Every subsystem registered by the master architecture MUST expose the following contract:

~~~yaml
subsystem:
  id:
  name:
  purpose:
  scope:

  owner:
  authority:
  source_of_truth:

  inputs: []
  outputs: []

  state:
    owned: []
    read: []
    write: []

  data:
    owner:
    schema:
    serialization:
    hashing:
    persistence:
    replication:

  interfaces:
    api: []
    abi: []
    ipc: []
    events: []

  security:
    trust_boundary:
    security_boundary:
    capabilities: []
    policies: []
    authorization:

  operations:
    failure_model:
    recovery_model:
    observability:

  compatibility:
    protocol_version:
    schema_version:
    compatibility_policy:
    migration_policy:

  lifecycle:
    specification:
    implementation:
    testing:
    ci:
    e2e:
    release:
    deprecation:

  evidence:
    source:
    build:
    test:
    ci:
    runtime:
    e2e:
~~~

Missing mandatory fields are an architecture completeness defect.

---

## 37. Architecture Completeness Matrix

The master architecture MUST maintain a machine-checkable matrix with at least:

| Subsystem | Purpose | Owner | Authority | State | API | Events | Security | Failure | Recovery | Version | Tests | CI | E2E |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Blockchain | required | required | required | required | required | required | required | required | required | required | required | required | required |
| VM | required | required | required | required | required | required | required | required | required | required | required | required | required |
| Network | required | required | required | required | required | required | required | required | required | required | required | required | required |
| Identity | required | required | required | required | required | required | required | required | required | required | required | required | required |
| Memory | required | required | required | required | required | required | required | required | required | required | required | required | required |
| Aurora | required | required | required | required | required | required | required | required | required | required | required | required | required |
| Genesis | required | required | required | required | required | required | required | required | required | required | required | required | required |
| GlobusOS | required | required | required | required | required | required | required | required | required | required | required | required | required |
| ShivaCore | required | required | required | required | required | required | required | required | required | required | required | required | required |

The matrix records architecture obligations; it MUST NOT fabricate implementation or CI status.

---

## 38. Master Architecture Invariants

1. **Standalone First:** Core repositories remain independently buildable and testable.
2. **SSOT Separation:** Integration architecture does not replace component SSOT.
3. **Authority Explicitness:** No implicit authority.
4. **Deterministic State:** Authoritative state transitions use explicit deterministic contracts.
5. **AI Boundary:** AI inference produces proposals/context unless explicitly authorized by a receiving runtime.
6. **Memory Boundary:** Memory is not automatically consensus state, world canon, or authority.
7. **Security Boundary:** Privilege requires identity, capability, policy and authorization.
8. **Evidence Integrity:** Evidence is tied to the exact source revision under assessment.
9. **Version Explicitness:** Cross-subsystem contracts are versioned.
10. **Fail Closed:** Invalid or unauthorized state-changing proposals are rejected.
11. **Recovery Explicitness:** Each stateful subsystem defines recovery/resynchronization behavior.
12. **Lifecycle Explicitness:** Architecture, implementation, verification and deployment status remain distinct.

---

## 39. Master System Flow

~~~text
IDENTITY / TRUST
       │
       ▼
CAPABILITY / POLICY
       │
       ▼
INTERFACE / EVENT
       │
 ┌─────┼───────────────────────────────────────┐
 ▼     ▼                    ▼                  ▼
NETWORK  BLOCKCHAIN/VM     AURORA             GENESIS
 │       │                  │                  │
 │       │                  ├── Agents         ├── World
 │       │                  ├── RAG            ├── ECS
 │       │                  ├── Memory         ├── Quest
 │       │                  ├── Tools          ├── Dialogue
 │       │                  └── Policy         └── Runtime
 │       │
 └───────┴──────────────┬──────────────────────┘
                        ▼
                  STATE / STORAGE
                        │
                        ▼
                INDEX / OBSERVABILITY
                        │
                        ▼
                    EVIDENCE
                        │
                        ▼
                 GOVERNANCE / LIFECYCLE
~~~

---

## 40. Repository Authority Map

| Domain | Canonical implementation authority |
|---|---|
| Ecosystem integration architecture | a-townchain-ecosystem |
| Standards / governance | atc-standards |
| A-TownChain protocol | canonical A-TownChain repositories |
| VM | atc-vm / canonical VM sources |
| ATCLang | atclang |
| Aurora | aurora-ai |
| Kernel / TCB | atc-shivacore |
| GlobusOS | globus-os |
| Genesis Engine | genesis-engine |
| Documentation / architecture knowledge | a-townchain-os-docs |

This table is an architecture mapping, not a claim that every listed capability is currently implemented.

---

## 41. Implementation Status Rule

~~~text
ARCHITECTURE
    ≠
IMPLEMENTATION
    ≠
TEST
    ≠
CI
    ≠
E2E
    ≠
DEPLOYMENT
~~~

No architecture section may be interpreted as implementation evidence without repository-local evidence.

---

## 42. Change Control

Changes to MASTER-ARCH-001 MUST:

1. identify the affected subsystem(s)
2. identify authority/state/interface impact
3. identify compatibility impact
4. identify security impact
5. identify migration impact
6. identify affected repository SSOTs
7. provide implementation/evidence follow-up where required
8. preserve the Standalone First rule

Changes that redefine authority boundaries require architecture and governance review according to the applicable standards.

---

## 43. Definition of Done for Master Architecture

MASTER-ARCH-001 is structurally complete when every registered subsystem has:

~~~text
Purpose
Scope
Owner
Authority
SSOT
Inputs
Outputs
State Ownership
Data Contract
Interface Contract
Event Contract
Security Boundary
Capability Requirements
Policy Requirements
Failure Model
Recovery Model
Persistence
Replication
Versioning
Compatibility
Tests
CI Evidence
E2E Evidence
Lifecycle
~~~

A missing implementation is not silently converted into an architecture PASS.

---

## Final Architectural Principle

> The A-TownChain ecosystem is a decentralized system-of-systems in which authority, state ownership, interfaces, security, execution, and evidence are explicit architectural dimensions.

> The master architecture defines how the systems fit together; each canonical repository remains responsible for its own implementation, verification, release, and evidence.
