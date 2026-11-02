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

Normative standards remain owned by `atc-standards`. The canonical implementation of each subsystem remains in its designated source repository. This document is maintained in `a-townchain-ecosystem` because that repository owns cross-repository architecture and integration evidence, not component implementation.

Lifecycle:
- Architecture proposal: DRAFT
- Normative approval: NOT_REQUESTED
- Implementation: NOT_CHANGED BY THIS DOCUMENT
- Verification: NOT_RUN BY THIS DOCUMENT

## 1. Decision

Adopt **ATC-SMM (A-TownChain Storage & Memory Management)** as the cross-ecosystem architecture name for storage and memory ownership, policy and interface coordination.

The architecture follows:

**One physical-memory authority, multiple domain-owned memory and storage consumers.**

1. ShivaCore is the sole authority for physical memory, kernel heap, page tables, address-space mechanisms, memory objects and capability-enforced mappings.
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
| `atc-shivacore` | Kernel architecture, reuse, HAL, ABI and governance specifications; not a second active kernel | `3b56253b39bdc6cbf719ed9a2a21461d73b8fc34` |
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

The `atc-shivacore` repository explicitly identifies itself as specifications/governance/support. It MUST NOT receive a second active kernel implementation.

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

## 4. Ownership and trust boundaries

### 4.1 ShivaCore — physical memory and protection authority

ShivaCore MUST own the privileged mechanisms required to enforce memory isolation:

- boot memory-map validation and reserved-region handling;
- physical frame allocation, reservation, release and reclamation;
- kernel heap and kernel-only mappings;
- address-space objects and page-table roots;
- mapping, unmapping, permission changes and rollback;
- page-fault validation and dispatch primitives;
- memory-object and mapping capability enforcement;
- TLB invalidation and cross-CPU shootdown before unsafe frame reuse;
- lifecycle-safe cleanup when processes or address spaces terminate.

Only the minimum mechanisms required for isolation belong in the TCB. Filesystem policy, databases, AI inference, blockchain state, content indexing, application policy and storage federation remain outside the TCB.

### 4.2 GlobusOS — userspace policy and storage services

GlobusOS MUST own:

- process/agent memory budgets and admission policy;
- service-level resource quotas and accounting;
- VFS, filesystem namespace, mounts and volume policy;
- block-device and storage-service lifecycle;
- persistent storage orchestration and recovery interfaces;
- service authorization and IPC to kernel memory objects.

The existing `globus-memory` crate is treated as a userspace abstraction/policy boundary until its complete call graph and hardware-backed integration are verified. It MUST NOT independently allocate physical frames or mutate canonical kernel page tables.

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

The following are proposed mandatory architecture invariants. They require owner/security review before normative adoption.

1. **Single allocator authority:** exactly one canonical physical frame allocator in ShivaCore.
2. **No ambient authority:** every privileged memory operation requires an explicit capability/object right.
3. **Least privilege:** allocations do not automatically receive execute or delegation rights.
4. **W^X:** writable and executable mappings MUST NOT be granted simultaneously unless a separately reviewed architecture exception exists.
5. **NX by default:** executable permission is explicit and separately authorized.
6. **Kernel/user isolation:** user address spaces cannot map supervisor-only kernel memory.
7. **Checked arithmetic:** address, length, page-count and range calculations use checked operations; overflow fails closed.
8. **Transactional mapping:** failed map/unmap/protect operations roll back intermediate page-table mutations and reclaim only resources proven unreferenced.
9. **Lifetime safety:** mappings, address spaces, capabilities and physical frames have explicit ownership and lifetime rules; stale references cannot survive reclamation.
10. **TLB safety:** a physical frame MUST NOT be reused until required local and remote TLB invalidations have completed.
11. **SMP safety:** shared frame allocator, page-table mutation and scheduler/address-space state are synchronized for the declared CPU execution model.
12. **Fail closed:** invalid faults, unauthorized mappings, malformed ranges and resource exhaustion return explicit errors or terminate the affected workload according to policy; they do not widen permissions.
13. **Secure reclamation:** sensitive memory is zeroized before reuse; ordinary memory reclamation cannot bypass object/capability revocation.
14. **DMA isolation:** DMA-capable devices use the declared IOMMU/DMA ownership and mapping contract; CPU page-table isolation alone is not sufficient.
15. **No persistence in the kernel:** filesystem, database, block-storage, AI memory and chain persistence logic remain userspace/domain services.

## 6. Memory object and interface contract

This document does not freeze concrete Rust types or syscall numbers. Those belong to the canonical ShivaCore ABI and HAL specifications.

The interface model MUST distinguish:

