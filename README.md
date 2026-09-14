<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-CORE-001
  name: atc-shivacore
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S4
  criticality: C1
-->

# ATC ShivaCore

> Capability-basierter Rust/no_std-Microkernel als sicherheitskritische Kernel-Basis für GlobusOS. ShivaCore ist kein Blockchain-, AI- oder Game-Layer.

**Project:** `atc-shivacore`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0`

## Role and Scope

ShivaCore stellt den wiederverwendbaren Kernel-/TCB-Baustein des Ökosystems bereit. Der Kernel konzentriert sich auf Isolation, Capability-basierte Autorisierung, Scheduling, Memory/IPC und die für das jeweilige Target erforderlichen Low-Level-Primitiven.

**In scope:**
- Rust/no_std-Microkernel und Capability-Schutzmodell
- CSpace/Capability Management
- Scheduling und Kernel-Lifecycle
- Memory Management und IPC
- Hardware-Abstraktion und Low-Level-Netzwerkprimitive, soweit Teil des Kernel-Scopes
- Boot-/Target-Unterstützung für die tatsächlich implementierten Architekturen

**Out of scope:**
- GlobusOS-Userspace und Systemdienste (`globus-os`)
- Aurora AI (`aurora-ai`)
- A-TownChain-Protokoll und Chain-State
- ATCLang-Contracts und ATC-VM
- Game-/GameFi-Anwendungen (`genesis-engine`, `genesis-chronicles`)

## Status

`development` bedeutet Entwicklungsstand; vorhandene Tests oder Audits sind kein automatischer Nachweis für `PRODUCTION_READY`.

Der Repository-Audit und die vorhandenen Testzahlen sind als Evidence für den jeweiligen Commit zu verstehen und müssen vor Release erneut verifiziert werden. Chain-Identität gehört zur Blockchain-/Netzwerkebene und wird nicht durch eine im Kernel-README behauptete feste Chain-ID definiert.

## Architecture

### Core components

- **CSpace:** Capability-basierte Rechteverwaltung und Schutzgrenzen.
- **Scheduler:** Kernel-Scheduling für die unterstützten Ausführungskontexte.
- **Memory & IPC:** Speicherverwaltung und Inter-Process Communication.
- **HAL / Target layer:** Hardware- und Architekturabstraktion.
- **Kernel networking primitives:** Low-Level-Komponenten; Protokollsemantik bleibt außerhalb des Microkernel-TCB, soweit nicht ausdrücklich spezifiziert.

### Execution boundary

```text
Hardware / Boot
      ↓
ShivaCore Microkernel / TCB
      ↓
GlobusOS kernel-facing services
      ↓
GlobusOS userspace / Aurora / applications
```

Blockchain-Ausführung folgt einer getrennten Grenze:

```text
ATCLang → ATC-VM → A-TownChain
```

ShivaCore stellt dafür nicht die Chain-Semantik bereit.

## Repository Structure

```text
.
├── .atc/                # ATC-Repository-Metadaten
├── .github/             # GitHub Workflows & Dependabot
├── docs/                # Dokumentation
├── modules/             # Kernel- und Tool-Module
├── AGENT_MANIFEST.md    # Agent Manifest
├── AGENTS.md            # AI Agent Instructions
├── ARCHITECTURE.md      # Kernel-Architektur
├── CHANGELOG.md         # Änderungshistorie
├── CODEOWNERS           # Repository-Eigentümer
├── CONTRIBUTING.md      # Beitragsrichtlinien
├── GOVERNANCE.md        # Governance-Regeln
├── LICENSE              # Apache-2.0
├── README.md            # Repository-Einstiegspunkt
├── ROADMAP.md           # Entwicklungs-Roadmap
├── SECURITY.md          # Sicherheitsrichtlinie
└── STATUS.md            # Repository-Status
```

## Requirements

- Rust 1.98.1 oder neuer, sofern der aktuelle Workspace dies voraussetzt
- Cargo und Build-Essentials
- unterstützte `x86_64-unknown-none` bzw. `aarch64-unknown-none` Targets, sofern vom jeweiligen Modul aktiviert
- Python 3.10+ für vorhandene Workspace-Hilfsskripte

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-shivacore.git
cd atc-shivacore
cargo build --workspace
```

## Usage

Für die vorhandene Boot-/Simulator-Konfiguration:

```bash
cargo run --bin boot --manifest-path modules/atc-shivacore/boot/Cargo.toml
```

Der konkrete Target- und Boot-Pfad ist vom aktuellen Workspace-Zustand abhängig.

## Testing

```bash
cargo test --workspace
```

Testergebnisse müssen für den jeweiligen Commit aus CI bzw. der lokalen Ausführung übernommen werden. Historische Testzahlen werden nicht als dauerhaft gültiger Zustand im README garantiert.

## Development

- Änderungen folgen `ATC-STD-000` und dem aktuellen ATC-Governance-Prozess.
- Architekturänderungen mit TCB-Auswirkung benötigen dokumentierte Governance-/Review-Evidence.
- Conventional Commits verwenden.
- Integration in `a-townchain-os` ist eine Integrationsaufgabe; ShivaCore bleibt als wiederverwendbarer Kernel eigenständig.

## Security

ShivaCore ist sicherheitskritische Infrastruktur. Sicherheitslücken nicht öffentlich über GitHub Issues veröffentlichen; den in `SECURITY.md` definierten Disclosure-Prozess verwenden.

## Documentation

- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`STATUS.md`](STATUS.md)
- [`SECURITY.md`](SECURITY.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)

## Governance

Das Repository unterliegt `ATC-STD-000` und den jeweils freigegebenen ATC-Standards. Standard-IDs werden ausschließlich über Registry/Governance vergeben. Neue family-scoped IDs verwenden das Format `ATC-STD-F{family}-{sequence}`; historische IDs bleiben als Legacy-Referenzen erhalten und werden nicht stillschweigend umnummeriert.

## Standards & Compliance

Anwendbare Standards sind im Repository und in der kanonischen ATC-Standards-Registry zu prüfen. Eine README-Tabelle mit `APPROVED` darf nicht mit einem Implementierungs-, Audit- oder Produktionsstatus gleichgesetzt werden.

## License

Apache-2.0. Siehe [`LICENSE`](LICENSE).

## Maintainers

- **Organization:** A-TownChain-Okosystems
- **Kernel project:** ShivaCore Core Team

## AI Agent Instructions

Vor Änderungen mindestens `AGENTS.md`, `AGENT_MANIFEST.md`, `ARCHITECTURE.md`, `STATUS.md` und `ROADMAP.md` prüfen. Änderungen am TCB, Capability-Modell, Bootpfad oder Sicherheitsgrenzen benötigen besonders sorgfältige Tests und Governance-Evidence.
