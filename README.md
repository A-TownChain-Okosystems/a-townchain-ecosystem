# A-TownChain Ecosystem

System-of-Systems-Integrations- und Evidence-Control-Plane des A-TownChain-Ökosystems. Importierte `git subtree`-Verzeichnisse sind Integrations-Snapshots; sie verschieben keine SSOT-Verantwortung und beweisen nicht, dass alle Komponenten synchron oder end-to-end verifiziert sind.

## Purpose

Ein Dach-Repository, das ATCLang, ATC-VM, ATC-Algorithm, ShivaCore-Kernel,
A-TownChain, Aurora AI und Genesis Engine als ein Gesamtsystem integriert,
bündelt und systemübergreifend verifiziert.

## Scope

In-Scope: System-Integration, komponentenübergreifende Gates und der
Komponenten-Sync. Out-of-Scope: Fachliche Entwicklung — die findet in den
kanonischen Quell-Repos statt (SSOT-Disziplin, ATC-STD-000).

| System | Kanonischer Pfad | Quell-Repo (SSOT) |
|---|---|---|
| ATCLang (Programmiersprache) | `components/atclang` | atclang |
| ATC-VM | `components/a-townchain/components/vm` | a-townchain (L1-Monorepo) |
| ATC-Algorithm | `components/a-townchain/components/algorithm` | a-townchain (L1-Monorepo) |
| ShivaCore-Kernel | `components/globus-os/modules/atc-shivacore/kernel` (Importpfad, falls vorhanden) | globus-os (kanonische Kernel-Quelle) |
| A-TownChain (L1, 13 Komponenten) | `components/a-townchain` | a-townchain |
| Aurora AI | `components/aurora-ai` | aurora-ai |
| Genesis Engine | `components/genesis-engine` | genesis-engine |

## Architecture

Schichten (unten → oben): ShivaCore-Kernel (TCB) → ATCLang → ATC-VM →
ATC-Algorithm/Node (A-TownChain L1) → Aurora AI (Policy/Agent-Ebene) →
Genesis Engine (Game-Plattform) → Anwendungen (Genesis Chronicles, Flagship).
Details: `ARCHITECTURE.md`.

## Features

- 5 Komponenten-Importe mit voller Git-Historie (subtree, kein Squash)
- Komponenten-eigene Determinism-Gates mit eigenen Allowlists
- Governance-Audit gegen die Org-Registry (ATC-STD-201/202/203)
- System-Mapping der 7 Kern-Systeme auf kanonische Pfade

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem.git
# Komponenten einzeln bauen: siehe jeweiliges components/<name>/README.md
```

## L1 Integration Contract

The intended end-to-end runtime path to be verified is:

`wallet/SDK transaction → mempool → proposer → block → network block validation → state transition → reward → validator vote → weighted finality → durable persistence → restart recovery → next block`.

This path is an integration contract, not by itself proof that the whole chain is currently verified. Each edge requires evidence tied to the exact source SHA and the relevant workflow run/job/step/logs. Multi-node behavior, chain-identity binding, block validation, finality and restart recovery must be reported separately where their evidence differs.

A successful relevant CI run is required before any verification claim, and all applicable release gates are required before production-readiness claims. Current development status remains **not production-ready** until mandatory gates and exact-SHA evidence support the claim.

## Testing

`ATC Test Suite` und `Determinism Gate` laufen pro Komponente mit deren
eigenen Tools und Allowlists (atclang seit 2026-09-17 Rust-only:
fmt/clippy/cargo test im Manifest-Pfad crates/atc-core; a-townchain ruff
kern-scoped; Rust-Komponenten statisch). `Repository Governance` auditiert
das Dach gegen die Org-Registry.

## Development

Fachliche Änderungen gehören ins Quell-Repo (SSOT). Sync in den Umbrella:
`git subtree pull --prefix=components/<name> <repo> main`. Direkte
Fach-Commits im Umbrella sind zu unterlassen.

## Security

Criticality: critical (aggregiert alle Kern-Systeme). Security-Policy je
Komponente siehe `components/<name>/SECURITY.md`; Meldewege siehe SECURITY.md.

## Roadmap

1. PR genesis-engine#13 mergen → Genesis-Komponente grün, danach subtree pull
2. Registry-Decommission-Mapping (repositories.yaml) für die 13 L1-Quell-Repos
3. Systemübergreifende Integrationstests (ATCLang → VM → Node → Engine)

## Version

Versionierung nach ATC-STD-201/202, Registry-SSOT `atc-standards`;
aktuelle Version siehe CHANGELOG.md.

## License

Apache-2.0 (siehe LICENSE).

## Governance

Audit-Status und Evidence je Komponente in deren Quell-Repo; Governance-Gate
des Daches gegen `atc-standards/registry/repositories.yaml` (SCR-0127).
