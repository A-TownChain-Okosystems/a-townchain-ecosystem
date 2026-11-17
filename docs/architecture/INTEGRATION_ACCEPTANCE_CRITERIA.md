---
document_id: ATC-KAI-OS-INTEGRATION-001
title: Integration Acceptance Criteria
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# Integration Acceptance Criteria

| Integration | Abnahmekriterium | Status 17.11.2026 |
|---|---|---|
| ATCLang + ATC-VM | Programm kompiliert, unabhaengig validiert, auf der VM ausgefuehrt, identischer Zustandsuebergang bei Wiederholung | nicht nachgewiesen |
| ATC-VM + A-TownChain | Vertragsausfuehrung deterministisch, Gas begrenzt, Abbruch eindeutig | nicht nachgewiesen |
| ShivaCore + GlobusOS | Sicheres Betriebssystem mit getesteter Authority und vollstaendiger Ressourcenfreigabe | Kernel-Suiten gruen, Integration nicht nachgewiesen |
| ShivaCore + Aurora | KI-Ressourcen nur ueber Capabilities, Isolationsverletzung fail-closed | nicht nachgewiesen |
| GlobusOS + Aurora | lokale Inferenz als Systemdienst | nicht nachgewiesen |
| Wallet + A-TownChain | Signierte Transaktion, Self-Custody, Replay-Schutz | nicht nachgewiesen |
| ATCLang + SDK | Entwicklerzugriff auf Compiler und VM ueber stabile Schnittstelle | nicht nachgewiesen |
| Alle sechs Domaenen | KAI-OS-Zustand, ein Ledger, ein Kernel, eine VM | nicht nachgewiesen |

Ein Integrationstest gilt nur mit Run, Job, Step, vollstaendigem Log und
archiviertem Artefakt als bestanden. Ein gruener Einzeljob ist kein
Integrationsnachweis.
