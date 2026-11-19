---
document_id: ATC-KAI-OS-OWNERSHIP-001
title: Repository Ownership Matrix
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# Repository Ownership Matrix

Regel: genau ein Eigentuemer je Funktion. Keine zweite Implementierung von
Kernel oder VM.

| Funktion | Alleiniger Eigentuemer | Bestand 17.11.2026 | Konflikt |
|---|---|---|---|
| Sprachkern, Compiler, Stdlib | `atclang` | vorhanden | K-03: Parallelbestand `a-townchain/ai/atclang` |
| Bytecode-Ausfuehrung | `a-townchain/components/vm` | vorhanden | `atc-vm` FREEZE, nicht wiederbeleben |
| Protokollalgorithmen | `a-townchain/components/algorithm` | vorhanden | - |
| Wallet, Self-Custody | `a-townchain/components/wallet` | vorhanden | abgegrenzt von archiviertem `atc-wallet` |
| Blockchain, Netzwerk, Ledger | `a-townchain` | HEAD `5a9add0e24` | - |
| Kernel, Authority, Capabilities | `globus-os/modules/atc-shivacore/kernel/` | vorhanden, `syscall.rs` mit HandleClose und Test | keine parallele Rechteverwaltung erlaubt |
| Betriebssystem, Dienste, Userland | `globus-os` | HEAD `a38738dfcd` | Desktop-Schicht: 12 von 32 Anforderungen offen |
| KI-Plattform, Runtime, Agenten | `aurora-ai` | HEAD `2855956b32` | nicht Teil von Chain oder VM |
| Entwickler-SDK | `atc-sdk` | vorhanden | - |
| Security, Tools, SDK des Kernels | `atc-shivacore` | vorhanden | **nicht** Eigentuemer der Kernelimplementierung |
| Normative Vertraege | `atc-standards` | HEAD `5ef28537b8` | - |
| Architektur, Integration, Evidenz | `a-townchain-ecosystem` | HEAD `e18bdb90db` | - |
