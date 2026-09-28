# Audit Status Semantics

| Status | Meaning | Minimum evidence |
|---|---|---|
| ARCHITECTURE_ONLY | capability exists only in architecture | architecture reference |
| SPECIFIED | contract/spec exists | normative contract |
| IMPLEMENTED | implementation exists | concrete source |
| TESTED | tests exist and are attributable | test source |
| EVIDENCE_STALE | evidence exists but is bound to another source SHA | evidence + SHA mismatch |
| CI_VERIFIED | exact-SHA CI PASS exists | workflow/run + source SHA |
| INTEGRATED | cross-component edge is proven | dependency/call-path evidence |
| E2E_VERIFIED | complete system path passes | exact-SHA E2E evidence |
| MISSING | required capability has no implementation/spec where required | audit finding |
| BLOCKED | implementation is prevented by a concrete dependency/gate | root cause |
| DUPLICATE | competing implementation owns same capability | duplicate evidence |
| DISCONNECTED | implementation exists but no required integration edge | edge audit |
| CONTRACT_DRIFT | implementations disagree with normative contract | conformance evidence |

## Promotion rules

```text
ARCHITECTURE_ONLY
 -> SPECIFIED
 -> IMPLEMENTED
 -> TESTED
 -> CI_VERIFIED
 -> INTEGRATED
 -> E2E_VERIFIED
```

A capability may remain at a lower state when a higher gate is not proven.

`EVIDENCE_STALE` is not a promotion state. It is a blocking evidence condition that MUST be resolved before `CI_VERIFIED`.

