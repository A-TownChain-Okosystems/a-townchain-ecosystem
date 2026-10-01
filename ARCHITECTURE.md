# A-TownChain Ecosystem — System-Architektur

## Architekturstatus

**Status:** ACTIVE  
**Architekturrolle:** System-of-Systems Master Architecture  
**Prinzip:** SSOT je Domäne, zentrale Integrationsarchitektur im Ecosystem  
**Quest AI:** als kanonische Game-Intelligence-Funktion in Aurora/Genesis integriert

> **Canonical architecture rule:** L0–L7 are architectural responsibility domains, not a mandatory runtime ordering. **X** is a cross-layer control plane, not an additional execution layer. Architecture, implementation, testing, CI evidence and E2E evidence are separate status dimensions.

## Canonical Layer Model

A-TownChain uses one canonical deterministic L1 responsibility model. The layers define architectural ownership and trust boundaries; they are not a mandatory single-process execution order. **X** is a cross-layer control plane and is not an additional execution layer.

```text
L0 — Hardware / Secure Platform
     CPU / RAM / Storage / TPM / TEE / Secure Boot

L1 — Node & Runtime Foundation
     OS / ShivaCore / Runtime / IPC / Capability Security

L2 — Blockchain Core
     Block Model / Transaction Model / State Model / Ledger /
     Mempool / Consensus / Finality / Chain Validation

L3 — ATC-VM
     Program Model / Typed Values / Execution /
     Gas & Resource Accounting / Storage / Deterministic VM Validation

L4 — Protocol Services
     P2P / Networking / Synchronization / Peer Discovery /
     RPC / API / State & Block Propagation

L5 — Economic & Security Services
     Accounts / Signatures / Staking / Rewards / Slashing /
     Treasury / Supply Rules

L6 — Smart-Contract / Application Environment
     ATCLang / Contract ABI / Contract Standards / Application Protocols

L7 — External Applications
     Wallet / SDK / Explorer / DApps / Developer Tooling

X — Cross-Layer Control Plane
     Identity / Capability / Policy / Governance / Cryptography /
     Audit / Evidence / Observability / Interoperability / Versioning
```

### Layer Semantics

| Layer | Primary responsibility | Authority | Determinism / state boundary |
|---|---|---|---|
| L0 | Hardware and secure-platform primitives | Platform boundary | Trusted execution substrate; no canonical chain authority |
| L1 | Node/runtime foundation, ShivaCore, IPC and capability enforcement | Runtime authority within host | Deterministic where protocol-critical; no canonical chain state |
| L2 | Blockchain Core: blocks, transactions, state, ledger, mempool, consensus, finality, chain validation | Canonical blockchain authority | Deterministic protocol state machine |
| L3 | ATC-VM program execution and resource accounting | Execution authority delegated by L2 | Deterministic execution only; no independent finality |
| L4 | P2P/networking, synchronization, discovery, RPC and propagation | Transport/service authority | Transports claims; does not make them canonical |
| L5 | Accounts, signatures, staking, rewards, slashing, treasury and supply rules | Economic/security domain authority under L2 | Protocol-defined deterministic rules and canonical economic state |
| L6 | ATCLang, ABI, contract standards and application protocols | Contract/application authority within declared interfaces | Contract semantics are deterministic when consensus-critical |
| L7 | Wallet, SDK, explorer, DApps and developer tooling | User/application boundary | No direct canonical blockchain authority |
| X | Identity, capability, policy, governance, crypto, audit/evidence, observability, interop and versioning | Cross-layer control authority | Cross-cutting; never implicit authority |

**Important:** layer numbering is an architectural responsibility model, not a runtime call graph. L1 being numbered below L2 does not mean storage/runtime executes before consensus.

### Architectural Separation Invariants

1. **L2 Blockchain Core ≠ L3 ATC-VM.** L2 owns blockchain state-machine and finality semantics; L3 performs deterministic program execution.
2. **L4 P2P transports claims; L2 validates protocol claims.** Network reachability or successful propagation never grants authority.
3. **L5 owns economic/security services; L2 owns blockchain consensus/finality.** Economic rules must not silently become a second consensus layer.
4. **L6 contracts are not the blockchain core.** ATCLang/ABI/application protocols invoke deterministic execution through declared interfaces.
5. **L7 is non-authoritative.** Wallets, SDKs, explorers and DApps cannot bypass L2/L3/L5 validation boundaries.
6. **X does not become an execution layer.** Control-plane policy can constrain actions but does not itself mutate canonical state.

## Canonical Deterministic Trust Boundary

The L1 trust boundary distinguishes integrity, authenticity, validity, consensus, finality and immutability. A hash establishes an integrity relationship; it does not by itself establish protocol acceptance or finality.

### Canonical transaction-to-finality path

```text
Transaction
    │
    ▼
Canonical Encoding
    │
    ▼
Signature / Authorization Validation
    │
    ▼
Transaction Validation
    │
    ├── chain_id / nonce
    ├── canonical numeric types
    ├── amount / fee rules
    ├── account/state constraints
    └── protocol rules
    │
    ▼
L2 Mempool Admission
    │
    ▼
L4 P2P Propagation / Synchronization
    │
    ▼
L2 Block Construction / Block Validation
    │
    ▼
Consensus
    │
    ▼
L3 ATC-VM Deterministic Execution
    │
    ▼
L2 State Transition / State Commitment
    │
    ▼
L2 Ledger Commit
    │
    ▼
Finality
    │
    ▼
Canonical State
    │
    ▼
L1 Durable Persistence / Read Models
```

### Integrity ≠ Consensus ≠ Finality ≠ Immutability

| Property | Meaning |
|---|---|
| **Integrity** | Data can be detected as changed through hashes/commitments. |
| **Authenticity** | A signature/identity proof establishes the cryptographic origin or authorization relationship required by the protocol. |
| **Validity** | The transaction/block/state transition satisfies the canonical protocol rules. |
| **Consensus** | The protocol determines which valid proposed history/state is accepted by participating validators/nodes. |
| **State transition** | The deterministic function transforms an accepted input state into the resulting state. |
| **Finality** | The protocol reaches its defined finality condition for the accepted block/state. |
| **Immutability** | Finalized history is not replaced by an ordinary alternative transition; exceptional changes require an explicitly authorized protocol/governance mechanism. |

For a deterministic L1, the core execution contract is `F(State, Block) = State'`. For identical canonical inputs and protocol/configuration versions, conforming implementations MUST produce the same resulting state commitment.

### P2P Protocol Contract

```text
Peer Identity
    ↓
Authentication
    ↓
Peer Discovery / Connection Management
    ↓
Message Validation
    ↓
Rate Limiting / Resource Controls
    ↓
Transaction / Block Propagation
    ↓
Synchronization / State Transfer
    ↓
Failure Detection / Recovery
```

P2P is a protocol-service domain, not undifferentiated infrastructure. Messages received through L4 remain untrusted inputs until the responsible L2/L3/L5 validation boundary accepts them.

### Boundary invariants

```text
Aurora
  ≠ Consensus
  ≠ ATC-VM
  ≠ canonical blockchain state

Genesis Engine
  ≠ Blockchain Core

GlobusOS / ShivaCore
  ≠ ATC-VM

Indexer / Explorer
  ≠ canonical Source of Truth

AI Memory
  ≠ canonical blockchain state
```

AI/model inference may produce proposals, commands or requests. Authoritative state changes require the responsible deterministic runtime plus applicable identity, capability, policy and protocol validation.

## Cross-Layer Control Plane

X is a cross-cutting control plane, not an execution layer and not an alternative owner of canonical state.

```text
                    X — CONTROL PLANE
┌─────────────────────────────────────────────────────────────┐
│ Identity │ Capability │ Policy │ Governance │ Cryptography  │
│ Audit │ Evidence │ Observability │ Interoperability        │
│ Versioning / Compatibility │ Authority / Trust Boundaries  │
└──────────────────────────────┬──────────────────────────────┘
                               │
             ┌─────────────────┼─────────────────┐
             ▼                 ▼                 ▼
            L0                L2 ... L3          L4 ... L7
```

X defines cross-layer contracts for identity, capability, policy, governance, cryptography, audit/evidence, observability, interoperability and versioning. Runtime enforcement remains with the responsible layer/domain.

The **Global Authority Matrix** MUST define, for every privileged action:

```text
Component / Principal
        ↓
Authority
        ↓
Allowed Action
        ↓
Required Capability
        ↓
Applicable Policy
        ↓
Validation / Enforcement Boundary
        ↓
Audit / Evidence
```

No component gains authority merely because it can reach another component, publish a message, or invoke an API.

## Primary Repository / Subsystem Mapping

A repository may span more than one architectural layer; the table identifies primary responsibility and does not override repository-level SSOT.

| Layer | Primary repositories / subsystems |
|---|---|
| L0 | Hardware/Secure Platform interfaces; secure-boot/TPM/TEE integrations |
| L1 | `atc-shivacore`, `globus-os`, node/runtime and IPC/capability subsystems |
| L2 | `a-townchain`, blockchain-core components, `atc-node` consensus/chain boundary |
| L3 | `a-townchain/components/vm`, VM execution and verifier components |
| L4 | `atc-node` P2P/network boundary, synchronization, RPC/API and propagation services |
| L5 | Account/signature, staking, rewards, slashing, treasury and supply-rule components |
| L6 | `atclang`, contract ABI/standards, `atc-contracts`, application protocols |
| L7 | `atc-wallet` UX, `atc-sdk`, `atc-explorer` UX, DApps and developer tooling |
| X | `atc-standards`, identity/capability/policy/governance/crypto/audit/evidence/observability/interoperability/versioning capabilities |

This mapping is architectural only. It does not assert implementation, test, CI or E2E status. Canonical repository paths remain governed by their repository-level SSOT contracts.

## Canonical Runtime / Data-Flow Separation

The responsibility-layer model MUST NOT be conflated with runtime ordering.

```text
L7 External Application
        │
        ▼
L6 Contract / Application Environment
        │
        ▼
L5 Economic & Security Services
        │
        ▼
L2 Transaction Validation / Mempool
        │
        ▼
L4 P2P Propagation / Synchronization
        │
        ▼
L2 Block Validation / Consensus
        │
        ▼
L3 ATC-VM Deterministic Execution
        │
        ▼
L2 State Transition / Ledger / Finality
        │
        ▼
L1 Runtime Persistence / State Storage
        │
        ▼
L0 Secure Platform
```

The actual implementation may pipeline, parallelize or reorder internal operations where protocol semantics permit. The canonical dependency and authority relationships MUST remain invariant.

### Determinism Contract

```text
same canonical input
        ↓
same encoding
        ↓
same validation
        ↓
same ordering
        ↓
same execution
        ↓
same state transition
        ↓
same state commitment
```

Determinism is therefore not an ATC-VM-only property. It spans canonical transaction encoding, validation, ordering, consensus inputs, state-transition semantics, VM execution, resource accounting, storage semantics and commitment calculation.

### Determinism and Authority Contract

The following rules are normative for the A-TownChain L1 architecture. They define the machine-verifiable relationship between architectural responsibility, authority, deterministic state transition, enforcement and evidence.

#### Normative determinism invariant

For a protocol/configuration version, identical canonical inputs MUST produce identical canonical outputs:

`F(State, Block) = State'`

The determinism contract is:

```text
same canonical input
        ↓
same canonical encoding
        ↓
same validation
        ↓
same ordering
        ↓
same execution
        ↓
same state transition
        ↓
same state commitment
```

A conforming implementation MUST NOT introduce unspecified ordering, platform-dependent numeric behavior, nondeterministic resource accounting, ambiguous serialization, hidden external state, or implementation-defined consensus semantics into the authoritative state transition.

#### Canonical input contract

Consensus-relevant values MUST have an explicitly defined canonical type, encoding and validation rule.

```text
Value
  ↓
Canonical Type
  ↓
Canonical Encoding
  ↓
Canonical Hash / Commitment / Signature Preimage
  ↓
Canonical Validation
```

Semantic equivalence MUST NOT be used as a substitute for canonical byte-level representation where the representation participates in authorization, hashing, ordering, consensus or state commitment.

#### Layer responsibility for determinism

