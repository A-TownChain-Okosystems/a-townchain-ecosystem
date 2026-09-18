---
standard:
  id: ATC-STD-SC-020
  title: "ATC-STD-SC-020 — AI-Assisted Smart Contract Development Standard"
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

# ATC-STD-SC-020 — AI-Assisted Smart Contract Development Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

KI-Agenten duerfen Contracts nicht eigenmaechtig deployen: standardisierter Workflow von der Specification bis zur Registry-Aktualisierung mit Human/Governance-Approval-Gates — verbindet die AAS/AI-DEV-Agentenstandards mit der SC-Familie.

## Scope

Gilt fuer alle Agenten (ATC-AI-ARCH-001 etc.), die Smart Contracts spezifizieren, implementieren, testen oder deployen.

## 1. Agenten-Workflow (REQ-SC-056)

```text
AI Agent → Contract Specification → Requirement Validation → Implementation
→ Automated Tests → Security Scan → Human/Governance Approval → Testnet Deployment
→ Validation → Mainnet Approval → Deployment → Verification → Registry Update
```

## 2. Deploy-Verbot (REQ-SC-057)

Ein Agent DARF einen Smart Contract nicht ohne bestandene Gates deployen; Mainnet-Deployment erfordert explizite Governance-/Owner-Freigabe (AAS-017 Human Approval, SC-G9/G10).

## 3. Nachweispflicht (REQ-SC-058)

Agenten-Beitraege sind AUD-pflichtig (AI-DEV-009, AAS-010 Evidence): Task → Commit → Testergebnis → Gate-Stand gehoeren in den AUD-Record.



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-056 | Agenten-Workflow (§1) | MUSS |
| REQ-SC-057 | Deploy-Verbot ohne Gates (§2) | MUSS |
| REQ-SC-058 | AUD-Nachweispflicht (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

KI-Deploy ohne Gate-Durchlauf ist der kritischste Automatisierungsfehler — hart verboten.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), AAS-007 (Task), AAS-017 (Human Approval), AI-DEV-009 (Audit)
- **INFORMATIVE:** ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/
