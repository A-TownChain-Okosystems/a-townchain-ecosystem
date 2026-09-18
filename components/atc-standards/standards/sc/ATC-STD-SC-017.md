---
standard:
  id: ATC-STD-SC-017
  title: "ATC-STD-SC-017 — GameFi Contract Standard"
  version: "1.0.0"
  status: approved
  category: sc
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  applies_to: "Alle Smart Contracts des A-TownChain-Oekosystems"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-SC-017 — GameFi Contract Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer Game-Contracts (Shivamon, Items, Rewards): deterministisches Gameplay-Asset-Verhalten, Item-Erstellung/Rarity ueber rollenkontrollierte Minting-Regeln, Reward-Emissionen mit Obergrenzen und Anti-Cheat-Events.

## Scope

Gilt fuer alle SC-GAME-Contracts (Genesis Chronicles / Shivamon).

## 1. Asset-Verhalten (REQ-SC-047)

GameFi-Assets (Shivamon, Items) MUESSEN deterministische Erstellungs- und Transfer-Regeln haben; Minting ueber rollenkontrollierte Game-Logik (SC-009).

## 2. Reward-Emissionen (REQ-SC-048)

Reward-Emissionen MUESSEN Obergrenzen (Invariant: rewardsIssued <= rewardPool) und dokumentierte Verteilungsregeln haben.

## 3. Anti-Cheat-Nachvollziehbarkeit (REQ-SC-049)

Wichtige Game-Ereignisse sind Event-pflichtig (SC-008) fuer Anti-Cheat-Monitoring (DefenderGPT).



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-047 | Asset-Verhalten (§1) | MUSS |
| REQ-SC-048 | Reward-Grenzen (§2) | MUSS |
| REQ-SC-049 | Anti-Cheat-Events (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

GameFi koppelt Wirtschaftswerte an Spiellogik — Exploits wirken direkt auf die Tokenomia.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) |

## References

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-011 (Token)
- **INFORMATIVE:** ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/