| Layer | Normative deterministic responsibility | Forbidden authority |
|---|---|---|
| L0 | Provide the trusted hardware/security substrate required by the protocol | Defining blockchain state or finality |
| L1 | Provide deterministic protocol-critical runtime/IPC/capability enforcement | Defining consensus or canonical blockchain state |
| L2 | Define canonical transaction/block validation, ordering, state transition, ledger, consensus and finality | Delegating canonical state authority to transport/UI/AI |
| L3 | Execute consensus-relevant programs deterministically and account resources deterministically | Independent consensus or finality |
| L4 | Validate/transport protocol messages and synchronize claims according to protocol rules | Granting authority through network reachability |
| L5 | Apply deterministic account, signature and economic/security rules | Becoming a second consensus/finality layer |
| L6 | Define contract/application semantics through declared deterministic interfaces | Bypassing L2/L3 authority boundaries |
| L7 | Consume and present canonical state through declared interfaces | Mutating canonical state outside authorized protocol paths |
| X | Define cross-layer identity, capability, policy, governance, crypto, audit/evidence and versioning contracts | Becoming a second execution/state machine |

#### WHO / WHAT / WHERE enforcement rule

Every privileged action MUST be expressible as:

```text
WHO
  → Component / Principal / Identity

WHAT
  → Authority / Allowed Action / State Effect

WHERE
  → Validation + Enforcement Boundary
```

The machine-readable Global Authority Matrix MUST additionally bind:

```text
WHO
 → Authority
 → Allowed Action
 → Required Capability
 → Applicable Policy
 → Validation / Enforcement Boundary
 → Failure Semantics
 → Audit / Evidence
```

Network reachability, API access, message propagation, model output, UI authority or repository ownership MUST NOT create implicit protocol authority.

#### Canonical trust and validation chain

```text
Integrity
    ↓
Authenticity / Authorization
    ↓
Validity
    ↓
Ordering
    ↓
Consensus
    ↓
Deterministic Execution
    ↓
State Transition
    ↓
State Commitment
    ↓
Finality
    ↓
Canonical State
    ↓
Durable Persistence
```

Each stage is a separate contract boundary. Passing one stage MUST NOT be treated as proof that subsequent stages have passed.

#### P0 contract alignment

The following P0 contracts are normative architecture interfaces and are represented in the machine-readable control-plane contract registry:

| Contract | Primary boundary | Required invariant |
|---|---|---|
| P0-NET-001 Network / P2P | L4 ↔ L2 | Transport never creates authority; messages are untrusted until validated |
| P0-ID-001 Identity / Trust | X ↔ L1/L2/L4/L5 | Identity/authentication is explicit and bound to authorization context |
| P0-STATE-001 State Ownership | L2 ↔ L1/L3/L5/L7 | L2 owns canonical blockchain state and state-transition authority |
| P0-AUTH-001 Global Authority Matrix | X ↔ all layers | Every privileged action has explicit WHO/WHAT/WHERE |
| P0-IF-001 Canonical Interfaces | X ↔ L0–L7 | Interfaces have canonical types, encodings, ownership and versioning |
| P0-FR-001 Failure / Recovery | L1/L2/L4/X | Failures are explicit, fail-closed where authority/evidence is ambiguous, and recovery cannot silently rewrite canonical state |

These contracts are intentionally defined at the ecosystem boundary. Component repositories remain implementation SSOTs; this repository records the cross-system contract and evidence requirements.

#### Evidence trail contract

Architecture compliance MUST be distinguishable from implementation and verification evidence.

```text
Requirement
    ↓
Normative Contract
    ↓
Implementation / Enforcement
    ↓
Exact Source SHA
    ↓
Workflow Run ID
    ↓
Job ID
    ↓
Step ID
    ↓
Log / Artifact Digest
    ↓
Verification Result
```

Required evidence states are:

`UNANALYZED → ANALYZED → FIXED → RERUNNING → VERIFIED`

with `RESIDUAL` and `BLOCKED` preserved as explicit non-verified terminal/holding states.

A repository, file, test name, workflow existence, historical PR, or successful non-exact-SHA run MUST NOT by itself establish VERIFIED status.

Evidence MUST remain bound to the exact source SHA and MUST preserve enough metadata to reconstruct the chain from source to result. Missing, ambiguous or truncated evidence MUST fail closed as BLOCKED rather than being interpreted as PASS.

#### Contract ownership rule

`a-townchain-ecosystem` owns these cross-system architecture contracts and evidence requirements. It MUST NOT duplicate or silently replace implementation contracts owned by component repositories. Any component implementation claim MUST reference that component's exact Source-of-Truth and exact-SHA evidence.



## Canonical Ownership Rules

1. **Standalone First, Ecosystem Second:** each core repository owns its implementation SSOT, build, tests, release and API/ABI contracts.
2. `a-townchain-ecosystem` owns system-level boundaries, integration architecture and evidence aggregation; it does not replace component implementation SSOTs.
3. `atc-standards` remains the standards/governance SSOT.
4. Architecture text is not implementation evidence.
5. A file or directory existing is not evidence that a capability is implemented.
6. CI evidence is valid only when tied to the exact source commit under assessment.
7. Indexers and explorers are read/presentation models and are not canonical state authorities.
8. AI memory is not canonical blockchain state unless an explicit deterministic protocol contract makes a state representation authoritative.
9. L4 is a target architectural domain where components may be future/partial; its presence here is not an implementation claim.

## Architecture Status Semantics

Architecture statements must remain separate from implementation evidence:

```text
ARCHITECTURE CONTRACT
        │
        ├── Implementation: PRESENT / MISSING / BLOCKED
        ├── Tests: TESTED / NOT_TESTED
        ├── CI: CI_VERIFIED only for exact SHA
        ├── Integration: INTEGRATED / NOT_INTEGRATED
        └── E2E: E2E_VERIFIED / NOT_E2E_VERIFIED

These are independent evidence dimensions, not a required implementation sequence.
```

Exceptional states may include:

`MISSING` · `BLOCKED` · `DUPLICATE` · `DISCONNECTED`

The remainder of this document defines Quest AI, Dialogue AI and Aurora/GlobusOS architecture within the canonical layer model. Existing boundary contracts are retained below and must not be interpreted as implementation claims.

---

# Quest AI — Master Architecture

Quest AI ist ein eigenständiger, wiederverwendbarer Game-Intelligence-Dienst innerhalb der Genesis-Plattform. Sie ist **kein reiner Textgenerator**, sondern verarbeitet World State, Player State, NPC-/Faction-State, Lore, Economy, Events und Gameplay-Systeme zu validierten Quest-Instanzen.

## Positionierung

```
                         Aurora AI
                             │
                 Agent / Planning / Models
                             │
                             ▼
                    ┌─────────────────┐
                    │    QUEST AI     │
                    ├─────────────────┤
                    │ Generator       │
                    │ Planner         │
                    │ Director        │
                    │ Validator       │
                    │ Recommender     │
                    │ Runtime         │
                    │ Reward Engine   │
                    │ Security        │
                    └────────┬────────┘
                             │
          ┌──────────────────┼──────────────────┐
          ▼                  ▼                  ▼
       World AI           Player AI          Lore AI
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ▼
                     Genesis Engine
                             │
       ┌─────────────┬───────┼────────┬─────────────┐
       ▼             ▼       ▼        ▼             ▼
      NPCs        Creatures Items   Factions      Events
```

## Quest AI Subsysteme

### 1. Quest Generator

Erzeugt Quest-Definitionen aus kanonischen Templates und aktuellem World State.

Unterstützte Questklassen:

- Main Quest
- Side Quest
- Daily / Weekly / Repeatable Quest
- Tutorial Quest
- Faction / Guild / Companion Quest
- Character Quest
- World Quest
- Event / Season Quest
- Challenge / Achievement Quest
- Hunt / Rescue / Escort / Defense
- Investigation / Exploration / Procurement
- Sabotage / Espionage / Extraction
- Boss / Raid / Rift / Anomaly
- globale World Events

### 2. Quest Planner

Plant Objectives, Reihenfolge, Voraussetzungen, Branches und Abhängigkeiten.

```
Quest
 ├── Prerequisites
 ├── Objectives[]
 ├── Dependencies[]
 ├── Branches[]
 ├── Failure Paths[]
 └── Consequences[]
```

### 3. Quest Director

Überwacht die Welt und entscheidet anhand des World State, wann dynamische Quests und Events aktiviert werden.

```
World State
   ↓
Event Detection
   ↓
Threat / Opportunity Analysis
   ↓
Quest Decision
   ↓
Quest Spawn
   ↓
Player Interaction
   ↓
World Consequence
```

Der Director darf deterministische Gameplay-Zustände nicht direkt aus nichtdeterministischer AI-Inferenz verändern. AI erzeugt Vorschläge/Commands; die Engine validiert und führt diese deterministisch aus.

### 4. Quest Validator

Jede generierte Quest durchläuft:

```
Generated Quest
    ↓
Schema Validation
    ↓
Prerequisite Validation
    ↓
World Validation
    ↓
Lore / Canon Validation
    ↓
Economy Validation
    ↓
Security / Exploit Validation
    ↓
Runtime Quest
```

Fail-closed: ungültige Quest-Definitionen werden nicht aktiviert.

### 5. Quest Runtime

Standardisierte Zustandsmaschine:

```
CREATED
  → AVAILABLE
  → ACCEPTED
  → ACTIVE
  → OBJECTIVE_PROGRESS
  → OBJECTIVE_COMPLETED
  → TURN_IN
  → REWARD_PENDING
  → COMPLETED

Alternative:
ACTIVE → FAILED
ACTIVE → ABANDONED
AVAILABLE → EXPIRED
ANY VALID STATE → CANCELLED
```

### 6. Objective System

Objectives sind modular und kombinierbar:

```
KILL / COLLECT / DELIVER / ESCORT / PROTECT
RESCUE / EXPLORE / DISCOVER / SCAN / INTERACT
CRAFT / FUSE / BREED / CAPTURE / HUNT / SURVIVE
DEFEND / DESTROY / ACTIVATE / DEACTIVATE
SOLVE / INVESTIGATE / TRAVEL / REACH / ESCAPE
BOSS / RAID
```

### 7. Dynamic Difficulty

Die Quest-Difficulty wird aus dem aktuellen Kontext abgeleitet:

```
Player Level
+ Equipment / Build
+ Party Size
+ Player Performance
+ World Threat
+ Mission Type
+ Quest Importance
= Recommended Difficulty
```

Die Berechnung muss nachvollziehbar und server-/runtime-seitig validierbar sein.

### 8. Personalized Quest System

Quest-Empfehlungen können Player-State berücksichtigen:

- bevorzugte Gameplay-Loops
- Fortschritt
- Reputation
- Faction State
- abgeschlossene/fehlgeschlagene Quests
- Entdeckungen
- verfügbare Ausrüstung
- World State

Personalisierung verändert nicht den kanonischen Questvertrag ohne explizite Versionierung.

### 9. Faction & NPC Quest AI

NPCs und Fraktionen liefern:

```
Identity
Personality
Goals
Relationships
Knowledge
Secrets
Faction State
World Context
```

Daraus entstehen kontextabhängige Questketten, Fraktionskonflikte und langfristige Konsequenzen.

### 10. Reward Engine

Rewards werden anhand von Quest-Kontext und Economy-State validiert:

```
Difficulty
+ Duration
+ Risk
+ Rarity
+ Player Level
+ Economy State
+ Quest Importance
→ Reward Proposal
→ Economy Validation
→ Authorized Reward Command
→ Genesis / authoritative Economy Runtime
→ Committed Reward State
```

Unterstützte Reward-Klassen:

- XP
- In-game currencies
- Items
- Weapons
- Mods
- Blueprints
- Materials
- Creatures
- Cosmetics
- Titles / Badges
- Faction / Guild Reputation
- Genesis-specific assets

Blockchain-Assets dürfen ausschließlich über die vorgesehenen Chain-/VM-Schnittstellen ausgegeben werden.

### 11. Quest Memory

Quest AI führt keinen unkontrollierten eigenen Welt-Canon. Persistenter Zustand wird aus kanonischen Game-/Player-/World-State-Quellen bezogen.

Beispiele:

```
Completed Quests
Failed Quests
Important Decisions
Faction Reputation
NPC Relationships
World Changes
Unlocked Locations
```

### 12. Security & Anti-Exploit

Quest AI integriert:

- Quest farming detection
- reward-loop detection
- bot/automation signals
- multi-account abuse signals
- duplicate reward prevention
- objective integrity
- economy abuse detection
- provenance/evidence tracking

Quest Rewards sind niemals allein aufgrund einer LLM-Antwort gültig.

---


# Quest AI — Extended System Definition

## Autonomes Quest-System

Quest AI ist eine zentrale Game-Intelligence-Schicht für Game- und World-KI. Sie generiert nicht nur Questtexte, sondern verbindet:

```
Lore + World + Characters + Creatures + Items + Weapons
+ Levels + Economy + Events + Player State + Factions
        ↓
     QUEST AI
        ↓
Quest Generation + Planning + Direction + Runtime
```

