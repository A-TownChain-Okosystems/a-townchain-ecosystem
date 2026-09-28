# A-TownChain Ecosystem — System-Architektur

## Architekturstatus

**Status:** ACTIVE  
**Architekturrolle:** System-of-Systems Master Architecture  
**Prinzip:** SSOT je Domäne, zentrale Integrationsarchitektur im Ecosystem  
**Quest AI:** als kanonische Game-Intelligence-Funktion in Aurora/Genesis integriert

> **Canonical architecture rule:** L0–L7 are architectural responsibility domains, not a mandatory runtime ordering. **X** is a cross-layer control plane, not an additional execution layer. Architecture, implementation, testing, CI evidence and E2E evidence are separate status dimensions.

## Canonical Layer Model

```text
L0 — System & Network
     Hardware / HAL / GlobusOS / ShivaCore / networking / transport / node runtime

L1 — Data & Storage
     Canonical persistence / state storage / history / read models / indexing

L2 — Blockchain Core & Consensus
     Transactions / authorization entry / mempool / blocks / consensus / finality / economics

L3 — Deterministic Execution / ATC-VM
     ATCLang / ATC-IR / ABI / bytecode / verifier / ATC-VM / deterministic state transition

L4 — Scaling & Execution Domains
     Rollups / validity domains / specialized execution / future scaling and settlement domains

L5 — Protocol & Economic Domains
     Identity / assets / oracle / compute / interoperability / marketplace / launchpad / protocol contracts

L6 — AI & Intelligence
     Aurora / agents / RAG / memory / Quest AI / Dialogue AI / World / Character / domain intelligence

L7 — Applications & User Experience
     Genesis Engine / games / worlds / franchises / wallet UX / explorer / user-facing applications

X — Cross-Layer Control Plane
     Identity / capability / authorization / policy / governance / cryptography /
     audit & evidence / observability / interoperability / versioning & compatibility
```

### Layer Semantics

| Layer | Authority | Determinism | State authority |
|---|---|---|---|
| L0 | System / kernel / network boundary | deterministic where specified | No canonical chain state |
| L1 | Data / persistence | deterministic persistence rules | Persistence of state committed by L2/L3; no protocol authority |
| L2 | Blockchain protocol | deterministic | Chain authority / consensus |
| L3 | ATC-VM execution | deterministic | Computes deterministic state transitions under L2 protocol authority |
| L4 | Execution/scaling domain | domain-contract dependent | Only explicitly authorized domain state |
| L5 | Protocol/economic domain | contract/policy defined | Authorized protocol state |
| L6 | AI / intelligence | not a consensus authority | No direct canonical chain authority |
| L7 | Application / UX | runtime/user dependent | No direct canonical chain authority |
| X | Governance / security / control | policy/contract defined | Controls authorization and access |

**Layer ordering is conceptual.** L1 being numbered below L2 does not mean storage executes before consensus. The runtime data path is defined separately below.

## Canonical Deterministic Trust Boundary

The authoritative blockchain path separates candidate execution from protocol finality:

```text
Transaction
    ↓
Signature / Authorization
    ↓
Mempool
    ↓
Block Proposal / Validation
    ↓
ATC-VM — deterministic candidate state transition
    ↓
Consensus / Validator Voting
    ↓
Finality / Commit
    ↓
Canonical State
    ↓
Durable Storage / Read Models
```

This diagram defines authority and commit semantics, not a single-process implementation. Execution may occur before finality as a candidate transition; only finalized protocol state becomes canonical.

### Boundary Invariants

```text
Aurora
  ≠ Consensus
  ≠ ATC-VM
  ≠ canonical blockchain state

Genesis
  ≠ Blockchain Core

GlobusOS / ShivaCore
  ≠ ATC-VM

Indexer
  ≠ Source of Truth

Explorer
  ≠ Source of Truth

AI Memory
  ≠ canonical blockchain state
```

AI/model inference can produce proposals, commands or requests. Authoritative state changes require the responsible deterministic runtime and applicable authorization/policy checks.

## Cross-Layer Control Plane

X is a cross-cutting control-plane model for identity, capability, authorization, policy, governance, security and evidence. It does not replace L0–L7, does not become a runtime execution layer, and does not independently execute blockchain state transitions. Normative standards remain owned by `atc-standards`; runtime enforcement remains with the responsible domain.

