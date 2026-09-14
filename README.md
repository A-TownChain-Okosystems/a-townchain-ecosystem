<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-GAME-001
  name: genesis-engine
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S1
  criticality: low
-->

# ATC Genesis Engine

[![ATC-COMPLIANCE](https://img.shields.io/badge/ATC-COMPLIANCE-v1.0-green)](./AGENTS.md)

> Modulare, KI-native Game-Engine für ECS, Kreaturen-Logik und Weltensimulation im A-TownChain-Ökosystem.

**Project:** `genesis-engine`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0 — A-TownChain-Okosystems`

## Overview

Genesis Engine ist die **general-purpose Game-Development-Plattform** des A-TownChain-Ökosystems. Sie stellt die generischen Engine-Funktionen bereit, während `genesis-chronicles` als Premium-Flagship-/Reference-Game auf dieser Engine aufsetzt.

Die Engine ist in vier Kernmodule gegliedert:

- `atc-genesis-engine` — Core Engine Loop und Lifecycle
- `atc-genesis-ecs` — Entity Component System
- `atc-genesis-creatures` — KI- und Kreaturenverhalten
- `atc-genesis-world` — Welt-, Terrain- und Umgebungssimulation

Die Engine befindet sich im Rebuild und ist **nicht als Production-Ready oder als finaler Releasezustand zu verstehen**.

Die Produktstrategie ist in [`GEN-PROD-001`](docs/specs/GEN-PROD-001-PRODUCT-STRATEGY.md) beschrieben. Die dort definierten Grenzen verhindern, dass generische Engine-Funktionen dauerhaft an ein einzelnes Spiel gekoppelt werden.

## Purpose

Genesis Engine ist für die generische Laufzeit- und Simulationsinfrastruktur der Genesis-Spielplattform verantwortlich:

- Engine-Loop und Lifecycle-Management
- ECS-basierte Simulation
- KI- und Kreaturenverhalten
- prozedurale Welt- und Umgebungssimulation
- Vorbereitung auf Multiplayer- und verifizierbare State-Integrationen
- Bereitstellung wiederverwendbarer Features für darauf aufbauende Spiele

`genesis-chronicles` ist ein Consumer der Engine. Spielspezifische Logik gehört in das jeweilige Spiel-Repository; generische Features werden nur über den vorgesehenen Feature-Promotion-Prozess in die Engine übernommen.

## Status

**Status:** `development` — Rebuild-Basis. Die dokumentierten Meilensteine und Gate-Kriterien sind in den kanonischen Status-/Roadmap-Dokumenten festgelegt.

Der Repository-Status darf nicht mit den Governance-Zuständen `APPROVED`, `AUDITED` oder `PRODUCTION_READY` gleichgesetzt werden. Eine dokumentierte Architekturentscheidung oder ein bestandenes Audit stellt für sich allein keinen Production-Release dar.

## Architecture

### Components

- `atc-genesis-engine` — Core-Engine-Loop, Lifecycle-Management, Pipeline und Subsystem-Orchestrierung.
- `atc-genesis-ecs` — High-Performance Entity Component System.
- `atc-genesis-creatures` — KI- und Kreaturen-Verhaltensmodellierung.
- `atc-genesis-world` — prozedurale Weltgenerierung, Terrain und Umgebungssimulation.

### Data Flow

```text
Input Events / Engine Tick
        ↓
atc-genesis-ecs
        ↓
atc-genesis-creatures
        ↓
atc-genesis-world
        ↓
Output State / Rendering
```

### Ecosystem Boundary

```text
ATCLang / ATC-VM / A-TownChain
              │
              │ Chain- und On-Chain-Funktionen
              ▼
       Genesis Integration
              │
              ▼
       Genesis Engine
              │
              ▼
     Genesis Chronicles / Games
```

Die Engine ist keine Blockchain, kein Kernel und kein Ersatz für ATC-VM oder ShivaCore. Chain-seitige Zustandsübergänge und Contracts bleiben an der dafür vorgesehenen Chain-/VM-Grenze.

## Features

- Modulare Rust-basierte Engine-Architektur
- High-Performance ECS Framework
- KI-native Kreatureneigenschaften und Verhaltensbäume
- Prozedurales Terrain- und Weltensystem
- Vorbereitung für Multiplayer und verifizierbare State-Integrationen
- Engine-/Game-Trennung über die Produktstrategie

## Repository Structure

```text
/
├── docs/       # Dokumentation, Spezifikationen und Repository-Standards
└── modules/    # Engine-Module
    ├── atc-genesis-engine
    ├── atc-genesis-ecs
    ├── atc-genesis-creatures
    └── atc-genesis-world
```

## Requirements

- Rust >= 1.75 / Cargo
- Python >= 3.11 für Tooling und Sync-Skripte
- Git >= 2.30

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/genesis-engine.git
cd genesis-engine
cargo build --workspace
```

## Usage

```bash
cargo run --package atc-genesis-engine
```

## Development

Entwicklung erfolgt nach den geltenden A-TownChain-Governance- und Repository-Standards. Commits müssen dem Conventional-Commit-Modell entsprechen.

Vor größeren Änderungen sind mindestens `STATUS.md`, `ARCHITECTURE.md`, `ROADMAP.md` und die relevanten Governance-Dokumente zu prüfen.

## Testing

```bash
cargo test --workspace
```

Testergebnisse sind als Evidence zu behandeln. Ein erfolgreicher lokaler Testlauf bedeutet nicht automatisch `AUDITED` oder `PRODUCTION_READY`.

## Security

Security Issues dürfen nicht öffentlich über GitHub Issues gemeldet werden. Sicherheitslücken sind über den offiziellen Security-Reporting-Prozess in `SECURITY.md` zu melden.

**Security class:** S1  
**Criticality:** low

## Documentation

- `docs/REPOSITORY_STANDARD.md` — Repository-Standard
- `docs/specs/GEN-PROD-001-PRODUCT-STRATEGY.md` — Produktstrategie
- `ARCHITECTURE.md` — technische Architektur
- `STATUS.md` — aktueller Projektstatus
- `ROADMAP.md` — Entwicklungs-Roadmap
- `a-townchain-os-docs` — zentrale Ökosystem-Dokumentation

## Governance

Das Repository folgt dem A-TownChain-Governance-Modell. Architektur- und Governance-Entscheidungen müssen über die vorgesehenen Entscheidungs- und Review-Prozesse erfolgen.

Canonical Standard-IDs werden ausschließlich über die Standards Registry und den dafür definierten Governance-Prozess vergeben. Die aktuelle Taxonomie verwendet Family-scoped IDs der Form `ATC-STD-Fxx-yyy`; bestehende Legacy-IDs bleiben historisch erhalten und werden nicht stillschweigend umnummeriert.

## Standards & Compliance

| Standard | Version | Verwendung |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | Governance Root |
| ATC-STD-README-001 | 1.0.0 | README-Struktur und Metadaten |
| ATC-STD-MD-001 | 1.0.0 | Markdown-Konformität |
| ATC-STD-201 | 1.0.1 | Repository Governance |
| ATC-STD-202 | 1.2.0 | Repository/Entwicklungsanforderungen |
| ATC-STD-203 | 1.0.1 | Security und Release Gates |

Die Tabelle dokumentiert die relevanten Standards; sie ist keine pauschale Behauptung, dass jeder Standardzustand dieses Entwicklungs-Repositories bereits `PRODUCTION_READY` ist.

## Roadmap

Siehe die kanonischen Quellen:

- `ROADMAP.md`
- `STATUS.md`
- zentrale Roadmap in `a-townchain-os-docs`
- GitHub Issues & Projects

## Contributing

Beiträge erfolgen über den definierten ATC-Governance-Prozess. Vor einem Merge müssen die für die Änderung relevanten Tests und Validatoren erfolgreich ausgeführt werden.

## License

Apache-2.0 — A-TownChain-Okosystems. Details siehe [`LICENSE`](LICENSE).

## Maintainers

**Organization:** A-TownChain-Okosystems  
**Maintainers:** ShivaCoreDev, aurora-superagent

## Repository Metadata

Maschinenlesbar: siehe HTML-Metadaten-Block im Header gemäß ATC-STD-README-001 §14.  
**Registry-ID:** `ATC-REPO-GAME-001`

## AI Agent Instructions

Für KI-Agenten:

1. Lies `STATUS.md`, `AGENT_MANIFEST.md`, `ARCHITECTURE.md` und `ROADMAP.md` vor größeren Änderungen.
2. Beachte die geltenden ATC-Standards und Repository-Governance.
3. Verwende Conventional Commits.
4. Führe nach Änderungen mindestens `cargo test --workspace` und die relevanten README-/Markdown-Validatoren aus.
5. Trenne deklarierte Zustände, Testergebnisse und Governance-Evidence strikt voneinander.
