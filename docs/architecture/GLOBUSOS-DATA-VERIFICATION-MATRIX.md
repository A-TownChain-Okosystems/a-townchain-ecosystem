---
document_id: ATC-GLOBUSOS-DATA-MATRIX-001
title: GlobusOS Data Verification Matrix
version: 1.0.0
status: PROPOSED
type: Anlage zu ATC-GLOBUSOS-DATA-ARCH-001
created: 2026-11-17
---

# GlobusOS Data Verification Matrix

Erhoben am 17.11.2026 im Bestand `globus-os`. Statusvokabular aus der
Gap-Matrix des Repositories, keine neue Skala.

## 1. Verarbeitungsschritte

| Schritt | Befund im Bestand | Status |
|---|---|---|
| Dateneingang ueber Treiber und HAL | `system/devices` vorhanden, echte Treiber offen | SKELETON |
| Annahme und IPC-Weiterleitung | `system/ipc`, `system/services` vorhanden | IMPLEMENTED_UNTESTED |
| Sicherheitspruefung vor Verarbeitung | `system/security/capability_registry.rs`, `identity.rs`; Kernel `cap_authority.rs` | IMPLEMENTED_UNTESTED |
| Verarbeitung CPU | `system/runtime`, `system/process`, `system/memory` | IMPLEMENTED_UNTESTED |
| Verarbeitung GPU | `system/graphics` mit GPU-Klassifikation, Hardware-Backend offen | SKELETON |
| Verarbeitung NPU | kein Backend im Bestand; Treffer nur in Shell- und Greeter-Texten | MISSING |
| Speicherung dauerhaft | `system/vfs` mit `persistent.rs`, `journal.rs`, `transaction.rs`, `recovery.rs`, `gpt.rs`, `file_store.rs`, `dir_store.rs`, 37 Tests | TESTED, Backend gegen echte Hardware offen |

## 2. Zehn Datenfunktionen

| Funktion | Befund | Status |
|---|---|---|
| Data Management | kein einheitlicher Dienst, verteilte Einzelpfade | MISSING |
| Data Classification | kein Klassifizierungsmerkmal im Datenpfad | MISSING |
| Data Access Control | Capability-Registry vorhanden, kein dateibezogenes Rechtekonzept | SKELETON |
| Data Processing Engine | keine eigene Engine, Verarbeitung verteilt | MISSING |
| Data Storage Management | VFS als Grundlage vorhanden, keine Quota- oder Lebenszyklusverwaltung | SKELETON |
| Data Encryption Engine | Keystore-Grenze in `system/identity` (Schluessel verlassen die Grenze nicht), keine Verschluesselung ruhender Dateien | SKELETON |
| Data Integrity | Journal und Transaktionen im VFS, Audit-Kette fuer Capability-Aenderungen mit Test `audit_chain_intact` | IMPLEMENTED_UNTESTED |
| Data Lifecycle Management | keine Aufbewahrungs- oder Loeschregeln | MISSING |
| Data Synchronization | kein Bestand, nur Netzwerk-Grundlage | MISSING |
| AI Data Interface | `system/aurora-core` mit contracts, state, types; keine Autorisierungsschicht | SKELETON |

## 3. Sicherheitsmechanismen

| Mechanismus | Befund | Status |
|---|---|---|
| Capability Security | `capability_registry.rs`, Kernel `cap_authority.rs`, Audit-Kette | IMPLEMENTED_UNTESTED |
| Prozessisolation | `system/process`, Kernel-Scheduler | IMPLEMENTED_UNTESTED |
| Memory Protection | `system/memory`, VMM mit Mapping-Tests | TESTED im Kernel, Integration offen |
| IPC Security | `system/ipc`, Rechtepruefung offen | SKELETON |
| Data Encryption | Keystore ja, ruhende Daten nein | SKELETON |
| Audit Logging | Audit-Kette fuer Capability-Mutationen, keine Abdeckung fuer Datei- und Netzzugriffe | SKELETON |
| Secure Boot | in Roadmap und Hardware-Doku gefordert, kein Nachweis | MISSING |

## 4. Korrekturen an der Einordnung der Vorlage

1. **Persistente Speicherung ist weiter als "Architektur festgelegt".**
   `system/vfs` enthaelt persistente Speicherung, Journal, Transaktionen,
   Recovery und GPT-Lesen mit 37 Tests. Die Formulierung des Komponenten-
   katalogs, VFS sei nur ein Namespace- und Mount-Vertrag, ist ueberholt.
2. **Aurora-Integration ist nicht "vorhanden", sondern Vertrag.**
   `system/aurora-core` liefert Vertraege, Zustand und Typen, keine Inferenz
   und keine Autorisierungsschicht.
3. **Klassifizierung ist der eigentliche Engpass.** Ohne Classification im
   Datenpfad sind Access Control, Lifecycle und AI Data Interface nicht
   durchsetzbar, weil keine Schutzstufe existiert, an der eine Policy
   ansetzen kann.
4. **NPU ist im Bestand nicht vorhanden.** Nennungen beschraenken sich auf
   Anzeigetexte.

## 5. Konsequenz fuer die Reihenfolge

Nicht mit dem AI Data Interface beginnen, sondern mit Classification und
Access Control. Ein KI-Datenzugriff ohne Schutzstufe ist nicht pruefbar.