```text
                 CROSS-LAYER CONTROL PLANE — X
 ┌──────────────────────────────────────────────────────────────┐
 │ Identity                                                     │
 │ Capability / Authorization                                   │
 │ Policy                                                      │
 │ Governance                                                   │
 │ Cryptography                                                 │
 │ Audit / Evidence                                             │
 │ Observability                                                │
 │ Interoperability                                             │
 │ Versioning / Compatibility                                   │
 └───────────────────────────┬──────────────────────────────────┘
                             │
             ┌───────────────┼───────────────┐
             ▼               ▼               ▼
            L0              L1              L2 ... L7
```

## Primary Repository / Subsystem Mapping

A repository may span more than one architectural layer; the table identifies its primary responsibility and does not override repository-level SSOT.

| Layer | Primary repositories / subsystems |
|---|---|
| L0 | `atc-shivacore`, `globus-os`, `atc-node` network/runtime boundary |
| L1 | `atc-storage`, `atc-indexer` read models, `atc-explorer` presentation/read model |
| L2 | `a-townchain`, `atc-node`, `atc-algorithm`, `atc-mining`, `atc-wallet` transaction interface |
| L3 | `atc-vm`, `atclang`, `atc-contracts` |
| L4 | `atc-zkp`, future scaling/execution-domain components |
| L5 | `atc-interop`, `atc-oracle`, `atc-compute`, `atc-marketplace`, `atc-launchpad`, protocol/asset/identity domains |
| L6 | `aurora-ai`, Quest AI, Dialogue AI and other domain-intelligence capabilities |
| L7 | `genesis-engine`, `genesis-chronicles`, `genesis-franchise-factory`, `atc-wallet` UX, `atc-explorer` UX, `atc-ide` |
| X | `atc-standards`, security/cryptography/identity/policy/audit capabilities across the ecosystem |

This mapping is architectural only. It does not assert implementation, test, CI or E2E status. Cross-layer repositories remain governed by their own canonical source-of-truth contracts.

## Canonical Runtime / Data-Flow Separation

The layer model and runtime flow must not be conflated.

```text
L2 Blockchain Core
    │
    ├── Transaction / Mempool
    └── Block Proposal / Validation
             │
             ▼
L3 Deterministic Execution
    │
    └── ATC-VM / candidate State Transition
             │
             ▼
L2 Consensus / Validator Voting
             │
             ▼
L2 Finality / Commit
             │
             ▼
L1 Canonical State Persistence
    │
    ├── Durable Storage
    └── Read Models / Indexing
```

The VM result is a candidate until accepted by the applicable consensus/finality rules. L1 persists committed state; it does not determine protocol finality.

Application and intelligence paths connect through defined interfaces rather than becoming part of the consensus/VM execution boundary:

```text
L6 Aurora / AI
      │
      ├── Proposal / Command / Request
      ▼
X Authorization / Policy / Capability
      │
      ▼
L7 Genesis / Application Runtime
      │
      ├── deterministic local state changes
      └── authorized L5/L2/L3 integration when required
```

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

### L0 — Foundation / Host / Network

Owns host assumptions, transport, platform capabilities and hardware-facing interfaces. L0 MUST NOT define application semantics.

### L1 — Data / Persistence

Owns durable storage, committed-state persistence, snapshots, journals, recovery, historical reads and derived read models. L1 MUST NOT decide blockchain finality.

### L2 — Blockchain / Protocol / Consensus

Owns transactions, authorization, mempool admission, block proposal, validation, peer protocol, validator participation, consensus and finality. L2 owns blockchain finality authority.

### L3 — Deterministic Execution / ATC-VM

Owns bytecode verification, deterministic execution, resource accounting, contract state-transition calculation, host interfaces, limits and execution receipts. L3 produces candidate transitions and MUST NOT independently finalize blockchain state.

### L4 — Scaling / Execution Domains

Owns execution domains, partitioned/parallel execution where defined, domain-local ordering and settlement interfaces. Every L4 domain MUST define its relationship to L2 canonical state.

### L5 — Protocol / Economic Domains

Owns monetary policy implementation, rewards, staking, slashing, treasury and protocol economic services. Economic state MUST define its authoritative owner, units and numeric types.

### L6 — Intelligence / AI

Owns models, inference, planning, retrieval, agents, multimodal processing and domain intelligence. L6 output is non-authoritative by default.

### L7 — Applications / Experience

Owns wallets, explorers, Genesis applications, editor UX, developer tooling and presentation. L7 MUST NOT bypass lower-layer authority boundaries.

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
