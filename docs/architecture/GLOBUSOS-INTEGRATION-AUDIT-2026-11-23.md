---
document_id: ATC-GLOBUSOS-INTEGRATION-AUDIT-001
title: GlobusOS Integration Audit und Schnittstellen-Fehlerregister
version: 1.0.0
status: FINAL (Erhebung), nicht normativ
audited_commit: a38738dfcd91125fb00317b93aacb342b4515e2a
verified_at_head: 0c792e3d
method: Quellcode-Lesen am Default-Branch, kein neuer Hardwarelauf
date: 2026-11-23
---

# GlobusOS Integration Audit

Statische Prüfung der Verbindungen zwischen den vorhandenen Komponenten, nicht der
Dokumentation. Grundlage ist der vom Owner benannte Commit `a38738df`; die Befunde
wurden zusätzlich am aktuellen HEAD `0c792e3d` gegengeprüft, weil dazwischen vier
Merges liegen (Registerfenster, Test-Mock, DMA-Allokator, Queue-inflight).

## 1. Verifikation der vorgelegten Befunde

Alle acht geprüften Befunde halten stand. Keiner war falsch.

| Nr | Befund | Urteil | Fundstelle |
|----|--------|--------|------------|
| 1 | LKM nur im Testbuild eingebunden | BESTÄTIGT | `kernel/src/lib.rs:107-109`, `#[cfg(test)] pub mod lkm;` |
| 2 | Service Space wird im Bootpfad nicht gestartet | BESTÄTIGT | kein Treffer für `ServiceSpaceState` in `kernel/src` |
| 3 | Service Space hängt direkt am Kernel-Crate | BESTÄTIGT | `service_space/Cargo.toml:27`, `shivacore = { path = "../kernel" }` |
| 4 | `qemu_boot.sh` bricht bei fehlendem Meilenstein nicht ab | BESTÄTIGT | `scripts/qemu_boot.sh:95`, `[FEHLT]` ohne `exit 1`; `die()` nur bei Vorfehlern |
| 5 | Parallele Capability-Verantwortung | BESTÄTIGT | Kernel: `capability.rs`, `kernel_caps.rs`, `cap_authority.rs`; System: `system/security/src/capability_registry.rs` |
| 6 | Parallele VFS-Verantwortung | BESTÄTIGT | Kernel: `vfs.rs`, `atcfs.rs`; System: `system/vfs` mit `PersistentFs<D: BlockDevice>` |
| 7 | Widersprüchliche Testzahlen | BESTÄTIGT | `README.md:200-202` nennt 1687 gesamt, 698 shivacore, 552 service-space |
| 8 | Prozessstart-ABI offen | BESTÄTIGT | `kernel/src/spawn.rs:14`, "ueber die ABI (SpawnProcess) ist noch nicht verdrahtet" |

### Nachtrag zur Testzahl

`evidence.yaml` bindet 1.864 Tests an `bound_commit: c75bb769`, nicht an `a38738df`.
Der Owner hat damit recht, dass die Zahl nicht HEAD-gebunden belegt ist.

Ergänzung, die den Befund präzisiert und nicht entkräftet: ein lokaler
`cargo test --workspace` auf `a38738df` ergibt 1.864 bestandene Tests, 0 rot. Die
Zahl ist also reproduzierbar, aber sie ist ein lokaler Lauf des Agenten ohne
CI-Bindung und ohne Hardware. Sie bleibt eine eigene Kennzahl, keine Freigabe.

## 2. Schnittstellenregister

Legende Erreichbarkeit: ERREICHBAR heisst, ein Aufrufer im laufenden System existiert
im Quellcode. NACHGEWIESEN heisst, ein Integrationstest verbindet beide Seiten.

