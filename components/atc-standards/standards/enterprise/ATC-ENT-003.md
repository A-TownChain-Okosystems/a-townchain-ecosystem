---
standard:
  id: ATC-ENT-003
  title: "ATC-ENT-003 — Entscheidungsmanagement Standard"
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

# ATC-ENT-003 — Entscheidungsmanagement Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-003 (Entscheidungsmanagement Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Kernregel

**Jede kritische Entscheidung muss einen Verantwortlichen, einen Status,
eine Begründung und eine nachvollziehbare Historie besitzen.** Keine
kritische Architekturentscheidung darf „irgendwo im Chat verschwinden".

## 2. Decision Record (DEC-NNNN)

```
DEC-0001
├── Problem
├── Optionen
├── Bewertung
├── Entscheidung
├── Verantwortlicher (ROLE-XXX)
├── Datum
├── Auswirkungen
├── Abhängigkeiten (DEC-/SCR-/Standard-Referenzen)
└── Review-Date
```

## 3. Status-Lifecycle

PROPOSED → UNDER_REVIEW → APPROVED | REJECTED → SUPERSEDED → ARCHIVED.

## 4. Verhältnis zu bestehenden Instrumenten

- Freigaben normativer Standards: unverändert ATC-STD-000 §9 mit
  approval/APPROVAL-DECISION-*.md (diese SIND DEC-Records in
  Kanon-Form; künftig zusätzlich DEC-ID vergeben).
- Bestands-Entscheidungen AD-016..AD-046 (Owner-Mandate, AGENT_MANIFEST)
  bleiben gültig und werden als DEC-Records anerkannt (grandfathered,
  keine Rückmigration erzwungen).
- SCR-XXXX (Standard-Änderungen): formale Änderungsspur bleibt SCR;
  DEC deckt Organisations-/Produkt-/Architektur-Entscheidungen ab.
- Priorisierungskonflikte zwischen Entscheidungen → AAS-013.

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
