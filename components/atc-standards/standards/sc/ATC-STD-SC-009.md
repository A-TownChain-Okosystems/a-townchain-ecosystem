---
standard:
  id: ATC-STD-SC-009
  title: "ATC-STD-SC-009 — Smart Contract Access Control Standard"
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

# ATC-STD-SC-009 — Smart Contract Access Control Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Administrative Funktionen sind niemals unkontrolliert oeffentlich erreichbar: Permission → Role → Governance → Timelock → Execution; standardisierte Rollen (OWNER..AUDITOR).

## Scope

Gilt fuer alle Contracts mit administrativen Funktionen; verstaerkt ATC-STD-203 §Autorisierung.

## 1. Schutz-Pflicht (REQ-SC-026)

Administrative Funktionen MUessen durch Modifier/Checks geschuetzt sein, z. B. ```solidity
modifier onlyGovernance() { require(msg.sender == governance, "NOT_GOVERNANCE"); _; }
```


## 2. Rollen-Enum (REQ-SC-027)

Rollen SOLLEN aus dieser Menge stammen: OWNER, ADMIN, GOVERNANCE, OPERATOR, MINTER, PAUSER, UPGRADER, ORACLE, BRIDGE, TREASURY, AUDITOR.

## 3. Autorisierungskette (REQ-SC-028)

Kritische Funktionen MUssen der Kette folgen: User → Permission Check → Role Check → Governance → Timelock → Execution.



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-026 | Schutz-Pflicht (§1) | MUSS |
| REQ-SC-027 | Rollen-Enum (§2) | SOLLTE |
| REQ-SC-028 | Autorisierungskette (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Rollenvergabe und -entzug sind Events-pflichtig (SC-008) und AUD-pflichtig.

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
