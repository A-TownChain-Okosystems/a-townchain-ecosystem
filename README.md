# A-TownChain Ecosystem

Das A-TownChain-Ökosystem ist **nicht nur eine Blockchain**. Es ist ein mehrteiliges Technologie- und Softwaresystem (System of Systems), dessen Blockchain **A-TownChain L1 nur eine eigenständige Domäne** darstellt. Zum Gesamtökosystem gehören außerdem ein Betriebssystem-/Kernel-Stack (GlobusOS/ShivaCore), eine Programmiersprache und Compiler-Toolchain (ATCLang), eine deterministische virtuelle Maschine (ATC-VM), eine AI-Plattform und Runtime (Aurora AI), eine eigenständige Engine (Genesis Engine) sowie Wallet-, SDK-, Identitäts-, Sicherheits-, ZKP-, Marketplace-, Entwickler- und Integrationsdienste.

Die Blockchain ist daher weder das gesamte Produkt noch die übergeordnete Instanz aller anderen Domänen. Die Domänen haben eigene Verantwortlichkeiten und Autoritätsgrenzen und interagieren über definierte Schnittstellen. Nicht jede Domäne muss von der Blockchain abhängig sein oder über sie ausgeführt werden.

Dieses Repository ist die System-of-Systems-Integrations- und Evidence-Control-Plane des Ökosystems. Importierte `git subtree`-Verzeichnisse sind Integrations-Snapshots; sie verschieben keine SSOT-Verantwortung und beweisen nicht, dass alle Komponenten synchron oder end-to-end verifiziert sind.

## Purpose

Dieses Repository koordiniert die Integration dieser Systeme, dokumentiert Cross-Repo-Verträge und aggregiert Evidence. Ein Import, ein grüner Einzel-Gate oder ein vorhandener Integrationstest beweist nicht, dass alle Komponenten als Gesamtsystem vollständig verifiziert sind.

## Scope

In-Scope: System-Integration, komponentenübergreifende Gates und der
Komponenten-Sync. Out-of-Scope: Fachliche Entwicklung — die findet in den
kanonischen Quell-Repos statt (SSOT-Disziplin, ATC-STD-000).

| System | Kanonischer Pfad | Quell-Repo (SSOT) |
|---|---|---|
| ATCLang (Programmiersprache) | `components/atclang` | atclang |
| ATC-VM | `components/a-townchain/components/vm` | a-townchain (L1-Monorepo) |
| Algorithmus/Konsens | `components/a-townchain/components/algorithm` | a-townchain (L1-Monorepo) |
| ShivaCore-Kernel | kein Mirror (seit 14.11.2026); Quelle `globus-os/modules/atc-shivacore/kernel` | globus-os (alleinige Kernel-SSOT; HEAD-Angabe im Komponenten-Repo live prüfen) |
| A-TownChain (L1) | `components/a-townchain` | a-townchain |
| Aurora AI | `components/aurora-ai` | aurora-ai |
| Genesis Engine | `components/genesis-engine` | genesis-engine |

## Architecture

### Independent product domains

The ecosystem is a multi-domain technology platform, not a blockchain-only product. A-TownChain L1 is one peer responsibility domain alongside the OS/kernel, AI, engine, language and VM domains. The following is a responsibility map, **not** a linear execution order, dependency chain or authority hierarchy:

```text
┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐
│ 01  GlobusOS        │  │ 02  A-TownChain      │  │ 03  Aurora AI       │
│ OS / Kernel / HAL   │  │ L1 / Consensus      │  │ AI Runtime / Agents │
└─────────────────────┘  └─────────────────────┘  └─────────────────────┘

┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐
│ 04  Genesis Engine  │  │ 05  ATCLang         │  │ 06  ATC-VM          │
│ Simulation / Tools  │  │ Language / Compiler │  │ Deterministic Exec. │
└─────────────────────┘  └─────────────────────┘  └─────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ 07  Ecosystem Services                                               │
│ Wallet · SDK · Identity · Standards · ZKP · Marketplace · Dev Tools  │
│ Integration and evidence control                                    │
└─────────────────────────────────────────────────────────────────────┘
```

The seven domains do not imply that all listed capabilities are implemented or connected. Each domain retains its own implementation source of truth. Cross-domain interaction is permitted only through explicit contracts, authorization boundaries and compatibility rules. The blockchain does not automatically own OS state, AI execution, Genesis simulation state, compiler semantics or every ecosystem service.

**Cross-cutting controls are not an eighth execution domain:** security, cryptography, identity, capability, governance, observability, interoperability and evidence span the domains. `atc-standards` owns normative standards; this repository owns integration architecture and evidence aggregation, not component implementation.

The detailed L0–L7 responsibility model, deterministic L1/L2/L3 runtime semantics and cross-layer control-plane contract are specified in `ARCHITECTURE.md`. That technical model is complementary to this product-domain map. Neither diagram should be interpreted as a mandatory linear execution sequence.

**Architecture principle:** Standalone First · Native Implementation · Ecosystem Second.

Details: `ARCHITECTURE.md`.

## Capabilities and limits

- Imported component trees are integration snapshots; their freshness must be checked against the canonical source SHA.
- Component-specific Determinism Gates use their own tools and allowlists; a pass in one component does not verify another.
- Governance checks validate repository policy and registry alignment; they do not prove production readiness.
- The seven-domain map records intended ownership and boundaries, not complete runtime connectivity.
- Architecture documentation is not implementation evidence; file presence is not proof of an executable capability.

## Strategic reverse-engineering audit

The ecosystem-wide strategic reverse-engineering program is tracked in:

[`docs/audits/KAI-OS-STRATEGIC-REVERSE-ENGINEERING-2026-11-27.md`](docs/audits/KAI-OS-STRATEGIC-REVERSE-ENGINEERING-2026-11-27.md)

The dossier defines source-level audit phases, exact-SHA evidence requirements, a provisional risk register and exit criteria. Its initial risks are hypotheses until independently revalidated against current repository HEADs and source symbols.

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem.git
# Komponenten einzeln bauen: siehe jeweiliges components/<name>/README.md
```

## L1 Integration Contract

The intended end-to-end runtime path to be verified is:

`wallet/SDK transaction → mempool → proposer → block → network block validation → state transition → reward → validator vote → weighted finality → durable persistence → restart recovery → next block`.

This path describes the blockchain vertical slice only. It is an integration contract, not the execution model of the entire ecosystem and not by itself proof that the whole chain is currently verified. Other domains have their own runtime and lifecycle paths. Each edge requires evidence tied to the exact source SHA and the relevant workflow run/job/step/logs. Multi-node behavior, chain-identity binding, block validation, finality and restart recovery must be reported separately where their evidence differs.

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

## Roadmap / open verification work

1. Reconcile the imported component snapshots against current canonical source SHAs and record any lag.
2. Complete the repository-registry decommission mapping for the migrated source repositories; do not archive or decommission sources without an explicit governance decision.
3. Run and record cross-component integration tests (ATCLang → canonical VM → node/chain runtime → consuming service) against exact SHAs.
4. Keep failed or pending integration and determinism gates visible; do not infer end-to-end verification from component-level green checks.

## Version

Versionierung nach ATC-STD-201/202, Registry-SSOT `atc-standards`;
aktuelle Version siehe CHANGELOG.md.

## License

Apache-2.0 (siehe LICENSE).

## Governance

Audit-Status und Evidence je Komponente in deren Quell-Repo; Governance-Gate
des Daches gegen `atc-standards/registry/repositories.yaml` (SCR-0127).
