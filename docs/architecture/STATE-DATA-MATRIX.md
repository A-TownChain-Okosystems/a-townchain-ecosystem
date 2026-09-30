# A-TownChain Ecosystem — State & Data Matrix

**Document ID:** ATC-STATE-DATA-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT  
**Rule:** Architecture presence does not imply implementation, testing, CI verification, integration, or E2E verification.

## Canonical contract

Every entry is traced:

`Domain → Subsystem → Contract → Source → Test → Workflow → Exact-SHA → Evidence → Verification → Residual`

Evidence separation:

`Error Evidence ≠ Finding Evidence ≠ Verification Evidence`

## Matrix

| ID | Domain | Function / Contract | Status |
|---|---|---|---|
| DAT-001 | Data Ownership | State Owner | ARCHITECTURE CONTRACT |
| DAT-002 | Data Ownership | Schema Owner | ARCHITECTURE CONTRACT |
| DAT-003 | Data Ownership | Mutation Authority | ARCHITECTURE CONTRACT |
| DAT-004 | Data Ownership | Read Model | ARCHITECTURE CONTRACT |
| DAT-005 | Data Ownership | Canonical Store | ARCHITECTURE CONTRACT |
| DAT-006 | Data Ownership | Derived Store | ARCHITECTURE CONTRACT |
| DAT-007 | Canonical State | State Root | ARCHITECTURE CONTRACT |
| DAT-008 | Canonical State | Versioned Schema | ARCHITECTURE CONTRACT |
| DAT-009 | Canonical State | Serialization | ARCHITECTURE CONTRACT |
| DAT-010 | Canonical State | Deserialization | ARCHITECTURE CONTRACT |
| DAT-011 | Canonical State | Validation | ARCHITECTURE CONTRACT |
| DAT-012 | Canonical State | Normalization | ARCHITECTURE CONTRACT |
| DAT-013 | Schemas | Persistence | ARCHITECTURE CONTRACT |
| DAT-014 | Schemas | WAL | ARCHITECTURE CONTRACT |
| DAT-015 | Schemas | Snapshot | ARCHITECTURE CONTRACT |
| DAT-016 | Schemas | Recovery | ARCHITECTURE CONTRACT |
| DAT-017 | Schemas | Pruning | ARCHITECTURE CONTRACT |
| DAT-018 | Schemas | Index | ARCHITECTURE CONTRACT |
| DAT-019 | Persistence | Query | ARCHITECTURE CONTRACT |
| DAT-020 | Persistence | Cache | ARCHITECTURE CONTRACT |
| DAT-021 | Persistence | Replication | ARCHITECTURE CONTRACT |
| DAT-022 | Persistence | Synchronization | ARCHITECTURE CONTRACT |
| DAT-023 | Persistence | Migration | ARCHITECTURE CONTRACT |
| DAT-024 | Persistence | Compatibility | ARCHITECTURE CONTRACT |
| DAT-025 | Indexing | Lineage | ARCHITECTURE CONTRACT |
| DAT-026 | Indexing | Provenance | ARCHITECTURE CONTRACT |
| DAT-027 | Indexing | Classification | ARCHITECTURE CONTRACT |
| DAT-028 | Indexing | Retention | ARCHITECTURE CONTRACT |
| DAT-029 | Indexing | Deletion | ARCHITECTURE CONTRACT |
| DAT-030 | Indexing | Encryption | ARCHITECTURE CONTRACT |
| DAT-031 | Data Flow | Integrity | ARCHITECTURE CONTRACT |
| DAT-032 | Data Flow | Consistency | ARCHITECTURE CONTRACT |
| DAT-033 | Data Flow | Conflict Resolution | ARCHITECTURE CONTRACT |
| DAT-034 | Data Flow | Concurrency | ARCHITECTURE CONTRACT |
| DAT-035 | Data Flow | Transaction Boundary | ARCHITECTURE CONTRACT |
| DAT-036 | Data Flow | Idempotency | ARCHITECTURE CONTRACT |
| DAT-037 | Migration | Ordering | ARCHITECTURE CONTRACT |
| DAT-038 | Migration | Partitioning | ARCHITECTURE CONTRACT |
| DAT-039 | Migration | Sharding | ARCHITECTURE CONTRACT |
| DAT-040 | Migration | Backup | ARCHITECTURE CONTRACT |
| DAT-041 | Migration | Restore | ARCHITECTURE CONTRACT |
| DAT-042 | Migration | Import | ARCHITECTURE CONTRACT |
| DAT-043 | Privacy | Export | ARCHITECTURE CONTRACT |
| DAT-044 | Privacy | Archive | ARCHITECTURE CONTRACT |
| DAT-045 | Privacy | Event Projection | ARCHITECTURE CONTRACT |
| DAT-046 | Privacy | Materialized View | ARCHITECTURE CONTRACT |
| DAT-047 | Privacy | API DTO | ARCHITECTURE CONTRACT |
| DAT-048 | Privacy | RPC Schema | ARCHITECTURE CONTRACT |
| DAT-049 | Consistency | Contract State | ARCHITECTURE CONTRACT |
| DAT-050 | Consistency | Wallet State | ARCHITECTURE CONTRACT |
| DAT-051 | Consistency | OS State | ARCHITECTURE CONTRACT |
| DAT-052 | Consistency | Aurora State | ARCHITECTURE CONTRACT |
| DAT-053 | Consistency | Session State | ARCHITECTURE CONTRACT |
| DAT-054 | Consistency | Identity State | ARCHITECTURE CONTRACT |
| DAT-055 | Data Verification | Marketplace State | ARCHITECTURE CONTRACT |
| DAT-056 | Data Verification | NFT State | ARCHITECTURE CONTRACT |
| DAT-057 | Data Verification | Genesis State | ARCHITECTURE CONTRACT |
| DAT-058 | Data Verification | Indexer State | ARCHITECTURE CONTRACT |
| DAT-059 | Data Verification | Evidence State | ARCHITECTURE CONTRACT |
| DAT-060 | Data Verification | Audit State | ARCHITECTURE CONTRACT |
| DAT-061 | Testing | Telemetry State | ARCHITECTURE CONTRACT |
| DAT-062 | Testing | Config State | ARCHITECTURE CONTRACT |
| DAT-063 | Testing | Secret Metadata | ARCHITECTURE CONTRACT |
| DAT-064 | Testing | Data Access Policy | ARCHITECTURE CONTRACT |
| DAT-065 | Testing | Data Lifecycle | ARCHITECTURE CONTRACT |
| DAT-066 | Testing | Schema Registry | ARCHITECTURE CONTRACT |
| DAT-067 | Evidence | Data Contract Registry | ARCHITECTURE CONTRACT |
| DAT-068 | Evidence | Cross-System Data Contract | ARCHITECTURE CONTRACT |
| DAT-069 | Evidence | Data Test | ARCHITECTURE CONTRACT |
| DAT-070 | Evidence | Data Determinism | ARCHITECTURE CONTRACT |
| DAT-071 | Evidence | Data Verification | ARCHITECTURE CONTRACT |
| DAT-072 | Evidence | Data Residual | ARCHITECTURE CONTRACT |

## Authority rules

- Architecture does not create authority.
- UI, browser, model, agent, orchestrator and derived data never become authoritative merely by producing output.
- Authoritative state remains owned by its designated domain boundary.
- Cross-boundary mutation requires the applicable contract, capability, policy and approval path.
- Exact-SHA CI is required before an implementation claim becomes VERIFIED.
- Historical CI runs are not evidence for another SHA.
- Missing authoritative context fails closed where the contract requires it.

## Verification model

`Architecture → Contract → Source → Test → Integration → Exact-SHA CI → Run → Job → Step → Exit Code → Log → Verification → Residual`
