---
document_id: ATC-SMM-ARCHITECTURE-DRAFT
title: "ATC-SMM — Ecosystem Storage & Memory Management Architecture"
version: 0.1.0
status: DRAFT
lifecycle: PROPOSED / NOT_NORMATIVE
scope: A-TownChain ecosystem
owner: A-TownChain Ecosystem Architecture Control Plane
created: 2026-11-02
updated: 2026-11-02
---

# ATC-SMM — Ecosystem Storage & Memory Management Architecture

## 0. Status and authority

This document is a cross-repository architecture proposal. It records an initial source inventory, SSOT reconciliation, ownership boundaries, invariants and verification requirements.

It is **not** an approved ATC standard, does not allocate a standard ID, does not authorize implementation changes, and does not supersede any repository-specific specification or governance decision.

Normative ecosystem-wide standards and registry allocation remain governed through `atc-standards`. ShivaCore-specific kernel architecture and interface authority is maintained in `atc-shivacore` through its active and frozen kernel specifications. The canonical implementation of the reusable kernel remains in `globus-os/modules/atc-shivacore/kernel/`. This document is maintained in `a-townchain-ecosystem` because that repository owns cross-repository architecture mapping and integration evidence, not component implementation. ATC-SMM MUST NOT silently amend or supersede a ShivaCore specification.

Lifecycle:
- Architecture proposal: DRAFT
- Normative approval: NOT_REQUESTED
- Implementation: NOT_CHANGED BY THIS DOCUMENT
- Verification: NOT_RUN BY THIS DOCUMENT

## 1. Decision

Adopt **ATC-SMM (A-TownChain Storage & Memory Management)** as the cross-ecosystem architecture name for storage and memory ownership, policy and interface coordination.

The architecture follows:

**One physical-memory authority, multiple domain-owned memory and storage consumers.**

1. ShivaCore is the kernel/TCB authority for privileged memory mechanisms, address-space objects, page tables, frame-capability enforcement and mapping protection. The frozen SC-001/SC-005/SC-007 contracts define frame creation from Untyped, rights and mapping semantics; allocation policy remains above the kernel boundary.
2. GlobusOS owns userspace resource policy, service lifecycle, filesystem/VFS, volumes and storage-service orchestration. It consumes ShivaCore memory primitives; it does not create a competing physical allocator or page-table authority.
3. Aurora owns AI/agent memory semantics and its own knowledge/context/vector services. It accesses system resources only through authorized GlobusOS services and capabilities.
4. A-TownChain / ATC-VM / atc-node own their respective deterministic state semantics, node persistence and recovery boundaries. Chain data is not kernel memory and is not stored in the kernel TCB.
5. Genesis Engine owns application/game asset, world, simulation and cache semantics. It uses OS storage and memory interfaces; it does not own physical memory.
6. Wallet key and recovery material remain within the wallet's dedicated security boundary and are never exposed to Aurora or general-purpose memory services.
7. atc-standards owns normative rules and standard-ID allocation. a-townchain-ecosystem owns cross-repository architecture mapping and evidence aggregation.

ATC-SMM is an architecture/control contract, not a new runtime daemon, kernel crate, database, allocator, or universal storage implementation.

## 2. Scope and terminology

The term **memory** is reserved for volatile execution memory and domain-owned working/context memory. The term **storage** is reserved for durable data and persistence.

| Domain | Meaning | Primary authority |
|---|---|---|
| Physical memory | RAM frames, reserved regions, frame lifetime | ShivaCore |
| Kernel memory | Kernel heap, kernel mappings, protected kernel objects | ShivaCore |
| Virtual memory | Address spaces, page tables, mappings, faults, TLB lifecycle | ShivaCore |
| Process resource policy | Per-process/tenant/agent budgets, quotas and admission policy | GlobusOS |
| Application memory | Process-local working sets, caches and runtime allocations | Owning application/runtime under OS policy |
| AI memory | Agent context, knowledge, embeddings, vector indexes, provenance and retention | Aurora |
| Blockchain state | Deterministic protocol/contract state and its version semantics | A-TownChain / ATC-VM protocol boundary |
| Node persistence | Durable chain database, snapshots, WAL/journal and recovery | Canonical node/storage implementation; exact module ownership to be reconciled |
| Filesystem storage | Namespaces, files, mounts, block-device access and permissions | GlobusOS services |
| Game state/assets | World, player, simulation, asset and save semantics | Genesis Engine / game domain |
| Secrets | Wallet keys, recovery material and other high-value credentials | Dedicated wallet/security boundary |

