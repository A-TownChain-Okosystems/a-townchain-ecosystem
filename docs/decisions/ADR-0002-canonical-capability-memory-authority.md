# ADR-0002: Canonical Capability and Physical-Memory Authority

- Status: Proposed — owner decision required; NOT ACCEPTED
- Scope: Architecture decision proposal for ShivaCore / GlobusOS memory authority
- Related PR: [a-townchain-ecosystem#241](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/241)
- Evidence baseline: GlobusOS `f370b681e5c9ff49577f25c24f15b5fc880d5cba`
- Normative dependencies: frozen ShivaCore SC-001, SC-005 and SC-007
- Existing delivery tracking: `FN-KERNEL-001`, `FN-VMM-001` (no new task IDs)
- Decision authority: ShivaCore specification/architecture owner; implementation owner for GlobusOS; applicable governance reviewers
- Change class: Architecture proposal only. No normative specification or implementation change is authorized by this ADR.

## 1. Context

The Memory Conformance Audit and the follow-up Capability / Untyped-Retype Authority Trace identified that the current GlobusOS kernel source does not demonstrate one SC-005 CSpace authority, SC-001 UntypedMemory/Retype ownership, or a typed capability path into the existing SC-007 VMM.

The inspected boot path constructs independent capability tables for MemorySubsystem, ProcessManager and VFS. The current capability implementation is a flat PID/CapId map with generic rights. Physical frames are enumerated from bootloader `Usable` regions by `BootInfoFrameAllocator`; no SC-001 UntypedMemory capability and Retype lifecycle was identified. The existing `vmm.rs` hardware page-table algorithm is an implementation asset, but its production frame backend and SC-005/SC-007 authority integration are not established.

This ADR prepares owner decisions. It does not resolve the SC-001 Limine B0–B5 versus current `bootloader_api` boot-contract discrepancy, and does not amend frozen contracts.

## 2. Decision drivers and constraints

1. Frozen SC-001, SC-005 and SC-007 remain normative and unchanged.
2. One canonical capability authority; no independently-created subsystem authority tables.
3. No ambient authority and no generic all-rights grants.
4. Physical frame provenance, ownership, derivation, consumption and reclamation must be auditable.
5. Existing `vmm.rs` hardware mapping and rollback logic must be evaluated for integration, not replaced by a parallel VMM.
6. `system/memory` remains an abstraction/model layer and must not become a second hardware memory authority.
7. Kernel code remains read-only until the required owners explicitly approve the architecture and implementation scope.
8. No release or VERIFIED status without exact-head tests, boot/E2E evidence and governance/security gates.

## 3. Proposed decisions for owner approval

### D1 — Canonical CSpace root and ownership

**Recommendation:** Establish exactly one kernel-owned CSpace authority root, created at the SC-001-defined boot/Initial Task authority boundary and governed by the SC-005 object-capability model. The precise creation point and root custodian must be confirmed against the resolved boot contract before implementation.

The kernel authority owner is responsible for:
- root creation and integrity;
- CNode hierarchy and slot allocation policy;
- typed capability mint/derive/revoke operations;
- derivation-depth and rights attenuation enforcement;
- global revocation coordination with frame mappings and AddressSpaces.

No subsystem constructor may silently create a new independent root/table.

**Owner decision required:** Confirm whether the canonical root is owned by the kernel's Initial Task/boot authority, a dedicated kernel authority object, or another construction explicitly permitted by frozen SC-001/SC-005. This ADR does not unilaterally choose between those normative interpretations.

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

**Recommendation:** Physical memory classification and handoff must originate from the resolved SC-001 boot contract. Every managed physical frame must be traceable to either the Kernel-Control reserve or an UntypedMemory capability owned/delegated through the prescribed Initial Task path.

The owner decision must define:
- exact Kernel-Control reserve boundaries and protected ranges;
- Untyped region granularity and identity;
- Initial Task ownership and delegation;
- treatment of firmware, bootloader, MMIO, reserved and unusable ranges;
- reconciliation of the current `bootloader_api` path with SC-001 Limine B0–B5.

A `MemoryRegionKind::Usable` iterator alone is not an ownership transfer and must not be treated as proof of Untyped authority.

### D4 — Retype lifecycle and accounting

**Recommendation:** Retype must be an authorized typed operation over an owned UntypedMemory object, with deterministic accounting and failure semantics.

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

**Recommendation:** Retain the existing `vmm.rs` hardware page-table mutation and transactional rollback implementation as the sole candidate VMM implementation. Integrate it behind the frozen SC-005/SC-007 invocation boundary:

- Frame capability GRANT authorizes the relevant frame transfer/use;
- AddressSpace capability MAP authorizes the target mapping;
- mapping rights must be a subset of frame rights and must enforce SC-007 W^X;
- kernel retains page-table ownership;
- UNMAP must complete the required local and remote TLB invalidation before frame reclamation;
- global frame/mapping ownership must support revocation across all AddressSpaces.

Raw frame/virtual-address/page-flag APIs must not become a bypass around the typed capability checks. No replacement VMM or parallel hardware mapper is proposed.

### D6 — Cross-space revocation and reclamation

**Recommendation:** A kernel-owned lifecycle authority must coordinate all mappings of a frame across AddressSpaces. Reclamation is permitted only after capability revocation, all required unmaps, TLB shootdown acknowledgements, and outstanding references/leases are complete. The exact synchronization and SMP failure policy must be specified before implementation.

## 4. Alternatives considered

| Alternative | Assessment |
|---|---|
| Keep independent subsystem CapabilityTables | Reject: fragments authority and revocation; does not satisfy the canonical SC-005 CSpace objective |
| Share one generic mutable CapabilityTable among all subsystems | Reject: removes instance duplication but retains flat generic rights and does not implement typed CSpace/CNode semantics |
| Give every subsystem unrestricted root access | Reject: ambient authority and excessive blast radius |
| Add a second allocator/VMM/capability subsystem alongside current code | Reject: creates competing hardware authorities and violates the existing SSOT boundary |
| Typed, scoped CSpace references plus SC-001 Untyped/Retype objects and existing VMM integration | Preferred direction, subject to owner approval and exact frozen-contract interpretation |

## 5. Consequences if approved

- One auditable authority chain from boot memory classification through Untyped ownership, Retype, typed Frame objects, AddressSpace mapping and revocation.
- Subsystems receive least-authority interfaces rather than independent capability stores.
- Existing VMM code is integrated and completed rather than duplicated.
- Implementation will require coordinated migration of ProcessManager, MemorySubsystem, VFS, IPC, syscall dispatch and any service-space consumer.
- Boot-contract resolution is a prerequisite; no claim of SC-001 hardware boot conformance is implied.

## 6. Open owner decisions — approval checklist

- [ ] Confirm canonical CSpace root custodian and creation point under SC-001/SC-005.
- [ ] Confirm CNode hierarchy, slot allocation and subsystem delegation boundaries.
- [ ] Confirm the boot protocol and resolve SC-001 Limine B0–B5 vs current `bootloader_api` discrepancy through the proper owner/versioned process.
- [ ] Approve Kernel-Control reserve and UntypedMemory partition/handoff.
- [ ] Approve Retype accounting, failure atomicity, destruction and retype-back semantics.
- [ ] Approve cross-AddressSpace revocation and TLB/SMP completion contract.
- [ ] Approve the existing VMM integration boundary and required evidence gates.
- [ ] Confirm whether ATCFS is a kernel object or service-space component and align its authorization boundary.

## 7. Implementation gate

Until every applicable owner decision above is recorded and approved:

- **Kernel implementation: BLOCKED.**
- **Frozen SC-001/SC-005/SC-007 changes: BLOCKED absent a separate normative change process.**
- **PR #241: documentation-only Draft; do not merge as normative approval.**
- **No new allocator, CSpace implementation, VMM, frame manager or fault subsystem.**
- Continue tracking through existing `FN-KERNEL-001` and `FN-VMM-001`.

After approval, implementation must proceed in separately reviewable increments against the canonical GlobusOS kernel SSOT, with exact commit/path/symbol/blob evidence, targeted unit and fault-injection tests, real SC-001 boot evidence, SC-005 capability tests, SC-007 mapping/revocation/TLB E2E tests, and exact-head CI artifacts. CI green alone is not VERIFIED.

## 8. Evidence references

- Architecture/conformance record: `docs/architecture/ATC-SMM-ARCHITECTURE-DRAFT.md` in PR #241, including MEM-09…MEM-24.
- GlobusOS source baseline: `f370b681e5c9ff49577f25c24f15b5fc880d5cba`.
- Frozen SC-001 blob: `6927a89aabc7a1c1a0bb9f07f911e95d3b698fa3`.
- Frozen SC-005 blob: `2e19216b43237fcbd6e175a7abf15c1781cde4ca`.
- Frozen SC-007 blob: `8d61e3e9fa8951ccf20924254ba1c48fb239bfee`.

This ADR is a proposal for review, not an accepted architecture decision, implementation authorization, or conformance claim.
