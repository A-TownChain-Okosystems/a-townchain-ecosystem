# Genesis Engine — Enterprise Engineering Baseline

**Status:** Development / Enterprise hardening in progress  
**Scope:** Workspace, runtime, networking, security, CI/CD and release engineering

## Runtime network trust boundary

The secure runtime path is intentionally explicit. A packet is not trusted merely because it decoded successfully.

```text
Transport
   ↓
Maximum packet-size boundary
   ↓
Binary decoding
   ↓
Session validation
   ↓
Peer / sequence / tick validation
   ↓
Expected-entity ownership validation
   ↓
Existing ECS entity validation
   ↓
ECS state application
```

`PacketGuard` enforces packet size, session identity, packet structure, entity/tick consistency, replay protection and monotonic tick ordering. The runtime additionally exposes `receive_secure_network_packet_for_entity`, which requires the caller to specify the entity expected for that trust boundary. A mismatch is rejected before ECS mutation.

Unknown ECS entities are never implicitly created from replication traffic.

## Replication wire-format correction

An audit found a protocol naming/unit mismatch: the replication field was named `rotation_millirad`, but the runtime actually serializes the quaternion x/y/z components at a scale of 1,000,000. The interpolation path also treated those values as milliradians.

This was corrected by renaming the field to `rotation_xyz_microunits` and applying the same `1e-6` scale during interpolation and runtime dequantization. Regression coverage now exercises the corrected rotation interpolation scale.

This is a wire-format semantic correction, not a byte-layout change: the field remains three signed 32-bit integers at the same offsets, so the packet remains 64 bytes including the 24-byte header.

## CI quality gates

`.github/workflows/enterprise-ci.yml` defines formatting, compilation, tests, Clippy, documentation, RustSec and Pull Request dependency-review gates. CI uses least-privilege permissions, timeouts and concurrency cancellation.

## Dependency governance

`.github/dependabot.yml` monitors Cargo and GitHub Actions dependencies. A committed `Cargo.lock` remains a release-hardening requirement; the current security job generates a lockfile only for its audit run.

## Determinism

Network positions use integer millimetres. Quaternion x/y/z components use deterministic integer micro-units. Sequence numbers and simulation ticks are explicit and monotonic. Snapshot and prediction structures provide deterministic replication primitives.

## Error register

| ID | Finding | Severity | State | Resolution |
|---|---|---|---|---|
| NET-001 | `rotation_millirad` did not describe the actual quaternion wire units; interpolation used the wrong scale | High | Resolved in source | Renamed to `rotation_xyz_microunits`; interpolation/dequantization aligned to `1e-6`; regression test added |
| REL-001 | No committed `Cargo.lock` | High | Open | Must be generated, reviewed and committed before reproducible release claims |
| SEC-001 | No verified cryptographic peer authentication/encryption | High | Open | Requires production transport/security implementation and evidence |
| NET-002 | No verified production UDP/QUIC transport | High | Open | Requires implementation and integration evidence |
| SEC-002 | No verified rate limiting/bandwidth/DoS controls | High | Open | Requires explicit resource-governance implementation and tests |
| REL-002 | No verified SBOM/signing/attestation | Medium | Open | Requires release supply-chain pipeline |

## Security not yet production-verified

The following are not claimed as implemented or verified:

- cryptographic peer authentication
- encrypted transport
- production UDP/QUIC transport
- rate limiting and bandwidth budgets
- connection lifecycle management
- key rotation
- transport-level DoS protection
- committed release lockfile
- SBOM and artifact signing/attestation

## Verification policy

A source-level correction is not treated as fully verified until the workspace quality gates run against the updated commit. The repository must not claim production readiness from source inspection alone.

## Release evidence

Production promotion requires actual CI/security/integration evidence, reproducible build evidence, release checksums, governance review and explicit release authorization.

> No Evidence, No Trust.
