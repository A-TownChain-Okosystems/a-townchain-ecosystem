---
document_id: ATC-DOC-SHIVACORE-STATUS-001
title: Repository Status - atc-shivacore
version: 2.1.0
status: active
standard: ATC-STD-MD-001
created: 2026-09-08
updated: 2026-09-15
---

# Status — atc-shivacore

## Property-Value Table

| Property | Value |
|---|---|
| Repository | atc-shivacore |
| Version | 0.1.0 |
| Lifecycle | development |
| Kernel model | capability-based Rust/no_std microkernel |
| Reuse target | OS-neutral kernel contract |
| Primary declared targets | x86_64; aarch64 contract target |
| Build evidence | Must be taken from current CI run |
| Boot evidence | BIOS/UEFI image-builder exists; current target must be CI-verified |
| Production status | NOT_READY |
| Security class | S4 / S-Klasse |
| Documentation | ATC-STD-README-001 / ATC-STD-MD-001 |

## Current cross-check

The active source tree was re-checked on 2026-09-15 against the repository implementation and organization architecture.

A real implementation blocker remains in `modules/atc-shivacore/kernel/src/lkm.rs`: `DependencyGraph::dependencies()` is still an `unimplemented!()` placeholder because the backing storage is a `BTreeSet` and cannot be returned as `&[String]`. `get_dependencies()` already provides the deterministic owned-vector form. This blocker is tracked as implementation work and must not be represented as production-ready functionality.

The archived copy under `a-townchain-os-docs/docs/archive/` is historical reference material and is not treated as active kernel source.

## Evidence policy

Historical test counts and past audit scores are not permanent state. The current commit's CI is authoritative for build/test claims.

## Reuse readiness

The reusable kernel contract is documented in:

- `docs/specs/SHIVA-KERNEL-REUSE-001.md`
- `docs/specs/SHIVA-HAL-001.md`
- `docs/specs/SHIVA-ABI-001.md`
- `docs/specs/SHIVA-BOOT-001.md`

These specifications define the architecture boundary. They do not by themselves prove production readiness.

## Release gate

`PRODUCTION_READY` requires successful target builds, boot/smoke evidence, IPC/ABI validation, security review, resolution of active implementation blockers and governance approval for the release candidate.