The existence of a cache, index, snapshot, mirror, subtree or in-memory model does not make it a canonical state authority.

## 3. Repository SSOT map

The following map records the ownership decision and the source baseline inspected for this proposal.

| Repository | Canonical role in ATC-SMM | Baseline inspected |
|---|---|---|
| `globus-os` | Active ShivaCore kernel source plus GlobusOS userspace, memory policy, VFS and services | `f370b681e5c9ff49577f25c24f15b5fc880d5cba` |
| `atc-shivacore` | ShivaCore kernel/TCB architecture authority; active/frozen kernel, HAL, ABI, boot and capability/address-space specifications; no duplicate active kernel source | `3b56253b39bdc6cbf719ed9a2a21461d73b8fc34` |
| `aurora-ai` | AI/agent memory and context implementation | `30e85eb8aba199f68b5f17f329eddbe168e50f87` |
| `a-townchain` | Chain orchestration and canonical integration; canonical L1 storage component at `components/storage` (subtree SSOT) | `ce92756c32f138e3bc51039060101f4d466027bd` |
| `atc-node` | Standalone node runtime source imported under `a-townchain/components/node`; node-level durable storage implementation not established by the inspected root manifest | `821d049efc5b592e4728b36b64038d4ce5e7d2c6` |
| `genesis-engine` | Game/runtime asset, world and simulation data domains | `3c2be8c11c00bf5b6a1a4de87760004e9c5d35ef` |
| `atc-standards` | Normative standards registry and change governance | `f54083518fb8a2d12482e0791f05bd8d86b4642d` |
| `a-townchain-ecosystem` | Cross-repository architecture, integration contracts and evidence control plane | `83b696c8fc63fa4d132da7975e0eee1267f45d5f` |
| `atc-storage` | Archived reference; not an active runtime dependency | `230746c1dc5e4c1355c2707368de50d6c1008660` |

These are observed repository HEADs from the inventory run, not release pins. Re-fetch each HEAD before implementation, merge, audit or release decisions.

### 3.1 Canonical kernel source correction

The active kernel implementation is:

`globus-os/modules/atc-shivacore/kernel/`

The `atc-shivacore` README and STATUS explicitly establish it as the reusable kernel-contract/specification and governance authority, while the active implementation is migrated to GlobusOS. It MUST NOT receive a second active kernel implementation.

### 3.2 Existing memory modules

Observed source files in `globus-os`:

- `modules/atc-shivacore/kernel/src/memory.rs`
- `modules/atc-shivacore/kernel/src/memory_manager.rs`
- `modules/atc-shivacore/kernel/src/allocator.rs`
- `system/memory/src/lib.rs`
- `system/memory/src/frames.rs`
- `system/memory/src/paging.rs`
- `system/memory/src/allocator.rs`
- `system/memory/src/fault.rs`

Observed source files in `aurora-ai`:

- `modules/atc-aurora-memory/src/lib.rs`
- `modules/atc-aurora-memory/Cargo.toml`
- `modules/atc-aurora-memory` module documentation

The source inventory proves file and API presence only. It does not establish that every path is used in the booted system, production-ready, or verified at the baseline SHA.

### 3.3 Binding ShivaCore specification reconciliation

The following ShivaCore documents were fetched at repository baseline `3b56253b39bdc6cbf719ed9a2a21461d73b8fc34`:

