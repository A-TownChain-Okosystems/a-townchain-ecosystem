---
document_id: ATC-KAI-OS-CONFLICTS-001
title: Architecture Conflicts
version: 0.2.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
updated: 2026-11-17
---

# Architecture Conflicts

Jeder Konflikt ist entweder **beseitigt** (mit Beleg) oder **offen** (mit
Entscheidungsbedarf). Nichts wird stillschweigend aufgeloest.

## Beseitigt

| ID | Konflikt | Aufloesung | Beleg |
|---|---|---|---|
| K-01 | Max Supply 360.000.000 gegen 21.000.000 | 360.000.000 ist kanonisch, der Test war die Abweichung und ist angeglichen | `atcoin.atc` (ATC_MAX_SUPPLY), `docs/SMART_CONTRACTS.md`, `docs/audits/CONTRACT-INTEGRITY-2026-11-08.md`, Komponentenkatalog; Fix a-townchain `3c7b4efa0` |
| K-02 | Chain-ID 658467 angeblich ungeklaert | AD-004 ist am 04.09.2026 mit 658467 entschieden | `registry/OVERVIEW.md`, `registry/STANDARDS_REGISTRY.md`, `ATC-99`; zusaetzlich `a-townchain/AGENT_MANIFEST.md` |
| K-01a | Obergrenze selbst | per Owner-Entscheidung 08.11.2026 auf 360.000.000 ATC festgelegt, 21 Mio ist die ausdruecklich nicht autorisierte Grenze | `tools/atc-sc-validator/tests/test_supply_cap_invariant.py`; Angleichen beider Testkopien in `3c7b4efa0` und `e923e89cd` |
| K-03 | Zwei ATCLang-Bestaende | `ai/atclang` ist als Kopie deklariert, kanonisch bleibt `atclang` | `a-townchain/ai/atclang/VENDORED.md` im Commit `3c7b4efa0`. Offen bleibt nur, ob der Baum entfernt wird |
| K-05 | `atc-vm` gegen `components/vm` | `atc-vm` ist FREEZE nach SCR-0127, kanonisch ist `a-townchain/components/vm` | Registry, Ownership-Matrix |
| K-07 | ATC-ARCH-001 und ATC-ARCH-002 gleichzeitig normativ | kein Widerspruch: ARCH-001 bleibt normativ, ARCH-002 tritt als zweite Quelle hinzu, beide mit getrennten Zustaendigkeiten (Gesamtarchitektur gegen Desktop-Schicht) | Owner-Instruktion, ATC-ARCH-002 §3.1 |

## Offen, Entscheidung erforderlich

| ID | Konflikt | Beleg | Blockade |
|---|---|---|---|
| K-04 | SCR-0129 mehrfach belegt: 1 Datensatz, 3 Pull Requests mit anderem Inhalt, davon einer gemergt | BLK-P2-12 | SCR-Freigabe nicht zuordenbar |
| K-06 | Consumer pinnen `reusable-markdownlint.yml`, die im Hub entfernt ist | dauerhaftes Alt-Rot in acht Repos | workflow-Scope |
| K-08 | Signaturverfahren: ATC-STD-600 §5.2 setzt Ed25519 als kanonisch, der Komponentenkatalog nennt fuer TX-V2 secp256k1 ECDSA, der Node-Code nutzt secp256k1 | `standards/os/ATC-STD-600.md` gegen `ATC-COMPONENT-FUNCTION-CATALOG-001.md` und `components/node/src/{consensus,did,mempool,validator}.rs` | BLK-P1-11, Kryptografie-Entscheidung ist Owner-Sache |
| K-10 | Feldbreite: ATC-STD-600 fuehrt `amount` als `u64`, 18 Dezimalstellen und 360.000.000 ATC erfordern aber rund 3,6e26 und damit u128 | `standards/os/ATC-STD-600.md` § 4 gegen `atcoin.atc` und Katalog (`u128`, 16 Byte BE) | BLK-P1-12 |

| K-11 | Vertragsbaum dreifach vorhanden (`apps/atc-contracts`, `components/contracts`, `modules/atc-contracts`) | BLK-P1-14: ein Fix erreichte nur eine von zwei Kopien | nur eine kanonische Quelle je Vertrag |
| K-12 | Governance-Vertraege setzen `total_supply` auf 21 Mio, kanonisch sind 360 Mio | auf 360e24 korrigiert in allen vier Kopien, zusaetzlich Vektor t03 und Hinweis in t04 (Commit `9ebc6dec6`, a-townchain #115) | Reststand nur in Archivbaeumen und im VM-E2E-Beispiel |

## Hinweis zu K-08 und K-10

Beide Konflikte liegen **in der normativen Quelle selbst** beziehungsweise
zwischen Norm und Umsetzung. Sie sind nicht durch eine Doku-Anpassung zu
loesen: eine Aenderung des Signaturverfahrens oder der Feldbreite ist eine
konsens- und kryptografie-kritische Entscheidung. Bis zur Entscheidung gilt
keine der beiden Varianten als freigegeben.