| ID | Schnittstelle | Eigentümer | Erreichbar | Nachgewiesen | Status |
|----|---------------|-----------|-----------|--------------|--------|
| I-01 | Bootloader → Kernel-Einstieg | `atc-bootloader` | ja | QEMU-Lauf 14.11. | TESTED (nicht HEAD-gebunden) |
| I-02 | Kernel → Userspace/Ring 3 | `shivacore` | ja | QEMU-Lauf 14.11. | TESTED (nicht HEAD-gebunden) |
| I-03 | Kernel → Prozessstart-ABI | `shivacore` | nein | nein | BLOCKED (I-D-01) |
| I-04 | Kernel → Service Space | `shivacore` / `service_space` | nein | nein | BLOCKED (I-D-02) |
| I-05 | Service Space → Kernel (Rückgriff) | `service_space` | ja | nein | IMPLEMENTED, Kopplung (I-D-03) |
| I-06 | VFS → BlockDevice | `globus-vfs` | ja | ja (Unit) | TESTED |
| I-07 | BlockDevice → NVMe | `globus-devices` | ja (Trait) | Modell + Fenster | TESTED ohne Hardware (I-D-04) |
| I-08 | NVMe → MMIO/BAR | `globus-devices` | ja (`BarWindow`) | ja (Unit) | TESTED, kein physisches Mapping |
| I-09 | NVMe → DMA | `globus-devices` | ja (`FrameAllocator`) | ja (Integration) | TESTED, simulierter Pool |
| I-10 | DMA → IOMMU | `globus-devices` | ja (`DmaRegion`) | ja (Unit) | TESTED, keine Hardwaretabellen |
| I-11 | NIC → Netzwerkstack → Socket | `globus-net` | nein | nein | BLOCKED (I-D-05) |
| I-12 | Greeter → Sitzung | `atc-shivacore/greeter` | teilweise | nein | PARTIAL (I-D-06) |
| I-13 | Capability-Autorität (eine Quelle) | `shivacore` | ja | nein | KONFLIKT (I-D-07) |
| I-14 | Audit-Kette → Datenzugriff | `shivacore` | teilweise | Capability-Änderungen | PARTIAL (I-D-08) |

## 3. Fehlerregister

Jeder Eintrag nennt Beleg, Wirkung und Abschlussbedingung. Kein Eintrag gilt als
behoben, solange nicht ein Integrationstest die Verbindung selbst nachweist.

### P0

**I-D-01 — Prozessstart-ABI nicht verdrahtet.** Beleg: `kernel/src/spawn.rs:14`.
Wirkung: kein produktiver Nutzerprozessstart, damit auch kein echter Login.
Abschluss: Aufrufpfad vom Syscall bis zum ELF-Loader mit Test, der einen Prozess
startet und dessen Rückkehr beobachtet.

**I-D-02 — Service Space nicht im Bootpfad.** Beleg: kein `ServiceSpaceState`-Bezug
im Kernel. Wirkung: Blockchain-, Netzwerk- und Contract-Dienste laufen in keinem
gestarteten System. Abschluss: Kernel startet den Service Space nach dem
Capability-Aufbau, mit Integrationstest.

**I-D-04 — Speicherstrecke ohne Hardware.** Beleg: `MemoryBlockDevice` in Tests,
`NvmeMmio` seit 23.11. mit `BarWindow`, `FrameAllocator` mit simuliertem Pool.
Wirkung: Dateisystemlogik bewiesen, Geräteanbindung nicht. Abschluss: physisches
BAR-Mapping und realer Frame-Bezug, verifiziert auf Hardware oder QEMU mit
durchgereichter NVMe.

**I-D-05 — Netzwerk ohne Gerätepfad.** Beleg: `system/net` vorhanden, kein
NIC-DMA-Pfad. Wirkung: kein Socket im laufenden System. Abschluss: Integrationstest
NIC → Stack → Socket.

### P1

**I-D-03 — Service Space greift direkt auf das Kernel-Crate zu.** Beleg:
`service_space/Cargo.toml:27`. Wirkung: keine stabile Grenze, jede Kernel-Änderung
ist eine Service-Space-Änderung. Abschluss: Zugriff ausschliesslich über
`libshivacore` (`system/shivacore-api`) als schmalen ABI-Vertrag.

