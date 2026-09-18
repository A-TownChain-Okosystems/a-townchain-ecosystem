---
standard:
  id: ATC-STD-AI-DEV-010
  title: "ATC-STD-AI-DEV-010 — AI Documentation Synchronization Standard"
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

# ATC-STD-AI-DEV-010 — AI Documentation Synchronization Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §12 (Konsistenzmatrix), ATC-STD-BUG-004 (Sync)

## Abstract

ATC-STD-AI-DEV-010 (AI Documentation Synchronization Standard) — Synchronisationspflicht gegen SPEC/STANDARD/WIKI/ARCHITECTURE/ROADMAP, SYNC-NNN-Records, kein COMPLETED bei deviation, Automatisierungsvorgabe); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Synchronisationspflicht

Jede Code-Änderung ist gegen die Dokumentations-Landschaft geprüft: SPEC,
STANDARD, WIKI, ARCHITECTURE, ROADMAP. Ergebnis in der Konsistenzmatrix
(updated|unchanged|consistent|checked|deviation). Keine Information darf
verloren gehen — Abweichungen werden synchronisiert oder als BLOCKED
dokumentiert.

## 2. Sync-Records

Dokument-Synchronisierungen werden als SYNC-NNN referenziert (BUG-004):
was, wohin, von wo, wann, durch wen (Agent-ID). Cross-Repo-Syncs
(z.B. wiki → docs) tragen Quell- und Ziel-Fundstellen.

## 3. Kein COMPLETED bei deviation

`specification: deviation` oder `wiki: deviation` ohne Folge-Sync =
Status BLOCKED (AI-DEV-001 §12). Der Folge-Sync ist als ACT-NNN im
gleichen Task oder als Folge-Task mit Querverweis dokumentiert.

## 4. Automatisierung

Wiederkehrende Synchronisationen sind zu automatisieren (Workflows,
Sync-Scripts); manuelle Syncs sind die Ausnahme und werden begründet.

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
