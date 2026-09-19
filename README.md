# A-TownChain Ecosystem

System-of-Systems-Integration des A-TownChain-Ökosystems: alle Kern-Systeme laufen
hier als ein Gesamtsystem zusammen. Vollständige Komponenten-Überführung per
`git subtree` **mit voller Historie** (siehe `MIGRATION_MANIFEST.yaml`).

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
| ShivaCore-Kernel | `components/atc-shivacore` | atc-shivacore |
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

The canonical runtime path is now explicitly verified as a connected chain:

`wallet/SDK transaction → mempool → proposer → block → network block validation → state transition → reward → validator vote → weighted finality → durable persistence → restart recovery → next block`.

Multi-node operation uses the canonical `atc-node` runtime and the `atc-blockchain` consensus/network boundary. Network handshakes bind chain identity and expose the accepting node's own height/tip; received blocks are validated before state adoption, and finality/evidence state is persisted for restart recovery.

A successful CI run is required before any production-readiness claim. Current development status remains **not production-ready** until all mandatory gates are green.

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
