---
standard:
  id: ATC-ENT-008
  title: "ATC-ENT-008 — Organisationsstruktur Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-ENT-008 — Organisationsstruktur Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-008 (Organisationsstruktur Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Organisationshierarchie (verbindlich)

```
A-TownChain Ökosystems
├── Executive / Governance
├── Architecture
├── Blockchain (L1, Consensus, ZKP, Token, Interoperability)
├── Software Engineering (Backend, Frontend, Infrastructure, QA)
├── AI Engineering (AI Agents, Model Infrastructure, Agent Governance, AI Security)
├── Security
├── DevOps / Infrastructure
├── Game / GameFi
├── Wallet / DeFi
├── Marketplace
├── Documentation / Wiki
└── Compliance / Audit
```

## 2. Pflichtattribute je Organisationseinheit (Registry `registry/org-units.yaml`)

```yaml
org_unit:
  id: UNIT-BLOCKCHAIN
  name: Blockchain
  owner: ROLE-ARCH          # ENT-002-Rolle
  deputy: ROLE-DEV
  responsibility: "..."
  permissions: [...]        # abgeleitet aus ENT-004
  kpis: [...]               # ENT-013
  repositories: [atc-blockchain, atc-node, atc-zkp]   # ENT-009
  standards: [ATC-STD-NET-003, ATC-STD-ZKP-001]       # required
  documentation: {...}       # Wiki/Architektur-Pflichten
  audit_requirements: {...}  # ENT-014
```

## 3. Regeln

- Jedes Repository gehört zu genau einer Organisationseinheit (Mapping in
  ENT-009 Repo-Registry); jede Einheit hat Owner UND Deputy.
- 26 aktive Repos: Zuordnung bestehender 5-Produktlinien-Struktur
  (Governance, Docs, Core, Protocol Services, User Products, Integration)
  auf die obigen Einheiten; Doppelposten dokumentieren (ENT-002 §4).
- Änderungen nur über ENT-010 mit Registry-Update und Notion-/Wiki-Sync
  (AI-DEV-010).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Enterprise-Transaktionen: Buchungs- und Audit-Trails unveraenderbar (append-only); Betrugsschutz-Mechanismen bei Zahlungswegen; keine Zugangsdaten in Geschaeftsdaten; Transaktions-Integritaet vor und nach Konsens-Teilnahme gewaehrleistet.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-280ff (Security-Familie), ATC-ENT-001 · INFORMATIVE: Registry-Kategorie enterprise