Die Quest AI beantwortet systemisch:
- welche Quest entsteht
- wann sie entsteht
- für wen sie entsteht
- wo sie stattfindet
- welche NPCs beteiligt sind
- welche Gegner/Creatures erscheinen
- welche Items/Waffen/Assets benötigt werden
- welche Belohnung angemessen ist
- welche Konsequenzen entstehen
- ob die Quest Teil eines Quest Graphs bzw. einer größeren Storyline wird

## Quest Director — Reactive World

```
World State
    ↓
Event Detection
    ↓
Threat / Opportunity Analysis
    ↓
Quest Decision
    ↓
Quest Spawn
    ↓
Player Interaction
    ↓
World Consequence
```

Ereignisse können dadurch Folgequests wie Fraktionsnachfolge, Vergeltung, Machtverschiebungen oder Einflussquests erzeugen.

## Quest Graph

```
Quest A
 ├── Success → Quest B → Quest C
 └── Failure → Quest D → Quest E
```

Der Graph unterstützt verzweigte Storylines, alternative Enden, versteckte Quests, Failure Paths, Fraktionskriege und dynamische Kampagnen.

## Erweiterte Quest Runtime State Machine

```
CREATED
AVAILABLE
ACCEPTED
ACTIVE
OBJECTIVE_STARTED
OBJECTIVE_PROGRESS
OBJECTIVE_COMPLETED
TURN_IN
REWARD_PENDING
COMPLETED
FAILED
ABANDONED
EXPIRED
LOCKED
CANCELLED
```

LOCKED bedeutet: Die Quest-Definition existiert, ist aber durch Prerequisites, Dependencies, World State oder andere autorisierte Bedingungen nicht aktivierbar.

## Game-System Inputs

```
WORLD STATE
PLAYER STATE
LORE / CANON
NPC STATE
FACTION STATE
ECONOMY
CREATURE / SHIVAMON STATE
ITEM DATABASE
WEAPON DATABASE
LEVEL / PROGRESSION STATE
EVENT SYSTEM
```

Diese Inputs sind Kontext-/Read-Modelle. Autoritative Zustandsänderungen erfolgen ausschließlich über die validierte Runtime.

## Dynamic Difficulty

```
Player Level
+ Equipment
+ Shivamon / Creature Power
+ Player Skill
+ Party Size
+ Previous Performance
+ World Threat
+ Mission Type
+ Quest Importance
= Recommended Quest Difficulty
```

Die Runtime kann Recommended Level, Enemy Level, Boss Level, Threat Level, Estimated Duration und Recommended Party Size ableiten.

## Personalized Quest Intelligence

```
Player Profile
      ↓
Playstyle / Progress Analysis
      ↓
Quest Recommendation
```

Personalisierung kann PvE/PvP, Exploration, Lore, Crafting, Boss-, Faction- und Competitive-Spielweisen berücksichtigen, ohne den kanonischen Contract stillschweigend zu verändern.

## Lore / Canon Boundary

```
Lore Database
      ↓
Canon Validator
      ↓
Quest Generator
      ↓
Lore Validation
      ↓
Canon Validation
      ↓
World Validation
      ↓
Quest Approval
```

Quest AI darf keinen autoritativen Lore-Canon eigenmächtig ändern.

## NPC- und Faction-Intelligence

```
Identity
Personality
Faction
Relationships
Goals
Fear
Knowledge
Secrets
Current State
```

Fraktionen besitzen eigene Ziele, Reputation und Zustände. Questketten können NPC- und Faction-State verändern und Folgequests auslösen.

## Multiplayer / Global Quest Layer

Unterstützte Quest-Skalen:
- Solo Quest
- Party Quest
- Guild Quest
- Alliance Quest
- World Quest
- Server Event
- Global Event
- Raid Quest

Global Events können server- oder weltweite Fortschrittszustände besitzen. Der Fortschritt wird deterministisch aus validierten Gameplay-Ereignissen berechnet.

## Reward & Economy Control

```
Difficulty
+ Duration
+ Risk
+ Rarity
+ Player Level
+ Economy State
+ Quest Importance
        ↓
Reward Proposal
        ↓
Economy Validation
        ↓
Reward Execution
```

Mögliche Reward-Klassen umfassen XP, ATC-/Game-Currencies, Items, Weapons, Mods, Blueprints, Materials, DNA/Creature Assets, Shivamon, Cosmetics, Titles, Badges, Faction/Guild Reputation und Genesis-spezifische Assets.

Tokenisierte bzw. Blockchain-relevante Assets benötigen die vorgesehenen Chain-/VM-Schnittstellen und dürfen nicht direkt durch AI-Inferenz ausgegeben werden.

## Quest Security Intelligence

```
QUEST AI
 ├── Economy AI
 ├── Security AI
 ├── Anti-Cheat
 └── Blockchain Security
```

Zu prüfende Abuse-Klassen:
- Quest Farming
- Botting / Automation
- Multi-Account Farming
- Reward Loops
- XP Exploits
- ATC Farming
- Item Duplication
- Objective Manipulation
- Duplicate Rewards

## Quest Memory

```
Completed Quests
Failed Quests
Important Decisions
Faction Reputation
NPC Relationships
World Changes
Unlocked Locations
```

Memory ist kein separater autoritativer World-Canon. Die Quelle bleibt der jeweilige kanonische Game-/World-/Player-State.

## Quest AI Service Boundary

```
quest-ai/
├── core/
│   ├── quest_generator
│   ├── quest_director
│   ├── quest_planner
│   ├── quest_validator
│   └── quest_runtime
├── generation/
│   ├── templates
│   ├── objectives
│   ├── narratives
│   ├── rewards
│   └── branching
├── intelligence/
│   ├── player_model
│   ├── world_model
│   ├── faction_model
│   ├── difficulty_ai
│   └── recommendation_ai
├── integration/
│   ├── world
│   ├── lore
│   ├── character
│   ├── creature
│   ├── item
│   ├── weapon
│   ├── level
│   ├── economy
│   └── blockchain
├── security/
│   ├── anti_exploit
│   ├── anti_bot
│   ├── reward_validation
│   └── quest_integrity
└── api/
    ├── quest_api
    ├── runtime_api
    ├── generation_api
    └── event_api
```

Diese Struktur ist ein logischer Architekturvertrag, keine Behauptung, dass jedes Modul bereits implementiert ist.

## Zieldefinition

```
QUEST AI
= Quest Generation
+ Quest Planning
+ Quest Director
+ Dynamic World Events
+ Narrative Branching
+ Player Personalization
+ Runtime Execution
+ Reward / Economy Control
+ Security / Integrity
```

Quest AI ist damit eine zentrale Game-Intelligence-Schicht der Genesis-/Shivamon-Welt und nicht lediglich ein Textgenerator.

---

# Quest Data Contract — Canonical

Der folgende Contract ist der **kanonische Quest-Datenvertrag auf Systemebene**. Feldnamen und semantische Gruppen sind Bestandteil des Integrationsvertrags; die konkrete Runtime-Repräsentation und Programmiersprache bleiben Aufgabe der Genesis-Engine-Implementierung.

```
Quest
├── ID
├── Version
├── Type
├── Category
├── Title
├── Description
├── Lore
├── Prerequisites
├── Objectives
├── NPCs
├── Locations
├── Enemies
├── Boss
├── Difficulty
├── LevelRange
├── TimeLimit
├── Rewards
├── Reputation
├── Consequences
├── Branches
├── Dependencies
├── State
├── AI Metadata
└── Security Metadata
```

### Contract-Semantik

| Feld | Systemische Bedeutung |
|---|---|
| **ID** | Eindeutige Quest-Identität |
| **Version** | Versionierter Contract / kompatible Quest-Definition |
| **Type** | Quest-Klasse bzw. Lifecycle-/Gameplay-Typ |
| **Category** | Fachliche Gameplay-/Narrativ-Kategorie |
| **Title** | Spieler-facing Questtitel |
| **Description** | Spieler-facing Questbeschreibung |
| **Lore** | Kanonischer narrativer Kontext und Lore-Referenzen |
| **Prerequisites** | Bedingungen für Verfügbarkeit/Aktivierung |
| **Objectives** | Deterministisch prüfbare Questziele |
| **NPCs** | Referenzen auf beteiligte NPCs |
| **Locations** | Referenzen auf relevante Weltorte |
| **Enemies** | Gegner-/Encounter-Referenzen |
| **Boss** | Optionaler Boss-/Boss-Encounter-Referenz |
| **Difficulty** | Basis- und/oder dynamische Schwierigkeit |
| **LevelRange** | Zulässiger bzw. empfohlener Spielerlevelbereich |
| **TimeLimit** | Zeitfenster und Ablaufbedingungen |
| **Rewards** | Validierte Belohnungsvorschläge/-definitionen |
| **Reputation** | Fraktions-/Guild-/sonstige Reputationsänderungen |
| **Consequences** | Welt-, NPC-, Faction- und Folgequest-Konsequenzen |
| **Branches** | Verzweigungen, Bedingungen und Outcomes |
| **Dependencies** | Abhängigkeiten von Quests, Events, Systemen oder World State |
| **State** | Deterministischer Quest-Lifecycle |
| **AI Metadata** | Herkunft, Generierung, Planung, Personalisierung und Validierungsmetadaten |
| **Security Metadata** | Provenance, Integritäts-, Anti-Exploit- und Auditdaten |

### AI Metadata

AI Metadata beschreibt die Herkunft und Verarbeitung einer Quest, nicht ihren autoritativen Spielzustand.

Mögliche Felder:

```
AI Metadata
├── GeneratedBy
├── Model
├── PromptVersion
├── GenerationSeed
├── Confidence
├── ValidationStatus
├── Personalization
├── DifficultyAdjustment
└── AIProvenance
```

### Security Metadata

Security Metadata unterstützt Integrität, Nachvollziehbarkeit und Abuse Prevention:

```
Security Metadata
├── QuestHash
├── SchemaVersion
├── Provenance
├── ValidationProof
├── RewardIntegrity
├── AntiExploit
├── DuplicateDetection
├── FarmingDetection
└── AuditTrail
```

AI Metadata und Security Metadata dürfen keine Umgehung der deterministischen Quest-Runtime ermöglichen.

---

# Deterministic AI Boundary

Für alle Quest-AI-Pfade gilt:

```
AI / Model Inference
        ↓
Quest Proposal / Command
        ↓
Schema + Policy Validation
        ↓
Genesis Engine Runtime
        ↓
Deterministic Simulation
        ↓
Validated State Change
```

AI darf insbesondere nicht:

- direkt ECS-State mutieren
- Konsens-State verändern
- ATC-VM-State umgehen
- Rewards ohne Runtime-Validierung vergeben
- Lore-Canon ohne autorisierten Change verändern

---

# Repository Ownership

| Capability | Canonical Owner |
|---|---|
| Quest AI architecture / canonical Quest Data Contract | a-townchain-ecosystem |
| Quest runtime / gameplay integration | genesis-engine |
| AI models / agents / planning / tools | aurora-ai |
| World / ECS / simulation | genesis-engine |
| On-chain state / contracts | a-townchain / ATC-VM / ATCLang |
| Kernel execution / TCB | atc-shivacore |
| Standards / governance | atc-standards |
| Documentation / architecture knowledge base | a-townchain-os-docs |

## SSOT-Regel

Die Master Architecture definiert **Systemgrenzen, Verantwortlichkeiten und Integrationsverträge**. Fachliche Implementierungen bleiben in den jeweiligen kanonischen Source-Repositories.

---

# Evidence & Governance

Quest AI gilt erst als implementiert, wenn entsprechende Evidence vorliegt:

- Quellcode
- Unit-/Integration-Tests
- deterministische Runtime-Tests
- Schema-/Contract-Tests
- Security-/Exploit-Tests
- Economy-Tests
- CI Evidence
- Provenance / Audit Trail

Deklarierte Architektur, geplante Komponenten und tatsächlich implementierte Features sind strikt getrennt zu dokumentieren.

## Integration Target

Der vollständige Game-Intelligence-Pfad ist:

```
Player / World Event
        ↓
World State
        ↓
Aurora AI Context
        ↓
Quest AI
  ├─ Generate
  ├─ Plan
  ├─ Validate
  └─ Direct
        ↓
Genesis Engine
        ↓
Deterministic Runtime
        ↓
World / Player State
        ↓
Genesis Authoritative Runtime
        ↓
Validated Rewards / Reputation / Consequences
        ↓
Committed Game State
        ↓
Persistent Game State
```


---

# Dialogue AI — Master Architecture

