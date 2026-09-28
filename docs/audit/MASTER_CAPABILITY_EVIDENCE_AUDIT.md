# Master Capability Evidence Audit

## Purpose

This document defines the machine-auditable contract for the Master Architecture implementation audit.

The audit MUST distinguish:

- architecture from implementation;
- implementation from tests;
- tests from CI verification;
- historical evidence from evidence bound to the assessed source commit;
- component existence from integration;
- local E2E from system E2E.

## Canonical status model

`ARCHITECTURE_ONLY`, `SPECIFIED`, `IMPLEMENTED`, `TESTED`, `EVIDENCE_STALE`, `CI_VERIFIED`, `INTEGRATED`, `E2E_VERIFIED`, `MISSING`, `BLOCKED`, `DUPLICATE`, `DISCONNECTED`, `CONTRACT_DRIFT`.

A status MUST NOT be promoted without the evidence required by the preceding gate.

## Evidence binding invariant

For a capability assessed at source commit `S`:

```text
evidence.bound_commit == S
AND
evidence.test_run.commit == S
AND
test_run.result == PASS
```

is required before the capability may be classified `CI_VERIFIED`.

A historical PASS whose commit differs from `S` is `EVIDENCE_STALE`, never `CI_VERIFIED`.

An immutable CI artifact for `S` is valid evidence even if a later metadata commit changes `.atc/evidence/evidence.yaml`; the artifact and workflow run remain bound to `S`.

## Capability audit record

Every capability record MUST contain:

1. current main SHA;
2. architecture reference;
3. owner repository;
4. implementation source paths;
5. contract/API/ABI paths;
6. dependency edges;
7. call/data path;
8. test paths;
9. exact-SHA CI run IDs;
10. evidence artifact or evidence file;
11. evidence binding result;
12. integration edge evidence;
13. system E2E evidence;
14. duplicate/orphan/drift result;
15. final status.

## Critical system paths

### L1/L2/L3

```text
SDK
  -> Node
  -> Mempool
  -> Signature / Authorization
  -> Consensus
  -> ATC-VM
  -> State Transition
  -> Canonical State
  -> Storage
  -> Indexer
```

### Language / execution

```text
ATCLang
  -> ATC-IR
  -> ABI
  -> Bytecode
  -> Verifier
  -> ATC-VM
  -> State Transition
```

### Aurora authority

```text
Model
  -> Aurora Runtime
  -> Capability
  -> Policy
  -> Approval
  -> Tool Executor
  -> Authorized Resource
```

### Genesis intelligence

```text
Quest / Dialogue Definition
  -> Runtime
  -> Aurora / Agent Interface
  -> Validation
  -> Deterministic Game-State Mutation
  -> Persistence
```

## Non-negotiable boundary invariants

- Aurora is not consensus authority.
- AI inference is not authoritative state mutation.
- Genesis Engine is not Blockchain Core.
- GlobusOS/ShivaCore is not ATC-VM.
- Indexer and Explorer are not canonical state authorities.
- AI Memory is not canonical blockchain state.
- Component repositories retain implementation SSOT.
- The ecosystem repository owns system architecture and evidence aggregation.

## Audit interpretation

`Source = YES` means concrete implementation was found.

`Tests = YES` means concrete test code was found.

Neither implies CI verification.

`CI_VERIFIED` requires exact source-SHA evidence.

`INTEGRATED` requires a concrete cross-component edge.

`E2E_VERIFIED` requires the complete intended path to execute successfully with evidence bound to the assessed revisions.

## Fix gate

No implementation correction should be made solely because a capability is absent from a first-pass matrix. First establish:

```text
CURRENT MAIN SHA
 -> SOURCE
 -> CONTRACT
 -> DEPENDENCY
 -> CALL PATH
 -> TEST
 -> EVIDENCE
 -> SHA BINDING
 -> INTEGRATION
 -> E2E
 -> DUPLICATE / DRIFT
 -> ROOT CAUSE
 -> MINIMAL FIX
```

The audit therefore remains the source of truth for the subsequent fix gate.
