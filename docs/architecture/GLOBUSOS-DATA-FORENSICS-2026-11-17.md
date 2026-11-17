---
document_id: ATC-GLOBUSOS-DATA-FORENSICS-001
title: GlobusOS Data Classification Forensics
version: 1.0.0
status: FINAL (Erhebung), nicht normativ
type: Anlage zu ATC-GLOBUSOS-DATA-ARCH-001
date: 2026-11-17
repository_inspected: globus-os
tracking_anchor: BLK-P2-13
references: [ATC-GLOBUSOS-DATA-ARCH-001, ATC-GLOBUSOS-DATA-MATRIX-001, GLOBUSOS_EVIDENCE_BACKED_GAP_MATRIX]
---

# GlobusOS Data Classification Forensics

Auftrag: vor jedem Implementierungsauftrag feststellen, ob eine
Schutzklassifizierung existiert, ob sie ueber Schnittstellen mitgefuehrt wird
und an welchen Stellen eine Durchsetzung fehlt.

Methode: Lesen der Datentypen in `globus-os` (32 Workspace-Member, 25 Crates
unter `system/`, plus `modules/`), Begriffssuche ueber alle Rust-Quellen,
Pruefung der Uebergaenge IPC, VFS, Aurora, Audit. Keine Ausfuehrung, keine
Laufzeitpruefung.

## 1. Begriffslage im Code

| Begriff | Dateien | Bewertung |
|---|---|---|
| `sensitivity` | 0 | nicht vorhanden |
| `data_class` / `trust_class` / `provenance` | 0 | nicht vorhanden |
| `classification` | 2 | nur Seitenfehler-Klassifikation (`memory/fault.rs`, Legacy `page_fault.rs`), kein Datenbezug |
| `confidential` | 2 | nur Kommentare in `identity/file_store.rs`, `identity/persistence.rs` als Absichtserklaerung |
| `restricted` | 2 | Ring-3-Rechte und "unrestricted operation for trusted services" in `net/lib.rs` |

Ergebnis: ein Schutzstufenmodell existiert im Code nicht. Es gibt Rechte
(Capabilities), aber keine Schutzstufe, an der ein Recht ansetzen koennte.

## 2. Die sieben Prueffelder

| Prueffeld | Befund | Status |
|---|---|---|
| Klassifizierung | kein Schutzstufenmodell, keine verbindliche Datenzuordnung | MISSING |
| Datenfluss | `ipc::MessageHeader` traegt `endpoint`, `opcode`, `payload_len`; `Message` traegt `header` und `payload: Vec<u8>`. Kein Label, keine Absenderidentitaet, kein Capability-Bezug | MISSING |
| Zugriffskontrolle | `security::Capability(u128)`, `Right`, `Grant`, `Authorization`, `CapabilityObject`, Registry vorhanden; Rechte sind operationsbezogen, ohne Datendimension | SKELETON fuer Datenzugriff |
| Persistenz | `vfs`-Inode traegt `mode: u16`, `file` traegt `FileMode`. Keine Schutzstufe, keine Herkunft | SKELETON |
| Lifecycle | keine Aufbewahrungs- oder Loeschregeln im Code | MISSING |
| Aurora | `aurora-core` hat `ToolRequest`, `ModelRequest`, `ExecutionRequest`, `CapabilityResult`; keine Klassifizierung vor der Uebergabe | SKELETON |
| Audit | Audit-Kette fuer Capability-Mutationen mit Test `audit_chain_intact`; keine Spur fuer Datenzugriffe oder Policy-Entscheidungen | SKELETON |

## 3. Fehlende Durchsetzungspunkte

| Punkt | Warum dort | Was fehlt |
|---|---|---|
| D-1 IPC-Header | jede Nachricht zwischen Diensten laeuft hier durch | ein Labelfeld und die Regel, dass Dienste ohne passende Capability die Nachricht verwerfen |
| D-2 VFS-Inode | jede dauerhafte Speicherung setzt hier an | Schutzstufe und Herkunft, mitgeschrieben und beim Lesen geprueft |
| D-3 Aurora-Aufruf | jeder Datenabruf der KI laeuft hier durch | Klassifizierung und Autorisierung vor der Uebergabe |
| D-4 Netzausgang | jede foederierte oder externe Weitergabe | Pruefung gegen die Schutzstufe |
| D-5 Audit-Erweiterung | Zugriffe sind heute nicht nachvollziehbar | Datensatz fuer Zugriff und Policy-Entscheidung in der bestehenden Kette |

## 4. Warum bestehende Komponenten nicht dupliziert werden muessen

- `security::CapabilityRegistry` und `cap_authority` im Kernel tragen die Rechte bereits. Eine Schutzstufe ist eine zusaetzliche Dimension, kein zweites Rechtssystem.
- Die Audit-Kette ist vorhanden und getestet. Sie braucht neue Datensaetze, kein zweites Log.
- `identity` haelt Schluessel innerhalb der Keystore-Grenze und gibt nur Signaturen heraus. Das ist die Vertrauensgrenze fuer Verschluesselung, kein neuer Krypto-Stack.
- `vfs` bringt Journal, Transaktion und Recovery mit. Die Schutzstufe ist ein Feld im Inode und ein Pruefpunkt beim Lesen, kein zweites Dateisystem.
- `aurora-core` liefert den Vertrag. Die Integration erfolgt dort, nicht ueber einen neuen Kanal.

## 5. Was diese Erhebung nicht abdeckt

- Die API-Oberflaechen der einzelnen Dienste wurden nicht Zeile fuer Zeile gelesen, sondern ueber Datentypen und Begriffe geprueft.
- Netzwerkpfade, Paket- und Update-Kette sowie `virtualization` wurden nicht im Detail untersucht.
- Es wurde kein Laufzeitverhalten geprueft; alle Aussagen beziehen sich auf Quelltext und Typen.

## 6. Empfehlung fuer die Reihenfolge

Die beiden zentralen Durchsetzungspunkte D-1 und D-2 sind einzelne Strukturen
(`MessageHeader`, Inode). Damit ist eine minimale, nicht duplizierende
Erweiterung moeglich: Schutzstufe im Header, Schutzstufe im Inode, Pruefung
an beiden Uebergaengen, Audit-Datensatz in der bestehenden Kette.

Klassifizierung vor Lifecycle und vor jedem KI-Zugriff. Ohne Schutzstufe im
Header ist jede spaetere Policy nicht durchsetzbar, weil die Information am
Uebergang verloren geht.
