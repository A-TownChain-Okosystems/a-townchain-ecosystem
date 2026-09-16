# Genesis Engine — Enterprise Engineering Baseline

**Status:** Development / Enterprise hardening in progress  
**Scope:** Workspace, runtime, networking, security, CI/CD and release engineering

## 1. Zielbild

Genesis Engine soll nicht nur funktionierenden Code liefern, sondern reproduzierbare, überprüfbare und wartbare Software-Artefakte. Der Enterprise-Baseline-Ansatz trennt dabei vier Ebenen:

1. **Code correctness** — Formatierung, Compilation, Tests und Clippy.
2. **Supply-chain security** — Dependency Review und RustSec-Auditing.
3. **Ownership & governance** — CODEOWNERS und geschützte Security-/Runtime-Bereiche.
4. **Runtime assurance** — deterministische Netzwerk-, ECS- und Simulationspfade.

Der Repository-Status bleibt `development`. Ein grüner CI-Lauf allein ist kein `PRODUCTION_READY`-Nachweis.

## 2. CI Quality Gates

`.github/workflows/enterprise-ci.yml` definiert folgende Gates:

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo doc --workspace --no-deps`
- `cargo audit`
- GitHub Dependency Review für Pull Requests

Jobs besitzen Timeouts und minimale GitHub-Token-Rechte. Workflow-Ausführungen werden pro Branch/Workflow über Concurrency dedupliziert.

## 3. Dependency Governance

`.github/dependabot.yml` überwacht Cargo- und GitHub-Actions-Abhängigkeiten. Security-relevante Dependency-Änderungen sollen als Review-pflichtige Änderungen behandelt werden.

Ein versionierter `Cargo.lock` ist für reproduzierbare Releases weiterhin ein Ziel der Release-Härtung. Bis dahin erzeugt der Security-Job den Lockfile reproduzierbar für den Audit-Lauf.

## 4. Ownership

`.github/CODEOWNERS` definiert einen Repository-weiten Ownership-Boundary. Besonders geschützt sind:

- `.github/`
- `SECURITY.md`
- `Cargo.toml`
- `Cargo.lock`, sobald versioniert
- `modules/atc-genesis-runtime/`
- `modules/atc-genesis-network/`

## 5. Network Security Model

Der Netzwerkstack besitzt eine feste Binärrepräsentation aus Header und State. Pakete werden auf folgende Eigenschaften geprüft:

- exakte Paketlänge
- Entity-Konsistenz zwischen Header und State
- Tick-Konsistenz
- monotone Sequenznummer
- monotone Tick-Reihenfolge
- Session-Zugehörigkeit über `PacketGuard`
- Peer-Kontext

Der Runtime-Pfad wendet einen akzeptierten State nur auf eine bereits existierende ECS-Entity an. Unbekannte Entities werden nicht implizit aus Netzwerkdaten erzeugt.

## 6. Runtime Trust Boundary

Die Verarbeitung ist bewusst in Stufen geteilt:

```text
Transport
   ↓
Packet decoding
   ↓
PacketGuard / ordering validation
   ↓
ReplicatedState
   ↓
Entity existence / ownership policy
   ↓
ECS state application
   ↓
Simulation / rendering
```

Ein Paket darf niemals direkt eine beliebige ECS-Entity erzeugen oder Governance-/Chain-Zustand überschreiben.

## 7. Determinism

Für reproduzierbare Simulation werden Integer-basierte Netzwerkwerte verwendet. Positionen werden in Millimetern übertragen. Netzwerksequenzen und Simulationsticks sind explizit und monoton.

Weitere Enterprise-Härtung umfasst:

- feste Timestep-Ausführung
- deterministische Sortierung
- Snapshot-Buffers
- Prediction-/Rollback-Infrastruktur
- Cross-platform Determinism Tests
- definierte Floating-Point-Grenzen

## 8. Security Non-Goals des aktuellen Stands

Der aktuelle Netzwerktransport ist noch keine vollständige Production-Network-Security-Lösung. Es fehlen insbesondere:

- kryptographische Peer-Authentisierung
- verschlüsselte Transportverbindung
- echte UDP/QUIC-Implementierung
- Rate Limiting
- Bandwidth Budgets
- Connection Lifecycle Management
- Key Rotation
- DoS-Schutz auf Transportebene

Diese Punkte dürfen nicht als implementiert betrachtet werden.

## 9. Release Readiness

Ein Release gilt erst nach expliziter Evidence als verifiziert. Mindest-Evidence:

1. CI Quality Gates bestanden.
2. Security/Dependency Gates bestanden.
3. reproduzierbarer Build nachgewiesen.
4. relevante Integrationstests bestanden.
5. Security Review abgeschlossen.
6. Release-Artefakte und Checksums erzeugt.
7. Governance- und Lizenzprüfung abgeschlossen.

`development`, `verified`, `release-candidate` und `production-ready` sind getrennte Zustände.

## 10. Evidence Principle

> No Evidence, No Trust.

Dokumentation beschreibt Absicht und Architektur. CI-Ergebnisse, Testreports, Security-Scans und Release-Artefakte belegen den tatsächlichen Zustand.
