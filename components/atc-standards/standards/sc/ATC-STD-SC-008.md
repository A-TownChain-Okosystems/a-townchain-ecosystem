---
standard:
  id: ATC-STD-SC-008
  title: "ATC-STD-SC-008 — Smart Contract Event Standard"
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

# ATC-STD-SC-008 — Smart Contract Event Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Standardisierte Events als Grundlage fuer AuditTrail, LogChain, DefenderGPT, MinerWatcherGPT, Governance-Monitoring, Explorer, Compliance und Forensics.

## Scope

Gilt fuer alle Contracts; Events sind Teil der oeffentlichen Interface-Dokumentation (ATC-STD-204).

## 1. Ereignis-Pflicht (REQ-SC-023)

Jeder zustandsrelevante Vorgang MUSS ein Event emittieren (Initialisierung, Konfigurationsaenderung, Emergency-State, Werte-Transfers, Rollenaenderungen): z. B. `ContractInitialized(bytes32 indexed contractId, uint256 version)`, `ConfigurationChanged(bytes32 indexed parameter, bytes32 oldValue, bytes32 newValue)`, `EmergencyStateChanged(bool paused)`.

## 2. Indizierung (REQ-SC-024)

Schluesselparameter (IDs, Adressen, Gruende) SOLLEN indexed sein, damit Off-Chain-Konsumenten (Explorer, AuditTrail, GPT-Watcher) effizient filtern koennen.

## 3. Konsumenten (REQ-SC-025)

Events BILDEN die Grundlage fuer: AuditTrail, LogChain, DefenderGPT, MinerWatcherGPT, Governance Monitoring, Explorer, Compliance, Forensics — aenderungen am Event-Set sind daher Interface-aenderungen (ATC-STD-204).



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-023 | Ereignis-Pflicht (§1) | MUSS |
| REQ-SC-024 | Indizierung (§2) | SOLLTE |
| REQ-SC-025 | Event = Interface (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Events DUERFEN KEINE personenbezogenen Daten oder Secrets enthalten.

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
