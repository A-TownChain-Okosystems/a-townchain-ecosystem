# P0 Boundary Contracts

Status: **ARCHITECTURE CONTRACT — normative boundary definition**
Source of truth: `ARCHITECTURE.md`
Scope: system-level boundaries owned by `a-townchain-ecosystem`; implementation remains in the responsible standalone repository.

These contracts close the six outstanding P0 architecture gaps without moving implementation ownership into the ecosystem repository.

## Contract set

| ID | Boundary | Owner | Required invariant |
|---|---|---|---|
| ARCH-P0-001 | Network / P2P | L0 + X | Network transport cannot become protocol state authority |
| ARCH-P0-002 | Identity / Trust | X | Identity authenticates principals; authorization requires capability/policy |
| ARCH-P0-003 | State Ownership | L1/L2/L3 + X | Exactly one canonical authority exists for each authoritative state domain |
| ARCH-P0-004 | Global Authority Matrix | X | Every cross-layer mutation has an explicit authority and denial rule |
| ARCH-P0-005 | Canonical Interfaces / Events | X + domain owners | Interfaces are versioned, typed, deterministic where protocol-critical |
| ARCH-P0-006 | Failure / Recovery | X + responsible domain | Fail-closed for security/protocol violations; recovery never bypasses authority |

## ARCH-P0-001 — Network / P2P

**Authority:** L0 provides transport/runtime; X applies identity, capability and policy; L2 decides protocol validity.

Rules:
1. Transport connectivity MUST NOT imply protocol trust.
2. Every authenticated peer MUST have a stable peer identity bound to an approved identity/trust contract.
3. P2P messages crossing a protocol boundary MUST identify protocol version, network/domain, sender identity and message type.
4. Invalid, replayed, unauthorized or version-incompatible protocol messages MUST be rejected without mutating canonical L1/L2/L3 state.
5. Gossip, discovery and transport availability MUST NOT determine finality.
6. Network failure MUST preserve the last finalized canonical state.

## ARCH-P0-002 — Identity / Trust

**Authority:** X.

Rules:
1. Authentication establishes **who/what** is presenting a credential; it does not grant a capability.
2. Authorization MUST evaluate principal, capability, resource, operation, policy and context.
3. Identity keys, transaction signing keys and consensus keys MUST remain distinct roles unless a frozen domain contract explicitly permits otherwise.
4. Revocation/expiry MUST fail closed for protected operations.
5. No ambient authority: a component MUST NOT gain authorization solely by process, network or filesystem presence.
6. Identity state used by protocol-critical validation MUST be deterministic and versioned.

## ARCH-P0-003 — State Ownership

Canonical authority:
- L0: hardware/kernel/network runtime state only.
- L1: durable persistence and read models; never consensus authority.
- L2: canonical blockchain state, transaction ordering, consensus and finality.
- L3: candidate deterministic state transitions under L2 authority.
- L4/L5: only explicitly delegated domain state.
- L6/L7: application/intelligence state; never direct canonical chain authority.
- X: authorization/policy metadata and evidence; never a substitute for L2 finality.

Rules:
1. Every authoritative state domain MUST name exactly one canonical owner.
2. Mirrors, caches, indexers and explorers MUST be explicitly non-authoritative.
3. A candidate L3 transition becomes canonical only through the applicable L2 finality/commit path.
4. Cross-domain state writes MUST reference an explicit interface/capability.
5. Recovery MUST restore from the canonical authority, not from a read model.

## ARCH-P0-004 — Global Authority Matrix

Every mutation MUST resolve to:

`principal → capability → operation → resource → policy → authority → evidence`

Minimum authority classes:
- `SYSTEM`: kernel/network boundary
- `STORAGE`: persistence only
- `CHAIN`: L2 protocol/finality
- `EXECUTION`: L3 candidate execution
- `DOMAIN`: delegated L4/L5 state
- `APPLICATION`: L7 state
- `INTELLIGENCE`: L6 proposals only
- `CONTROL`: X identity/policy/governance/evidence

Deny-by-default requirements:
1. Unknown authority class MUST deny.
2. Missing capability MUST deny.
3. Version mismatch MUST deny for protocol-critical operations.
4. Failed policy evaluation MUST deny.
5. Evidence MUST never be used as authorization by itself.

## ARCH-P0-005 — Canonical Interfaces / Events

1. Every cross-domain interface MUST have a stable identifier and semantic version.
2. Protocol-critical binary interfaces MUST define canonical encoding; JSON MUST NOT be the consensus wire format.
3. Event schemas MUST distinguish authoritative events from derived/read-model events.
4. Interface changes MUST be backward-compatibility classified: breaking, additive, or compatible.
5. Message/event IDs MUST be deterministic where replay or consensus correctness depends on them.
6. Producers MUST NOT emit an event claiming finality before the owning authority has finalized the state.
7. Consumers MUST reject unknown mandatory fields/versions according to the interface's compatibility policy.

## ARCH-P0-006 — Failure / Recovery

Failure classes:
- `TRANSPORT`: reconnect/retry without state-authority escalation.
- `AUTHENTICATION`: reject and audit.
- `AUTHORIZATION`: deny and audit.
- `VALIDATION`: reject mutation.
- `EXECUTION`: discard candidate transition; preserve last finalized state.
- `CONSENSUS`: halt finalization until protocol rules permit progress.
- `STORAGE`: enter recovery/read-only mode; restore from canonical committed state.
- `CONTROL_PLANE`: fail closed for protected operations; preserve already-finalized protocol state.

Recovery MUST:
1. be idempotent;
2. preserve monotonic finalized height/state;
3. never invent canonical state;
4. never bypass identity/capability/policy;
5. produce auditable evidence;
6. have a bounded retry/escalation policy.

## Status model

These contracts define architecture only. Each contract requires independent evidence dimensions:

`CONTRACT → IMPLEMENTATION → TEST → CI_EXACT_SHA → INTEGRATION → E2E`

No contract is considered implemented merely because this document or its machine-readable representation exists.
