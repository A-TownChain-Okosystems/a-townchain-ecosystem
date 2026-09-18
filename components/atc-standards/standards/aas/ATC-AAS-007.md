---
standard:
  id: ATC-AAS-007
  title: "ATC-AAS-007 — Agent Task Standard"
  version: "1.0.0"
  status: approved
  category: aas
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

# ATC-AAS-007 — Agent Task Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-007 (Agent Task Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Feldstruktur (Zusatzfelder zu AI-DEV-004 §1)

```yaml
task_id: ATC-TASK-00427
title: ...
objective: ...                      # messbares Ziel
repository: ...                     # Ziel-Repo (muss im Scope, ATC-AAS-004)
requirements: [REQ-...]             # Verweis auf Anforderungsquellen
dependencies: [ATC-TASK-NNNN]
allowed_files: []                   # Verfeinerung von ATC-AAS-004
forbidden_files: []
acceptance_criteria: []             # prüfbare Kriterien
test_requirements: {}                # Mindest-Validierung, ATC-AAS-011
documentation_requirements: {}      # Sync-Pflicht, AI-DEV-010
status: <AI-DEV-004 §2 Lifecycle>
```

## 2. Regeln

- Task-ID-Format und Allokation unverändert AI-DEV-004 §3 (fortlaufend,
  nie wiederverwendet, ATC-STD-000 §37).
- Datei-Arbeitsbereich einer Task ist die Schnittmenge aus Scope
  (AAS-004) und allowed_files; forbidden_files hat Vorrang.
- COMPLETED nur bei erfüllten acceptance_criteria UND grüner Validierung
  UND Audit-Record (AI-DEV-009).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Agenten-spezifisch: Agent-Identitaet via AGENT_MANIFEST verifizierbar; Permissions nach Least-Privilege; Delegationen dokumentiert und widerrufbar; Zugangsdaten ausschliesslich als $ENV-Platzhalter (ATC-STD-203); Agent-Kommunikation authentifiziert, nie anonym.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-AAS-001 · INFORMATIVE: AGENT_MANIFEST.md v3.1.7, Registry-Kategorie aas
