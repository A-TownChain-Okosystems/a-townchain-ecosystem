---
document_id: ATC-KAI-OS-DEPGRAPH-001
title: System Dependency Graph
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# System Dependency Graph

Kanten sind Integrationsabhaengigkeiten, keine Entwicklungsblockaden. Jede
Domaene ist ohne die nachfolgende entwickelbar.

- ATCLang erzeugt Bytecode fuer die ATC-VM. Die Sprache ist ohne lauffaehige VM entwickelbar, die Ausfuehrung nicht.
- ATC-VM konsumiert Bytecode aus ATCLang und liefert Ausfuehrung an A-TownChain.
- A-TownChain nutzt die VM fuer Vertragsausfuehrung, ihre Netzwerkschicht haengt nicht an der VM.
- ShivaCore liefert Capabilities und Syscalls an GlobusOS und autorisiert Aurora-Ressourcen.
- GlobusOS nutzt den Kernel und stellt Dienste bereit, auf denen Aurora laeuft.
- Aurora laeuft lokal ohne Chain und ohne VM; die Bindung an ShivaCore erfolgt ueber Capabilities.
- Wallet spricht mit A-TownChain ueber den Transaktionsvertrag.
- atc-standards liefert die Vertraege an alle; a-townchain-ecosystem prueft die Einhaltung.

Kritischer Pfad fuer M8: Transaktions- und Kryptografievertrag -> VM-Bytecode ->
ATCLang-Backend -> Chain-Ausfuehrung -> Kernel-Capability -> Systemintegration.
