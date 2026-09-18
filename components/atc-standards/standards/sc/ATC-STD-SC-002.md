---
standard:
  id: ATC-STD-SC-002
  title: "ATC-STD-SC-002 — Smart Contract Identity Standard"
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

# ATC-STD-SC-002 — Smart Contract Identity Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Jeder Contract ist eindeutig identifizierbar und rueckverfolgbar: contract_id, Version, Netzwerk, Sprache, Runtime, Upgradeability sowie vollstaendige Quell- und Deployment-Verknuepfung (Repository → Commit → Deployment-Adresse).

## Scope

Gilt fuer alle Contracts; maschinenlesbare Identitaet in Contract-Metadaten und Contract Registry (SC-019).

## 1. Pflichtidentitaet (REQ-SC-006)

Jeder Contract MUSS definieren: contract_id (ATC-SC-<KATEGORIE>-NNN), name, version, network, chain_id, contract_type, status, language, runtime, owner, upgradeable. Beispiel: `contract_id: ATC-SC-TOKEN-001, version: 1.0.0, network: mainnet, chain_id: ATC-1, language: Solidity, runtime: EVM, upgradeable: false`.

## 2. Quell-Verknuepfung (REQ-SC-007)

Zusaetzlich MUSS jeder Contract referenzieren: source_repository, contract_path, commit, compiler, compiler_version, optimization, audit_status, deployment_date, deployment_address — damit jeder Contract eindeutig mit Code → Repository → Commit → Deployment verbunden ist.



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-006 | Identitaetsfelder (§1) | MUSS |
| REQ-SC-007 | Code-Repo-Commit-Deployment-Link (§2) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Identitaetsdaten enthalten keine Secrets; deployment_address nur nach SC-G10.

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
