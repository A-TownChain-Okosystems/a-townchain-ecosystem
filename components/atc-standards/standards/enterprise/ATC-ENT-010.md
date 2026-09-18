---
standard:
  id: ATC-ENT-010
  title: "ATC-ENT-010 — Enterprise Change Management Standard"
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

# ATC-ENT-010 — Enterprise Change Management Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-010 (Enterprise Change Management Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Change-Pipeline (Pflichtreihenfolge)

```
CHANGE REQUEST → IMPACT ANALYSIS → TECHNICAL REVIEW → SECURITY REVIEW →
APPROVAL → IMPLEMENTATION → TEST → AUDIT → DOCUMENTATION UPDATE → RELEASE
```

Keine Ausnahmeschritte; Parallelisierung nur zwischen den Reviews.

## 2. Änderungsklassen

- Standards: unverändert SCR (ATC-STD-000 §19-33) — SCR bleibt die
  formale Änderungsspur für normative Texte.
- Organisation (Rollen, Einheiten, Zuständigkeiten): ENT-010 mit
  DEC-Record (ENT-003) + Registry-Update.
- Architektur/Produkt: ENT-010 mit Impact auf Dependency-Graph
  (ATC-STD-204) + betroffene Interface-Tests (IFC-NNNN).
- Richtlinien: ENT-010 §-Pipeline mit POL-Update (ENT-005).

## 3. Impact-Analyse (Pflichtinhalte)

Betroffene Repos (ENT-009), Standards, Interfaces (ATC-STD-204),
Risiken (ENT-011), Abhängigkeiten, Migrations-/Rollback-Plan
(Rollback-Pflicht nach AAS-009 §1).

## 4. Notfall-Änderung (Emergency)

Wie Verfassung §32: nachgelagerte Dokumentation binnen 72h, volle
Pipeline rückwirkend; Owner-Benachrichtigung sofort. Kein Stillstand
kritischer Systeme erzwingbar.

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
