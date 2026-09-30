# A-TownChain Ecosystem — Event Matrix

**Document ID:** ATC-EVENT-MATRIX-001  
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
| EVT-001 | Event Core | Event Identity | ARCHITECTURE CONTRACT |
| EVT-002 | Event Core | Event Type | ARCHITECTURE CONTRACT |
| EVT-003 | Event Core | Event Version | ARCHITECTURE CONTRACT |
| EVT-004 | Event Core | Event Schema | ARCHITECTURE CONTRACT |
| EVT-005 | Event Core | Event Producer | ARCHITECTURE CONTRACT |
| EVT-006 | Event Core | Event Consumer | ARCHITECTURE CONTRACT |
| EVT-007 | Blockchain Events | Event Authority | ARCHITECTURE CONTRACT |
| EVT-008 | Blockchain Events | Event Envelope | ARCHITECTURE CONTRACT |
| EVT-009 | Blockchain Events | Event Timestamp | ARCHITECTURE CONTRACT |
| EVT-010 | Blockchain Events | Event Correlation ID | ARCHITECTURE CONTRACT |
| EVT-011 | Blockchain Events | Event Causation ID | ARCHITECTURE CONTRACT |
| EVT-012 | Blockchain Events | Event Payload | ARCHITECTURE CONTRACT |
| EVT-013 | OS Events | Event Validation | ARCHITECTURE CONTRACT |
| EVT-014 | OS Events | Event Serialization | ARCHITECTURE CONTRACT |
| EVT-015 | OS Events | Event Deserialization | ARCHITECTURE CONTRACT |
| EVT-016 | OS Events | Event Bus | ARCHITECTURE CONTRACT |
| EVT-017 | OS Events | Event Router | ARCHITECTURE CONTRACT |
| EVT-018 | OS Events | Event Filter | ARCHITECTURE CONTRACT |
| EVT-019 | Aurora Events | Event Subscription | ARCHITECTURE CONTRACT |
| EVT-020 | Aurora Events | Event Dispatch | ARCHITECTURE CONTRACT |
| EVT-021 | Aurora Events | Event Delivery | ARCHITECTURE CONTRACT |
| EVT-022 | Aurora Events | At-Least-Once | ARCHITECTURE CONTRACT |
| EVT-023 | Aurora Events | At-Most-Once | ARCHITECTURE CONTRACT |
| EVT-024 | Aurora Events | Exactly-Once Boundary | ARCHITECTURE CONTRACT |
| EVT-025 | Application Events | Ordering | ARCHITECTURE CONTRACT |
| EVT-026 | Application Events | Sequence Number | ARCHITECTURE CONTRACT |
| EVT-027 | Application Events | Partition | ARCHITECTURE CONTRACT |
| EVT-028 | Application Events | Replay | ARCHITECTURE CONTRACT |
| EVT-029 | Application Events | Checkpoint | ARCHITECTURE CONTRACT |
| EVT-030 | Application Events | Offset | ARCHITECTURE CONTRACT |
| EVT-031 | Event Transport | Retry | ARCHITECTURE CONTRACT |
| EVT-032 | Event Transport | Dead Letter | ARCHITECTURE CONTRACT |
| EVT-033 | Event Transport | Backoff | ARCHITECTURE CONTRACT |
| EVT-034 | Event Transport | Deduplication | ARCHITECTURE CONTRACT |
| EVT-035 | Event Transport | Idempotency | ARCHITECTURE CONTRACT |
| EVT-036 | Event Transport | Correlation | ARCHITECTURE CONTRACT |
| EVT-037 | Ordering | Aggregation | ARCHITECTURE CONTRACT |
| EVT-038 | Ordering | Projection | ARCHITECTURE CONTRACT |
| EVT-039 | Ordering | State Trigger | ARCHITECTURE CONTRACT |
| EVT-040 | Ordering | State Change | ARCHITECTURE CONTRACT |
| EVT-041 | Ordering | Block Event | ARCHITECTURE CONTRACT |
| EVT-042 | Ordering | Transaction Event | ARCHITECTURE CONTRACT |
| EVT-043 | Delivery | Consensus Event | ARCHITECTURE CONTRACT |
| EVT-044 | Delivery | Validator Event | ARCHITECTURE CONTRACT |
| EVT-045 | Delivery | Mempool Event | ARCHITECTURE CONTRACT |
| EVT-046 | Delivery | VM Event | ARCHITECTURE CONTRACT |
| EVT-047 | Delivery | Contract Event | ARCHITECTURE CONTRACT |
| EVT-048 | Delivery | Wallet Event | ARCHITECTURE CONTRACT |
| EVT-049 | Replay | Identity Event | ARCHITECTURE CONTRACT |
| EVT-050 | Replay | Policy Event | ARCHITECTURE CONTRACT |
| EVT-051 | Replay | Approval Event | ARCHITECTURE CONTRACT |
| EVT-052 | Replay | Aurora Agent Event | ARCHITECTURE CONTRACT |
| EVT-053 | Replay | Tool Event | ARCHITECTURE CONTRACT |
| EVT-054 | Replay | OS Process Event | ARCHITECTURE CONTRACT |
| EVT-055 | Security | Device Event | ARCHITECTURE CONTRACT |
| EVT-056 | Security | Update Event | ARCHITECTURE CONTRACT |
| EVT-057 | Security | Recovery Event | ARCHITECTURE CONTRACT |
| EVT-058 | Security | Genesis Event | ARCHITECTURE CONTRACT |
| EVT-059 | Security | Marketplace Event | ARCHITECTURE CONTRACT |
| EVT-060 | Security | NFT Event | ARCHITECTURE CONTRACT |
| EVT-061 | Observability | Indexer Event | ARCHITECTURE CONTRACT |
| EVT-062 | Observability | Audit Event | ARCHITECTURE CONTRACT |
| EVT-063 | Observability | Security Event | ARCHITECTURE CONTRACT |
| EVT-064 | Observability | Incident Event | ARCHITECTURE CONTRACT |
| EVT-065 | Observability | Telemetry Event | ARCHITECTURE CONTRACT |
| EVT-066 | Observability | Event Authorization | ARCHITECTURE CONTRACT |
| EVT-067 | Testing | Event Integrity | ARCHITECTURE CONTRACT |
| EVT-068 | Testing | Event Provenance | ARCHITECTURE CONTRACT |
| EVT-069 | Testing | Event Retention | ARCHITECTURE CONTRACT |
| EVT-070 | Testing | Event Privacy | ARCHITECTURE CONTRACT |
| EVT-071 | Testing | Event Testing | ARCHITECTURE CONTRACT |
| EVT-072 | Testing | Event Determinism | ARCHITECTURE CONTRACT |

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
