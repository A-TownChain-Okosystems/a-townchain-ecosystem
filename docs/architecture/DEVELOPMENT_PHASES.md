---
document_id: ATC-KAI-OS-PHASES-001
title: Development Phases
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# Development Phases

| Phase | Inhalt | Eigentuemer | Austrittskriterium |
|---|---|---|---|
| 0 | Architektur und Standards | atc-standards | keine widerspruechlichen Vertraege |
| 1 | Kryptografie und Identitaet | a-townchain, globus-os, aurora-ai | gemeinsame Testvektoren |
| 2 | ShivaCore Kernel Core | globus-os | bootfaehiger Kernel mit Authority |
| 3 | ATC-VM | a-townchain | deterministische, isolierte Ausfuehrung |
| 4 | ATCLang | atclang | Compiler bis Bytecode, unabhaengige Validierung |
| 5 | A-TownChain | a-townchain | mehrere Nodes, identischer Ledger |
| 6 | GlobusOS-Systemdienste | globus-os | eigenstaendiger Start, Update, Recovery |
| 7 | Aurora AI | aurora-ai | lokale Inferenz und Agenten innerhalb von Capabilities |
| 8 | Gesamtintegration | a-townchain-ecosystem | funktionierende Systemverbindungen |
| 9 | Security, E2E, Release | alle | Sicherheits- und Release-Evidenz |

Eine Phase ist nicht abgeschlossen, weil Module existieren, sondern wenn das
Austrittskriterium mit Nachweis nach der Qualitaetsleiter erfuellt ist.
Phase 0 ist derzeit nicht abgeschlossen: drei Vertragskonflikte sind offen
(K-01, K-03, K-04).
