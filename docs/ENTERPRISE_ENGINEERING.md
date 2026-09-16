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

## CI quality gates

`.github/workflows/enterprise-ci.yml` defines formatting, compilation, tests, Clippy, documentation, RustSec and Pull Request dependency-review gates. CI uses least-privilege permissions, timeouts and concurrency cancellation.

## Dependency governance

`.github/dependabot.yml` monitors Cargo and GitHub Actions dependencies. A committed `Cargo.lock` remains a release-hardening requirement; the current security job generates a lockfile only for its audit run.

## Determinism

Network positions use integer millimetres. Sequence numbers and simulation ticks are explicit and monotonic. Snapshot and prediction structures provide deterministic replication primitives.

The current rotation wire field retains the historical `rotation_millirad` name, although the runtime stores scaled quaternion x/y/z components. This naming mismatch is tracked as a protocol-cleanup item and must be resolved before declaring the network wire protocol stable.

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

## Release evidence

Production promotion requires actual CI/security/integration evidence, reproducible build evidence, release checksums, governance review and explicit release authorization.

> No Evidence, No Trust.
