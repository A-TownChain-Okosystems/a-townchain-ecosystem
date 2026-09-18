---
standard:
  id: ATC-STD-SC-012
  title: "ATC-STD-SC-012 — NFT Contract Standard"
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

# ATC-STD-SC-012 — NFT Contract Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer NFT-, Asset- und Collection-Contracts: eindeutige Token-IDs, Ownership-Transfer, Approvals, Metadata-Referenzen (off-chain), Royalty-Unterstützung und Event-Pflicht.

## Scope

Gilt fuer alle SC-NFT- und SC-MARKET-Contracts (Marketplace siehe SC-013-Verzahnung).

## 1. Kernpflichten (REQ-SC-034)

NFT-Contracts MUESSEN eindeutige Token-IDs, Ownership-Transfer, Approvals und Metadata-Referenzen implementieren; Transfers und Approvals sind Event-pflichtig (SC-008).

## 2. Collections & Royalties (REQ-SC-035)

Collections SOLLEN Royalty-Mechanismen und Creator-Attribute unterstuetzen; Marketplace-Integration (SC-MARKET) MUSS ueber definierte Interfaces laufen (ATC-STD-204).



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-034 | Kernpflichten (§1) | MUSS |
| REQ-SC-035 | Collections/Royalties (§2) | SOLLTE |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Metadata-Off-Chain-Verweise auf Manipulation pruefen (SC-003).

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-008 (Events)
- **INFORMATIVE:** ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/
