---
standard:
  id: ATC-ENT-007
  title: "ATC-ENT-007 — Eskalationsmanagement Standard"
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

# ATC-ENT-007 — Eskalationsmanagement Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-007 (Eskalationsmanagement Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Eskalationsstufen (Organisation)

E1 Ausführende Rolle (DEV/QA/Agent) → E2 Fachverantwortlicher
(ARCH/SEC/CISO) → E3 CTO/CEO → E4 Owner-Entscheidung (Verfassung §14.1).
Agenten-Eskalationen münden immer in E2+ (menschliche Rolle).

## 2. Eskalationsgründe (Pflichtliste)

Blocker ohne Selbstbehebung (AAS-020), Governance-Konflikte (AAS-013),
Autorisierungsbedarf (AAS-017 §2-Pflichtliste), Risiken ab Schwelle
(ENT-011, S0/S1), Konfliktinteressen (ENT-006).

## 3. Eskalations-Record

`{id: ESC-NNNN, from_role, to_role, reason, evidence, decision_ref
(DEC-NNNN), timestamp}` — append-only (ENT-014); Antwortpflicht mit
befristeter Reaktionszeit (S0/S1: 24h, S2/S3: 72h, sonst review_cycle).

## 4. Verhältnis zu AI-DEV-011

Agenten-Eskalationen nutzen unverändert AI-DEV-011; ESC-Records werden
bei Rollen-Eskalation (E2+) gebildet. Keine parallele Agenten-Eskalation
ohne menschliche Rolle.

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