| Specification | Blob SHA | Status / binding ATC-SMM consequence |
|---|---|---|
| `SHIVA-KERNEL-REUSE-001.md` | `dfd8744dabda2d136a501851f94fbcf8c550701d` | `active`; kernel contract is OS-neutral; memory/address-space primitives and isolation belong to TCB; filesystems, chain and AI stay above it |
| `SHIVA-HAL-001.md` | `f071bc13e610e88a540fe4d57ffc9e286ee87da9` | `active`; target page-table primitives, synchronization and boot handoff are HAL responsibilities; architecture-specific unsafe code isolated and tested |
| `SHIVA-ABI-001.md` | `edb0e219bdab32fd99ae4d1513dcbb7deab1e9bf` | `active`; address-space and memory-object handles are explicit capabilities; syscalls validate caller, capability, pointers and lengths; no ambient authority; ABI changes versioned |
| `SHIVA-BOOT-001.md` | `64e73fa8c711dc076330dae8dbfc81c04b989bb3` | `active`; kernel receives boot memory map/CPU topology and initializes its own page tables and allocator state before exposing services |
| `SC-005-CAPABILITY_SYSTEM.md` | `2e19216b43237fcbd6e175a7abf15c1781cde4ca` | `FROZEN v0.1.0`; typed rights, derivation only reduces rights, revocation removes derived authority; no all-rights initial capability or ambient derive |
| `SC-007-ADDRESSSPACE_MEMORY.md` | `8d61e3e9fa8951ccf20924254ba1c48fb239bfee` | `FROZEN v0.1.0`; map requires AddressSpace MAP + Frame GRANT; mapping rights are a subset of Frame-Cap rights; W^X is mandatory policy; unmap and frame revocation include synchronous TLB shootdown |
| `SHIVA-GLOBUS-INTEGRATION-001.md` | `b46302a3bfd2808016b18b0824517da724558dbc` | `DRAFT`; ShivaCore owns address spaces/protection and capability primitives; GlobusOS owns process/service policy, VFS, storage, networking and drivers |

The frozen SC-005 and SC-007 documents are more specific than generic ATC-SMM language. ATC-SMM therefore records their requirements as inherited constraints, not as newly invented requirements. A change to frozen semantics requires the ShivaCore owner’s documented SC-DEC/SC-ARCH governance path; ATC-SMM alone cannot authorize it.

## 4. Ownership and trust boundaries

### 4.1 ShivaCore — physical memory and protection authority

ShivaCore MUST own the privileged mechanisms required to enforce memory isolation, as constrained by the active/frozen ShivaCore contracts:

- validate the boot handoff and memory-map inputs; establish kernel page tables and allocator state before exposing services;
- manage kernel page-table objects and AddressSpace objects;
- enforce map/unmap/switch operations through explicit capability-authorized ABI operations;
- require AddressSpace-Cap `{MAP}` plus Frame-Cap `{GRANT}` for mapping; mapping rights MUST be a subset of Frame-Cap rights;
- enforce SC-005 typed object-rights, monotonic derivation and revocation semantics;
- enforce W^X as the frozen SC-007 policy; changes require the documented SC-DEC/SC-ARCH review, not a runtime service-space override;
- perform synchronous unmap/TLB-shootdown semantics required by SC-007, including cross-CPU behavior before stale mappings can be used;
- deliver memory faults through the specified AddressSpace fault-endpoint and thread-suspend contract;
- isolate target-specific page-table, synchronization and CPU operations behind the HAL.

**Important distinction:** SC-007 says frames originate from Untyped via SC-001 Retype; mapping creates visibility, not memory. Allocation policy is service-space policy. Therefore ATC-SMM MUST NOT describe a generic kernel heap or a frame-scanning allocator as the complete normative process-memory allocation model. Existing `memory.rs`/`memory_manager.rs` code must be classified as interim implementation until conformance with SC-001…SC-007 is demonstrated.

Only the minimum mechanisms required for isolation belong in the TCB. Filesystem policy, databases, AI inference, blockchain state, content indexing, application policy and storage federation remain outside the TCB.