Dialogue AI ist eine eigenständige, wiederverwendbare **Conversational Intelligence Layer** der Genesis-/Shivamon-Plattform. Sie ist kein reiner Chatbot und kein unkontrollierter LLM-Ausgabekanal. Sie verbindet Character State, Conversation Context, NPC Memory, Quest State, World State, Lore/Canon, Factions, Reputation, Emotion und autorisierte Gameplay-Actions zu validierten Dialog-Interaktionen.

## Positionierung

```
                         Aurora AI
                             │
                 Models / Agents / Planning
                             │
                             ▼
                 ┌─────────────────────────┐
                 │       DIALOGUE AI       │
                 ├─────────────────────────┤
                 │ Dialogue Core           │
                 │ Context Engine          │
                 │ Character Intelligence  │
                 │ Memory                  │
                 │ Emotion Engine          │
                 │ Dialogue Planner        │
                 │ Generator               │
                 │ Lore / Canon Grounding  │
                 │ Validator               │
                 │ Action Interface        │
                 │ Voice / TTS             │
                 │ Security / Audit        │
                 └────────────┬────────────┘
                              │
          ┌───────────────────┼───────────────────┐
          ▼                   ▼                   ▼
       Quest AI            World AI           Lore AI
          │                   │                   │
          └───────────────────┼───────────────────┘
                              ▼
                       Genesis Engine
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
             NPCs          Factions          Events
                              │
                              ▼
                           Player
```

## Dialogue AI Subsysteme

### 1. Dialogue Core

Verantwortlich für:

- Intent Detection
- Response Generation
- Context Management
- Conversation State
- Dialogue Planning
- Session Lifecycle

Die Dialogue Core darf keinen autoritativen Game State direkt mutieren.

### 2. Character Intelligence

Jeder NPC kann ein versioniertes Character Profile besitzen:

```
Character
├── Identity
├── Personality
├── Motivation
├── Knowledge
├── Relationships
├── Emotional State
├── Memory Policy
└── Dialogue Policy
```

Beispiel:

```yaml
character:
  id: NPC-0001
  name: "Arkon"
  personality:
    traits: [loyal, suspicious, ambitious]
  motivation:
    primary: "protect_the_city"
    secondary: "discover_the_ur_archive"
  knowledge:
    world: true
    local: true
    forbidden_lore: false
  relationships:
    player: 35
    faction_guard: 80
    demon_faction: -90
  emotional_state:
    emotion: cautious
    intensity: 0.72
  memory:
    enabled: true
```

Das Profil definiert Kontext und Constraints; es ist nicht selbst der autoritative World State.

### 3. Context Engine

Vor jeder Antwort wird ein deterministisch zusammengestellter Context aus autorisierten Read-Modellen gebildet:

```
Player
  ↓
Conversation
  ↓
NPC Memory
  ↓
Quest State
  ↓
World State
  ↓
Lore / Canon
  ↓
Player Reputation
  ↓
Faction State
  ↓
Current Location
  ↓
Time / Event
  ↓
Dialogue Planner
  ↓
Response Proposal
```

Context muss versionierbar, nachvollziehbar und auditierbar sein.

### 4. Dialogue State Machine

Dialog-Lifecycle und erlaubte Übergänge werden durch Runtime-/Gameplay-Regeln begrenzt:

```
START
 ↓
GREETING
 ├── FRIENDLY → INFORMATION
 ├── HOSTILE  → THREAT
 └── UNKNOWN  → INVESTIGATION
                  ↓
               DECISION
              /        \
         ACCEPT        REFUSE
            ↓             ↓
          QUEST          EXIT
```

LLM-Inferenz erzeugt keine frei definierbaren State Transitions. Die Runtime entscheidet, welche Übergänge zulässig sind.

### 5. Dialogue Actions

Dialogue AI kann strukturierte Action-Proposals erzeugen:

```json
{
  "speech": "Ich kann dir helfen, aber zuerst musst du mir etwas beweisen.",
  "emotion": "suspicious",
  "intent": "quest_offer",
  "actions": [
    {
      "type": "offer_quest",
      "quest_id": "QUEST-1042"
    }
  ]
}
```

Unterstützte Action-Klassen können umfassen:

```
SAY
ASK
ANSWER
OFFER_QUEST
ACCEPT_QUEST
DECLINE_QUEST
START_TRADE
GIVE_ITEM
REQUEST_ITEM
CHANGE_REPUTATION
CHANGE_RELATIONSHIP
TRIGGER_EVENT
START_COMBAT
END_DIALOGUE
UNLOCK_LOCATION
REVEAL_LORE
```

Actions sind **Vorschläge/Commands**, keine unmittelbaren autoritativen Zustandsänderungen.

### 6. Memory System

Memory wird in drei logische Klassen getrennt:

```
Short-Term Memory
├── Conversation ID
├── Messages
├── Topics
├── Current Intent
└── Current Emotion

Episodic Memory
├── Important Decisions
├── Player Interactions
├── Quest Outcomes
├── Betrayals / Alliances
└── Relevant World Events

Semantic Memory
├── Character Knowledge
├── Local Knowledge
├── Faction Knowledge
├── Lore References
└── Player Reputation
```

Memory besitzt eine Importance-/Retention-Policy. Nicht jede Äußerung wird persistent gespeichert.

Persistente Memory darf keinen autoritativen Canon außerhalb der vorgesehenen Quellen erzeugen.

### 7. Emotion Engine

Emotionen sind strukturierte Zustandsdaten:

```
Emotion
+ Intensity
+ Duration / Decay
+ Trigger
```

Beispiel:

```yaml
emotion:
  type: anger
  intensity: 0.86
  trigger: player_betrayal
  decay: 0.04
```

Emotion kann Text, Voice, Animation und NPC-Reaktionsparameter beeinflussen, darf aber keine nicht autorisierte Gameplay-Mutation auslösen.

### 8. Quest AI Integration

Dialogue AI und Quest AI bilden eine gekoppelte Intelligence-Schicht:

```
QUEST AI
    │
    ├── Quest Generation
    ├── Quest State
    ├── Objectives
    └── Rewards
          │
          ▼
    DIALOGUE AI
          │
          ├── Quest Introduction
          ├── Quest Discussion
          ├── Dynamic Hints
          ├── Negotiation
          └── Quest Completion
```

Quest AI bleibt für Quest-Verträge und Quest-Runtime verantwortlich. Dialogue AI stellt die konversationelle Interaktionsschicht bereit.

### 9. Lore / Canon Integration

Dialogue AI darf keine beliebigen Lore-Fakten als Canon etablieren:

```
Player Question
      ↓
Lore Database / Knowledge Graph
      ↓
RAG / Retrieval
      ↓
Canon Validation
      ↓
Dialogue Generation
      ↓
Response Validation
      ↓
Dialogue Response
```

Dialogue AI darf den autoritativen Lore-Canon nur über einen autorisierten Change-Prozess verändern.

### 10. World / Gameplay Integration

Dialogue AI kann Kontext aus folgenden Domänen konsumieren:

```
World State
Quest State
NPC State
Faction State
Player State
Reputation
Items
Weapons
Trading
Crafting
Combat
Events
Locations
Timeline
```

Autoritative Mutationen erfolgen ausschließlich über Genesis Engine Runtime bzw. die jeweils zuständige kanonische Systemgrenze.

### 11. Voice Dialogue

Voice ist eine nachgelagerte Darstellungsschicht:

```
Dialogue Response
      ↓
Emotion
      ↓
Voice Profile
      ↓
TTS
      ↓
Audio
      ↓
Lip Sync
      ↓
NPC Animation
```

Voice/TTS ist austauschbar und darf die deterministische Gameplay-Runtime nicht ersetzen.

### 12. Multilingual Dialogue

Die semantische Dialogue Representation bleibt sprachneutral:

```
Player Language
      ↓
Intent / Context
      ↓
Language-Neutral Dialogue Representation
      ↓
Response Planning
      ↓
Target Language
      ↓
TTS
```

Zielsprachen können u. a. Deutsch, Englisch, Französisch, Spanisch, Italienisch, Portugiesisch, Japanisch, Koreanisch und Chinesisch umfassen. Sprachunterstützung ist ein Capability-Ziel; konkrete Sprachabdeckung muss durch Evidence nachgewiesen werden.

### 13. Dialogue Validation

Jede generierte Response bzw. Action Proposal wird geprüft:

```
Dialogue Proposal
      ↓
Schema Validation
      ↓
Character Consistency
      ↓
Lore / Canon Validation
      ↓
Quest Consistency
      ↓
World-State Validation
      ↓
Content / Policy Validation
      ↓
Action Authorization
      ↓
Runtime
```

Zusätzliche Qualitätsprüfungen:

- Repetition Detection
- Anachronism Detection
- Canon Conflict Detection
- Personality Drift Detection
- Quest Logic Validation
- Prompt / Injection Resistance
- Provenance Tracking

Fail-closed für ungültige oder nicht autorisierte Actions.

## Deterministic Dialogue Boundary

Für alle produktiven Dialogue-AI-Pfade gilt:

```
AI / Model Inference
        ↓
Dialogue Proposal
        ↓
Schema + Policy Validation
        ↓
Authorized Action / Response
        ↓
Genesis Engine Runtime
        ↓
Deterministic State Change
        ↓
Persistent Game State
```

Dialogue AI darf insbesondere nicht:

- direkt ECS-State mutieren
- Quest-State außerhalb der Quest-Runtime verändern
- Economy-Rewards selbst vergeben
- Blockchain-/ATC-State direkt verändern
- Lore-Canon ohne autorisierten Change verändern
- Sicherheits- oder Policy-Gates umgehen

## Dialogue AI Service Boundary

Logische Architektur:

```
dialogue-ai/
├── core/
│   ├── dialogue_core
│   ├── context_engine
│   ├── dialogue_planner
│   ├── dialogue_runtime
│   └── dialogue_state
├── intelligence/
│   ├── character
│   ├── personality
│   ├── motivation
│   ├── relationship
│   ├── emotion
│   └── memory
├── generation/
│   ├── response
│   ├── branching
│   ├── multilingual
│   └── templates
├── integration/
│   ├── quest
│   ├── world
│   ├── lore
│   ├── faction
│   ├── item
│   ├── combat
│   ├── economy
│   └── events
├── voice/
│   ├── tts
│   ├── voice_profiles
│   ├── emotion
│   └── lipsync
├── validation/
│   ├── schema
│   ├── canon
│   ├── character
│   ├── world_state
│   └── policy
├── security/
│   ├── prompt_security
│   ├── action_authorization
│   ├── provenance
│   └── audit
├── eval/
│   ├── dialogue_quality
│   ├── consistency
│   ├── regression
│   └── determinism_boundary
└── api/
    ├── session_api
    ├── message_api
    ├── response_api
    ├── action_api
    ├── memory_api
    └── validation_api
```

Diese Struktur ist ein logischer Architekturvertrag und keine Behauptung, dass die genannten Module bereits implementiert sind.

## Dialogue Runtime API Contract

Logische Schnittstellen:

```
POST /dialogue/session
POST /dialogue/message
POST /dialogue/response
POST /dialogue/action
GET  /dialogue/state
GET  /dialogue/memory
POST /dialogue/memory
POST /dialogue/generate
POST /dialogue/validate
```

Die konkrete Transporttechnologie und Runtime-Implementierung bleiben Aufgabe der zuständigen Implementierungs-Repositories.

## Repository Ownership

| Capability | Canonical Owner |
|---|---|
| Dialogue AI architecture / canonical Dialogue Data Contract | a-townchain-ecosystem |
| Dialogue runtime / NPC & gameplay integration | genesis-engine |
| Models / agents / planning / inference orchestration | aurora-ai |
| Character / NPC / world state | genesis-engine |
| Quest contracts / Quest runtime | `genesis-engine` |
| Lore / canon source | zuständiges kanonisches Lore-/Content-System |
| Voice / TTS adapters | Genesis Engine / AI capability implementation |
| Standards / governance | atc-standards |
| Documentation / architecture knowledge base | a-townchain-os-docs |

## SSOT-Regel

Die Master Architecture definiert **Systemgrenzen, Verantwortlichkeiten, Datenverträge und Integrationsregeln**. Implementierungen bleiben in den jeweiligen kanonischen Source-Repositories.

## Evidence & Governance

Dialogue AI gilt erst als implementiert, wenn entsprechende Evidence vorliegt:

- Quellcode
- Unit-/Integration-Tests
- Schema-/Contract-Tests
- Character-Consistency-Tests
- Lore-/Canon-Tests
- Action Authorization Tests
- Memory Tests
- Security-/Prompt-Injection-Tests
- Runtime-/Integration-Tests
- CI Evidence
- Provenance / Audit Trail

Architektur, geplante Komponenten und tatsächlich implementierte Features sind strikt getrennt zu dokumentieren.

