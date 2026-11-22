---
document_id: ATC-GLOBUSOS-KERNEL-READINESS-001
title: GlobusOS Kernel und System Readiness-Matrix
version: 1.0.0
status: FINAL (Messung), nicht normativ
type: Anlage zu ATC-KAI-OS-ROADMAP-001
updated: 2026-11-23
baseline: a38738df (globus-os, main, 2026-11-16)
measured_by: cargo metadata, cargo test -p je Crate, Quelltextzaehlung
---

# GlobusOS Readiness-Matrix

Messbasis ist `a38738df`, der Default-Branch-Stand von `globus-os`. Alle Zahlen
stammen aus Laeufen auf diesem Commit, nicht aus Dokumentation.

**Kein Eintrag erhaelt CI_VERIFIED oder hoeher.** Die CI-Verifikation ist per
Owner-Entscheidung vom 19.11.2026 ausgesetzt. `TESTED` bedeutet: lokal
ausgefuehrt, gruen, ohne unabhaengige Wiederholung.

## Gesamtbild

| Kennzahl | Wert |
|---|---|
| Crates im Workspace | 32 |
| Quellzeilen (Rust, ohne target) | 84409 |
| Unit-Tests lokal gruen | 1864 |
| Unit-Tests fehlgeschlagen | 0 |
| Build | alle 32 Crates bauen |
| Hoechster erreichter Status | TESTED |
| Hardware-verifizierte Crates | 0 |

Die 1.864 gruenen Tests sind ein Nachweis fuer Softwareverhalten. Sie sind kein
Nachweis fuer Hardware-Reife, Determinismus unter Last oder Produktionsbetrieb.

## Matrix

