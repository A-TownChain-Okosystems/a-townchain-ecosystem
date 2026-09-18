---
standard:
  id: ATC-STD-NET-007
  title: "ATC-STD-NET-007 — Network Security Standard"
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

# ATC-STD-NET-007 — Network Security Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Sicherheitsniveau und -pflichten je Netzwerkstufe. Nicht gilt: Applikations-Security ausserhalb des Netzwerkbetriebs
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Sicherheit skaliert mit der Stufe: was auf Devnet erlaubt ist, ist auf
Mainnet kategorisch verboten. Die Stufe bestimmt das Trust-Modell.

## 1. Security-Matrix (REQ-NET-061)

| Stufe | Niveau | Debug-Endpunkte | Security-Audit |
|---|---|---|---|
| Devnet | niedrig-mittel | erlaubt | optional je Feature |
| Testnet | mittel-hoch | nur mit Feature-Flag | GATE-012-Pflicht (SECURITY TEST) |
| Mainnet | maximal | VERBOTEN | GATE-013-Pflicht (SECURITY AUDIT PASS) |

## 2. Mainnet-Härtelinie (REQ-NET-062: MUST)

Keine Debug-Endpunkte · keine Test-Token · keine ungeprueften Smart
Contracts (ATVM-Bytecode-Verifier + License Gate als Trust Boundary,
AD-021/022) · Validator-Zugang protocol-defined · Dependencies gemaess
ATC-STD-203 Dependency Policy.

## 3. Testnet-Sicherheitspruefungen (REQ-NET-063)

Security Test, Chaos Test, Fault Injection und Penetration der
NET-002-Testpflicht sind auf Testnet zu absolvieren — NICHT auf Devnet,
NICHT erstmals auf Mainnet.

## Requirements

- id: REQ-NET-061
  title: "Security-Matrix je Stufe verbindlich"
  severity: MANDATORY
- id: REQ-NET-062
  title: "Mainnet-Haertelinie (4 Verbote + Trust Boundary)"
  severity: MANDATORY
- id: REQ-NET-063
  title: "Security-/Chaos-Tests auf Testnet, nie erstmals auf Mainnet"
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
