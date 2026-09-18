---
standard:
  id: ATC-STD-SC-015
  title: "ATC-STD-SC-015 — Bridge Contract Standard"
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

# ATC-STD-SC-015 — Bridge Contract Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Cross-Chain-Anforderungen: vollstaendige Nachrichtenfelder (Message ID, Source/Destination Chain, Nonce, Sender, Receiver, Amount, Payload Hash, Validator/Relayer, Finality), Replay Protection und der kanonische Flow Deposit → Verification → Message → Consensus → Destination Verification → Execution.

## Scope

Gilt fuer alle SC-BRIDGE-Contracts; ist die normative Fassung des Owner-Entwurfs (eigenstaendiger Deep-Standard ATC-STD-SC-BRIDGE-001 kann per SCR nachfolgen).

## 1. Nachrichtenfelder (REQ-SC-042)

Bridge-Nachrichten MUESSEN enthalten: Message ID, Source Chain, Destination Chain, Nonce, Sender, Receiver, Amount, Payload Hash, Validator/Relayer, Finality.

## 2. Replay Protection & Finality (REQ-SC-043)

Bridges MUESSEN Replay Protection (Nonce-Tracking) und Finality-Regeln erzwingen; die Bruecken-Invariante `bridgeMinted <= bridgeDeposited` ist Invariant-Test-pflichtig (SC-004).

## 3. Flow (REQ-SC-044)

```text
Deposit → Verification → Message → Consensus → Destination Verification → Execution
```



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-042 | Nachrichtenfelder (§1) | MUSS |
| REQ-SC-043 | Replay/Finality (§2) | MUSS |
| REQ-SC-044 | Flow (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Bridge-Exploits sind die teuersten in der Branche; unabhaengiges Audit zwingend.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-004 (Invarianten)
- **INFORMATIVE:** ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/