## Integration Target

Der vollständige Game-Intelligence- und Dialogue-Pfad ist:

```
Player / World Event
        ↓
World State
        ↓
Aurora AI Context
        ↓
┌─────────────────────────────┐
│ Dialogue AI                  │
│  Context → Plan → Generate  │
│  → Validate → Action         │
└──────────────┬──────────────┘
               │
        ┌──────┴──────┐
        ▼             ▼
     Quest AI      Lore / World AI
        │             │
        └──────┬──────┘
               ▼
        Genesis Engine
               ↓
       Deterministic Runtime
               ↓
    World / Player / NPC State
               ↓
 Rewards / Reputation / Consequences
               ↓
       Persistent Game State
```

Dialogue AI ist damit eine eigenständige Intelligence Capability innerhalb der Master Architecture und kein Ersatz für Genesis Runtime, Quest Runtime, Lore SSOT oder autoritative Game-State-Systeme.


---

# Aurora AI — Canonical GlobusOS Control / Intelligence Plane

Aurora AI is the **AI Control / Intelligence Plane of GlobusOS**. It is not the operating-system kernel and does not own blockchain consensus authority.

## Canonical OS Boundary

```text
AURORA AI
    │
    │ Intelligence / Planning / Agent Execution
    ▼
GLOBUS OS
    │
    │ OS Services / IPC / Resource Management
    ▼
SHIVACORE
    │
    │ Kernel / TCB / Capabilities / Isolation
    ▼
HARDWARE
```

- **Aurora:** models, agents, planning, skills, tools, memory/RAG, multimodal processing, domain intelligence, automation and evidence.
- **GlobusOS:** operating-system services, IPC, resource/device integration, userspace and OS-level policy boundaries.
- **ShivaCore:** trusted kernel/TCB, isolation, capabilities, scheduling, memory and hardware boundary.

These are architectural responsibilities, not implementation claims. Actual Aurora capability status is determined by **AURORA-001 — Platform Completeness & Authority Gate** in the `aurora-ai` repository.

## Canonical Authority Contract

```text
MODEL
  │ proposes
  ▼
AGENT
  │ requests
  ▼
CAPABILITY
  │ checked by
  ▼
POLICY
  │ may require
  ▼
APPROVAL
  │ authorizes
  ▼
TOOL
  │ executes
  ▼
GLOBUS OS
  │ capability + IPC
  ▼
SHIVACORE
```

The following are architectural invariants:

- AI models have no implicit execution authority.
- Agents cannot bypass capability checks.
- Capabilities are subject to policy evaluation.
- Sensitive operations may require explicit approval.
- Tools execute only through authorized interfaces.
- Aurora has no direct kernel authority.
- Aurora has no direct blockchain consensus authority.
- Privileged execution must be auditable.

## A-TownChain Integration Boundary

```text
AURORA
   │ request
   ▼
A-TownChain Interface
   ▼
Node
   ▼
Consensus
   ▼
ATC-VM
   ▼
State
```

Aurora may propose, plan and request blockchain operations. Consensus, VM execution and authoritative state transitions remain under the deterministic A-TownChain components.

## Aurora Platform Planes

The canonical Aurora target architecture consists of:

1. **Model Plane** — ModelHub, provider adapters, registry, versioning, model policy.
2. **Runtime Plane** — scheduler, context engine, agent runtime, planner, execution, event bridge.
3. **Agent Plane** — registry, lifecycle, roles, planning, skills, multi-agent coordination.
4. **Authority Plane** — capabilities, policy engine, approval, human gate, authorization.
5. **Tool Plane** — tool registry, adapters, execution and sandboxing.
6. **Memory / Knowledge Plane** — working, episodic, semantic and federated memory, RAG, retrieval, knowledge graph and provenance.
7. **Multimodal Plane** — text, speech, audio, vision, image, video and documents.
8. **Domain Intelligence** — Dialogue, Quest, World, Character, Creature, Faction and Security AI.
9. **Governance / Evidence** — audit, provenance, telemetry, evaluation and evidence.
10. **Integration Plane** — GlobusOS IPC, Genesis interface, A-TownChain interface and external services.

The plane list defines the canonical target architecture. It does not assert that every plane or component is already implemented.

## AURORA-001 Evidence Rule

Aurora component evidence uses the same independent dimensions as the master architecture:

```text
ARCHITECTURE CONTRACT
        │
        ├── Implementation: PRESENT / MISSING / BLOCKED
        ├── Tests: TESTED / NOT_TESTED
        ├── CI: CI_VERIFIED only for exact SHA
        ├── Integration: INTEGRATED / NOT_INTEGRATED
        └── E2E: E2E_VERIFIED / NOT_E2E_VERIFIED
```

Exceptional states are `MISSING`, `BLOCKED`, `DUPLICATE` and `DISCONNECTED`.

These are evidence dimensions, not a required implementation sequence. A file, directory or architecture statement is not implementation evidence. CI evidence is valid only when tied to the exact source commit under assessment.

## Repository Ownership

| Capability | Canonical Owner |
|---|---|
| Aurora AI implementation | `aurora-ai` |
| Aurora platform architecture / AURORA-001 evidence | `aurora-ai` |
| Ecosystem system boundaries and integration contract | `a-townchain-ecosystem` |
| GlobusOS | `globus-os` |
| Kernel / TCB / capabilities | `atc-shivacore` |
| Genesis runtime | `genesis-engine` |
| Blockchain / VM | A-TownChain canonical L1 / VM repositories |
| Standards / governance | `atc-standards` |

The ecosystem repository remains the integration/master-architecture SSOT; component implementations remain in their canonical repositories.


---

## Architecture Consistency Gate — ARCH-001

The following invariants are normative and resolve cross-section ambiguity:

### A. Authority vs. persistence
- L2 owns consensus/finality authority.
- L3 computes deterministic state transitions under L2 protocol authority.
- L1 persists committed state and serves history/read models; storage cannot finalize protocol state.
- Indexers and explorers are derived/read models and cannot become canonical state authorities.

### B. Execution vs. finality
- VM execution may produce a candidate transition before consensus finality.
- Only a finalized/committed transition becomes canonical protocol state.
- Durable persistence follows the commit contract; recovery must validate the committed state before serving it as canonical.

### C. Aurora / AI authority
- Aurora, agents and models are never implicit super-authorities.
- AI output is a proposal/request unless a target domain's deterministic authorization contract accepts it.
- Aurora does not replace Chain, Genesis, GlobusOS or ShivaCore authority.

### D. ShivaCore / GlobusOS / ATC-VM boundary
- ShivaCore is the kernel/TCB boundary.
- GlobusOS is the OS/service boundary above ShivaCore.
- ATC-VM is the deterministic contract-execution boundary and is not the kernel.
- Hardware-backed security claims require separate hardware evidence.

### E. SSOT and evidence
- `atc-standards` is normative standards/governance SSOT.
- `a-townchain-ecosystem` owns system architecture, boundaries and integration evidence aggregation.
- Component repositories own implementation, tests, release and API/ABI SSOT.
- Architecture text, repository/file existence, PR state and historical CI runs are not current implementation evidence.
- CI verification is valid only for the exact source SHA under assessment.

### F. State domains
Every state domain must have exactly one authoritative owner for writes/commit semantics. Read replicas, caches, indexes, memories and projections may consume state but cannot silently become authoritative.

### G. Interface and event discipline
All cross-domain writes must cross a declared, versioned interface with authentication/authorization, schema validation, replay/idempotency rules and audit semantics. Events are transport/integration artifacts unless their target domain explicitly defines them as authoritative inputs.

This gate is an architecture consistency contract. It does not claim that the listed contracts are implemented or CI/E2E verified.

---

# ARCH-002 — Detailed System Architecture Contract

This section expands the master architecture into explicit layers, planes, domains, authority boundaries, lifecycle stages, interfaces, state ownership, failure handling, security, observability, deployment and evidence rules.

It is a normative target-architecture and boundary contract. It does not imply that every described component is implemented.

## 1. Architecture Model

The system is organized across four orthogonal dimensions:

1. Layers — architectural responsibility and dependency direction.
2. Planes — cross-cutting capabilities.
3. Domains — authoritative ownership boundaries.
4. Evidence — independent proof dimensions.

These dimensions MUST NOT be conflated.

~~~text
SYSTEM ARCHITECTURE
        |
  +-----+------+----------------+
  |            |                |
LAYERS       PLANES           DOMAINS
  |            |                |
responsibility cross-cutting   state/authority
  +------------+----------------+
               |
       INTERFACE CONTRACTS
               |
         EVIDENCE CONTRACT
~~~

## 2. Canonical Layer Contract

The canonical L0–L7 + X model is the single responsibility/trust-boundary model for the ecosystem.

### L0 — Hardware / Secure Platform
Owns CPU, RAM, storage, TPM, TEE, secure boot and hardware security primitives. L0 provides the trusted substrate and does not own blockchain state.

### L1 — Node & Runtime Foundation
Owns OS/runtime integration, ShivaCore, IPC and capability security. L1 provides the execution/node foundation but does not define blockchain consensus or finality.

### L2 — Blockchain Core
Owns block, transaction and state models, ledger, mempool, consensus, finality and chain validation. L2 is the canonical blockchain authority.

### L3 — ATC-VM
Owns program model, typed values, deterministic execution, gas/resource accounting, VM storage and VM validation. L3 executes under L2 protocol authority and does not independently finalize state.

### L4 — Protocol Services
Owns P2P/networking, synchronization, peer discovery, RPC/API and block/state propagation. L4 transports and synchronizes claims; it does not make claims canonical.

### L5 — Economic & Security Services
Owns accounts, signatures, staking, rewards, slashing, treasury and supply rules. Economic/security state must have explicit ownership and numeric/unit contracts.

### L6 — Smart-Contract / Application Environment
Owns ATCLang, contract ABI, contract standards and application protocols. Consensus-critical contract behavior MUST execute through deterministic L3 interfaces and L2 authority.

### L7 — External Applications
Owns wallet, SDK, explorer, DApps and developer tooling. L7 MUST NOT bypass lower-layer validation, authorization or finality boundaries.

### X — Cross-Layer Control Plane
Owns cross-layer identity, capability, policy, governance, cryptography, audit/evidence, observability, interoperability and versioning contracts. X constrains and records authority; it is not a second execution layer.

### L2/L3 and P2P separation
`L2 Blockchain Core` and `L3 ATC-VM` are distinct trust boundaries. P2P is a protocol-service responsibility of L4. Network delivery never substitutes for L2 validation or consensus.

## 3. Cross-Layer X Plane

X is a cross-cutting control plane, not an execution layer.

~~~text
Identity
  |
Authentication
  |
Authorization
  |
Capability
  |
Policy
  |
Approval
  |
Audit / Evidence
~~~

X may constrain or authorize operations in L1-L7 but MUST NOT silently become owner of another domain's canonical state.

## 4. Authority Model

Authority follows canonical state ownership, not UI position, model capability or orchestration centrality.

~~~text
Human / Governance
        |
Policy / Approval
        |
Target Domain Authority
   +----+-----+-------+-------+
   |          |       |       |
 Chain       VM    Genesis    OS
 consensus execution runtime services
   |          |       |       |
 finality  transition state   policy
~~~

Aurora MAY propose and orchestrate actions but MUST NOT become an implicit global authority.

## 5. Canonical Blockchain Lifecycle

~~~text
Construct Transaction
        |
Canonical Encoding
        |
Signature / Authorization
        |
Admission / Validation
        |
Mempool
        |
Block Proposal
        |
Block Validation
        |
ATC-VM Candidate Execution
        |
Execution Receipt
        |
Consensus / Validator Voting
        |
Finality / Commit
        |
Canonical State Update
        |
Durable Persistence
        |
Indexing / Read Models
        |
External Observation
~~~

A mempool transaction is not final. A VM result is not final. A persisted candidate is not canonical protocol state unless the commit contract establishes it as such.

## 6. Canonical Block Lifecycle

~~~text
RECEIVED
   |
VALIDATING
   |
VALID
   |
PROPOSED
   |
VOTING
   |
FINALIZED
   |
COMMITTED
   |
PERSISTED
   |
INDEXED
~~~

Rejected or failed blocks MUST NOT transition to FINALIZED.

## 7. State Machine Contract

Every authoritative state domain MUST define:

- state identifier;
- owner;
- version;
- schema;
- allowed transitions;
- transition authority;
- invariants;
- persistence boundary;
- recovery rule;
- migration rule;
- event contract;
- audit requirements.

Canonical transition:

~~~text
Previous State
      |
Authorized Input
      |
Validation
      |
