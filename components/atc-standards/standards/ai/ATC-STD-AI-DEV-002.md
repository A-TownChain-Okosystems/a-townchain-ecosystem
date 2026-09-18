---
standard:
  id: ATC-STD-AI-DEV-002
  title: "ATC-STD-AI-DEV-002 — Agent Capabilities & Permissions Standard"
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

# ATC-STD-AI-DEV-002 — Agent Capabilities & Permissions Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §3 (Capability-Matrix)

## Abstract

ATC-STD-AI-DEV-002 (Agent Capabilities & Permissions Standard) — Capability-Matrix normativ, Permission-Modell (Least Privilege), Restriktions-Invarianten, Verstossbehandlung); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Capability-Matrix

`capabilities.yaml` ist normativ: Jede Fähigkeit ist benannt, versioniert und
ausschließlich in deklarierter Form ausübbar. Nicht deklarierte Fähigkeit =
nicht vorhanden. Änderungen der Matrix erfordern Manifest-Versionierung
(`agent.version` bump) und Owner-Benachrichtigung bei sicherheitsrelevanten
Erweiterungen.

## 2. Permission-Modell (Least Privilege)

`permissions.yaml` deklariert je Repository: read/write/merge/delete/release.
Defaults: read=true, write je Repo-Notwendigkeit, merge/delete/release=false.
Eskalation (z.B. merge-Recht) nur über dokumentierten Owner-Auftrag mit
Befristung; nach Ablauf automatische Rückstufung.

## 3. Restriktions-Invarianten

Nicht entfernbar, unabhängig von Konfiguration: `no_production_deployment`,
`no_secret_access`, `no_direct_main_merge` (Ausnahme: dokumentierte
Owner-Ausnahme gemäß SCR-0003 Option B mit Commit-Nachweis).

## 4. Verstoßbehandlung

Handeln außerhalb der Matrix = Governance-Verstoß; sofortiges Anhalten,
Dokumentation als Finding (F-NNN) und BLOCKED-Status des Tasks. Keine
Rückwirkungsbereinigung ohne Audit-Record (AI-DEV-009 §3).

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
