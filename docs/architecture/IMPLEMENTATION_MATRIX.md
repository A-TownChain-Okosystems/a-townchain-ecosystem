---
document_id: ATC-KAI-OS-IMPL-001
title: Implementation Matrix
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# Implementation Matrix

Leiter: IMPLEMENTED, TESTED, CI_VERIFIED, E2E_VERIFIED, VERIFIED. Stand
17.11.2026, nur belegte Eintraege. Leere Zellen bedeuten "nicht nachgewiesen",
nicht "nicht vorhanden".

| Domaene | Repo-HEAD | IMPLEMENTED | TESTED | CI_VERIFIED | E2E_VERIFIED |
|---|---|---|---|---|---|
| ATCLang | `?` | teilweise (Lexer, Parser, IR) | projektweise | nein | nein |
| ATC-VM | `5a9add0e24` | teilweise | projektweise | nein | nein |
| A-TownChain | `5a9add0e24` | teilweise | projektweise | nein | nein |
| ShivaCore Kernel | `a38738dfcd` | weitgehend (Syscalls inkl. HandleClose) | ja, Kernel-Suiten gruen | nein | nein |
| GlobusOS | `a38738dfcd` | teilweise | teilweise | nein | nein |
| Aurora AI | `2855956b32` | teilweise | 31 Tests gruen | nein | nein |
| Standards | `5ef28537b8` | ja | ja | main gruen | nein |

Konsequenz: die Aussage "Systemfunktion verifiziert" ist fuer keine Domaene
tragfaehig. Die vier Kennzahlen werden getrennt gefuehrt: gemergte PRs,
bestandene CI-Pruefungen, geschlossene Blocker, verifizierte Systemfunktionen.