### 4.2 GlobusOS — userspace policy and storage services

GlobusOS MUST own:

- process/agent memory budgets and admission policy;
- service-level resource quotas and accounting;
- VFS, filesystem namespace, mounts and volume policy;
- block-device and storage-service lifecycle;
- persistent storage orchestration and recovery interfaces;
- service authorization and IPC to kernel memory objects.

The existing `globus-memory` crate is treated as an OS-side policy/model/API layer, not a second hardware memory authority. It may request memory through the ShivaCore capability/Untyped/AddressSpace contract; it MUST NOT independently own physical frames, page-table roots or hardware mapping mutation. Its current model code is not proof of conformance or a hardware-backed mapping path.

### 4.3 Aurora AI — semantic AI memory

Aurora MUST own its memory-domain semantics, including:

- agent and tenant ownership;
- conversation/context windows;
- knowledge base and retrieval indexes;
- vector/embedding indexes and their versioning;
- provenance, confidence and classification;
- retention, deletion and poisoning controls;
- optional federation and cross-device synchronization.

Aurora MUST NOT:
- access kernel physical memory or page tables directly;
- acquire ambient filesystem or device privileges;
- promote model inference into authoritative fact without explicit policy;
- access wallet keys or recovery material;
- turn AI memory into canonical blockchain state.

The existing `ATC-AI-MEM-001` is a draft AI-memory security specification. The existing decentralized-memory architecture is a target contract, not proof of a production federation implementation.

### 4.4 Blockchain and ATC-VM

- ATC-VM defines deterministic contract storage semantics and execution limits.
- A-TownChain defines canonical protocol/state-transition authority.
- The migration manifest identifies `a-townchain` as the monorepo source of truth and maps archived `atc-storage` into `a-townchain/components/storage`.
- The inspected `components/storage/src/store.rs` is currently an in-memory `BTreeMap` KV store. Its FNV-1a root is explicitly development-only and MUST NOT be treated as a cryptographic state root, consensus commitment or durable database.
- The standalone `atc-node` repository remains a node-runtime source imported under `a-townchain/components/node`; the inspected root manifest does not establish a durable storage implementation there.
- Durable database success MUST NOT be confused with consensus finality.
- Candidate state MUST NOT become canonical until accepted by the protocol's finality/commit rules.
- AI memory, indexer read models and explorer projections MUST NOT become canonical chain state.

Therefore, `a-townchain/components/storage` is the canonical storage-component location, but its inspected implementation is only an in-memory MVP. Durable node persistence, crash recovery and state-root cryptography remain explicit implementation/evidence gaps. No replacement storage engine is authorized by this draft.

### 4.5 Genesis Engine

Genesis owns the meaning and lifecycle of game assets, world state, simulation state, saves and caches. It consumes GlobusOS storage and memory services through explicit APIs. It does not become a kernel memory authority or a chain-state authority.

### 4.6 Wallet and secrets

Wallet private keys, seed/recovery material and signing state MUST remain in the wallet's dedicated security boundary. General memory APIs MUST NOT return raw secret material to Aurora, Genesis or ordinary applications. Secret-bearing pages require explicit lifecycle, non-swapping/locking policy where supported, zeroization before reuse and auditable access controls.

## 5. Kernel memory invariants

The following rules are inherited from the cited ShivaCore specifications or are ATC-SMM integration constraints. They are not replacements for those specifications.

