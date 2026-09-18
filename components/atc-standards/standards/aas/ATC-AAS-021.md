---
standard:
  id: ATC-AAS-021
  title: "ATC-AAS-021 — Agent Quality Standard"
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

# ATC-AAS-021 — Agent Quality Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-021 (Agent Quality Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. KPIs je Agent (aus Audit-Records ableitbar)

| KPI | Bedeutung |
|---|---|
| Task Success Rate | erfolgreiche Aufgaben / alle Aufgaben |
| Test Pass Rate | bestandene Tests / ausgeführte Tests |
| Regression Rate | verursachte Regressionen |
| Rework Rate | nachträgliche Korrekturen an eigener Arbeit |
| Security Findings | S0/S1-Findings des Agenten |
| Documentation Compliance | Sync-Pflicht-Erfüllung (AI-DEV-010) |
| False Claim Rate | unbelegte Behauptungen (AAS-012-Verstöße) |
| Human Escalation Rate | notwendige Eskalationen |

## 2. Erhebung

Automatisch aus Task-/Audit-Records (AI-DEV-004/009) und dem
Aktionsprotokoll (AAS-018); kein manuelles Scoring. Report je Agent
an den Owner (kadenz: monatlich, automatisierbar über Workflow).

## 3. Konsequenzen

False-Claim-Rate > 0 oder Security-Findings S0 → Capability-Review
(AAS-002 §1) und ggf. Deaktivierung (AAS-022 §3 Retirement).

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
