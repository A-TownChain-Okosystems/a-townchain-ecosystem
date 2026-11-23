---
document_id: ATC-GLOBUSOS-INTERFACE-AUDIT-001
title: GlobusOS Interface and Lifecycle Audit
version: 1.0.0
status: DRAFT
type: Architecture Control Document, NICHT normativ
basis: globus-os main a38738df, gegengeprueft am HEAD 0c792e3d
updated: 2026-11-23
---

# GlobusOS Interface and Lifecycle Audit

Zweiter Audit-Schritt nach `GLOBUSOS-INTEGRATION-AUDIT-2026-11-23.md`.
Regel dieses Dokuments: **ein erfolgreicher Build ist keine erfolgreiche
Verbindung.** Jede Zeile braucht Quelle, Ziel, Vertrag, Implementierungsstelle,
Test und Ergebnis.

Ergebniswerte: `PASS`, `FAIL`, `NOT TESTED`, `BLOCKED`.
Ein fehlender Test ist `NOT TESTED` und niemals `PASS`.

## 1. Gegengeprueft: die Korrekturen

| Aussage | Befund |
| --- | --- |
| Greeter ist ein eigenstaendiges Userspace-Programm, nicht nur ein Modell | BESTAETIGT, `modules/atc-shivacore/greeter/src/main.rs` |
| Kernel hat einen IPC-Transport, aber keinen vollstaendigen Nutzdatenpfad | BESTAETIGT, siehe I-06 |
| Bootpfad enthaelt ACPI, GDT/IDT, Paging, Heap, LAPIC/IOAPIC, Ramdisk, ELF, Ring 3 | BESTAETIGT, 121 Treffer der Bootbegriffe in `kernel/src/main.rs` |
| Flaky-Test dokumentiert | BESTAETIGT, `docs/RUSTDOC_GATE.md:59-60`, 1 bis 2 Fehlschlaege aus 15 bis 20, seit T43 |

## 2. Der harte Befund: die Syscall-Flaeche

Der Dispatch in `kernel/src/syscall.rs` bedient nachweislich vier Operationen:

| Symbol | Zeile |
| --- | --- |
| `Syscall::IpcSend` | 198 |
| `Syscall::IpcReceive` | 263 |
| `Syscall::CapabilityQuery` | 331 |
| `Syscall::HandleClose` | 344 |

Es gibt **keinen** Dateisystem-Syscall und **keinen** Prozessstart-Syscall.
Damit sind INT-002 und INT-003 nicht unvollstaendig, sondern strukturell
offen: ein Userspace-Programm kann weder eine Datei oeffnen noch einen Prozess
starten. Jede weitere GlobusOS-Funktion haengt an diesen beiden fehlenden
ABI-Eintraegen.

## 3. Schnittstellenregister

Schema wie vorgegeben. Nur belegte Stellen; alles andere ist `NOT TESTED`.

| ID | Quelle | Ziel | Vertrag | Implementierung | Test | Ergebnis | Fehler | Eigentuemer |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| I-01 | UEFI/BIOS | Bootloader | Bootprotokoll | `modules/atc-bootloader` | QEMU-Smoke 14.11. | NOT TESTED | INT-010 | globus-os |
| I-02 | Bootloader | Kernel | ELF-Einstieg | `kernel/src/main.rs` | QEMU-Smoke 14.11. | NOT TESTED | INT-010 | globus-os |
| I-03 | Kernel | ACPI/GDT/IDT/Paging/Heap | Bootreihenfolge | `kernel/src/main.rs` | QEMU-Smoke 14.11. | NOT TESTED | INT-010 | globus-os |
| I-04 | Kernel | Ring-3-Prozess | ELF-Loader, Adressraum | `kernel/src/elf_loader.rs`, `vmm.rs` | Unit-Tests | PASS (Software) | - | globus-os |
| I-05 | Ring 3 | Kernel | Syscall-ABI | `kernel/src/syscall.rs` | Unit-Tests | PASS (4 Operationen) | INT-002, INT-003 | globus-os |
| I-06 | Prozess | Prozess | IPC-Nachricht | `kernel/src/ipc.rs`, `system/ipc` | Unit-Tests | PASS 64-Bit-Wort, Nutzdaten NOT TESTED | INT-006 | globus-os |
| I-07 | Prozess | Kernel | Capability-Pruefung | `kernel/src/cap_authority.rs` | Unit-Tests | PASS | INT-011 | globus-os |
| I-08 | Kernel | Service Space | `ServiceSpaceState::init()` | `service_space/src/service_init.rs` | kein Aufruf im Bootpfad | FAIL | INT-001 | globus-os |
| I-09 | Service Space | Kernel-Crate | `shivacore = { path = "../kernel" }` | `service_space/Cargo.toml:27` | - | FAIL (Kopplung) | INT-001 | globus-os |
| I-10 | Greeter | Kernel | Identitaetspruefung, Konsolen-Capability | `greeter/src/main.rs`, `kernel/src/identity_file.rs` | Unit-Tests | PASS Pruefung, Sitzung NOT TESTED | INT-012 | globus-os |
| I-11 | VFS | Block-Device | `BlockDevice`-Trait | `system/vfs/src/persistent.rs:100` | 37 Tests | PASS (Software) | INT-004 | globus-os |
| I-12 | NVMe | Block-Device | `NvmeMmio`, `NvmeQueue` | `system/devices/src/nvme_mmio.rs`, `nvme.rs` | 3 Integrationstests | PASS (Software) | INT-004 | globus-os |
| I-13 | DMA | IOMMU | `DmaRegion`, `Iommu` | `system/devices/src/dma_alloc.rs`, `iommu.rs` | 8 Tests | PASS (Software) | INT-004 | globus-os |
| I-14 | IOMMU | Hardware | Seitentabellen | nur `SoftwareIommu` | keiner | BLOCKED | INT-004 | globus-os |
| I-15 | NIC | Socket | `globus-net`, `ethernet` | `system/net/src/lib.rs` | 3 Tests | NOT TESTED (Kette) | INT-005 | globus-os |
| I-16 | Kernel | LKM | Modullader | `kernel/src/lkm.rs`, `lib.rs:107-109` | Testbuild | FAIL (nur `cfg(test)`) | INT-007 | globus-os |
| I-17 | CI | Repository | Boot-Gate | `scripts/qemu_boot.sh:95` | Skript | FAIL (kein Exit 1) | INT-009 | globus-os |

