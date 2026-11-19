---
document_id: ATC-KAI-OS-ROADMAP-001
title: KAI-OS Master Development Roadmap
version: 1.0.0
status: PROPOSED
type: Architecture Control Document, NICHT normativ
normative_authority: atc-standards (ATC-STD-000, ATC-ARCH-001, ATC-ARCH-002)
owner: Michael Wroblewski
created: 2026-11-17
references: [ATC-ARCH-001, ATC-ARCH-002, ATC-STD-000, AD-008, ATC-BLOCKER-REGISTER-001]
annexes: [REPOSITORY_OWNERSHIP_MATRIX, SYSTEM_DEPENDENCY_GRAPH, IMPLEMENTATION_MATRIX, MISSING_MODULES_REGISTER, ARCHITECTURE_CONFLICTS, SECURITY_REQUIREMENTS, INTEGRATION_ACCEPTANCE_CRITERIA, DEVELOPMENT_PHASES]
---

# KAI-OS Master Development Roadmap

> **Charakter:** Architektur- und Steuerungsdokument der Integrations- und
> Evidenzebene. Es setzt **keine** Normen. Verbindliche technische Verträge
> entstehen ausschliesslich in `atc-standards` ueber ATC-STD-000 und ein
> genehmigtes SCR. Wo dieses Dokument eine Festlegung braucht, verweist es auf
> den Standard und markiert die Luecke, statt sie zu schliessen.

## 1. Sechs Entwicklungsdomaenen

| Domaene | Repository | Verifiziert am 17.11.2026 |
|---|---|---|
| ATCLang | `atclang` | vorhanden, HEAD `nicht erhoben` |
| ATC-VM | `a-townchain/components/vm` | Pfad vorhanden; `atc-vm` ist FREEZE (SCR-0127), keine zweite Implementierung |
| A-TownChain | `a-townchain` | HEAD `5a9add0e24` |
| ShivaCore Kernel | `globus-os/modules/atc-shivacore/kernel/` | Pfad vorhanden; `atc-shivacore` bleibt Security, Tools, SDK |
| GlobusOS | `globus-os` | HEAD `a38738dfcd` |
| Aurora AI | `aurora-ai` | HEAD `2855956b32` |

Steuerungsebenen: `atc-standards` (normativ, HEAD `5ef28537b8`) und
`a-townchain-ecosystem` (Architektur-, Integrations- und Evidenzkontrolle).

Die Darstellung beschreibt Verantwortungsbereiche, keine Hierarchie.

## 2. Drei Eigenschaften je System

Standalone, native, integrationsfaehig. Die Domaenen teilen Vertraege, nicht
Implementierungen. Kernel und GlobusOS bleiben getrennte Entwicklungsdomaenen,
auch wenn der Kernel physisch im GlobusOS-Repository liegt.

## 3. Zehn Entwicklungsphasen

Details je Phase: `DEVELOPMENT_PHASES.md`. Reihenfolge: Standards, Kryptografie
und Identitaet, Kernel, VM, ATCLang, Blockchain, Systemdienste, KI, Integration,
Security und Release.

## 4. Vier parallele Spuren

Ausfuehrung (VM, ATCLang), Blockchain, Betriebssystem (Kernel, GlobusOS), KI.
Gemeinsame Klammer sind die Vertraege aus Phase 0 und die Integrationsebene.

## 5. Qualitaetsleiter

IMPLEMENTED, TESTED, CI_VERIFIED, E2E_VERIFIED, VERIFIED, dazu RESIDUAL,
BLOCKED, EXCLUDED. Ein Nachweis nennt Default-Branch-HEAD, Commit-SHA, Pfad,
Symbol oder Blob-SHA, Run, Job, Step, Log und Artefakt-Hash. **CI gruen ist nicht
VERIFIED.**

Stand 17.11.2026: keine der sechs Domaenen traegt CI_VERIFIED oder hoeher fuer
eine Systemfunktion. `IMPLEMENTED` ist fuer mehrere Domaenen belegt, `TESTED`
nur projektweise.

## 6. Erste Arbeitspakete

| AP | Inhalt | Status 17.11.2026 |
|---|---|---|
| AP-001 | Repository-Inventur gegen aktuelle Default-Branch-HEADs | teilweise: 13 Repositories mit HEAD erhoben, `atclang` identifiziert |
| AP-002 | Architektur- und SSOT-Matrix je Funktion | Entwurf in `REPOSITORY_OWNERSHIP_MATRIX.md`, drei Konflikte offen |
| AP-003 | P0-Konfliktbereinigung | offen, siehe `ARCHITECTURE_CONFLICTS.md` |
| AP-004 | ShivaCore und ATC-VM vervollstaendigen | offen |
| AP-005 | ATCLang-Compiler mit Blockchain-Ausfuehrung verbinden | offen |

## 7. Korrekturen an der Vorlage

Drei Angaben der Vorlage halten der Pruefung nicht stand oder sind praeziser
gefasst:

1. **ATCLang-Repository ist identifiziert.** `atclang` existiert, ebenso ein
   zweiter Bestand unter `a-townchain/ai/atclang`. Die Inventur ist damit nicht
   offen, sondern fuehrt einen Eigentumskonflikt (K-03).
2. **Chain-ID 658467** ist in `a-townchain/AGENT_MANIFEST.md` als
   `AD-004 RESOLVED` gefuehrt. Der Registerstand des Entscheidungsregisters ist
   separat zu pruefen, der Repo-Beleg ist vorhanden.
3. **Max Supply ist widerspruechlich.** `apps/atc-contracts/atcoin/atcoin.atc`
   setzt 360.000.000 ATC als kanonisches L1-Limit, der Test
   `apps/atc-contracts/exec_chain/tests/t03_atcoin_supply_cap.atc` arbeitet mit
   21.000.000. Das ist ein Konsens-Konflikt, nicht eine Konfigurationsfrage
   (K-01).

## 8. Was dieses Dokument nicht leistet

Es enthaelt keine Implementierungsnachweise, keine Gate-Ergebnisse und keine
Freigaben. Der belastbare Stand je Blockade steht ausschliesslich im
`BLOCKER_REGISTER.yaml` mit den Feldern status, evidence, evidence_head,
verification, closure_criteria, last_verified und supersedes.