1. **Capability authority:** kernel-object access is capability-only; no ambient authority. (SHIVA-ABI-001; SC-005)
2. **Typed rights:** Frame rights and AddressSpace rights are distinct; mapping requires AddressSpace `{MAP}` plus Frame `{GRANT}`. (SC-005; SC-007)
3. **Monotonic rights:** capability derivation cannot increase rights; reduction is performed through the frozen mint/revoke model. (SC-005)
4. **W^X:** no VA region may be simultaneously writable and executable. This is frozen SC-007 policy; an exception cannot be introduced by ATC-SMM or runtime policy.
5. **Kernel-exclusive page tables:** userspace never writes page-table entries directly. (SC-007)
6. **Synchronous unmap/revocation:** unmap and frame revocation complete the specified TLB shootdown before stale access can persist. (SC-007)
7. **Untyped frame source:** frames are created through the SC-001 Untyped/Retype model; map does not allocate memory. (SC-007)
8. **ABI validation:** validate caller context, capability, user pointer and length; return explicit errors; version breaking ABI changes. (SHIVA-ABI-001)
9. **HAL isolation:** page-table primitives and required synchronization are target-HAL responsibilities; architecture-specific unsafe code is isolated and tested. (SHIVA-HAL-001)
10. **Boot ordering:** initialize kernel page tables and allocator state from validated boot information before exposing services. (SHIVA-BOOT-001)
11. **Fault contract:** page faults follow the SC-007 suspend/notification/resume or terminal-suspend rules; no generic implicit allocation-on-fault is assumed.
12. **Implementation evidence:** source presence is not conformance; target build, boot, ABI/IPC and security evidence must be tied to exact source SHAs. (SHIVA-KERNEL-REUSE-001; HAL; BOOT)
13. **No persistence in the TCB:** filesystems, database, block-storage, AI memory and chain persistence remain userspace/domain services. (SHIVA-KERNEL-REUSE-001; SHIVA-GLOBUS-INTEGRATION-001)

Additional memory-safety recommendations such as checked arithmetic, rollback, object lifetime, secure reclamation and DMA/IOMMU isolation remain ATC-SMM review/acceptance criteria unless already specified by a more specific ShivaCore contract. They MUST NOT be represented as already implemented.

## 6. Memory object and interface contract

ATC-SMM does not define a parallel syscall model or replace the ShivaCore ABI. The canonical types and operations remain governed by SHIVA-ABI-001 and SC-001…SC-007.

The cross-layer model MUST preserve these distinctions:

- `UntypedMemory`: capability-authorized source of memory authority; frame creation uses SC-001 Retype.
- `FrameCap`: typed capability with SC-005 Frame rights; `GRANT` authorizes mapping.
- `AddressSpaceCap`: typed capability with `MAP`, `UNMAP`, `SWITCH` rights.
- `PageTable`: kernel-exclusive object; userspace cannot edit entries.
- `Mapping`: VA-to-frame relationship; mapping rights are a subset of the Frame-Cap rights.
- `MemoryQuota`: GlobusOS/service-space policy, not a kernel frame-allocation authority.
- `MemoryFault`: follows SC-007 fault endpoint and thread state semantics; it is not implicitly an allocation request.

ATC-SMM deliberately does not prescribe a new generic `MemoryRights` mask, allocator API, syscall number or Rust ABI type. In particular, it MUST NOT collapse Frame rights (`READ/WRITE/EXECUTE/GRANT`) and AddressSpace rights (`MAP/UNMAP/SWITCH`) into one generic object-rights mask.

The implementation lifecycle is specified by the frozen SC-001…SC-007 contracts. At the integration level it is: acquire/derive Untyped authority → Retype frame → derive least-privilege Frame-Cap → authorize AddressSpace-Cap → map → use → unmap/revoke with synchronous TLB semantics → reclaim only after all references/mappings are gone. Exact operation atomicity and failure semantics remain those of the underlying SC specifications.

The userspace ABI MUST be versioned. Internal kernel structs MUST NOT become an implicit userspace ABI.

## 7. Persistent storage contract

Persistent storage is separate from RAM management and remains outside the ShivaCore TCB.

Every persistent subsystem MUST define:
- canonical owner and data authority;
- schema and serialization version;
- atomicity and durability model;
- consistency model;
- concurrency/writer ownership;
- WAL/journaling or equivalent crash-consistency mechanism where applicable;
- integrity checks and corruption detection;
- snapshot and recovery semantics;
- migration and compatibility policy;
- disk exhaustion and partial-write behavior;
- backup/restore authority;
- deletion, retention and cryptographic erasure semantics.

