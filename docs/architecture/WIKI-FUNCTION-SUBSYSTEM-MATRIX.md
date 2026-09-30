# Wiki Function — Subsystem Matrix

**Parent Matrix:** ATC-WIKI-FUNCTION-001  
**Subsystem Matrix ID:** ATC-WIKI-FUNCTION-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-WIKI-01 | Core / Lifecycle | Wiki Function Core / Lifecycle subsystem | Wiki Function | ARCHITECTURE CONTRACT |
| SUB-WIKI-02 | Contract / Schema | Wiki Function Contract / Schema subsystem | Wiki Function | ARCHITECTURE CONTRACT |
| SUB-WIKI-03 | Source / Implementation | Wiki Function Source / Implementation subsystem | Wiki Function | ARCHITECTURE CONTRACT |
| SUB-WIKI-04 | State / Data | Wiki Function State / Data subsystem | Wiki Function | ARCHITECTURE CONTRACT |
| SUB-WIKI-05 | Event / Flow | Wiki Function Event / Flow subsystem | Wiki Function | ARCHITECTURE CONTRACT |
| SUB-WIKI-06 | Integration / Interface | Wiki Function Integration / Interface subsystem | Wiki Function | ARCHITECTURE CONTRACT |
| SUB-WIKI-07 | Security / Authority | Wiki Function Security / Authority subsystem | Wiki Function | ARCHITECTURE CONTRACT |
| SUB-WIKI-08 | Testing / Evidence / Verification | Wiki Function Testing / Evidence / Verification subsystem | Wiki Function | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
