# A-TownChain Ecosystem — Cross-Boundary State & Event Flow Matrix

**Document ID:** ATC-CROSS-BOUNDARY-STATE-EVENT-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose

This matrix defines which state crosses each system boundary, which events represent or trigger cross-boundary processing, and which authority remains responsible for the resulting state transition.

**Architecture ≠ Implementation ≠ Integration Test ≠ CI Evidence ≠ Verification.**

## Canonical flow

`Source → State → Mutation/Trigger → Event → Payload → Transport → Target → Target State → Authority → Capability → Policy → Approval → Evidence → Verification`

## Boundary inventory

| ID | Boundary | Trigger / Operation | State Domain | Event Domain | Transport | Status |
|---|---|---|---|---|---|---|
| CBE-001 | Aurora ↔ GlobusOS | Aurora request | OS state | OS event | IPC | ARCHITECTURE CONTRACT |
| CBE-002 | GlobusOS ↔ ShivaCore | service request | kernel state | kernel event | syscall/IPC | ARCHITECTURE CONTRACT |
| CBE-003 | ShivaCore ↔ Hardware | hardware operation | device state | hardware event | HAL | ARCHITECTURE CONTRACT |
| CBE-004 | Aurora ↔ A-TownChain | transaction intent | chain state | transaction/finality event | RPC | ARCHITECTURE CONTRACT |
| CBE-005 | Wallet ↔ Node | signed transaction | mempool state | accept/reject event | RPC | ARCHITECTURE CONTRACT |
| CBE-006 | SDK ↔ Node/API | API request | API/session state | response/event | RPC/HTTP | ARCHITECTURE CONTRACT |
| CBE-007 | Mempool ↔ Consensus | candidate transaction | mempool/consensus state | proposal/vote event | internal protocol | ARCHITECTURE CONTRACT |
| CBE-008 | Consensus ↔ State | commit decision | canonical chain state | commit/finality event | state transition | ARCHITECTURE CONTRACT |
| CBE-009 | ATC-VM ↔ Blockchain State | VM execution | contract/state storage | VM/state event | execution host ABI | ARCHITECTURE CONTRACT |
| CBE-010 | ATCLang/Compiler ↔ VM | compiled artifact | artifact/runtime state | deployment/validation event | bytecode/ABI | ARCHITECTURE CONTRACT |
| CBE-011 | Frontend ↔ Backend | user request | session/application state | API/domain event | HTTPS/RPC | ARCHITECTURE CONTRACT |
| CBE-012 | Browser ↔ Ecosystem | navigation/action | browser/app state | navigation/security event | HTTPS/gth | ARCHITECTURE CONTRACT |
| CBE-013 | Genesis ↔ Blockchain | game transaction | game/canonical state | game/blockchain event | SDK/RPC | ARCHITECTURE CONTRACT |
| CBE-014 | Marketplace ↔ Blockchain | order/settlement | market state | order/settlement event | SDK/RPC | ARCHITECTURE CONTRACT |
| CBE-015 | NFT ↔ Blockchain | token operation | token ownership state | mint/transfer event | SDK/RPC | ARCHITECTURE CONTRACT |
| CBE-016 | Aurora ↔ API Orchestrator | agent intent | workflow state | workflow event | API | ARCHITECTURE CONTRACT |
| CBE-017 | Policy ↔ Execution | policy decision | authorization state | decision event | orchestrator | ARCHITECTURE CONTRACT |
| CBE-018 | Event Bus ↔ Consumers | published event | consumer checkpoint | delivery/replay event | event transport | ARCHITECTURE CONTRACT |
| CBE-019 | Indexer ↔ Applications | indexed update | derived index state | index event | query/API | ARCHITECTURE CONTRACT |
| CBE-020 | Evidence ↔ Verification | execution evidence | verification state | verification event | evidence pipeline | ARCHITECTURE CONTRACT |

## Mandatory cross-boundary contract

Every CBE entry MUST define:

- source and target
- source and target authority
- state owner
- state schema/version
- read/write permissions
- canonical versus derived state
- mutation authority
- event identity/type/version
- producer/consumer
- payload schema
- correlation and causation IDs
- sequence and ordering
- delivery semantics
- replay/checkpoint semantics
- deduplication and idempotency
- transport
- authentication/authorization
- capability/policy/approval
- state-before, trigger, mutation, emitted event, state-after
- timeout/retry/compensation/rollback
- audit/evidence/verification

## Authority invariants

1. An event is not by itself an authorization.
2. An event is not by itself an authoritative state transition.
3. Derived/indexed state never overrides canonical state.
4. UI, browser, model, agent and orchestrator output never becomes authorization merely by crossing a boundary.
5. Wallet signing remains an explicit cryptographic authority boundary.
6. A-TownChain consensus/state remains authoritative for blockchain state.
7. ShivaCore remains authoritative for the trusted kernel boundary.
8. Missing authoritative context fails closed where required.

## Verification chain

`Architecture → Boundary Contract → Source → Test → Integration/E2E → Exact-SHA CI → Run → Job → Step → Exit Code → Log → Verification → Residual`

## Evidence separation

`Error Evidence ≠ Finding Evidence ≠ Verification Evidence`