A content hash, cache entry, snapshot, index or database write does not independently establish authoritative state or consensus finality.

The archived `atc-storage` repository MUST NOT be treated as an active dependency without an explicit reactivation decision and SSOT review.

## 8. Cross-system API and data-flow rules

- Kernel mechanisms are exposed through the ShivaCore ABI and capability model.
- GlobusOS mediates system services and resource policy.
- Aurora and Genesis access memory/storage only through authorized userspace service interfaces.
- Chain/node persistence remains behind the node and protocol-owned state boundary.
- No cross-repository component may bypass its declared owner by directly importing private implementation internals.
- Imported subtree snapshots are integration artifacts, not alternative SSOTs.
- API/ABI changes require versioning, compatibility review and migration evidence.

## 9. Initial source findings and residuals

| ID | Finding | Evidence basis | Status |
|---|---|---|---|
| MEM-01 | `memory_manager.rs` creates a generic `ResourceType::Memory` capability with `READ | WRITE | EXEC | DELEGATE` on allocation; this is broader and less typed than SC-005 Frame/AddressSpace rights. It does not by itself prove that a hardware page mapping receives executable permission. | GlobusOS `memory_manager.rs` blob `6d1d8ec948b3d3d0d683e1a0e92d9c77ee2fbfaa`; ShivaCore SC-005 blob `2e19216b43237fcbd6e175a7abf15c1781cde4ca` | RESIDUAL / HIGH / SPEC-CONFORMANCE REVIEW |
| MEM-02 | The same manager routes small allocations to kernel `alloc::alloc` and large allocations to `UserspaceBump`, explicitly documented as virtual-address simulation; this is not the SC-001 Retype + SC-007 map process-memory contract. | GlobusOS `memory_manager.rs` blob `6d1d8ec948b3d3d0d683e1a0e92d9c77ee2fbfaa`; SC-007 blob `8d61e3e9fa8951ccf20924254ba1c48fb239bfee` | INTERIM / NOT_CONFORMANT_AS_PROCESS_MAPPING |
| MEM-03 | `globus-memory/src/paging.rs` models mappings in a `BTreeMap`; it is not itself the kernel-exclusive hardware page-table object/mutation path required by SC-007. | GlobusOS source at inspected baseline; SC-007 | MODEL / NOT_HARDWARE_VERIFIED |
| MEM-04 | `globus-memory/src/frames.rs` exposes a bounded monotonic frame model; conformance to Untyped/Retype and full lifetime/reclamation semantics is not established. | GlobusOS source at inspected baseline; SC-001/SC-007 | PARTIAL / NOT_CONFORMANT_EVIDENCE |
| MEM-05 | `memory.rs` uses a boot-info usable-frame iterator with `.nth(next)`; the inspected implementation is an interim boot allocator and does not demonstrate SC-001 Untyped/Retype capability ownership. | GlobusOS `memory.rs` source; SC-001/SC-007 | INTERIM / REVIEW_REQUIRED |
| MEM-06 | Mapping mutation rollback, page-table cleanup and object/mapping lifetime remain P0 acceptance criteria already tracked by the existing delivery item. | `FN-VMM-001` in ecosystem delivery registry | IN_PROGRESS / EVIDENCE MISSING |
| MEM-07 | SC-007 requires synchronous TLB shootdown; PR #89 explicitly says remote CPU TLB shootdown and true SMP scheduler/frame synchronization are not implemented or evidenced at its current inspected head. | ShivaCore SC-007 blob `8d61e3e9fa8951ccf20924254ba1c48fb239bfee`; GlobusOS PR #89 head `b8e73a18c9f2022e6e3d0d321bea36b8bc448ff2` | BLOCKED / SPEC-COMPLIANCE GAP |
| MEM-08 | The standalone `atc-storage` repository is archived; its migrated component is located at `a-townchain/components/storage` | `MIGRATION_MANIFEST.yaml`, repository metadata and control-plane registry | SSOT LOCATION CONFIRMED; implementation maturity separate |
| STOR-01 | `components/storage/src/store.rs` is an in-memory `BTreeMap` KV store with an explicitly non-cryptographic FNV-1a development fingerprint | Source blob `3755e0ce0f41e645ad40d00838403a624244a730` | RESIDUAL / NOT_DURABLE / NOT_CRYPTOGRAPHIC |
| STOR-02 | Durable node persistence and restart recovery are not established by the inspected `atc-node` root manifest or architecture document | `atc-node/Cargo.toml` and `ARCHITECTURE.md` inspected at baseline | NOT_VERIFIED / INVENTORY_REQUIRED |

