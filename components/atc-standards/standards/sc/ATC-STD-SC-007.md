---
standard:
  id: ATC-STD-SC-007
  title: "ATC-STD-SC-007 — Smart Contract Upgrade Standard"
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

# ATC-STD-SC-007 — Smart Contract Upgrade Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Default ist Immutabilitaet. Upgradeable Contracts deklarieren Mechanismus, Admin und Timelock explizit und unterliegen zusaetzlichen Governance- und Security-Gates.

## Scope

Gilt fuer alle Contracts mit Upgradeability; Verzahnung mit ATC-STD-204 (Interface-Stabilitaet) und Governance-Standards.

## 1. Deklarationspflicht (REQ-SC-020)

Jeder Contract MUSS explizit deklarieren: `upgradeability: {enabled: false}` (immutable, Default) oder `upgradeability: {enabled: true, mechanism: proxy, admin: ATC-Governance, timelock: required}`.

## 2. Zusatz-Gates (REQ-SC-021)

Upgradeable Contracts BENOTIGEN zusaetzliche Governance- und Security-Gates: Storage-Collision-Pruefung (SC-003), Upgrade-Tests (SC-004 §1), Timelock-Pflicht.

## 3. Default: Immutable (REQ-SC-022)

Ohne explizite Deklaration gilt ein Contract als immutable; Upgradeability muss begruendet per DEC-Record (ATC-ENT-002) entschieden sein.



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-020 | Deklarationspflicht (§1) | MUSS |
| REQ-SC-021 | Zusatz-Gates (§2) | MUSS |
| REQ-SC-022 | Default immutable (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Upgrade-Mechanismen sind das hoechste Zentralisierungsrisiko; Admin- und Timelock-Regeln sind Governance-pflichtig.

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