**I-D-06 — Greeter ohne Sitzung.** Beleg: Oberfläche und Passwortprüfung vorhanden,
Sitzungseröffnung fehlt. Abschluss: Login startet über I-D-01 eine Sitzung.

**I-D-07 — Zwei Capability-Autoritäten.** Beleg: `kernel/src/cap_authority.rs` und
`system/security/src/capability_registry.rs`. Wirkung: Widerspruch zur
Owner-Vorgabe, dass ShivaCore die alleinige Authority-SSOT ist. Abschluss: eine
Implementierung, die andere als Fassade oder entfernt, mit Nachweis.

**I-D-08 — Audit-Kette deckt nur Capability-Änderungen.** Beleg: `audit_chain_intact`
in `cap_authority.rs`. Wirkung: Datenzugriffe sind nicht nachvollziehbar, was
BLK-P2-13 blockiert. Abschluss: Zugriffsergebnisse in derselben Kette.

**I-D-09 — Zwei VFS-Verantwortungen.** Beleg: `kernel/src/vfs.rs`, `atcfs.rs` gegen
`system/vfs`. Abschluss: Eigentümer festlegen, die andere Seite als Client.

**I-D-10 — Boot-Testskript meldet Erfolg ohne Erfolg.** Beleg:
`scripts/qemu_boot.sh:95`. Wirkung: ein unvollständiger Boot kann als grün gelten.
Abschluss: fehlender Pflicht-Meilenstein beendet mit Exit 1; Log, Image-SHA,
Commit-SHA und QEMU-Konfiguration werden archiviert.

**I-D-11 — LKM nicht im produktiven Binary.** Beleg: `kernel/src/lib.rs:107-109`.
Abschluss: entweder produktiv einbinden mit Lade-/Entladetest oder als
Testinfrastruktur deklarieren und aus dem Produktionsumfang nehmen.

**I-D-12 — Dokumentation nennt veraltete Testzahlen.** Beleg: `README.md:200-202`
gegen `evidence.yaml`. Abschluss: eine Quelle, Zahlen generiert statt gepflegt.

## 4. Readiness-Bewertung

| Prüfbereich | Bewertung |
|-------------|-----------|
| Quellcodebestand | umfangreich, 32 Crates, 84.409 Zeilen |
| Kernel-Grundfunktionen | IMPLEMENTED, lokal TESTED |
| Kernel-Build | TESTED (lokal, a38738df und HEAD) |
| Unit-Testbestand | 1.864 lokal grün, nicht CI-gebunden |
| QEMU-Boot | TESTED am 14.11., nicht HEAD-gebunden |
| Kernel-/Userspace-ABI | PARTIAL (I-D-01) |
| Service-Integration | BLOCKED (I-D-02, I-D-03) |
| Persistenter Speicher | PARTIAL (I-D-04) |
| Netzwerk | BLOCKED (I-D-05) |
| Hardwaretreiber | BLOCKED |
| Desktop/Login-Sitzung | PARTIAL (I-D-06) |
| Sicherheitsfreigabe | nicht begonnen (I-D-07, I-D-08) |
| Release-Reife | nicht erreicht |

## 5. Empfehlung

Die Feature-Sperre des Owners wird übernommen. Neue Funktionen erst, wenn für jede
Schnittstelle des Registers drei Bedingungen erfüllt sind: ein Eigentümer, ein
erreichbarer Aufrufpfad, ein Integrationstest.

Reihenfolge der Behebung, nach Wirkung und nicht nach Aufwand: I-D-02 und I-D-01
zuerst, weil ohne sie kein Dienst und kein Nutzerprozess im laufenden System
existiert. Danach I-D-07 und I-D-08, weil sie die Sicherheitszusage tragen. Dann
I-D-04 und I-D-05, die Hardware. I-D-10 und I-D-11 sind billig und verhindern
falsche Erfolgsmeldungen, deshalb parallel.

BLK-P2-13 bleibt Gate: solange kein Schutzstufenmodell normativ ist, gilt keine
Storage-Komponente als produktionsreif, unabhängig davon, wie viele Tests grün sind.