- `PhysicalFrameRange`: owned/reserved physical frames;
- `MemoryObject`: kernel-managed memory object with explicit owner and lifecycle;
- `AddressSpace`: an isolated virtual address-space object;
- `Mapping`: a relationship between a memory object/range and a virtual range;
- `MemoryRights`: explicit READ, WRITE, EXECUTE and controlled delegation rights;
- `MemoryQuota`: userspace policy limit, not physical-memory authority;
- `MemoryFault`: validated fault metadata, not an implicit allocation request.

Required lifecycle:

`RESERVE → ALLOCATE → CREATE_OBJECT → AUTHORIZE → MAP → USE → UNMAP → REVOKE → ZEROIZE_IF_REQUIRED → RECLAIM`

Every transition MUST define:
- caller identity and capability;
- object owner and lifetime;
- alignment and range validation;
- permission transition;
- concurrency/locking requirements;
- rollback behavior;
- error semantics;
- audit/diagnostic event where security-relevant.

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
| MEM-01 | `memory_manager.rs` currently creates READ \| WRITE \| EXEC \| DELEGATE rights on allocation | Source at GlobusOS baseline; capability creation call | RESIDUAL / HIGH |
| MEM-02 | `UserspaceBump` is explicitly described as a virtual-address simulation, not a hardware-backed process mapping | Source comments and allocation branch in `memory_manager.rs` | NOT_VERIFIED as real userspace mapping |
| MEM-03 | `globus-memory/src/paging.rs` models mappings in a `std::collections::BTreeMap`; it does not itself mutate hardware page tables | Source implementation | MODEL / NOT_HARDWARE_VERIFIED |
| MEM-04 | `globus-memory/src/frames.rs` implements a bounded monotonic allocator model; a full free/reclaim/coalescing lifecycle is not shown in this module | Source implementation | PARTIAL |
| MEM-05 | `memory.rs` frame allocation reconstructs/scans the usable-frame iterator with `nth(next)`; allocation complexity and lifecycle need review | Source implementation | REVIEW_REQUIRED |
| MEM-06 | Mapping mutation rollback, page-table cleanup and object/mapping lifetime remain explicit P0 acceptance criteria | `FN-VMM-001` in ecosystem delivery registry | IN_PROGRESS |
| MEM-07 | Cross-CPU TLB shootdown, per-CPU scheduler state and frame reclamation are open in the active kernel-isolation work | GlobusOS PR #89 description and patch | BLOCKED / RESIDUAL |
| MEM-08 | The standalone `atc-storage` repository is archived; its migrated component is located at `a-townchain/components/storage` | `MIGRATION_MANIFEST.yaml`, repository metadata and control-plane registry | SSOT LOCATION CONFIRMED; implementation maturity separate |
| STOR-01 | `components/storage/src/store.rs` is an in-memory `BTreeMap` KV store with an explicitly non-cryptographic FNV-1a development fingerprint | Source blob `3755e0ce0f41e645ad40d00838403a624244a730` | RESIDUAL / NOT_DURABLE / NOT_CRYPTOGRAPHIC |
| STOR-02 | Durable node persistence and restart recovery are not established by the inspected `atc-node` root manifest or architecture document | `atc-node/Cargo.toml` and `ARCHITECTURE.md` inspected at baseline | NOT_VERIFIED / INVENTORY_REQUIRED |

These findings are source-level observations. They are not a substitute for a complete call-graph audit, target build, QEMU/UEFI boot, runtime test, security review or exact-SHA CI evidence.

## 10. Existing work that MUST be reconciled, not duplicated

- GlobusOS PR #89: open Draft; address-space isolation and service-space decoupling work. Its current description explicitly leaves SMP safety and remote TLB shootdown unresolved.
- GlobusOS PR #108: open; touches `memory_manager.rs` while resolving kernel/boot Clippy blockers. Do not create a competing patch against the same file.
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
2. **Kernel contract review:** reconcile this proposal with `SHIVA-KERNEL-REUSE-001`, `SHIVA-HAL-001`, `SHIVA-ABI-001`, `FN-KERNEL-001` and `FN-VMM-001`.
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
- Kernel implementation SSOT: `globus-os/modules/atc-shivacore/kernel/`
- Normative SSOT: `atc-standards`
- Cross-repository architecture/evidence owner: `a-townchain-ecosystem`
- New runtime implementation: NOT_AUTHORIZED BY THIS DOCUMENT
- New normative standard ID: NOT_ALLOCATED
- Inventory completeness: PARTIAL
- Architecture approval: PENDING
- Implementation verification: NOT_ESTABLISHED
- Production readiness: NOT_CLAIMED
