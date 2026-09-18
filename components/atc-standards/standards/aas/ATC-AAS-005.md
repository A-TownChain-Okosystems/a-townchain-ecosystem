---
standard:
  id: ATC-AAS-005
  title: "ATC-AAS-005 — Agent Discovery Standard"
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

# ATC-AAS-005 — Agent Discovery Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-005 (Agent Discovery Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Selbstfragen (Pflicht-Antworten vor jeder Aktion)

Wer bin ich? (ATC-AAS-001) · In welchem Repository arbeite ich? (ATC-AAS-004)
· Welchen Standard muss ich befolgen? (ATC-AAS-006, Repo-Manifest
ATC-AAS-025) · Welche Aufgabe habe ich? (ATC-AAS-007) · Wo finde ich die
Anforderungen? (Discovery-Kette AI-DEV-003 §1) · Was ist als Nächstes zu tun?
(AI-DEV-006 §3 Next-Action-Pflicht)

## 2. Manifest-Dateiset (Ablageort .github/ai/ gemäß AI-DEV-001 §6)

```
.github/ai/
├── agent.yaml          # Identität (ATC-AAS-001)
├── instructions.md     # Rollen-Instruktionen
├── capabilities.yaml   # ATC-AAS-002
├── permissions.yaml    # ATC-AAS-003
├── workflow.yaml       # Lifecycle-Konfiguration (ATC-AAS-008)
└── context.yaml        # Kontext-Bindung (ATC-AAS-006)
```

Hinweis: Der Owner-Entwurf nannte `.agent/` als Ort; verbindlich ist die
bereits approvedete Ablage `.github/ai/` (AI-DEV-001 §6). Dateinamen des
Entwurfs werden vollständig übernommen.

## 3. Entry-Chain beim Betreten eines Repos

IDENTITY → SCOPE → RULES → TASK → ACTION → TEST → EVIDENCE → REVIEW
(ATC-AAS-025 §3).

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