These findings are source-level observations. They are not a substitute for a complete call-graph audit, target build, QEMU/UEFI boot, runtime test, security review or exact-SHA CI evidence.

## 10. Existing work that MUST be reconciled, not duplicated

- GlobusOS PR #89: open Draft at inspected head `b8e73a18c9f2022e6e3d0d321bea36b8bc448ff2`; separate CR3 roots and user-address-space isolation work exist, but remote CPU TLB shootdown, AP startup, per-CPU scheduler integration and synchronized frame lifecycle remain open. Its CI must be checked on the exact current head before any status claim.
- GlobusOS PR #108: open, non-draft at inspected head `1b2fdf062c943dab6c5b32b92dd373be73918ceb`; its patch touches `memory_manager.rs` only for a compile-time assertion, not the capability-allocation logic. It also touches VMM test scoping and LKM/bootloader lint issues. Coordinate on the same file, but do not overstate the overlap: its current patch does not fix MEM-01/MEM-02.
- GlobusOS PR #28: open; decentralized memory-node contract. This is an Aurora/federation boundary, not a physical-memory allocator.
- GlobusOS PR #35: open; performance/optimization center. Its stated authority is recommendation-only and outside the TCB.
- Ecosystem delivery item `FN-VMM-001`: existing P0 tracking for virtual-memory mapping mutation/lifetime. Reuse this item; do not allocate a duplicate function ID.
- Aurora `ATC-AI-MEM-001` and `DECENTRALIZED_MEMORY_ARCHITECTURE.md`: existing AI memory security/federation work. Keep its ownership in Aurora and GlobusOS.

No PR is merged, approved or verified by this architecture document.

## 11. Verification and acceptance matrix

### Kernel P0

1. Boot memory map: usable/reserved/firmware/MMIO ranges are classified correctly.
2. Physical allocator: alignment, overflow, reserved-hole exclusion, exhaustion, release and reclamation.
3. Kernel heap: bounds, alignment, allocation/free correctness and corruption handling.
4. Address spaces: independent roots, supervisor-only kernel mappings and process teardown.
5. Mapping lifecycle: map/unmap/protect, partial failure rollback and bottom-up page-table cleanup.
6. Permissions: READ/WRITE/EXECUTE checks, NX default, W^X and no automatic delegation.
7. Faults: invalid, present, user/kernel, read/write/execute and malformed-range negative tests.
8. TLB: local invalidation and cross-CPU shootdown before frame reuse.
9. SMP: per-CPU scheduler/address-space state and synchronized allocator/page-table operations.
10. Capabilities: unauthorized access denied, attenuation, revocation and no ambient rights.
11. DMA: IOMMU mapping and device isolation tests for supported targets.
12. Recovery: OOM, process termination, interrupted mutation and kernel diagnostic behavior.

### GlobusOS P0

- Quota accounting and enforcement through authorized kernel objects.
- Userspace memory service cannot bypass ShivaCore.
- VFS/block-device service lifecycle and crash-consistent persistence.
- Storage corruption, disk-full, reboot and restore tests.

### Aurora P1

- Tenant/agent isolation, provenance, poisoning defenses and versioned indexes.
- Retention/deletion includes primary records, vector indexes and derived caches.
- Federation requires explicit identity, capability, scope, consent and revocation.
- Model inference remains data/proposal, never authority.

