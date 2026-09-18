---
standard:
  id: ATC-AAS-001
  title: "ATC-AAS-001 — Agent Identity Standard"
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

# ATC-AAS-001 — Agent Identity Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Block:** ATC-AAS (AI Agent Standards) · **Priorität:** P0 · **Erweitert:** ATC-STD-AI-DEV-001 §2 (Agent-Manifest)

## Abstract

ATC-AAS-001 (Agent Identity Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Identitäts-Pflichtstruktur

Jeder KI-Agent ist eindeutig identifizierbar. Kanonische Ablage:
`.github/ai/agent.yaml` (Ablageort gemäß AI-DEV-001 §6).

```yaml
agent_id: ATC-AI-DEV-001          # Namensraum AI-DEV-001 §2: ATC-AI-{ROLE}-NNN
agent_name: ATC Development Agent
agent_type: software-development
agent_version: 1.0.0              # Versionierung nach ATC-AAS-022
owner: A-TownChain-Okosystems
repository_scope: []              # konkretisiert durch ATC-AAS-004
capabilities: []                  # konkrete Kategorien: ATC-AAS-002
permissions: {}                   # Operationen: ATC-AAS-003
environment: {runtime, network, sandbox}   # konkrete Kategorien: ATC-AAS-014
status: ACTIVE|RETIRED
```

## 2. Regeln

- Agenten ohne gültiges Identity-Manifest gelten als nicht vorhanden
  (kein Zugang, keine Commits, keine Tasks).
- `agent_id` folgt AI-DEV-001 §2 und naming-conventions (`aiAgentId`).
- Ein Agent ist immer genau EINEM Identity-Manifest zugeordnet; Session-
  Kontext darf die Identität nicht ändern.

## 3. Verhältnis zu AI-DEV-001

AAS-001 konkretisiert die abstrakte Manifest-Pflicht aus AI-DEV-001 §2 um
die Felder `environment` und `status`; kein Widerspruch — AI-DEV-001 bleibt
Dach-Norm der Identität.

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

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-AAS-002 · INFORMATIVE: AGENT_MANIFEST.md v3.1.7, Registry-Kategorie aas
