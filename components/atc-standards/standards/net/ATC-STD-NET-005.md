---
standard:
  id: ATC-STD-NET-005
  title: "ATC-STD-NET-005 — Network Genesis Standard"
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

# ATC-STD-NET-005 — Network Genesis Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Genesis-Konfiguration und -Lifecycle aller drei Stufen. Nicht gilt: laufender Chain-Betrieb
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Die Genesis ist die Wurzel des Netzwerks — je Stufe mit unterschiedlicher
Mutabilitaet. Ihre Felder sind normativ und maschinenlesbar zu hinterlegen.

## 1. Genesis-Matrix (REQ-NET-041)

| Stufe | Genesis | Reset/Neuerstellung |
|---|---|---|
| Devnet | mutable | automatisch erlaubt |
| Testnet | versioniert (SemVer + Hash-Pinning) | nur kontrolliert (SCR) |
| Mainnet | immutable ab Genesis-Moment | VERBOTEN |

## 2. Pflichtfelder (REQ-NET-042: MUST)

Chain-ID (Allokation registry/networks.yaml; Mainnet 658467) ·
Network-Name (ATC-DEVNET/ATC-TESTNET/ATC-MAINNET) · Token-Parameter ·
Validator-Initialmenge · Zeitstempel/Epoch · ggf. Initial-State ·
Genesis-Hash (Testnet/Mainnet).

## 3. Mainnet-Genesis-Validierung (REQ-NET-043)

Vor dem Genesis-Moment: GENESIS VALIDATION als GATE-013-Stufe —
Feldpruefung, Hash-Pinning, Reproduzierbarkeit, Governance-Approval.
Nach dem Moment: Genesis ist unveraenderlich (NET-003 REQ-NET-021).

## 4. Allokation (REQ-NET-044)

Chain-IDs werden zentral in registry/networks.yaml allokiert (naechste
freie Nummer, SCR-0001-Prozess). Mainnet 658467 ist reserviert (AD-004).
Devnet-/Testnet-IDs sind bis zur Genesis-Erstellung aenderbar.

## Requirements

- id: REQ-NET-041
  title: "Genesis-Matrix je Stufe (mutable/versioniert/immutable)"
  severity: MANDATORY
- id: REQ-NET-042
  title: "Genesis-Pflichtfelder vollstaendig und maschinenlesbar"
  severity: MANDATORY
- id: REQ-NET-043
  title: "Mainnet-Genesis-Validierung vor Genesis-Moment (GATE-013-Stufe)"
  severity: MANDATORY
- id: REQ-NET-044
  title: "Chain-ID-Allokation zentral ueber registry/networks.yaml"
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
