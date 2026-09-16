# Genesis Engine — Enterprise Engineering Baseline

**Status:** Development / Enterprise hardening in progress  
**Scope:** Workspace, runtime, networking, security, CI/CD and release engineering

## 1. Zielbild

Genesis Engine soll nicht nur funktionierenden Code liefern, sondern reproduzierbare, überprüfbare und wartbare Software-Artefakte. Der Enterprise-Baseline-Ansatz trennt Code Correctness, Supply-Chain-Security, Ownership/Governance und Runtime Assurance.

Der Repository-Status bleibt `development`. Ein grüner CI-Lauf allein ist kein `PRODUCTION_READY`-Nachweis.

## 2. CI Quality Gates

`.github/workflows/enterprise-ci.yml` definiert:

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo doc --workspace --no-deps`
- `cargo audit`
- GitHub Dependency Review für Pull Requests

Jobs besitzen Timeouts und minimale GitHub-Token-Rechte. Workflow-Ausführungen werden über Concurrency dedupliziert.

## 3. Dependency Governance

`.github/dependabot.yml` überwacht Cargo- und GitHub-Actions-Abhängigkeiten.

Ein versionierter `Cargo.lock` bleibt ein Ziel der Release-Härtung. Der aktuelle Security-Job erzeugt für den Audit-Lauf einen Lockfile; das ist noch kein Ersatz für einen versionierten Release-Lockfile.

## 4. Ownership

`.github/CODEOWNERS` definiert einen Repository-weiten Ownership-Boundary. Besonders geschützt sind CI, Security, Cargo-Metadaten, Runtime und Network.

## 5. Network Security Model

Der Netzwerkstack verwendet eine feste Binärrepräsentation aus Header und State. Die Security-Schicht prüft:

- maximale Paketgröße
- exakte Paketlänge
- Entity-Konsistenz zwischen Header und State
- Tick-Konsistenz
- monotone Sequenznummer
- monotone Tick-Reihenfolge
- Session-Zugehörigkeit über `PacketGuard`
- Peer-Kontext

`PacketGuard` ist jetzt vor dem ECS-Apply-Pfad explizit nutzbar. Der Runtime-Einstieg `receive_secure_network_packet` akzeptiert nur Pakete, die durch den Guard validiert wurden.

Der Runtime-Pfad wendet einen akzeptierten State nur auf eine bereits existierende ECS-Entity an. Unbekannte Entities werden nicht implizit aus Netzwerkdaten erzeugt.

## 6. Runtime Trust Boundary

```text
Transport
   ↓
Packet size boundary
   ↓
Packet decoding
   ↓
Session / peer / sequence validation
   ↓
ReplicatedState
   ↓
Entity existence policy
   ↓
ECS state application
   ↓
Simulation / rendering
```

Ein Netzwerkpaket darf niemals direkt eine beliebige ECS-Entity erzeugen oder Governance-/Chain-Zustand überschreiben.

## 7. Determinism

Positionen werden als Millimeterwerte übertragen. Netzwerksequenzen und Simulationsticks sind explizit und monoton. Snapshot- und Prediction-Strukturen unterstützen deterministische Replikation.

Weitere geplante Härtung:

- feste Timestep-Ausführung
- Cross-platform Determinism Tests
- definierte Floating-Point-Grenzen
- vollständiges Snapshot/Rollback
- reproduzierbare Release-Builds

## 8. Security Non-Goals des aktuellen Stands

Noch nicht als implementiert/verifiziert gelten:

- kryptographische Peer-Authentisierung
- verschlüsselte Transportverbindung
- echte UDP/QUIC-Implementierung
- Rate Limiting
- Bandwidth Budgets
- Connection Lifecycle Management
- Key Rotation
- DoS-Schutz auf Transportebene

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
