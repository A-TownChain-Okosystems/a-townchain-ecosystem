---
standard:
  id: ATC-AAS-018
  title: "ATC-AAS-018 — Agent Audit Trail Standard"
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

# ATC-AAS-018 — Agent Audit Trail Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-018 (Agent Audit Trail Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Aktions-Protokollierung

Jede Agenten-Aktion (nicht nur Task-Abschluss) wird protokolliert:

```
timestamp · agent_id · task_id · repository · action · command ·
files_changed · result · risk_level · human_approval
```

## 2. Zweistufiges Modell

- **Aktionsprotokoll** (feingranular, je Operation): maschinenlesbar, in
  `.github/ai/audit/` als Event-Stream je Task
  (`.github/ai/audit/ATC-TASK-NNNN.events.yaml`), append-only.
- **Abschluss-Record AUD-NNN** (AI-DEV-009 §1, unverändert): Zusammenfassung
  bei COMPLETED/CANCELLED/BLOCKED/HANDOVER.

## 3. Regeln

- Beide Ebenen append-only; Korrektur nur per corrects:-Folgercord.
- risk_level: S0-S4 (ATC-STD-BUG-002); S0/S1-Aktionen triggieren
  Human-Approval-Pflicht (AAS-017 §2).
- Anschluss an LogChain/AuditTrail/DefenderGPT-Konzepte erfolgt über die
  Produkt-Repositories (atc-indexer, aurora-ai); dieser Standard definiert
  nur das Protokollformat.

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
