---
standard:
  id: ATC-STD-AI-DEV-005
  title: "ATC-STD-AI-DEV-005 — Finding & Evidence Standard"
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

# ATC-STD-AI-DEV-005 — Finding & Evidence Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §4 (Evidence-Pflicht), ATC-STD-BUG-001/BUG-002

## Abstract

ATC-STD-AI-DEV-005 (Finding & Evidence Standard) — Agenten-Findings F-NNN (BUG-001-Struktur), Evidence-Mindeststruktur je Fundstelle, Erkenntnis ohne Fundstelle ungueltig (ASSUMPTION-Trennung), Finding-zu-Action-Pflicht); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Findings der Agentenarbeit

Agenten-Findings nutzen F-NNN (BUG-001-Struktur): severity (S0-S4), category,
title, evidence, expected, actual, impact. Herkunftsverweis: `source:
ai-agent (ATC-AI-{ROLE}-NNN)`.

## 2. Evidence-Mindeststruktur

Je Finding ≥1 Fundstelle mit: `source_type` (repository, code, issue,
pull_request, wiki, standard, roadmap, todo, test, ci_cd, architecture,
configuration, external), Lokation (repository/path/lines bzw. issue/PR/section),
Erhebungszeitpunkt. Externe Quellen sind als solche gekennzeichnet (Vertrauens-
niveau niedriger als Repository-Artefakte).

## 3. Erkenntnis ohne Fundstelle

Ungültig. Modellannahmen sind als ASSUMPTION-ANNN zu führen (AI-DEV-001 §10),
nicht als Finding. Ein Finding behauptet Verifiziertes.

## 4. Kette Finding → Action

Jedes Finding mündet in mindestens ein ACT-NNN (AI-DEV-006) oder eine
dokumentierte Nicht-Handlungs-Entscheidung mit Begründung ("wont-fix because
…" mit Standard-/Owner-Bezug).

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
