---
standard:
  id: ATC-STD-NET-006
  title: "ATC-STD-NET-006 — Network Upgrade Standard"
  version: "1.0.0"
  status: approved
  category: net
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: null
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-NET-006 — Network Upgrade Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Formalisierter Protokoll-Upgrade-Prozess, insbesondere Mainnet. Nicht gilt: Devnet-Experimente (NET-001 §2)
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Auf Mainnet ist der Consensus frozen — jede Protokollaenderung laeuft
ueber einen formalisierten Upgrade-Prozess mit Governance-Approval
(REQ-NET-051).

## 1. Upgrade-Prozess (REQ-NET-052: MUST — Reihenfolge verbindlich)

```
PROPOSAL (SCR-NNN) → TECHNICAL REVIEW → TESTNET-NACHWEIS (GATE-012-Re-Run)
→ GOVERNANCE APPROVAL → AKTIVIERUNGSHOEHE/-EPOCH FESTLEGEN →
KOORDINIERTES DEPLOYMENT → BEOBACHTUNG → ABGESCHLOSSEN
```

## 2. Rollback-Politik (REQ-NET-053)

Jedes Upgrade definiert VOR der Aktivierung einen dokumentierten
Rollback-/Notfallpfad (§32 Emergency, ATC-STD-000). Auf Mainnet gibt es
keinen Chain-State-Rollback — nur Protokoll-Downgrades vor Aktivierung
oder Folge-Upgrades.

## 3. Stufenspezifik (REQ-NET-054)

Devnet: Upgrades frei, Feature-Flags experimentell. Testnet:
Upgrade-Mechanismus selbst ist Testpflicht-Gegenstand (NET-002 §2).
Mainnet: nur NET-006-Prozess.

## Requirements

- id: REQ-NET-051
  title: "Mainnet-Consensus frozen; Aenderungen nur via Upgrade-Prozess"
  severity: MANDATORY
- id: REQ-NET-052
  title: "8-stufiger Upgrade-Prozess mit Governance-Approval"
  severity: MANDATORY
- id: REQ-NET-053
  title: "Rollback-/Notfallpfad vor Aktivierung dokumentiert"
  severity: MANDATORY
- id: REQ-NET-054
  title: "Stufenspezifik Devnet/Testnet/Mainnet fuer Upgrades"
  severity: MANDATORY


## Compliance

Geprueft per Gate-Aufzeichnung (GATE-011…013-Records) und Review gegen
deklarierten REQ-NET-Anforderungen (AUD-NNN-Abschluss gemaess ATC-STD-BUG-004). Maschinenlesbar:
schemas/network-environment.schema.json validiert Umgebungskonfigurationen
(.atc/network/{devnet,testnet,mainnet}.yaml); Registry: registry/networks.yaml.
Verstoss gegen MANDATORY = GATE BLOCKED.

## Security Considerations

Sicherheitsniveau der Stufe gemaess Matrix (NET-007): Devnet
niedrig-mittel (Debug erlaubt), Testnet mittel-hoch (Security-/Chaos-Tests
Pflicht), Mainnet maximal (keine Debug-Endpunkte, keine ungeprueften
Vertraege, ATVM-Verifier + License Gate als Trust Boundary).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Mandat AD-041) |

## References

**NORMATIVE:** ATC-STD-NET-001…008 (Reihe), ATC-STD-000 (§7 IDs, §33),
ATC-STD-203 (Release-Gates, Conventional Commits), ATC-STD-BUG-003/004
(Fix-/Merge-Gate), registry/networks.yaml · schemas/network-environment.schema.json
**INFORMATIVE:** AD-004 (Chain-ID 658467), AD-021/022 (ATCLang-Baseline),
AD-027 (Lauffaehigkeits-Roadmap M4), docs/roadmap/
