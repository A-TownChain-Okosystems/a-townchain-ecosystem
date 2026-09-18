---
standard:
  id: ATC-ENT-012
  title: "ATC-ENT-012 — Wissensmanagement & Consistency-Gate Standard"
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

# ATC-ENT-012 — Wissensmanagement & Consistency-Gate Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-012 (Wissensmanagement & Consistency-Gate Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Kernregel

**Code, Standards, Wiki, Architektur und Roadmap dürfen nicht
auseinanderlaufen.** (AI-DEV-010 §1 als Unternehmensgrundsatz)

## 2. Consistency Gate (Release-blockierend)

```
CODE CHANGE
  ├── Wiki  ├── Architecture  ├── Standards  ├── API Specification
  ├── Roadmap  └── Changelog
        ↓
  CONSISTENCY CHECK → PASS: RELEASE | FAIL: BLOCK
```

FAIL blockiert den Release (Merge-Gate AI-DEV-007 §6); Abweichungen
werden synchronisiert (SYNC-NNNN, BUG-004) oder als BLOCKED mit
Finding dokumentiert. Kein „Release jetzt, Doku später".

## 3. Wissensarchive (SSOT-Kette)

registry/*.yaml (Standards, Abhängigkeiten, Versionen, Org, Repos,
Risiken) = Maschinen-SSOT → abgeleitete Sichten (STANDARDS_REGISTRY.md,
Wiki, Notion, Roadmap). Generierte Sichten sind als abgeleitet markiert
(F-005-Präzedenz); Änderungen an Sichten ohne SSOT-Update sind ungültig.

## 4. Automatisierung

Wiederkehrende Konsistenzprüfungen sind als Workflow/Automatisierung
umzusetzen (AI-DEV-010 §4); Ausnahmen dokumentiert begründen.

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