| Crate | Pfad | Dateien | LOC | Tests (gruen/rot) | Interne Abhaengigkeiten | Sicherheitsmarker | Hardware | Status |
|---|---|---|---|---|---|---|---|---|
| `shivacore` | `modules/atc-shivacore/kernel` | 59 | 34726 | 822 / 0 | globus-diagnostics, globus-entropy-core, globus-identity-core, libshivacore | 26 | QEMU-Smoke historisch gruen (Okt. 2026); Hardware-Validierung BLOCKED | TESTED |
| `shivacore-service-space` | `modules/atc-shivacore/service_space` | 25 | 26977 | 730 / 0 | globus-devices, libshivacore, shivacore | 10 | QEMU-Smoke; Hardware BLOCKED | TESTED |
| `globus-identity` | `system/identity` | 13 | 3347 | 60 / 0 | globus-entropy-core, globus-ipc | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-devices` | `system/devices` | 18 | 2928 | 37 / 0 | globus-diagnostics | 1 | BLOCKED (PCIe/IOMMU/NVMe/NIC real) | TESTED |
| `globus-vfs` | `system/vfs` | 18 | 2715 | 37 / 0 | globus-devices | 1 | BLOCKED (echtes Blockgeraet) | TESTED |
| `globus-media` | `system/media` | 16 | 2368 | 3 / 0 | - | 2 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-graphics` | `system/graphics` | 9 | 2182 | 23 / 0 | globus-devices, globus-media | 2 | BLOCKED (GPU-Backend) | TESTED |
| `libshivacore` | `system/shivacore-api` | 3 | 1254 | 39 / 0 | globus-ipc | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-audio` | `system/audio` | 7 | 959 | 9 / 0 | - | 0 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-entropy-core` | `system/entropy-core` | 1 | 837 | 21 / 0 | - | 0 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-virtualization` | `system/virtualization` | 1 | 596 | 7 / 0 | - | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-package` | `system/package` | 2 | 532 | 8 / 0 | - | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `aurora-core` | `system/aurora-core` | 5 | 457 | 2 / 0 | - | 4 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-runtime` | `system/runtime` | 3 | 447 | 7 / 0 | 9 interne Crates | 2 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-identity-core` | `system/identity-core` | 2 | 440 | 9 / 0 | - | 0 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-memory` | `system/memory` | 5 | 438 | 7 / 0 | - | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-update` | `system/update` | 4 | 435 | 9 / 0 | globus-package | 0 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-ipc` | `system/ipc` | 2 | 387 | 8 / 0 | - | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `atc-globus-shell` | `modules/atc-globus-shell` | 6 | 293 | 5 / 0 | - | 0 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-system-core` | `system/core` | 2 | 239 | 2 / 0 | globus-ipc, globus-memory, globus-process, globus-security | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-security` | `system/security` | 3 | 235 | 4 / 0 | - | 2 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-wallet-service` | `system/wallet-service` | 1 | 233 | 2 / 0 | globus-identity, globus-ipc, globus-security | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-net` | `system/net` | 3 | 222 | 3 / 0 | - | 0 | BLOCKED (NIC-DMA) | TESTED |
| `atc-linux-edition` | `modules/atc-linux-edition` | 6 | 195 | 0 | - | 0 | keine Hardware-Abhaengigkeit | IMPLEMENTED |
| `atc-windows-edition` | `modules/atc-windows-edition` | 6 | 195 | 0 | - | 0 | keine Hardware-Abhaengigkeit | IMPLEMENTED |
| `globus-settings` | `system/settings` | 1 | 189 | 3 / 0 | - | 1 | keine Hardware-Abhaengigkeit | TESTED |
| `globus-diagnostics` | `system/diagnostics` | 1 | 178 | 2 / 0 | - | 0 | keine Hardware-Abhaengigkeit | TESTED |
| `atc_bootloader` | `modules/atc-bootloader` | 5 | 154 | 4 / 0 | - | 0 | BLOCKED (UEFI/ACPI auf echter Hardware) | TESTED |
| `globus-process` | `system/process` | 2 | 123 | 1 / 0 | - | 0 | keine Hardware-Abhaengigkeit | TESTED |
| `atc_drivers` | `modules/atc-drivers` | 8 | 59 | 0 | globus-devices, globus-diagnostics | 0 | BLOCKED (Geraetetreiber real) | IMPLEMENTED (Geruest) |
| `globus-os-sdk` | `sdk` | 1 | 35 | 0 | - | 0 | keine Hardware-Abhaengigkeit | IMPLEMENTED (Geruest) |
| `globus-services` | `system/services` | 1 | 34 | 0 | - | 0 | keine Hardware-Abhaengigkeit | IMPLEMENTED (Geruest) |

## Was der Auftrag voraussetzt, aber bereits existiert

Der Beschluss sieht als Phase B die Block-Device-Abstraktion, danach NVMe und
persistentes Storage vor. Die Inventur zeigt: diese Schicht ist bereits gebaut.
Sie wird hier aufgefuehrt, damit sie nicht ein zweites Mal entsteht.

| Erwartet | Vorhanden | Beleg |
|---|---|---|
| Native Block-Device-Abstraktion | ja | `system/devices/src/block.rs`: `BlockDevice`-Trait (Z. 81), `BlockGeometry`, `BlockRequest`, `validate_request` (Z. 63), `MemoryBlockDevice` |
| Geraeteregistrierung | ja | `system/devices/src/block_manager.rs`: `BlockDeviceRegistry` mit register/read/write/geometry, `BlockDeviceId` |
| NVMe-Kontrollpfad | ja, MMIO-Programmierung | `system/devices/src/nvme_controller.rs`: `validate_config`, `configure_admin_queues`, `enable(&mut impl NvmeMmio)` |
| NVMe-Queues und DMA | ja | `system/devices/src/nvme.rs`: `NvmeQueue`, `NvmeCommand`, `NvmeCompletion`, `NvmeDmaBuffer` |
| IOMMU-Isolation | Vertrag plus Softwaremodell | `system/devices/src/iommu.rs`: `DmaDomainId`, `DmaRegion`, `validate_region`, `SoftwareIommu` |
| Persistentes Dateisystem auf Blockgeraet | ja | `system/vfs/src/persistent.rs`: `PersistentFs<D: BlockDevice>` mit Superblock, Bitmap, Layout, Journal, Recovery |
| Block-Cache | ja | `system/devices/src/block_cache.rs` |

Damit ist die vom Beschluss geforderte Trennung bereits umgesetzt: VFS kennt nur
`BlockDevice`, nicht NVMe. `globus-vfs` haengt an `globus-devices`, nicht
umgekehrt.

## Die tatsaechliche Luecke

Nicht die Schnittstelle fehlt, sondern der Weg auf echte Hardware:

1. **MMIO-Backend**: `NvmeMmio` ist ein Trait. Eine Implementierung, die echte
   BARs programmiert, existiert nicht.
2. **Physische DMA-Puffer**: `NvmeDmaBuffer` ist ein Vertrag ohne Allokator, der
   physisch zusammenhaengenden Speicher liefert.
3. **IOMMU-Hardware**: `SoftwareIommu` prueft Regeln, programmiert aber keine
   VT-d/AMD-Vi-Tabellen.
4. **Boot-Wiring**: Page Tables und Frame-Allokator sind fuer QEMU verdrahtet,
   nicht fuer einen echten Bootpfad.
5. **Hardware-Validierung**: 0 Crates haben einen Lauf auf physischer Hardware.

## Statusgrenze

`TESTED` heisst in dieser Matrix: lokal mit `cargo test -p <crate>` ausgefuehrt
und gruen. Es bedeutet nicht, dass die Tests unter Last, auf anderer Hardware
oder wiederholt deterministisch laufen. Der naechste erreichbare Status ist
`AUDITED`, und dafuer fehlt ein unabhaengiges Audit, nicht mehr Code.