Deterministic Transition
      |
Invariant Check
      |
Commit Decision
      |
New Canonical State
~~~

No component may mutate another domain's canonical state through an undocumented side channel.

## 8. State Ownership

| State | Authoritative owner | Consumers |
|---|---|---|
| Consensus state | Chain / L2 | nodes, indexers, explorers |
| Contract execution state | protocol-defined chain/VM state owner | nodes, contracts |
| Persistent chain data | L1 storage | nodes, indexers |
| Economic protocol state | designated L5/chain domain | chain, wallets, applications |
| Wallet local state | wallet | user applications |
| Genesis world state | Genesis Engine | Aurora, clients, tools |
| Quest state | Genesis/Quest runtime | Quest AI, UI |
| Dialogue context | Dialogue subsystem | Aurora/agents |
| AI working memory | Aurora/runtime | agents |
| OS service state | GlobusOS | services/applications |
| Kernel state | ShivaCore | GlobusOS |
| Index state | indexer | explorer/API |
| Cache state | owning service | consumers |

Derived state MUST remain subordinate to its authoritative source.

## 9. Event Architecture

Events are not automatically authoritative.

~~~text
Authoritative State Transition
          |
        Commit
          |
    Canonical Event
          |
       Transport
          |
       Consumers
          |
 Derived State / Actions
~~~

Cross-domain events SHOULD define event ID, type, schema version, source domain, source entity, source state version, sequence information, correlation ID, causation ID, authorization context and integrity metadata.

Consumers MUST implement the applicable replay, deduplication and idempotency contract.

## 10. Command vs Event

~~~text
COMMAND = requested state transition
        |
 Authorization
        |
 Validation
        |
 Execution
        |
 Commit
        |
EVENT = accepted transition occurred
~~~

A command is intent. An event is evidence of an accepted transition. An event MUST NOT become authorization merely because it exists.

## 11. Interface Contract

Every cross-domain interface MUST define owner, consumer, version, transport, schema, authentication, authorization, capabilities, timeout, retry semantics, idempotency, ordering, error model, compatibility, deprecation, observability and security requirements.

Supported interface classes include synchronous API, asynchronous command, event stream, IPC, P2P protocol, storage contract, ABI and developer interface.

No integration MAY depend on undocumented internal implementation details of another repository.

## 12. Dependency Direction

The numeric layer order is NOT a runtime execution sequence.

Dependency direction is:

~~~text
Foundation
    ^
Kernel / OS
    ^
Protocol / Chain
    ^
Execution
    ^
Domain Services
    ^
Applications
    ^
Experience
~~~

The arrow means dependency on the lower boundary.

Forbidden authority cycles include:
- Genesis depending on Aurora as state authority.
- Aurora depending on Genesis as global authority.
- Chain depending on Explorer.
- Explorer becoming Chain authority.
- Storage becoming Consensus authority.
- UI becoming protocol authority.
- Indexer becoming canonical state owner.

## 13. ATC-VM Boundary

Canonical contract path:

~~~text
ATCLang
   |
ATC-IR / ABI
   |
ATC Bytecode
   |
Verifier
   |
ATC-VM
   |
Deterministic Host Interface
   |
Candidate State Transition
~~~

The VM contract MUST define bytecode validity, instruction limits, stack/memory/storage limits, gas/resource limits, deterministic host calls, rollback semantics, error semantics and execution receipts.

ShivaCore MUST NOT be treated as a substitute for the canonical contract VM.

## 14. Genesis Engine Boundary

Genesis owns authoritative Genesis application/game state.

~~~text
Input / Player Action
        |
Command Validation
        |
Game Rules
        |
Simulation
        |
Quest / Dialogue / World Systems
        |
State Transition
        |
Invariant Validation
        |
Committed Game State
        |
Persistence
        |
Events / Replication / Presentation
~~~

AI operates around the runtime:

~~~text
Aurora / AI
   |
propose / plan / generate
   |
Genesis Authority
   |
validate
   |
execute
   |
commit
~~~

AI MUST NOT directly mutate canonical Genesis state without the declared runtime authority boundary.

## 15. Aurora Authority Flow

~~~text
Model Output
    |
Agent Intent
    |
Capability Check
    |
Policy Evaluation
    |
Approval if required
    |
Tool Authorization
    |
Tool Execution
    |
Target Domain Validation
    |
Target Domain Commit
    |
Audit / Evidence
~~~

Model output MUST be treated as untrusted input to the authority plane.

## 16. Memory Contract

Memory classes are distinct:

1. Working memory.
2. Episodic memory.
3. Semantic memory.
4. Retrieval index.
5. Knowledge graph.
6. Provenance.
7. Canonical domain state.

Only the target domain's canonical state is authoritative by default. AI memory MUST NOT silently overwrite Chain, Genesis, OS or kernel state.

## 17. Security Architecture

~~~text
Hardware / Root of Trust
        |
Boot / Platform Integrity
        |
ShivaCore Isolation
        |
GlobusOS Policy / IPC
        |
Identity
        |
Capability
        |
Authorization
        |
Protocol / Application Validation
        |
Audit / Evidence
~~~

Security controls MUST follow least privilege. Capabilities MUST be explicit. Ambient authority MUST NOT be assumed. Hardware-backed claims require hardware-specific evidence.

## 18. Identity Architecture

Security-sensitive principals MAY include human, user, organization, service, node, validator, wallet, contract, agent, model, tool, device and kernel/service principals.

Identity does not grant authority.

~~~text
Identity
  |
Authentication
  |
Principal Resolution
  |
Capability
  |
Policy
  |
Authorization
  |
Action
  |
Audit
~~~

## 19. Failure and Recovery

Every authoritative domain MUST define behavior for invalid input, authorization failure, execution failure, timeout, transport failure, partial failure, storage failure, consensus failure, dependency failure, replay, duplicate delivery, version mismatch and corrupted state.

Recovery:

~~~text
Detect Failure
    |
Stop Unsafe Progress
    |
Recover / Replay
    |
Validate Invariants
    |
Re-establish Canonical State
    |
Resume
    |
Emit Evidence
~~~

Recovery MUST NOT silently promote unverified state to canonical state.

## 20. Observability

Critical transitions SHOULD carry correlated request ID, transaction ID, block ID, state version, event ID, command ID, agent run ID, tool invocation ID, trace ID, source SHA and deployment/version ID where applicable.

Observability distinguishes:

- logs;
- metrics;
- traces;
- audit;
- provenance;
- test evidence;
- CI evidence;
- E2E evidence.

Operational telemetry is not a substitute for correctness evidence.

## 21. Versioning and Compatibility

Externally consumed contracts MUST define compatibility policy.

Versioned artifacts include APIs, ABIs, transaction encoding, signing domains, event schemas, state schemas, VM bytecode formats, network protocols, IPC contracts and tool schemas.

Breaking changes MUST define migration, compatibility window, rollback strategy, activation condition and evidence requirement.

## 22. Deployment Lifecycle

~~~text
Source
  |
Build
  |
Artifact
  |
Verification
  |
Release
  |
Deployment
  |
Health Validation
  |
Operational Evidence
~~~

Every release artifact MUST identify its source revision. Deployment success does not prove protocol correctness.

## 23. Repository Architecture

Standalone First, Ecosystem Second:

~~~text
Component Repository
        |
 Source / Build / Tests / Release / API-ABI
        |
        v
Ecosystem Integration
        |
        v
System-Level Evidence
~~~

Core repositories MUST retain their own implementation, build, tests, release and API/ABI SSOT.

The ecosystem repository MUST NOT become a hidden build dependency of a core repository.

## 24. Governance Architecture

~~~text
atc-standards
      |
Standards / Contracts
      |
Component Implementation
      |
Component Verification
      |
Ecosystem Integration
      |
System Evidence
~~~

Governance, security, architecture, implementation and release authority SHOULD remain separated where required by the applicable standards.

## 25. Evidence Architecture

Evidence dimensions are independent:

| Dimension | Meaning |
|---|---|
| PRESENT | artifact exists |
| SPECIFIED | contract is defined |
| IMPLEMENTED | source matches contract |
| TESTED | relevant tests exist and pass |
| CI_VERIFIED | required CI passed for exact SHA |
| INTEGRATED | declared interface is connected |
| E2E_VERIFIED | declared end-to-end path passed |
| RELEASED | release gate passed |
| DEPLOYED | artifact deployed |
| OPERATIONAL | runtime evidence demonstrates operation |

Thus:

~~~text
File exists
   != Implementation
   != Tested
   != Exact-SHA CI
   != E2E
   != Released
   != Operational
~~~

## 26. Canonical End-to-End Paths

### Blockchain

~~~text
SDK
 |
Wallet / Signing
 |
Node
 |
Mempool
 |
Validation
 |
Consensus
 |
ATC-VM
 |
State Transition
 |
Finality
 |
Storage
 |
Indexer
 |
Explorer / API
~~~

### AI Action

~~~text
User / Event
 |
Aurora Context
 |
Model
 |
Agent
 |
Plan
 |
Capability
 |
Policy
 |
Approval
 |
Tool
 |
Target Domain
 |
Validation
 |
Commit
 |
Audit
~~~

### Genesis

~~~text
Player / World Event
 |
Genesis Input
 |
Simulation / Rules
 |
AI Proposal where applicable
 |
Validation
 |
Authoritative Runtime
 |
Committed Game State
 |
Persistence
 |
Events
 |
Presentation
~~~

### OS

~~~text
Application
 |
GlobusOS Service
 |
IPC / Capability
 |
Policy
 |
ShivaCore
 |
Hardware
~~~

## 27. Cross-System Invariants

1. Exactly one authoritative owner per canonical state domain.
2. No AI component receives implicit super-authority.
3. No UI component becomes protocol authority.
4. No indexer becomes canonical state authority.
5. No storage component decides consensus finality.
6. No VM execution result is final without the applicable finality/commit rule.
7. No cross-domain write bypasses its declared interface.
8. No event is treated as authorization without an explicit contract.
9. No implementation claim is inferred from architecture text.
10. No CI claim is valid without exact-SHA association.
11. No E2E claim is valid without exercising the declared path.
12. No hardware-backed claim is valid without hardware evidence.
13. No core repository silently depends on the ecosystem repository.
14. No duplicate implementation silently becomes a second SSOT.
15. Canonical state migrations MUST define compatibility and recovery.

## 28. Architecture Change Gate

~~~text
Proposal
  |
Affected Domains
  |
Authority Analysis
  |
SSOT Analysis
  |
Dependency Analysis
  |
State / Migration Analysis
  |
Security Analysis
  |
Interface / Compatibility Analysis
  |
Implementation Plan
  |
Tests
  |
Exact-SHA CI
  |
Integration
  |
E2E where applicable
  |
Architecture Evidence
~~~

Documentation-only changes MUST NOT be presented as implementation changes.

## 29. Architectural Completeness

A domain contract is complete when it defines, at minimum:

Identity, Purpose, Scope, Owner, Authority, SSOT, State, Inputs, Outputs, Interfaces, Events, Security, Capabilities, Policies, Failure, Recovery, Versioning, Migration, Observability, Lifecycle and Evidence.

Architectural completeness does not imply implementation completeness.

## 30. Master Dependency View

~~~text
                         HUMAN / GOVERNANCE
                                |
                         Policy / Approval
                                |
          +---------------------+---------------------+
          |                     |                     |
       AURORA                GENESIS               CHAIN
    Intelligence          Game Runtime          L2 Protocol
          |                     |              +-----+-----+
          |                     |              |           |
          |                     |          Consensus      L3 VM
          |                     |              |           |
          |                     |              +-----+-----+
          |                     |                    |
          |                     |                 Finality
          |                     |                    |
          |                     |              Canonical State
          |                     |                    |
          |                     |                L1 Storage
          |                     |
          v                     v
      GlobusOS            Genesis State
          |                     |
          v                     v
      ShivaCore           Persistence
          |
          v
       Hardware
~~~

This diagram describes authority/dependency boundaries, not a claim that every arrow is a direct runtime call.

## 31. Conflict Resolution Precedence

When architecture sections appear to conflict, resolution MUST follow:

1. Explicit authority contract.
2. State ownership contract.
3. Canonical interface / ABI contract.
4. Layer responsibility.
5. Domain runtime contract.
6. Integration contract.
7. Descriptive diagram.
8. Non-normative example.

A diagram MUST NOT override a normative authority or state-ownership rule.

## 32. Master Rule

Authority follows the canonical state owner and its declared commit contract.

Execution, intelligence, storage, indexing and presentation remain subordinate to that authority boundary.

 
---

# ARCH-003 — System Operational Architecture

