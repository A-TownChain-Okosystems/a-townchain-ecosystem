---
document_id: ATC-KAI-OS-CONFLICTS-001
title: Architecture Conflicts
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# Architecture Conflicts

Jeder Konflikt wird entschieden, nicht stillschweigend aufgeloest.

| ID | Konflikt | Beleg | Wirkung |
|---|---|---|---|
| K-01 | Max Supply: 360.000.000 ATC gegen 21.000.000 | `apps/atc-contracts/atcoin/atcoin.atc` gegen `exec_chain/tests/t03_atcoin_supply_cap.atc` | konsenskritisch, BLK-P1-10 |
| K-02 | Chain-ID 658467 | `a-townchain/AGENT_MANIFEST.md`: AD-004 RESOLVED | Repo-Beleg vorhanden, Registerstand zu pruefen |
| K-03 | Zwei ATCLang-Bestaende: `atclang` und `a-townchain/ai/atclang` | Pfadpruefung 17.11.2026 | Eigentum ungeklaert, AP-002 |
| K-04 | SCR-0129 mehrfach belegt | BLK-P2-12 | Freigabe nicht zuordenbar |
| K-05 | `atc-vm` FREEZE gegen kanonischen Pfad `components/vm` | SCR-0127 | keine Wiederbelebung |
| K-06 | markdownlint-Consumer pinnen eine im Hub entfernte Reusable-Datei | dauerhaftes Alt-Rot in acht Repos | workflow-Scope noetig |
| K-07 | ATC-ARCH-001 und ATC-ARCH-002 gleichzeitig normativ | `atc-standards` main | Verhaeltnis muss benannt werden, BLK-P1-04 |
| K-08 | ATC-STD-600 (u64 BE, Ed25519) gegen secp256k1-Bestaende | `standards/os/ATC-STD-600.md` gegen `components/node/*` | nicht abschliessend geprueft |
| K-09 | Capability-Authority: ShivaCore als einzige Quelle | ATC-ARCH-002, AD-008 | parallele Rechteverwaltung unzulaessig |
