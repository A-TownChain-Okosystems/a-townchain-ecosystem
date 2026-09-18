---
standard:
  id: ATC-STD-NET-008
  title: "ATC-STD-NET-008 — Network Recovery Standard"
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

# ATC-STD-NET-008 — Network Recovery Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Recovery-, Synchronisations- und Resilienz-Nachweise je Stufe. Nicht gilt: Anwendungs-Backups ausserhalb des Chain-States
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Recovery wird VOR der Promotion bewiesen, nicht im Ernstfall erfunden — Nachweise MUST vollstaendig vorliegen:
Recovery Test ist GATE-012-Stufe, Recovery Validation ist
GATE-013-Stufe (REQ-NET-071).

## 1. Recovery-Matrix (REQ-NET-072)

| Stufe | Recovery-Nachweis |
|---|---|
| Devnet | Resets erlaubt, keine Nachweispflicht |
| Testnet | RECOVERY TEST + STATE SYNCHRONIZATION in GATE-012 |
| Mainnet | RECOVERY VALIDATION in GATE-013 (MUST); Notfallpfad NET-006 §2 |

## 2. Pflicht-Nachweise Testnet (REQ-NET-073)

Node-Absturz + Re-Sync · State-Synchronization nach Partition ·
Slashing-/Recovery-Semantik · Validator-Ausfall-Quorum ·
Mempool-/VM-Zustandskonsistenz. Alle Szenarien MUessen nachgewiesen werden.

## 3. Mainnet-Invariante (REQ-NET-074)

Recovery auf Mainnet bedeutet Rekonstruktion aus verifizierten
Chain-Daten (never trust unverifizierten State): Reset bleibt VERBOTEN;
einzige Heilung ist deterministischer Re-Play der verifizierten Kette.

## Requirements

- id: REQ-NET-071
  title: "Recovery wird vor Promotion bewiesen (GATE-012/013-Stufen)"
  severity: MANDATORY
- id: REQ-NET-072
  title: "Recovery-Matrix je Stufe"
  severity: MANDATORY
- id: REQ-NET-073
  title: "Testnet-Recovery-Nachweise (5 Szenarien)"
  severity: MANDATORY
- id: REQ-NET-074
  title: "Mainnet-Recovery nur als deterministischer Re-Play; Reset bleibt verboten"
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
