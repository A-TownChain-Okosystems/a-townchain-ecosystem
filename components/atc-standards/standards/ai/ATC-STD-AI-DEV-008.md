---
standard:
  id: ATC-STD-AI-DEV-008
  title: "ATC-STD-AI-DEV-008 — AI Testing & Validation Standard"
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

# ATC-STD-AI-DEV-008 — AI Testing & Validation Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §7 (TESTING/VALIDATING), §12 (Test Gate)

## Abstract

ATC-STD-AI-DEV-008 (AI Testing & Validation Standard) — Test-Pflicht je Aenderung, Validierung nur mit CI-Run-Referenz (TEST-NNN), Fail-Handling (DEBUGGING-Pflicht, kein Bypass), Uebergabe an Audit-Record); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Test-Pflicht

Keine IMPLEMENTING-Änderung ohne zugehörige Tests (neu oder erweitert).
Ausnahmen nur bei reinen Dokumentations-/Konfigurations-Änderungen — im
Task-Record als solche deklariert.

## 2. Validierungsergebnis

`validation` MUSS je Dimension (tests, lint, build, security) einen Status
(PASS|FAIL|PENDING) UND eine CI-Run-Referenz tragen (AI-DEV-009 §1).
"PASS ohne Run" ist ungültig. Regressionstests werden als TEST-NNN
referenziert (BUG-002).

## 3. Fail-Handling

FAIL → Status TESTING→DEBUGGING (AI-DEV-001 §7), Finding anlegen (S-Skala),
Fix als neue ACT-NNN. Kein COMPLETED mit FAIL. Kein Bypassen roter Tests
(im Build-Skript auskommentierte Tests = Governance-Verstoß, S1).

## 4. Übergabe an Audit

Das Validierungsergebnis ist vollständig in den Audit-Record (AI-DEV-009 §1)
zu übernehmen; konsistenzprüfung code↔tests (§12 Konsistenzmatrix) ist Teil
der Validation.

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
