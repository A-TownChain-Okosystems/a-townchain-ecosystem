---
document_id: ATC-DOC-ALG-004
title: "Architecture — ATC Algorithm"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-08
standard: ATC-STD-MD-001
---

# Architecture — ATC Algorithm

## Übersicht

`atc-algorithm` ist ein historischer Integrationsspiegel des früheren `atc-algorithm`-Repos; der kanonische Algorithmuspfad liegt jetzt in `a-townchain/components/algorithm` der A-TownChain (Chain-ID 658467). Das frühere Design dokumentiert PoH/PoS/PoW. Es ist nicht als eingefrorener Konsensvertrag zu behandeln; der aktuelle Konsens bleibt SPEC-DRAFT bis Spec-Freeze, vollständiger Implementierung, Conformance und Exact-SHA-Evidence.

## Komponenten

| Component | Purpose | Required |
|---|---|---|
| `poh` | Proof of History Zeitstempel-Kette (SHA-256 Verifizierbare Ticks) | Yes |
| `pos` | Proof of Stake Validator-Gewichtung & Staking-Logik | Yes |
| `pow` | Proof of Work Schwierigkeitsanteil gegen Stake-Zentralisierung | Yes |
| `hybrid_engine` | Historische Hybrid-Auswahl/Fork-Choice/Finalitäts-Orchestrierung | Historical |

## Datenfluss & Abhängigkeiten

```text
Transaktionen -> PoH (Zeitstempel-Kette via SHA-256) -> Hybrid Selection (PoS/PoW) -> Finalisierung -> Block Ingestion
```

- **Inbound:** Transaktionen und Block-Header von `a-townchain` Node.
- **Outbound:** Verifizierte Ticks, Validator-Auswahl und Konsens-Entscheidungen an `a-townchain` und Execution Engine `atc-vm`.
