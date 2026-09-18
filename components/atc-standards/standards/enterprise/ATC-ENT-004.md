---
standard:
  id: ATC-ENT-004
  title: "ATC-ENT-004 — Delegation & Berechtigungen Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-ENT-004 — Delegation & Berechtigungen Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-004 (Delegation & Berechtigungen Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Delegationsmodell

- Delegation erfolgt immer von einer Rolle an eine Rolle (nie an eine
  Person/direkt an einen Agenten ohne Rollenbindung).
- Eine Delegation ist schriftlich (DEC-Record), befristet und zweckgebunden;
  Ablauf = automatische Rückfallung.
- Delegationskette dokumentieren: `delegated_from: ROLE-CTO → ROLE-ARCH,
  scope: X, expires: YYYY-MM-DD`.

## 2. Nicht delegierbar (Invariante)

Approver-Rolle für normative Standards (§14.1: nur Owner), Mainnet-Release,
Verfassungsänderungen, Rollenänderungen an Approver-Positionen.

## 3. Berechtigungsmatrix

Menschliche Rollen: ENT-002 Rollen-Definition + Entgeltungsbereiche.
Agenten: Capability/Permission-Matrix aus AAS-002/003 — Delegation an
Agenten ist immer durch die Matrix gedeckelt (keine Erweiterung durch
Delegation; Ausnahme nur mit Owner-Freigabe + Frist wie AAS-003 §3).

## 4. Review

Delegationen werden je review_cycle geprüft; unbenutzte/abgelaufene
Delegationen stilllegbar (ARCHIVED) mit Audit-Vermerk (ENT-014).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Enterprise-Transaktionen: Buchungs- und Audit-Trails unveraenderbar (append-only); Betrugsschutz-Mechanismen bei Zahlungswegen; keine Zugangsdaten in Geschaeftsdaten; Transaktions-Integritaet vor und nach Konsens-Teilnahme gewaehrleistet.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-280ff (Security-Familie), ATC-ENT-001 · INFORMATIVE: Registry-Kategorie enterprise
