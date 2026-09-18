---
standard:
  id: ATC-STD-ZKP-005
  title: "ATC-STD-ZKP-005 — Commitment & Nullifier Standard"
  version: "1.0.0"
  status: approved
  category: zkp
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: null
  superseded_by: null

  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-ZKP-005 — Commitment & Nullifier Standard (v1.0.0, APPROVED)

> **Version:** 1.0.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

Kryptografische Commitments (private Balances: public Commitment C, private Balance/Owner-Secret) und Nullifier (Double-Spend-Prävention bei privaten Assets).

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-501 | Commitments MUESSEN mit den Primitiven des ATC Cryptographic Core (SHA-256, AD-001) gebildet oder explizit per AD freigegeben sein. | §S.2 |
| id: REQ-ZKP-502 | Jede private Ausgabe MUSS einen Nullifier konsumieren; Doppelte Nullifier MUESSEN abgelehnt werden. | §S.2 |
| id: REQ-ZKP-503 | Balance-Beweise (z.B. 'Besitzer darf mindestens X ausgeben') MUessen den Kontostand nicht offenlegen. | §S.2 |
| id: REQ-ZKP-504 | Nullifier- und Commitment-Manager MUESSEN unabhaengig testbar sein (Unit-Test-Pflicht). | §S.3 |

## Compliance

Geprueft durch: (1) atc-std-validator (Registry-Sync S-14, Kopf-Sync S-19), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen werden als Findings (ATC-STD-BUG-001) gefuehrt.

## Security Considerations

ZKP-Komponenten sind S4-kritisch (Kryptografie-Bindung, ATC-STD-100): Implementierung MUSS in Rust erfolgen; Trusted-Setup-Kriterien und Fuzzing-Pflichten gelten nach ATC-STD-ZKP-010. Der G18 Security-Audit (AD-023) ist Voraussetzung fuer jeden Freeze.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

- 1.0.0 (2026-09-07): Initiale Fassung aus dem Owner-Entwurf ZKP-Layer (AD-045). Status: CANDIDATE — wartet auf APPROVED gemaess ATC-STD-000 §9.

## References

**NORMATIVE**
- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung, v1.1.0 APPROVED)
- ATC-STD-ZKP-001 — ZKP Architecture Standard
- registry/categories.yaml — zkp-Serie (ATC-STD-ZKP-001-999, AD-045)

**INFORMATIVE**
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie, Designentscheidung Verifikationsschicht statt eigenem Netzwerk)
- ZKP_ARCHITECTURE.md (atc-zkp/docs) — Zielarchitektur und Anwendungsfaelle