ARCH-003 closes the remaining operational architecture gaps identified by the master architecture review. It defines contracts for determinism, randomness, cryptography, networking, upgrades, bootstrap, interoperability, privacy, supply-chain security, resilience, testing, performance, configuration, external exposure, emergency control and multiplayer/runtime authority.

ARCH-003 is normative at the architecture-contract level. It does not claim implementation or verification.

## 1. Time and Determinism Contract

Every deterministic subsystem MUST declare its authoritative time source.

Time classes are distinct:

~~~text
Wall Clock
   |
Monotonic Clock
   |
Consensus / Block Time
   |
Application Time
   |
Simulation Time
~~~

Rules:
- Consensus state MUST NOT depend on an uncontrolled wall clock.
- VM execution MUST use only explicitly deterministic time inputs.
- Genesis simulation MUST define tick and time-step semantics.
- Timeout semantics MUST identify the clock used.
- Replay MUST reproduce the same state transition for the same canonical inputs.
- Clock drift, timestamp validity and acceptable ranges MUST be protocol-defined where consensus-relevant.

## 2. Randomness and Entropy Contract

Randomness classes MUST remain separate:

~~~text
Cryptographic Entropy
   !=
Consensus Randomness
   !=
Deterministic PRNG
   !=
Game Randomness
   !=
AI Sampling
~~~

Consensus- or protocol-relevant randomness MUST define its source, commitment/derivation, verification and replay semantics.

Game randomness MUST NOT silently become protocol randomness.

AI sampling MUST NOT be treated as authoritative randomness.

## 3. Cryptographic and Key Lifecycle Contract

Security-sensitive keys MUST have a lifecycle:

~~~text
Generate
  ↓
Register / Bind Identity
  ↓
Store
  ↓
Use
  ↓
Rotate
  ↓
Revoke
  ↓
Recover / Retire
~~~

The contract MUST define:
- algorithm;
- key purpose;
- signing domain;
- storage boundary;
- authorization;
- rotation;
- revocation;
- compromise response;
- recovery;
- audit.

Transaction signing MUST use the canonical domain-separation contract, including the network/chain identity and canonical transaction encoding.

Validator, node, wallet, service, agent, tool and hardware-backed keys MUST NOT share implicit authority.

## 4. Networking and P2P Contract

Network lifecycle:

~~~text
Discovery
  ↓
Handshake
  ↓
Identity Authentication
  ↓
Protocol / Capability Negotiation
  ↓
Connection Policy
  ↓
Message Validation
  ↓
Propagation
  ↓
Peer Scoring
  ↓
Disconnect / Restrict
~~~

The network contract MUST define:
- peer identity;
- protocol versions;
- message schemas;
- maximum message size;
- connection limits;
- rate limits;
- peer scoring;
- Sybil resistance;
- eclipse resistance;
- DoS protection;
- gossip rules;
- partition behavior;
- retry/backoff;
- observability.

A network peer is not automatically trusted because it is reachable.

## 5. Protocol Upgrade and Migration Contract

Protocol changes MUST follow an explicit lifecycle:

~~~text
Proposal
  ↓
Specification
  ↓
Technical / Security / Architecture Review
  ↓
Approval
  ↓
Compatibility Analysis
  ↓
Activation Condition
  ↓
Deployment
  ↓
Activation
  ↓
Verification
  ↓
Post-Activation Monitoring
~~~

Every breaking change MUST define:
- affected state;
- version;
- activation condition;
- compatibility window;
- migration;
- rollback boundary;
- recovery;
- evidence.

Rollback MUST NOT create an invalid or ambiguous canonical state.

Emergency protocol changes require explicit emergency authority and separate evidence.

## 6. Bootstrap, Genesis and State-Sync Contract

Canonical bootstrap:

~~~text
Network Identity
  ↓
Genesis Configuration
  ↓
Genesis Hash
  ↓
Protocol Version
  ↓
Initial State
  ↓
Initial Validator / Authority Set
  ↓
Bootstrap Verification
  ↓
Network Start
~~~

State synchronization:

~~~text
Snapshot
  ↓
Metadata / Commitment
  ↓
Source Authentication
  ↓
Download
  ↓
Integrity Verification
  ↓
Restore
  ↓
Replay Missing Range
  ↓
Canonical-State Verification
  ↓
Serve
~~~

A node MUST NOT serve an unverified recovered state as canonical.

Genesis configuration MUST define chain identity, initial protocol parameters, initial economic state and reproducibility requirements.

## 7. Interoperability, Oracle and Bridge Contract

Interop, Oracle and Bridge are distinct domains.

### Oracle

~~~text
External Data
  ↓
Provider
  ↓
Attestation
  ↓
Validation
  ↓
Aggregation
  ↓
Freshness Check
  ↓
Canonical Oracle State
~~~

### Cross-Chain Interop

~~~text
External Chain
  ↓
Proof / Attestation
  ↓
Verification
  ↓
Interop Gateway
  ↓
Policy
  ↓
Canonical Command
  ↓
Target Domain
~~~

### Bridge

A bridge MUST additionally define:
- asset representation;
- lock/mint or burn/release semantics;
- message nonce;
- replay protection;
- external finality requirement;
- timeout;
- pause/circuit breaker;
- recovery;
- reconciliation.

External data MUST NOT become canonical merely because an oracle or bridge reports it.

## 8. Data, Privacy and Retention Contract

Data classes MUST be explicit:

~~~text
Public
Private
Confidential
Secret
Sensitive
Derived
Canonical
Ephemeral
~~~

Data lifecycle:

~~~text
Collection
  ↓
Classification
  ↓
Purpose / Authorization
  ↓
Processing
  ↓
Storage
  ↓
Retention
  ↓
Deletion / Tombstone where applicable
  ↓
Audit
~~~

On-chain immutable data MUST be treated differently from mutable off-chain data.

AI memory, retrieval indexes and derived knowledge MUST carry provenance and access policy where required.

Secrets MUST NOT be stored in ordinary application state.

## 9. Secrets Management Contract

Secrets lifecycle:

~~~text
Provision
  ↓
Bind to Workload Identity
  ↓
Inject
  ↓
Use
  ↓
Rotate
  ↓
Revoke
  ↓
Destroy
~~~

Secrets MUST be:
- least-privilege;
- scoped;
- auditable;
- non-loggable;
- rotatable;
- revocable.

Aurora tools and external service credentials MUST execute under explicit workload identities.

## 10. Supply-Chain and Artifact Security Contract

Release chain:

~~~text
Source
  ↓
Dependency Resolution
  ↓
Dependency Verification
  ↓
Build
  ↓
SBOM / Provenance
  ↓
Artifact Signing
  ↓
Verification
  ↓
Release
  ↓
Deployment
~~~

Critical artifacts SHOULD support:
- reproducible builds where applicable;
- pinned dependencies;
- dependency provenance;
- SBOM;
- artifact signatures;
- source/build provenance;
- vulnerability policy;
- compromised dependency response.

A signed artifact proves integrity/authenticity of the signing boundary; it does not prove functional correctness.

## 11. Backup, Disaster Recovery and Incident Contract

Backup:

~~~text
State
  ↓
Snapshot / Backup
  ↓
Integrity Verification
  ↓
Protected Storage
  ↓
Restore Test
~~~

Disaster recovery MUST define:
- RPO;
- RTO;
- backup frequency;
- retention;
- geographic redundancy;
- immutable backup where required;
- corruption detection;
- restore verification.

Incident lifecycle:

~~~text
Detection
  ↓
Classification
  ↓
Containment
  ↓
Evidence Preservation
  ↓
Mitigation
  ↓
Recovery
  ↓
Verification
  ↓
Post-Incident Review
  ↓
Corrective Architecture / Standard Change
~~~

## 12. Configuration Contract

Configuration classes are distinct:

- static configuration;
- environment configuration;
- protocol configuration;
- runtime configuration;
- governance configuration;
- feature configuration;
- secret configuration.

Configuration lifecycle:

~~~text
Source
  ↓
Schema Validation
  ↓
Default Resolution
  ↓
Policy Validation
  ↓
Activation
  ↓
Audit
~~~

Configuration MUST NOT silently override protocol semantics.

Protocol-relevant configuration MUST be versioned and governed.

## 13. Resource Governance Contract

Resource classes include:

- CPU;
- memory;
- storage;
- network;
- I/O;
- GPU;
- AI inference;
- tool execution;
- disk quota;
- bandwidth.

Admission:

~~~text
Resource Request
  ↓
Identity / Capability
  ↓
Quota
  ↓
Policy
  ↓
Admission
  ↓
Execution
  ↓
Accounting
  ↓
Limit / Reclaim
~~~

Resource exhaustion MUST have explicit failure and degradation semantics.

## 14. Testing and Verification Architecture

The verification hierarchy is:

~~~text
Static Analysis
   ↓
Unit Tests
   ↓
Property Tests
   ↓
Fuzz Tests
   ↓
Component Integration
   ↓
Cross-Repository Integration
   ↓
Network Tests
   ↓
Fault Injection
   ↓
E2E
   ↓
Soak / Load
   ↓
Recovery / Chaos
~~~

Blockchain-specific verification SHOULD include:
- deterministic state-transition vectors;
- serialization vectors;
- signing vectors;
- consensus vectors;
- fork tests;
- replay tests;
- migration tests.

Critical invariants SHOULD be represented as executable properties. Formal verification MAY be applied where the assurance requirement justifies it.

No lower-level test substitutes for an E2E requirement.

## 15. Performance and Capacity Contract

Every critical subsystem SHOULD define:

- latency;
- throughput;
- resource budget;
- queue limits;
- saturation point;
- backpressure;
- graceful degradation;
- recovery behavior.

Capacity lifecycle:

~~~text
Capacity Model
  ↓
Load Model
  ↓
Scaling Boundary
  ↓
Admission / Backpressure
  ↓
Degradation
  ↓
Recovery
~~~

Performance claims MUST identify workload, environment, version and measurement method.

## 16. External Exposure Contract

External interfaces MUST be classified:

- internal API;
- authenticated API;
- public RPC;
- developer API;
- administrative API;
- governance API;
- indexer API;
- AI tool API.

Every exposed interface MUST define authentication, authorization, rate limits, quotas, abuse protection, versioning, compatibility and audit semantics.

Administrative interfaces MUST NOT inherit authority merely from network reachability.

## 17. Emergency Authority Contract

Emergency authority is distinct from normal authority.

~~~text
Detection
  ↓
Emergency Authority
  ↓
Scope Verification
  ↓
Required Approval
  ↓
Pause / Restrict / Contain
  ↓
Evidence
  ↓
Recovery
  ↓
Verification
  ↓
Resume
~~~

Emergency controls MUST define:
- authorized principals;
- permitted scope;
- trigger conditions;
- approval requirements;
- maximum duration;
- audit;
- recovery;
- expiry.

Emergency authority MUST NOT become an undocumented permanent superuser.

## 18. AI Security Contract

### Model Security

Models SHOULD have:
- provenance;
- version identity;
- integrity verification;
- policy classification;
- rollback capability.

### Agent Security

Agents MUST have:
- identity;
- explicit capabilities;
- delegation boundary;
- action budget;
- recursion/runaway limits;
- audit identity.

### Tool Security

~~~text
Agent
  ↓
Tool Request
  ↓
Schema Validation
  ↓
Capability
  ↓
Policy
  ↓
Sandbox
  ↓
Execution
  ↓
Result Validation
  ↓
Audit
~~~

Tool output MUST be treated as untrusted input unless the tool contract explicitly establishes a trusted result boundary.

### AI Memory Security

Memory SHOULD support:
- provenance;
- trust classification;
- authorization;
- poisoning resistance;
- tenant isolation where applicable;
- retention;
- deletion semantics.

## 19. Genesis Simulation and Multiplayer Contract

Genesis separates authoritative simulation from presentation.

~~~text
Client Input
  ↓
Network
  ↓
Authoritative Simulation
  ↓
Rules / Systems
  ↓
State Transition
  ↓
Commit
  ↓
Replication
  ↓
Client Presentation
~~~

Client prediction is not authoritative state.

Genesis MUST define, where multiplayer applies:
- tick model;
- simulation time;
- authoritative server/domain;
- client prediction;
- reconciliation;
- rollback;
- replication;
- entity ownership;
- shard/world ownership;
- persistence.

Rendering MUST NOT mutate authoritative gameplay state.

## 20. Developer Platform Contract

Developer-facing architecture includes:

- SDK;
- CLI;
- RPC clients;
- contract compiler/toolchain;
- ATCLang tooling;
- ABI tooling;
- wallet tooling;
- debugger;
- simulator;
- local network/test environment;
- deployment tooling.

