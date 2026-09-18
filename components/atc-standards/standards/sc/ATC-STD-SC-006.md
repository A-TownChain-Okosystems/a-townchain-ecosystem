---
standard:
  id: ATC-STD-SC-006
  title: "ATC-STD-SC-006 — Smart Contract Deployment Standard"
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

# ATC-STD-SC-006 — Smart Contract Deployment Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Jedes Deployment erzeugt einen maschinenlesbaren Deployment Record (deployment_id, contract_id, network, address, block, transaction, commit, compiler, deployer, timestamp, verification, audit) — jederzeit feststellbar: welcher Code laeuft auf welcher Chain unter welcher Adresse.

## Scope

Gilt fuer alle Deployments (Testnet + Mainnet); Records in contracts/registry/deployments.yaml (SC-019).

## 1. Deployment Record (REQ-SC-017)

Jeder Deployment MUSS einen Deployment Record erzeugen: deployment_id (ATC-DEP-NNNN), contract_id, network, chain_id, address, block, transaction, commit, compiler, compiler_version, deployer, timestamp, verification, audit.

## 2. Verification (REQ-SC-018)

Nach Deployment MUSS die Contract-Verification (SC-G11) erfolgen und im Deployment Record vermerkt sein.

## 3. Registry-Update (REQ-SC-019)

Jedes Deployment MUSS zeitnah in die Contract Registry eingetragen werden (SC-G12, SC-019).



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-017 | Deployment Record (§1) | MUSS |
| REQ-SC-018 | Verification (§2) | MUSS |
| REQ-SC-019 | Registry-Update (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Deployment-Records enthalten keine Secrets; deployer-Adressen sind oeffentliche On-Chain-Daten.

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

- **NORMATIVE:** ATC-STD-000 (Verfassung), ATC-STD-201/202, ATC-STD-203
  (Security), ATC-STD-204 (Dependency & Interface), ATC-STD-SC-001
  (Familien-Hauptstandard)
- **INFORMATIVE:** ATC-AAS-001..025 (Agenten-Standards), AI-DEV-001..012,
  contracts/registry/ (Contract Registry)
