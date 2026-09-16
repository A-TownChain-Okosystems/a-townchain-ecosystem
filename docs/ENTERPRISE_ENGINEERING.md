# Genesis Engine — Enterprise Engineering Baseline

**Status:** Development / Enterprise hardening in progress  
**Scope:** Workspace, runtime, networking, security, CI/CD and release engineering

## Integration audit

The runtime depends directly on `atc-genesis-ecs` through a local Cargo path dependency. The repository tree contains `modules/atc-genesis-ecs`, but the root workspace member list did not previously register that crate. This was a workspace consistency defect: the dependency existed physically and in `modules/atc-genesis-runtime/Cargo.toml`, while the root workspace omitted it.

The root `Cargo.toml` now explicitly includes `modules/atc-genesis-ecs`. This makes ECS part of the same workspace graph as runtime, physics, gameplay, input, animation, audio, networking and the other engine modules.

## Runtime network trust boundary

Transport → packet-size boundary → binary decoding → wire-format validation → session validation → peer/sequence/tick validation → expected-entity ownership validation → existing ECS entity validation → ECS state application.

`PacketGuard` enforces packet size, session identity, packet structure, entity/tick consistency, replay protection, monotonic tick ordering, and replicated quaternion bounds. Unknown ECS entities are never implicitly created from replication traffic.

## Replication and security hardening

The former `rotation_millirad` field was inconsistent with the actual quaternion x/y/z micro-unit representation. It is now `rotation_xyz_microunits`, with a consistent `1e-6` interpretation. The packet byte layout remains 64 bytes.

Invalid quaternion vector magnitudes are rejected using deterministic integer-only validation before network acceptance or ECS mutation. Local publication rejects non-finite transforms and normalizes valid quaternions before quantization. Regression coverage exercises malformed rotations, interpolation rejection, secure-packet rejection, non-finite transforms and no-mutation-on-rejection behavior.

## Error register

| ID | Finding | Severity | State | Resolution |
|---|---|---|---|---|
| NET-001 | Replication rotation units were inconsistent | High | Resolved in source | Unified on `rotation_xyz_microunits` |
| SEC-003 | Invalid quaternion vector magnitude could cross the network trust boundary | High | Resolved in source | Integer norm validation and fail-closed rejection |
| CONS-001 | Stale `rotation_millirad` references remained after wire-format correction | High | Resolved in source | References removed and repository search performed |
| CONS-002 | `atc-genesis-ecs` was a runtime path dependency but absent from root workspace members | High | Resolved in source | ECS explicitly added to `[workspace].members` |
| REL-001 | No committed `Cargo.lock` | High | Open | Generate, review and commit before reproducible release claims |
| SEC-001 | No verified cryptographic peer authentication/encryption | High | Open | Production transport security required |
| NET-002 | No verified UDP/QUIC production transport | High | Open | Production transport implementation and integration evidence required |
| SEC-002 | No verified rate limiting/bandwidth/DoS controls | High | Open | Resource governance and abuse tests required |
| REL-002 | No verified SBOM/signing/attestation | Medium | Open | Release supply-chain pipeline required |

## Verification status

Source-level verification for this audit confirms the ECS directory exists, runtime declares the ECS path dependency, and the root workspace now explicitly registers ECS. The runtime source connects ECS with world, physics, gameplay, animation, renderer and network paths.

A successful local `cargo check`, `cargo test`, `cargo fmt`, `cargo clippy` or `cargo doc` run is **not** claimed. Full CI remains the authoritative verification layer.

## Remaining production security gaps

- cryptographic peer authentication
- encrypted transport
- production UDP/QUIC transport
- rate limiting and bandwidth budgets
- connection lifecycle management
- key rotation
- transport-level DoS protection
- committed release lockfile
- SBOM and artifact signing/attestation

> No Evidence, No Trust.
