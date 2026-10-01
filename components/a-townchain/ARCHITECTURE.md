---
document_id: ATC-DOC-ARC-ATCH-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — a-townchain

## Übersicht

Diese Datei ist eine Integrations-/Migrationskopie und keine kanonische Implementierungsquelle. Der kanonische `a-townchain`-Core ist Protocol Layer L2. ATC-VM ist Protocol Layer L3 unter `a-townchain/components/vm`. Chain-ID 658467 ist die aktuelle kanonische numerische Chain-ID. Eine konkrete Consensus-Variante ist erst nach Standard-Freeze und Exact-SHA-Conformance kanonisch.

## Subsysteme

1. **Chain Protocol Core:** Block-/Transaktionsmodell, Chain-ID 658467, Validierungsregeln (`modules/`).
2. **Konsens-Bindung:** `atc-algorithm` ist das kanonische Consensus-Target. Die README des Algorithmus-Repositories weist den Konsensus aktuell als `R1-SKELETON / DEVELOPMENT` und P0-release-blockierend aus; daher darf diese Integrationskopie keine konkrete Variante als bereits produktionsbereit darstellen.
3. **ZKP-Integration:** Proof-Verifikation — kryptografische Primitive kanonisch in `atc-zkp`.
4. **On-Chain-Governance:** Abstimmungs- und Parameter-Änderungsmechanismen.
5. **Chain-DNS:** Namensauflösung auf der Chain.
6. **Kernel-Service (AD-012):** Dienste-Integration in das KAI-OS.

## Verantwortungsgrenzen

- `atc-node` betreibt das Protokoll (Full-Node-Binary/Runtime) — hier wird es definiert.
- `atc-algorithm` implementiert den Konsens — dieses Repo bindet ihn.
- `a-townchain/components/vm` führt Contracts aus — das Protokoll definiert die Ausführungssemantik.

## Registry-Einordnung

| Property | Value |
|---|---|
| Layer | L2 — Blockchain Core |
| Criticality | C1 |
| Security-Klasse | S4 |
| Maturity | R-Level laut `.atc/repository.yaml` · Statusleiter in `.atc/evidence/evidence.yaml` (SCR-0080) |
| Canonical | a-townchain (Chain-Protokoll, AD-012 Kernel-Service) |
| Domäne | domaene laut registry/repositories.yaml |

> Ehrlichkeitsregel: CLAIMED ≠ PASS · IMPLEMENTED ≠ VERIFIED — der verbindliche Implementierungsstand
> liegt ausschließlich in `.atc/evidence/evidence.yaml`, nicht in dieser Spezifikation.
