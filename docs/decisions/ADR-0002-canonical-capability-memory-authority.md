# ADR-0002: Canonical Capability and Physical-Memory Authority

- Status: Accepted — architecture-owner approval recorded 2026-11-02; implementation is staged behind the technical gates below
- Scope: Architecture decision proposal for ShivaCore / GlobusOS memory authority
- Related PR: [a-townchain-ecosystem#241](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/241)
- Evidence baseline: GlobusOS `f370b681e5c9ff49577f25c24f15b5fc880d5cba`
- Normative dependencies: frozen ShivaCore SC-001, SC-005 and SC-007
- Existing delivery tracking: `FN-KERNEL-001`, `FN-VMM-001` (no new task IDs)
- Architecture-owner approval: D1–D6 explicitly approved by the user acting as A-TownChain/ShivaCore architecture owner on 2026-11-02
- Remaining authorities: GlobusOS implementation review, ShivaCore normative governance for any required spec/boot-contract reconciliation, and applicable security/release reviewers
- Change class: Accepted architecture direction and implementation authorization for staged work. Frozen normative specifications remain unchanged; any required normative reconciliation follows a separate versioned owner/governance process.

## 1. Context

The Memory Conformance Audit and the follow-up Capability / Untyped-Retype Authority Trace identified that the current GlobusOS kernel source does not demonstrate one SC-005 CSpace authority, SC-001 UntypedMemory/Retype ownership, or a typed capability path into the existing SC-007 VMM.

The inspected boot path constructs independent capability tables for MemorySubsystem, ProcessManager and VFS. The current capability implementation is a flat PID/CapId map with generic rights. Physical frames are enumerated from bootloader `Usable` regions by `BootInfoFrameAllocator`; no SC-001 UntypedMemory capability and Retype lifecycle was identified. The existing `vmm.rs` hardware page-table algorithm is an implementation asset, but its production frame backend and SC-005/SC-007 authority integration are not established.

The architecture owner has approved D1–D6 as the binding implementation direction. This approval does not itself resolve the SC-001 Limine B0–B5 versus current `bootloader_api` boot-contract discrepancy, does not amend frozen contracts, and does not establish hardware conformance.

## 2. Decision drivers and constraints

1. Frozen SC-001, SC-005 and SC-007 remain normative and unchanged.
2. One canonical capability authority; no independently-created subsystem authority tables.
3. No ambient authority and no generic all-rights grants.
4. Physical frame provenance, ownership, derivation, consumption and reclamation must be auditable.
5. Existing `vmm.rs` hardware mapping and rollback logic must be evaluated for integration, not replaced by a parallel VMM.
6. `system/memory` remains an abstraction/model layer and must not become a second hardware memory authority.
7. Kernel implementation is authorized only in staged increments consistent with this ADR; boot-contract-dependent code remains gated until the SC-001/SHIVA-BOOT discrepancy is resolved through the versioned owner process.
8. No release or VERIFIED status without exact-head tests, boot/E2E evidence and governance/security gates.

## 3. Proposed decisions for owner approval

### D1 — Canonical CSpace root and ownership

**Approved decision:** Establish exactly one kernel-owned CSpace authority root, created at the SC-001-defined boot/Initial Task authority boundary and governed by the SC-005 object-capability model. The kernel authority custodian is the sole authority for root integrity, CNode hierarchy, slot allocation, typed mint/derive/revoke operations, rights attenuation and global revocation coordination. Its concrete initialization hook must conform to the resolved SC-001 boot handoff.

The kernel authority owner is responsible for:
- root creation and integrity;
- CNode hierarchy and slot allocation policy;
- typed capability mint/derive/revoke operations;
- derivation-depth and rights attenuation enforcement;
- global revocation coordination with frame mappings and AddressSpaces.

No subsystem constructor may silently create a new independent root/table.

**Approval recorded:** The single kernel-owned root and SC-001 Initial Task authority boundary are approved. The implementation must not introduce a second root or choose a boot handoff that contradicts frozen SC-001.

### D2 — Capability distribution to subsystems

**Recommendation:** Pass constrained references/handles to CSpace slots or narrowly-scoped invocation interfaces. Do not clone or inject independent mutable capability tables.

| Consumer | Proposed access boundary | Must not receive |
|---|---|---|
| ProcessManager | Process/AddressSpace lifecycle operations and explicitly delegated slots | Global CSpace root or ambient mint authority |
| MemorySubsystem | Typed memory-object operations and scoped allocation/retype authority | Unrestricted generic capability table |
| VFS | Object-bound file/inode capabilities with operation-specific rights | Unrelated memory or global authority |
| IPC | Endpoint capabilities and badge/transfer semantics per SC-005 | Generic resource-wide authority |
| Syscall dispatcher | Resolve caller CSpace and invoke typed kernel object operations | Caller-selected arbitrary capability table |
| Service-space consumers | Explicitly delegated service capabilities at the kernel/service boundary | Direct authority over kernel CSpace internals |

All authorization must bind the capability's type, object identity, owner/derivation lineage and requested operation. A rights-bit check without target-object binding is insufficient.

### D3 — Untyped physical-memory ownership

**Approved decision:** SC-001 is the normative target for physical-memory classification and handoff. Every managed physical frame must be traceable to either the Kernel-Control reserve or an UntypedMemory capability owned/delegated through the prescribed Initial Task path. The current `bootloader_api` implementation is non-conformant until a versioned owner/governance resolution reconciles it with SC-001 Limine B0–B5; do not claim boot conformance or implement an incompatible adapter.

The implementation design and boot-contract resolution evidence must define:
- exact Kernel-Control reserve boundaries and protected ranges;
- Untyped region granularity and identity;
- Initial Task ownership and delegation;
- treatment of firmware, bootloader, MMIO, reserved and unusable ranges;
- reconciliation of the current `bootloader_api` path with SC-001 Limine B0–B5.

A `MemoryRegionKind::Usable` iterator alone is not an ownership transfer and must not be treated as proof of Untyped authority.

### D4 — Retype lifecycle and accounting

**Approved decision:** Retype is an authorized typed operation over an owned UntypedMemory object, with deterministic accounting and failure semantics. Implementation must satisfy the following lifecycle contract:

The implementation design must specify:
1. source Untyped capability and owner;
2. object type, size, alignment and address constraints;
3. atomic consumption/accounting on success and no partial ownership mutation on failure;
4. derived object capability and rights;
5. destruction/revocation ordering;
6. whether and under what exact conditions memory may be reset/retyped back;
7. zeroing, sanitization and memcheck obligations before reuse;
8. audit evidence linking source region, derived object, current owner and terminal state.

No allocator API or frame-reclaim behavior should be implemented until these semantics are approved.

### D5 — Existing VMM integration

**Approved decision:** Retain the existing `vmm.rs` hardware page-table mutation and transactional rollback implementation as the sole VMM implementation. Integrate it behind the frozen SC-005/SC-007 invocation boundary:

- Frame capability GRANT authorizes the relevant frame transfer/use;
- AddressSpace capability MAP authorizes the target mapping;
- mapping rights must be a subset of frame rights and must enforce SC-007 W^X;
- kernel retains page-table ownership;
- UNMAP must complete the required local and remote TLB invalidation before frame reclamation;
- global frame/mapping ownership must support revocation across all AddressSpaces.

Raw frame/virtual-address/page-flag APIs must not become a bypass around the typed capability checks. No replacement VMM or parallel hardware mapper is proposed.

### D6 — Cross-space revocation and reclamation

**Approved decision:** A kernel-owned lifecycle authority coordinates all mappings of a frame across AddressSpaces. Reclamation is permitted only after capability revocation, all required unmaps, TLB shootdown acknowledgements, and outstanding references/leases are complete. The synchronization and SMP failure policy must be specified and tested before enabling reclamation.

## 4. Alternatives considered

| Alternative | Assessment |
|---|---|
| Keep independent subsystem CapabilityTables | Reject: fragments authority and revocation; does not satisfy the canonical SC-005 CSpace objective |
| Share one generic mutable CapabilityTable among all subsystems | Reject: removes instance duplication but retains flat generic rights and does not implement typed CSpace/CNode semantics |
| Give every subsystem unrestricted root access | Reject: ambient authority and excessive blast radius |
| Add a second allocator/VMM/capability subsystem alongside current code | Reject: creates competing hardware authorities and violates the existing SSOT boundary |
| Typed, scoped CSpace references plus SC-001 Untyped/Retype objects and existing VMM integration | Preferred direction, approved direction, subject to exact frozen-contract interpretation and implementation evidence |

## 5. Consequences of approval

- One auditable authority chain from boot memory classification through Untyped ownership, Retype, typed Frame objects, AddressSpace mapping and revocation.
- Subsystems receive least-authority interfaces rather than independent capability stores.
- Existing VMM code is integrated and completed rather than duplicated.
- Implementation will require coordinated migration of ProcessManager, MemorySubsystem, VFS, IPC, syscall dispatch and any service-space consumer.
- Boot-contract resolution is a prerequisite; no claim of SC-001 hardware boot conformance is implied.

## 6. Implementation closure checklist — not a reversal of D1–D6 approval

- [x] D1 approved: one kernel-owned CSpace root at the SC-001 Initial Task authority boundary.
- [x] D2 approved: constrained subsystem capability references; no independent subsystem tables or ambient root access.
- [ ] D3 implementation prerequisite: complete the versioned SC-001 / SHIVA-BOOT boot-contract reconciliation; SC-001 remains normative meanwhile.
- [x] D3 target approved: Kernel-Control and UntypedMemory partition/handoff follow SC-001.
- [x] D4 approved: typed Retype, deterministic accounting, failure atomicity, destruction/revocation and controlled reclaim.
- [x] D6 approved: cross-AddressSpace revocation and TLB/SMP completion before reclamation.
- [x] D5 approved: existing `vmm.rs` remains the sole VMM and is integrated through SC-005/SC-007.
- [ ] Implementation detail: classify ATCFS at its canonical ownership boundary and enforce the D2 least-authority interface.

## 7. Implementation gate

The architecture direction D1–D6 is approved. Implementation is now authorized in staged, reviewable increments, but each increment remains gated by its technical prerequisites:

- **Boot-contract-dependent physical-memory/Untyped implementation: BLOCKED** until the SC-001 / SHIVA-BOOT discrepancy is resolved by the versioned owner process.
- **Any change to frozen SC-001/SC-005/SC-007: BLOCKED** absent a separate normative change process.
- **Other implementation increments: AUTHORIZED** only where they can be proven not to depend on the unresolved boot contract and conform to the frozen contracts.
- **Frozen SC-001/SC-005/SC-007 changes: BLOCKED absent a separate normative change process.**
- **PR #241: documentation-only Draft; do not merge as normative approval.**
- **No parallel allocator, CSpace authority, VMM, frame manager or fault subsystem.** Changes must extend the canonical GlobusOS kernel SSOT and reuse `FN-KERNEL-001` / `FN-VMM-001`.
- Continue tracking through existing `FN-KERNEL-001` and `FN-VMM-001`.

Following the recorded D1–D6 approval, implementation proceeds in separately reviewable increments against the canonical GlobusOS kernel SSOT, with exact commit/path/symbol/blob evidence, targeted unit and fault-injection tests, real SC-001 boot evidence, SC-005 capability tests, SC-007 mapping/revocation/TLB E2E tests, and exact-head CI artifacts. CI green alone is not VERIFIED.

## 8. Evidence references

- Architecture/conformance record: `docs/architecture/ATC-SMM-ARCHITECTURE-DRAFT.md` in PR #241, including MEM-09…MEM-24.
- GlobusOS source baseline: `f370b681e5c9ff49577f25c24f15b5fc880d5cba`.
- Initial D5 security increment: GlobusOS PR [#111](https://github.com/A-TownChain-Okosystems/globus-os/pull/111), head `26e9cfa78f46a1eecd208139013e7c12f7939e10`, `allocator.rs` blob `8ab10a3bc9950d537263abc31a97527d93d51e10`; adds NX to the kernel heap mapping and a feature-gated W^X flag test. PR is Draft; CI is pending and this is not yet VERIFIED.
- Initial D2 least-authority hardening: GlobusOS PR [#112](https://github.com/A-TownChain-Okosystems/globus-os/pull/112), head `12abc51e0ea34e1fb9acac4b8b16ac5d7924557d`; removes implicit EXEC from generic memory capabilities created by `ProcessManager::spawn` and `KernelMemoryManager::allocate`, with negative assertions in existing tests. Process blob `d9966f5debecf7d17913135fe1da83c0352cf242`; memory-manager blob `6303a3f39285b3d64dc435f449908a130f0f419f`. PR is Draft; exact-head checks are queued/in progress; not CI_VERIFIED or SC-005 conformant.
- Frozen SC-001 blob: `6927a89aabc7a1c1a0bb9f07f911e95d3b698fa3`.
- Frozen SC-005 blob: `2e19216b43237fcbd6e175a7abf15c1781cde4ca`.
- Frozen SC-007 blob: `8d61e3e9fa8951ccf20924254ba1c48fb239bfee`.
- SC-005 CSpace migration contract: GlobusOS PR [#113](https://github.com/A-TownChain-Okosystems/globus-os/pull/113), head `75103900f9d4613e0e3227db58c1a0434d2a5713`, file `docs/architecture/SC-005-CSPACE-MIGRATION-CONTRACT.md`. Documentation-only translation of frozen SC-005 REQ-SC005-01…11 / INV-01…09, cross-checked against frozen SC-003 and SC-004, with expanded production/test call-site census and explicit unresolved 16-bit rights-bit encoding decision; PR remains Draft; no implementation or conformance claim.

This ADR records accepted architecture direction D1–D6 and authorization for staged implementation. It is not evidence of implementation, CI/E2E verification, hardware boot conformance, or production readiness.