Developer tools MUST consume versioned contracts rather than undocumented internal structures.

## 21. Formal Invariant Contract

Critical invariants SHOULD be mapped to executable verification artifacts.

~~~text
Invariant
  ↓
Formal Property / Test Vector
  ↓
Reference Implementation
  ↓
Property Test
  ↓
Fuzz / Differential Test
  ↓
Exact-SHA CI
  ↓
E2E Evidence where applicable
~~~

Examples include monetary supply, transaction validity, signing-domain correctness, VM safety, capability isolation and state-transition determinism.

## 22. Additional Cross-System Invariants

16. Consensus-relevant time is deterministic and explicitly defined.
17. Protocol randomness has a verifiable source and replay semantics.
18. Key purpose and signing domain are explicit.
19. External data is not canonical without verification and target-domain acceptance.
20. Recovered state is not canonical until integrity and protocol invariants are verified.
21. Configuration cannot silently change protocol semantics.
22. Resource exhaustion has defined failure behavior.
23. External APIs cannot acquire authority from network reachability.
24. Emergency authority is scoped, auditable and non-permanent.
25. AI memory is not canonical domain state by default.
26. Client prediction is not authoritative Genesis state.
27. Artifact integrity does not equal functional correctness.
28. Performance claims require reproducible measurement context.
29. Critical invariants map to executable verification evidence.
30. Upgrade activation requires explicit compatibility and recovery semantics.

## 23. Architecture Gap Closure Rule

ARCH-001 defines authority and evidence consistency.
ARCH-002 defines detailed system boundaries and lifecycle.
ARCH-003 defines operational, security, resilience and verification contracts.

Together:

~~~text
ARCH-001
Authority / Evidence
      ↓
ARCH-002
System / Domain / Interface
      ↓
ARCH-003
Operations / Security / Resilience / Verification
~~~

These contracts remain architectural SSOTs. Implementation status MUST still be established from component repositories and exact-SHA evidence.

 
---
 
# ARCH-004 — Functional Operating System Architecture
 
ARCH-004 defines the minimum functional contract for a real bootable and usable GlobusOS/ShivaCore operating system. It closes the distinction between kernel architecture, OS architecture and an actually functioning OS.
 
ARCH-004 is an architecture contract only. It MUST NOT be interpreted as implementation, test, CI, hardware or release evidence.
 
## 1. Boot-to-Userspace Contract
 
The canonical minimum OS path is:
 
~~~text
Firmware / UEFI
  ↓
Bootloader
  ↓
Kernel Image Verification / Load
  ↓
ShivaCore Entry
  ↓
CPU / Exception / Interrupt Initialization
  ↓
Physical + Virtual Memory
  ↓
Timer + Scheduler
  ↓
Kernel Objects / Capabilities
  ↓
Syscall Boundary
  ↓
Userspace Address Space
  ↓
ELF / Program Loader
  ↓
Initial Userspace Process
  ↓
PID 1 / Init
  ↓
Service Manager
  ↓
Core GlobusOS Services
  ↓
Shell / Application
~~~
 
A platform MUST NOT be described as a functioning general-purpose OS until this path is implemented and verified for its declared target environment.
 
## 2. CPU and Architecture Contract
 
ShivaCore MUST explicitly define per supported architecture:
 
- boot protocol;
- CPU initialization;
- privilege levels;
- exception vectors;
- interrupt model;
- timer source;
- context-switch mechanism;
- atomic/synchronization primitives;
- SMP initialization;
- CPU topology;
- architecture-specific memory-management behavior;
- shutdown/reboot semantics.
 
Architecture support MUST be tracked independently. Code existing for one architecture MUST NOT imply support for another.
 
## 3. Interrupt, Exception and Timer Contract
 
The kernel MUST provide:
 
- exception dispatch;
- interrupt routing;
- interrupt masking/unmasking;
- timer interrupts;
- syscall entry;
- page-fault handling;
- fatal-fault handling;
- interrupt-safe synchronization rules;
- timer ownership and cancellation semantics.
 
Fatal kernel faults MUST have a defined evidence-producing failure path such as panic, crash record or controlled halt.
 
## 4. Memory Management Contract
 
Memory management MUST define:
 
~~~text
Physical Memory
  ↓
Frame / Page Allocator
  ↓
Page Tables / MMU
  ↓
Kernel Address Space
  ↓
User Address Spaces
  ↓
Heap / Allocator
~~~
 
The contract MUST cover:
 
- physical allocation;
- virtual mapping;
- page permissions;
- user/kernel isolation;
- page faults;
- shared memory;
- memory ownership;
- mapping lifetime;
- allocator failure;
- out-of-memory behavior;
- memory accounting;
- SMP synchronization.
 
Kernel and userspace memory MUST have explicit protection boundaries.
 
## 5. Scheduling and Process Contract
 
The kernel MUST define:
 
- process;
- thread;
- task;
- scheduler;
- runnable/blocked/sleeping states;
- context switching;
- priorities or scheduling classes;
- timer-driven wakeup;
- process termination;
- thread termination;
- parent/child semantics;
- CPU affinity where SMP applies.
 
A scheduler contract MUST define behavior under CPU saturation and resource exhaustion.
 
## 6. Userspace and Program Loading Contract
 
A general-purpose OS MUST define a real userspace execution boundary:
 
~~~text
Kernel
  ↓
Address Space Creation
  ↓
ELF / Program Validation
  ↓
Executable Mapping
  ↓
User Stack / Initial Runtime State
  ↓
Ring-3 / User Privilege
  ↓
Program Execution
~~~
 
The program loader MUST validate executable format, memory ranges, permissions and entry conditions before transfer of control.
 
Userspace MUST NOT obtain kernel authority by merely controlling executable input.
 
## 7. Syscall ABI Contract
 
The kernel/userspace boundary MUST define a versioned syscall ABI including:
 
- syscall numbering;
- calling convention;
- argument validation;
- pointer validation;
- object/capability references;
- return values;
- error model;
- interruption/cancellation;
- ABI versioning;
- compatibility/deprecation rules.
 
Syscalls MUST be treated as hostile input boundaries.
 
## 8. IPC and Capability Contract
 
The microkernel communication path MUST be:
 
~~~text
Process A
  ↓
Capability / Endpoint Authorization
  ↓
IPC Message Validation
  ↓
Kernel IPC
  ↓
Process B
~~~
 
IPC MUST define:
 
- endpoints;
- send/receive/reply;
- synchronous/asynchronous semantics;
- message limits;
- timeout;
- cancellation;
- shared-memory transfer where applicable;
- capability transfer;
- object lifetime;
- failure behavior;
- replay/idempotency semantics where required.
 
Capability checks MUST be enforced by the kernel, not only by userspace convention.
 
## 9. Device, Driver and HAL Contract
 
Hardware access MUST follow:
 
~~~text
Hardware
  ↓
Architecture HAL
  ↓
Bus / Device Discovery
  ↓
Driver
  ↓
Kernel / Driver Interface
  ↓
OS Service
  ↓
Userspace API
~~~
 
At minimum the declared boot target SHOULD provide working paths for:
 
- console/serial;
- timer;
- interrupt controller;
- storage;
- network;
- input;
- display/framebuffer where a graphical target is claimed.
 
Optional hardware such as TPM, TEE, GPU, NPU, USB or Bluetooth MUST be separately capability- and hardware-verified.
 
## 10. Storage and VFS Contract
 
A usable OS MUST define:
 
~~~text
Block Device
  ↓
Block Layer
  ↓
Filesystem
  ↓
VFS
  ↓
File Descriptor / Handle API
  ↓
Userspace
~~~
 
The contract MUST cover:
 
- device discovery;
- partitions;
- filesystem format;
- mount/unmount;
- files/directories;
- handles/descriptors;
- permissions;
- concurrent access;
- crash consistency;
- corruption detection/recovery;
- fsck or equivalent;
- persistence guarantees.
 
## 11. Init and Service Lifecycle Contract
 
After userspace starts:
 
~~~text
Kernel
  ↓
Initial Process
  ↓
PID 1 / Init
  ↓
Service Dependency Resolution
  ↓
Service Start
  ↓
Health Check
  ↓
RUNNING
~~~
 
PID 1/service management MUST define:
 
- dependency ordering;
- startup failure;
- restart policy;
- health checks;
- shutdown ordering;
- signal/event handling;
- service isolation;
- capability assignment;
- resource limits;
- boot failure reporting.
 
## 12. Core GlobusOS Services Contract
 
A minimal functional GlobusOS service set SHOULD include:
 
- process/service manager;
- identity/authentication;
- capability/policy service;
- storage service;
- device manager;
- network manager;
- time service;
- logging/audit service;
- configuration service;
- update/recovery service;
- IPC/service registry.
 
Each service MUST have a declared owner, interface, state model, capability requirements, lifecycle and recovery semantics.
 
## 13. Networking and Socket Contract
 
A network-capable OS MUST define:
 
~~~text
NIC
  ↓
Driver
  ↓
Link / Network Stack
  ↓
IP / Transport
  ↓
Socket API
  ↓
Userspace Service
~~~
 
The contract MUST define interface lifecycle, addressing, routing, sockets, DNS where applicable, firewall/policy boundaries, error behavior, resource limits and network service isolation.
 
Protocol implementations MUST NOT be treated as available merely because a library or source file exists.
 
## 14. Identity, Authentication and Authorization Contract
 
The OS identity path MUST be:
 
~~~text
Principal
  ↓
Identity
  ↓
Authentication
  ↓
Session / Workload Identity
  ↓
Authorization
  ↓
Capability
  ↓
Resource / Service
~~~
 
Credential storage, session lifecycle, key purpose, revocation, recovery and audit MUST be explicit.
 
GlobusOS identity MUST NOT silently inherit blockchain, wallet, Aurora or Genesis authority.
 
## 15. Secure Boot and Trusted Boot Contract
 
Where secure boot is claimed:
 
~~~text
Firmware
  ↓
Boot Policy
  ↓
Bootloader Verification
  ↓
Kernel Verification
  ↓
Measured / Attested State where supported
  ↓
ShivaCore
~~~
 
Secure Boot, measured boot, TPM and TEE MUST remain distinct claims. Each requires its own implementation and hardware evidence.
 
## 16. Update, Recovery and A/B Contract
 
System updates MUST define:
 
~~~text
Artifact
  ↓
Signature / Provenance Verification
  ↓
Compatibility Check
  ↓
Inactive Slot
  ↓
Install
  ↓
Reboot
  ↓
Health Validation
  ↓
Commit
~~~
 
Failure MUST lead to a defined rollback/recovery boundary. An update mechanism MUST NOT leave an ambiguous partially activated system state.
 
## 17. Userspace Runtime and Standard Services
 
A usable userspace SHOULD provide stable libraries/APIs for:
 
- process creation;
- memory allocation;
- threads/synchronization;
- filesystem;
- networking;
- IPC;
- time;
- logging;
- cryptography;
- configuration;
- environment/session management.
 
The API surface MUST consume versioned OS contracts rather than private kernel implementation details.
 
## 18. Display, Input, Audio and Desktop Contract
 
A graphical OS target additionally requires:
 
~~~text
Display / GPU
  ↓
Graphics Driver / HAL
  ↓
Compositor / Display Server
  ↓
Window / Session Manager
  ↓
Input
  ↓
Desktop Shell
  ↓
Applications
~~~
 
Rendering MUST remain separate from authoritative system state. Input MUST cross explicit authorization and event boundaries. Audio, camera, GPU and other peripheral access MUST be capability-controlled.
 
## 19. OS Resource and Isolation Contract
 
Every process/service SHOULD have explicit accounting and policy for:
 
- CPU;
- memory;
- storage;
- network;
- file handles;
- IPC objects;
- device access;
- GPU/NPU resources where applicable.
 
Resource exhaustion MUST be isolated so that an untrusted service cannot trivially destabilize the kernel or unrelated services.
 
## 20. Observability and Diagnostics Contract
 
A functioning OS MUST expose enough diagnostics to establish whether boot and runtime contracts are working.
 
Minimum evidence SHOULD include:
 
- boot stage;
- kernel version/build identity;
- hardware/architecture identity;
- process/thread lifecycle;
- memory state;
- service state;
- IPC failures;
- device discovery;
- storage/network state;
- security events;
- crash/panic information;
- update/rollback state.
 
Diagnostics MUST NOT leak secrets or unrestricted credentials.
 
## 21. OS Test Pyramid and Boot Gates
 
The OS verification chain MUST include, where applicable:
 
~~~text
Static Analysis
  ↓
Kernel Unit Tests
  ↓
Architecture / ABI Tests
  ↓
Userspace Tests
  ↓
IPC / Capability Tests
  ↓