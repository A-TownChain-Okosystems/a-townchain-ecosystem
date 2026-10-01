# A-TownChain P0 Contract Alignment

This document is the human-readable companion to `control-plane/P0_CONTRACTS.yaml`.

## Normative model

Every privileged action is represented as:

```text
WHO → WHAT → WHERE
```

and expanded to:

```text
Principal
→ Authority
→ Allowed Action
→ Required Capability
→ Applicable Policy
→ Validation / Enforcement Boundary
→ Failure Semantics
→ Audit / Evidence
```

## P0 contracts

| ID | Contract | Boundary | Core rule |
|---|---|---|---|
| P0-NET-001 | Network / P2P | L4 ↔ L2 | Transport never creates authority |
| P0-ID-001 | Identity / Trust | X ↔ L1/L2/L4/L5 | Identity and authorization are explicit |
| P0-STATE-001 | State Ownership | L2 ↔ L1/L3/L5/L7 | L2 owns canonical blockchain state |
| P0-AUTH-001 | Global Authority Matrix | X ↔ L0–L7 | Every privileged action has WHO/WHAT/WHERE |
| P0-IF-001 | Canonical Interfaces | X ↔ L0–L7 | Types, encoding, ownership and versioning are explicit |
| P0-FR-001 | Failure / Recovery | L1/L2/L4 ↔ X | Recovery cannot silently rewrite canonical state |

## Verification boundary

These files define ecosystem-level contracts. They do **not** assert that every component implementation already satisfies them.

A contract becomes VERIFIED only when exact Source-SHA evidence proves the applicable implementation/enforcement and its required validation.

```text
Contract
  ↓
Implementation
  ↓
Exact Source SHA
  ↓
Run
  ↓
Job
  ↓
Step
  ↓
Log / Artifact Digest
  ↓
Result
```

Missing or ambiguous evidence is BLOCKED, not PASS.
