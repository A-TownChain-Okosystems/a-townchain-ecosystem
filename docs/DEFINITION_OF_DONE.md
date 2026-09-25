# A-TownChain Ecosystem — Definition of Done

A component is COMPLIANT only when every criterion required for its declared lifecycle stage is satisfied.

## Functional
- Required behavior is implemented.
- Error paths are explicit.
- No production placeholder silently simulates required behavior.

## Security
- Trust and capability boundaries are explicit.
- Fail-closed behavior is tested.
- Security-sensitive changes have regression coverage.

## Determinism
For consensus-critical components, identical inputs produce identical outputs; serialization/version rules are explicit; execution/resource limits are bounded; state transitions are reproducible.

## Integration
Declared dependencies resolve; public interfaces match contracts; required end-to-end behavior is tested; restart/recovery is covered where stateful.

## Conformance
Applicable ATC standards are identified; conformance checks pass; no conflicting local rule silently overrides the normative SSOT.

## Evidence
Evidence identifies the verified repository, revision, test/CI execution and relevant artifact/result. Stale evidence does not establish current compliance.

## Operational readiness
Build is reproducible; required CI gates are green; release diagnostics are retained; known open blockers are explicitly accounted for.

## Final rule

IMPLEMENTED != VERIFIED
VERIFIED != RELEASE-READY
DOCUMENTED != IMPLEMENTED

COMPLIANT = implementation + tests + CI + evidence + required governance
