# A-TownChain Ecosystem — Canonical System Specification

**Role:** normative system-level specification surface separated from the master architecture.

## Status semantics

Architecture, specification, implementation, tests, CI evidence, integration and E2E evidence are independent dimensions. This document defines contracts; it does not assert implementation status.

## Canonical contract set

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

---

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

---

## AURORA-001 Evidence Rule

Aurora component status follows:

```text
ARCHITECTURE_ONLY
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
```

Exceptional states are `MISSING`, `BLOCKED`, `DUPLICATE` and `DISCONNECTED`.

A file, directory or architecture statement is not implementation evidence. CI evidence is valid only when tied to the exact source commit under assessment.

---

## Specification ownership

- System architecture and cross-domain contracts: `a-townchain-ecosystem`
- Normative standards/governance: `atc-standards`
- Executable implementation: each component's canonical repository
- Test/CI/E2E evidence: the exact implementation commit and its verification records

A specification is not an implementation claim.