## 4. Lifecycle-Bereiche

### A. Prozess- und Userspace-Lifecycle

ELF laden und Adressraum erzeugen: vorhanden und getestet.
Capability-Zuweisung: vorhanden.
Prozess starten: **kein Syscall** (INT-002).
Prozess beenden und neu starten: nicht nachgewiesen.
Ressourcenfreigabe: `HandleClose` vorhanden, Widerruf getestet.

Status: `FAIL` bei Start und Neustart, das ist der Blocker fuer jede Sitzung.

### B. Geraete- und Treiber-Lifecycle

PCI-Enumeration: vorhanden (`pci_scan`, `pci`).
Treiberzuordnung und Zustandsaufsicht: `driver_lifecycle` vorhanden.
DMA-Autorisierung: `dma_auth` vorhanden, Software.
IOMMU-Hardware: fehlt (I-14).
IRQ-Zuweisung: `interrupt`, `msi` vorhanden.
Geraet freigeben: nicht nachgewiesen.

Status: Software vollstaendig, Hardware `BLOCKED`.

### C. Daten- und Speicher-Lifecycle

VFS, Block-Device, Journal, Recovery, PersistentFs: vorhanden und getestet.
NVMe: Modell plus Registerfenster, kein physisches Mapping.
Daten-Schutzstufenmodell: fehlt vollstaendig (BLK-P2-13).

Status: `PASS` in Software, `BLOCKED` fuer Produktionsreife durch BLK-P2-13.

### D. Systemdienst-Lifecycle

Service Manager: `kernel/src/services.rs` vorhanden, produktiver Start fehlt.
Service Space: Initialisierung existiert, wird nicht aufgerufen (I-08).
Netzwerk: nicht in der laufenden Kette nachgewiesen.
Identitaet: vorhanden. Wallet: `wallet-service` vorhanden.
Update: `globus-update` vorhanden, Atomaritaet nicht nachgewiesen.
Aurora-Anbindung: nur Vertrag (`aurora-core`).

Status: `FAIL`, kein Dienst erreicht den laufenden Betrieb.

## 5. Reihenfolge, die aus dem Register folgt

1. INT-002 Prozessstart-ABI und INT-003 VFS-Syscall. Ohne beide ist kein
   Dienst und keine Sitzung beobachtbar.
2. INT-001 Service Space im Startpfad, danach I-09 entkoppeln.
3. INT-011 Capability-Autoritaet auf eine Implementierung konsolidieren.
4. INT-008 Flake untersuchen, INT-009 Boot-Gate auf Exit 1.
5. Erst danach INT-004 und INT-005, die Hardware-Kette.

INT-006, INT-007, INT-012 folgen nachrangig.

## 6. Grenzen dieses Audits

Gelesen wurden Quellcode und Dokumente, nicht ausgefuehrt wurde ein
vollstaendiger Boot auf dem aktuellen HEAD. Die QEMU-Nachweise vom 14.11. sind
an einen anderen Commit gebunden und hier ausdruecklich `NOT TESTED`.
Die vier Lifecycle-Bereiche sind auf Schnittstellenebene bewertet, nicht
Zeile fuer Zeile jeder Funktion.
