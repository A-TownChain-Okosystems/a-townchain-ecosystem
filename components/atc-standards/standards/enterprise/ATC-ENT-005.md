---
standard:
  id: ATC-ENT-005
  title: "ATC-ENT-005 — Unternehmensrichtlinien Standard"
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

# ATC-ENT-005 — Unternehmensrichtlinien Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-005 (Unternehmensrichtlinien Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Richtlinien-Hierarchie

1. ATC-STD-000 (Verfassung) — nicht überlagerbar
2. ATC-ENT-Richtlinien ( Policies, ENT-Block)
3. Domänen-Standards (AI-DEV, AAS, 201-204, …)
4. Repo-interne Richtlinien (CONTRIBUTING, AGENTS.md)

Niedrigere Ebene darf höhere Ebene präzisieren, nie aufweichen.

## 2. Richtlinien-Registry

Jede Richtlinie ist ein Dokument nach ENT-001 §3 (Metadaten) mit
`policy: {id: POL-NNNN, scope, enforcement, owner}` — enforcement:
automatisiert (CI/Workflow) bevorzugt, sonst Audit-Stichprobe (ENT-014).

## 3. Mindest-Policies (Initialbestand)

- Security-Policy (Secrets: AAS-014; Scanning: ATC-STD-203)
- Data-Policy (Datenklassifikation, Offenlegung)
- AI-Policy (Agenten-Einsatz nach AAS/AI-DEV; False-Claim-Verbot: AAS-012)
- Release-Policy (NET-003/004 Mainnet/Release-Gates)

## 4. Änderungen

Nur über ENT-010 (Change) mit Impact auf betroffene Standards;
Richtlinien-Änderungen lösen Review-Pflicht abhängiger Dokumente aus
(review_cycle, ENT-001 §3).

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
