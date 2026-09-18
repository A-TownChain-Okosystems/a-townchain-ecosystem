---
standard:
  id: ATC-AAS-020
  title: "ATC-AAS-020 — Agent Failure Standard"
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

# ATC-AAS-020 — Agent Failure Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-020 (Agent Failure Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Fehlerzustände

```
FAILED             — Ausführung fehlgeschlagen
BLOCKED            — extern/philosophisch blockiert (Finding, Konflikt, Frist)
NEEDS-HUMAN        — Owner-Entscheidung zwingend (AI-DEV-011 §2)
ROLLBACK-REQUIRED  — Änderung hat System instabil gemacht (AAS-009 Rollback)
```

## 2. Schleifenvermeidung (Pflicht)

```yaml
max_attempts: 3
after_3_failures:
  - create_finding      # F-NNN mit Evidenz (AI-DEV-005)
  - stop_task           # Status BLOCKED, keine Wiederholung
  - request_human_review
```

Keine Endlosschleifen: Jeder Wiederholungsversuch ist im Task-Record
protokolliert; nach `max_attempts` ist Autonomie für diese Task beendet.

## 3. ROLLBACK-REQUIRED

Sofortige Eskalation (S1), Ausführung des dokumentierten Rollbacks
(AAS-009 §1 ROLLBACK) nach Owner- oder Rollback-Standard-Freigabe,
Finding mit Ursachenanalyse (AI-DEV-006: observation/decision/reason).

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
