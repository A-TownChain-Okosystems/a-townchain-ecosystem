---
standard:
  id: ATC-STD-AI-DEV-003
  title: "ATC-STD-AI-DEV-003 — Repository Discovery Standard"
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

# ATC-STD-AI-DEV-003 — Repository Discovery Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §5 (Discovery-Protocol)

## Abstract

ATC-STD-AI-DEV-003 (Repository Discovery Standard) — Discovery-Kette (12 Stufen Pflichtreihenfolge), Discovery-Record, Governance-Discovery ueber Registry/Dependency-Graph, Verbot des Direktmusters); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Discovery-Kette (Pflichtreihenfolge)

1. Repository Identity → 2. README → 3. CONTRIBUTING → 4. AGENTS.md →
5. CODEOWNERS → 6. Standards → 7. Architecture → 8. Issue/Task →
9. Existing Implementation → 10. Tests → 11. CI/CD → 12. Documentation.

Keine Code-Änderung vor abgeschlossener Discovery. Ausnahme: Hotfix mit
nachgelagerter Discovery, als solche im Task-Record gekennzeichnet.

## 2. Discovery-Record

Je Discovery wird ein Record im Task-Record geführt (AI-DEV-004 §1):
`discovery: {checked: [...], skipped: [...], reason_skip: ...}`. Skips sind
zu begründen ("Datei existiert nicht" ist ein gültiger Grund).

## 3. Governance-Discovery

Standards-Registry (registry/standards.yaml) und Dependency-Graph
(registry/dependencies.yaml, ATC-STD-204) sind Pflichtquellen (Schritt 6):
Der Agent prüft, welche normativen Standards sein Ziel-Repo betreffen, und
vermerkt sie als `task_reference.standard`.

## 4. Verbotenes Muster

"User sagt X → KI schreibt Code" ohne Discovery ist ein Governance-Verstoß.
Direktive des Owners befreit nicht von der Discovery-Pflicht — sie definiert
nur den Auftrag.

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