### Chain / ATC Node P0

- Deterministic state semantics are separated from database persistence.
- Durable commit, recovery, replay and corruption behavior are tested.
- Candidate state is not exposed as canonical before protocol finality.
- Exact node-storage SSOT is reconciled before any storage-engine change.

### Genesis P1

- Asset/world/save ownership and schema versions are explicit.
- Caches are rebuildable and are not authoritative state.
- AI-generated state changes pass deterministic validation before commit.

## 12. Implementation sequence

1. **Inventory closure:** re-fetch exact repository HEADs; inspect manifests, call graphs, memory APIs, ownership, tests, active PRs and subtree drift.
2. **Kernel contract review:** inherit `SHIVA-KERNEL-REUSE-001`, `SHIVA-HAL-001`, `SHIVA-ABI-001`, `SHIVA-BOOT-001`, frozen `SC-005` and `SC-007`; resolve the SC-001 Retype dependency and `SHIVA-GLOBUS-INTEGRATION-001` boundary. Reuse `FN-KERNEL-001` and `FN-VMM-001`; do not create duplicate delivery IDs.
3. **Existing-branch coordination:** review PR #89 and PR #108; do not duplicate or overwrite their work.
4. **Kernel P0 remediation:** prioritize permission defaults, mapping rollback/lifetime, TLB/SMP safety, frame lifecycle and capability enforcement.
5. **GlobusOS policy contract:** define quota/accounting and userspace memory-service APIs on top of the kernel contract.
6. **Persistent storage census:** reconcile `a-townchain`, `atc-node`, imported storage subtree and archived `atc-storage`; do not select a new engine until ownership is proven.
7. **Aurora and Genesis integration:** bind their domain stores to authorized OS services without moving their semantic ownership.
8. **Normative governance:** only after owner/security/architecture review, submit the appropriate SCR and registry-first standard proposal to `atc-standards`. No ID is allocated by this draft.

## 13. Evidence rules

For every implementation or verification claim, record:
- exact repository and commit SHA;
- path, symbol and blob SHA;
- test command and target configuration;
- GitHub Actions Run → Job → Step;
- exit status and relevant log;
- artifact SHA-256 where an artifact is produced;
- residuals and linked issue/PR/function IDs.

Statuses remain separate:
- `IMPLEMENTED`: source implementation exists;
- `TESTED`: specified test executed successfully;
- `CI_VERIFIED`: required CI passed for the exact SHA;
- `E2E_VERIFIED`: cross-component behavior passed for the pinned source set;
- `VERIFIED`: all applicable verification gates passed;
- `RESIDUAL`: known unresolved risk;
- `BLOCKED`: required dependency/evidence is missing;
- `EXCLUDED`: explicitly outside active scope.

Unknown evidence is BLOCKED or NOT_VERIFIED, never inferred PASS.

## 14. Current decision record

- Architecture name: ATC-SMM
- Scope: full A-TownChain ecosystem
- First technical priority: ShivaCore kernel memory safety and ownership boundaries
- Kernel architecture/specification authority: `atc-shivacore` (active/frozen ShivaCore specifications)
- Kernel implementation SSOT: `globus-os/modules/atc-shivacore/kernel/`
- Ecosystem standards registry/governance: `atc-standards`
- Cross-repository architecture/evidence owner: `a-townchain-ecosystem`
- New runtime implementation: NOT_AUTHORIZED BY THIS DOCUMENT
- New normative standard ID: NOT_ALLOCATED
- ShivaCore specification reconciliation: PARTIAL (SHIVA-KERNEL-REUSE/HAL/ABI/BOOT, SC-005, SC-007 and integration draft inspected; SC-001 full text/path still requires exact retrieval)
- Inventory completeness: PARTIAL
- Architecture approval: PENDING
- Implementation verification: NOT_ESTABLISHED
- Production readiness: NOT_CLAIMED
