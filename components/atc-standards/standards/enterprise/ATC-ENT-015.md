---
standard:
  id: ATC-ENT-015
  title: "ATC-ENT-015 — Qualitätsmanagement & Definition of Done Standard"
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

# ATC-ENT-015 — Qualitätsmanagement & Definition of Done Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-015 (Qualitätsmanagement & Definition of Done Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Definition of Done (Unternehmens-DoD)

```
CODE        ✓ implementiert (Review nach AI-DEV-007)
TEST        ✓ Unit-Tests · ✓ Integration-Tests (L1-L4 je AAS-011)
SECURITY    ✓ Security Scan · ✓ Dependency Scan (ATC-STD-203)
DOKU        ✓ Wiki ✓ README ✓ Architektur aktualisiert (ENT-012)
GOVERNANCE  ✓ Change Request/DEC ✓ Approval (ENT-003/010)
RELEASE     ✓ Version ✓ Changelog ✓ Audit-Record (ENT-014)
```

## 2. Regeln

- Nicht erfüllter DoD-Punkt = nicht Done: kein COMPLETED (AI-DEV-004),
  kein Release (Consistency Gate ENT-012 §2).
- DoD gilt für menschliche und Agenten-Beiträge gleichermaßen; Agenten
  liefern für jeden Haken Evidenz (AAS-010), Menschen deklarieren mit
  Verweis (Commit/PR/Record).
- Abweichungen (z.B. reine Doku-Änderung ohne Tests) sind im Task-Record
  als solche deklariert (AI-DEV-008 §1).

## 3. Review

DoD-Konformität wird stichprobenhaft auditiert (ROLE-AUDITOR, ENT-014 §4);
Missachtung = Finding + KPI-Wirkung (ENT-013 §1 Rework/Regression Rate).

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
