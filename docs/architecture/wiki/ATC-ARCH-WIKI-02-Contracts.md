# Chapter 02 — Sublayers, components and function contracts

**Status:** `PROPOSED / NOT VERIFIED`

## Architecture hierarchy

`L0 governance → L1 system layers / X cross-cutting layers → sublayers → components → function contracts → interfaces/dependencies → source mappings → evidence → lifecycle`

Cross-cutting relationships may give a node multiple architectural parents. Each production implementation must still have one canonical owner.

## Sublayer rules

- Give each sublayer a stable ID, parent, purpose, boundaries, owner and status.
- Record requirements, components, function contracts, interfaces, dependencies, security/resource constraints, tests, evidence and lifecycle.
- A generic sublayer family is a checklist, not an exhaustive inventory.
- Use `UNRESOLVED` for unknowns; use `EXCLUDED` only with a reason and required approval.
- Preserve established IDs; perform collision checks before issuing new IDs.

## Component record minimum

`component_id`, name/version/status/owner, parent layer and sublayer IDs, scope and exclusions, canonical repository/path, interfaces/contracts/dependencies, security boundary, resource profile, exact source commit and blob SHA where evidenced, test/evidence references and findings.

## Function-contract record minimum

Each contract must have a stable ID and parent links; owner and status; behavioral semantics; typed inputs/outputs and serialization; preconditions, postconditions and invariants; side effects/idempotency; errors, limits, timeout/retry/cancellation behavior; security/privacy constraints; interface/version compatibility; canonical source mapping; acceptance criteria; positive, negative, boundary and fault tests as applicable; evidence and reviewer decision.

## Mapping and exclusion semantics

- `MAPPED`: source relationship is evidenced; behavior is not thereby verified.
- `UNRESOLVED`: source, ownership or semantics remain unknown.
- `EXCLUDED`: rationale, scope and approval (where required) are recorded.
- Tests, docs, generated files and CI may be excluded from runtime-function counts only with explicit rationale; preserve their verification/evidence links separately.

## Completeness

Every discovered in-scope component/function/interface must be represented or explicitly dispositioned. Counts must be reproducible from machine-readable inventory artifacts and tied to exact source revisions.
