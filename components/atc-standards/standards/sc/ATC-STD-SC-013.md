---
standard:
  id: ATC-STD-SC-013
  title: "ATC-STD-SC-013 — DeFi Contract Standard"
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

# ATC-STD-SC-013 — DeFi Contract Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer Staking-, Liquidity- und Swap-Contracts: Invarianten fuer Wertfluesse, Manipulationsresistenz (Flash-Loan, Oracle, Price), klare Withdrawal-Regeln und Emergency-Mechanismen.

## Scope

Gilt fuer alle SC-DEFI-Contracts.

## 1. Wertfluss-Invarianten (REQ-SC-036)

DeFi-Contracts MUESSEN Wertfluss-Invarianten als Invariant-Tests codieren (z. B. stakedBalance <= totalDeposited; liquidityConstantin in beiden Assets).

## 2. Manipulationsresistenz (REQ-SC-037)

Staking/Liquidity/Swaps MUESSEN gegen Flash-Loan-, Oracle- und Price-Manipulation gehaertet sein (SC-003 §1).

## 3. Withdrawal & Emergency (REQ-SC-038)

Withdrawal-Regeln MUESSEN dokumentiert sein; Emergency-Verhalten nach dem Emergency-Zustaende-Modell (SC-001/Serie: NORMAL→SUSPICIOUS→PAUSED→INVESTIGATION→RECOVERY→RESUME).



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-036 | Wertfluss-Invarianten (§1) | MUSS |
| REQ-SC-037 | Manipulationsresistenz (§2) | MUSS |
| REQ-SC-038 | Withdrawal/Emergency (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

DeFi-Contracts sind das hoechste Exploit-Risiko; unabhaengiges Audit (SC-005) Pflicht.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-003 (Security), ATC-STD-SC-004 (Testing)
- **INFORMATIVE:** ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/
