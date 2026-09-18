---
standard:
  id: ATC-STD-AI-DEV-011
  title: "ATC-STD-AI-DEV-011 — Human Approval & Escalation Standard"
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

# ATC-STD-AI-DEV-011 — Human Approval & Escalation Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §13 (human_review), ATC-STD-000 §14.1 (Rollen)

## Abstract

ATC-STD-AI-DEV-011 (Human Approval & Escalation Standard) — Human-Decision-Pflichtfelder (Task+PR), Eskalationsstufen a-d, Genehmigungsformen (Mandat/Vermerk/SCR), Rollentrennung Agent nie Approver); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Human-Decision-Pflichtfelder

`human_review: {required, requested}` je Task (AI-DEV-004 §1) UND je
PR ("Human Decision Required", AI-DEV-007 §4). Required=true blockiert
COMPLETED bis dokumentierte Owner-Entscheidung.

## 2. Eskalationsstufen

Eskalation an den Owner zwingend bei: (a) Annahmen mit
verification.required, (b) Governance-Konflikten (Standard vs. Realität),
(c) Autorisierungsbedarf jenseits der Capability-Matrix, (d) BLOCKED ohne
selbst behebbaren Weg. Eskalation enthält: Befund, Optionen, Empfehlung,
Frist.

## 3. Genehmigungsformen

Owner-Genehmigungen erfolgen als: Direktmandat (Chat, mit Zitat im
Approval-Doc), Freigabe-Vermerk (approval/APPROVAL-DECISION-*.md) oder
SCR-Entscheidung. Stillschweigende Zustimmung ist keine Genehmigung.

## 4. Rollentrennung

Agenten sind niemals Approver (ATC-STD-000 §14.1). Agenten-dokumentierte
Freigaben sind ungültig und als Governance-Verstoß zu werten.

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
