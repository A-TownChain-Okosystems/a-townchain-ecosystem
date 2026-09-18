---
standard:
  id: ATC-STD-SC-004
  title: "ATC-STD-SC-004 — Smart Contract Testing Standard"
  version: "1.0.0"
  status: approved
  category: sc
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  applies_to: "Alle Smart Contracts des A-TownChain-Oekosystems"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-SC-004 — Smart Contract Testing Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Kein Production-Deployment ohne automatisierte Tests: Unit-, Integration-, Fuzz-, Invariant-, Negative-, Security-, Upgrade- und Gas-Tests sowie Testnet-Tests — mit expliziten Invarianten (z. B. totalSupply <= maxSupply).

## Scope

Gilt fuer alle Contracts vor SC-G3/G4/G8; Teststrategie je Contract dokumentiert.

## 1. Testarten (REQ-SC-011)

Mindestens MUSS getestet werden: Unit Tests, Integration Tests, Fuzz Tests, Invariant Tests, Negative Tests, Security Tests, Upgrade Tests, Gas Tests, Testnet Tests.

## 2. Invarianten (REQ-SC-012)

Kerninvarianten MUESSEN explizit als Invariant-Tests codiert sein, z. B. `totalSupply <= maxSupply`, `lockedBalance <= userBalance`, `bridgeMinted <= bridgeDeposited`.

## 3. Deployment-Blocker (REQ-SC-013)

Ohne bestandene SC-G3 (Unit) und SC-G4 (Fuzz/Invariant) ist ein Deployment UNZULAESSIG.



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-011 | Testarten (§1) | MUSS |
| REQ-SC-012 | Invariant-Tests (§2) | MUSS |
| REQ-SC-013 | Deployment-Blocker (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Testvektoren und Ergebnisse sind AUD-pflichtig (AI-DEV-009, SC-020).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) |

## References

- **NORMATIVE:** ATC-STD-000 (Verfassung), ATC-STD-201/202, ATC-STD-203
  (Security), ATC-STD-204 (Dependency & Interface), ATC-STD-SC-001
  (Familien-Hauptstandard)
- **INFORMATIVE:** ATC-AAS-001..025 (Agenten-Standards), AI-DEV-001..012,
  contracts/registry/ (Contract Registry)
