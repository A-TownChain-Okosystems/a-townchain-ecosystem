# A-TownChain Ecosystem — System-Architektur

## Architekturstatus

**Status:** ACTIVE  
**Architekturrolle:** System-of-Systems Master Architecture  
**Prinzip:** SSOT je Domäne, zentrale Integrationsarchitektur im Ecosystem  
**Quest AI:** als kanonische Game-Intelligence-Funktion in Aurora/Genesis integriert

## Schichtenmodell

```
+----------------------------------------------------------------+
| Applications                                                  |
| Genesis Chronicles / Games / Franchises / User Experiences    |
+----------------------------------------------------------------+
| Genesis Engine — L6 Game Platform                             |
| Runtime / ECS / World / Gameplay / AI / Quest AI / Dialogue / LiveOps |
+----------------------------------------------------------------+
| Aurora AI — AI / Agent / Policy Layer                          |
| Models / Agents / Memory / Tools / Planning / Dialogue / Governance |
+----------------------------------------------------------------+
| A-TownChain L1                                                 |
| Node / Wallet / SDK / VM / Algorithm / State / Storage / ...  |
+----------------------------------------------------------------+
| ATCLang                                                       |
| On-chain programs / contracts / deterministic artifacts       |
+----------------------------------------------------------------+
| ShivaCore Kernel — TCB                                         |
| Hardware / HAL / memory / IPC / scheduling / security         |
+----------------------------------------------------------------+
```

## System Integration

Der kanonische Daten- und Kontrollfluss lautet:

```
ShivaCore
    ↓
ATCLang
    ↓
ATC-VM
    ↓
A-TownChain L1
    ↓
Aurora AI
    ↓
Genesis Engine
    ↓
Games / Worlds / Franchises
```

Die Engine bleibt außerhalb des Konsens-Kernels. On-chain relevante Zustandsänderungen passieren ausschließlich über die definierte Chain/VM-Grenze.

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
→ Reward
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
Rewards / Reputation / Consequences
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
| Quest contracts / Quest runtime | genesis-engine / a-townchain-ecosystem architecture |
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
