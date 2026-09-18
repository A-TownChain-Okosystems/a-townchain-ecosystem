---
standard:
  id: ATC-STD-AI-DEV-006
  title: "ATC-STD-AI-DEV-006 — AI Decision & Action Standard"
  version: "1.0.0"
  status: approved
  category: ai-dev
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

# ATC-STD-AI-DEV-006 — AI Decision & Action Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §9 (OBSERVATION vs. DECISION), §8 (Actions)

## Abstract

ATC-STD-AI-DEV-006 (AI Decision & Action Standard) — OBSERVATION/DECISION/reason-Trennung, ACT-NNN-Struktur, Next-Action-Pflicht, Entscheidungsverbund ohne autonome Aenderung); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Trennungsgrundsatz

Jede Entscheidung besteht aus drei getrennten Feldern:
- `observation`: Verifizierter Sachverhalt (mit Fundstelle, AI-DEV-005 §2)
- `decision`: Die daraus abgeleitete Handlung
- `reason`: Warum — mit REQ-/Standard-/Task-Bezug

Eine Modellannahme wird nie als Fakt behandelt; Fakt und Annahme sind
unterschieden (`based_on: evidence` vs. `based_on: ASSUMPTION-ANNN`).

## 2. Action-Struktur (ACT-NNN)

`{id, based_on: [F-NNN], action_type, target, objective, reason,
depends_on: [ACT-NNN], required_before, completion_condition}` —
IDs fortlaufend je Task, nie wiederverwendet.

## 3. Next-Action-Pflicht

Nach jeder abgeschlossenen Aktion existiert eine explizite `next_action`
(AI-DEV-001 §8) oder der Task ist COMPLETED mit Audit-Record (AI-DEV-009).
Implizite "weiter sehen wir dann" ist unzulässig.

## 4. Entscheidungsverbund

Autonome Code-Änderungen ohne dokumentierte Entscheidung+reason sind
Governance-Verstoß. Bei Unsicherheit: ASSUMPTION-Register + ggf. Eskalation
an Owner (AI-DEV-011 §2).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-203, ATC-STD-AI-DEV-001 · INFORMATIVE: Roadmap MK8 (Security), Model-Registry
