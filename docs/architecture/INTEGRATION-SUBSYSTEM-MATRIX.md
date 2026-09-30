# Integration — Subsystem Matrix

**Parent Matrix:** ATC-INTEGRATION-001  
**Subsystem Matrix ID:** ATC-INTEGRATION-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-INT-01 | Core / Lifecycle | Integration Core / Lifecycle subsystem | Integration | ARCHITECTURE CONTRACT |
| SUB-INT-02 | Contract / Schema | Integration Contract / Schema subsystem | Integration | ARCHITECTURE CONTRACT |
| SUB-INT-03 | Source / Implementation | Integration Source / Implementation subsystem | Integration | ARCHITECTURE CONTRACT |
| SUB-INT-04 | State / Data | Integration State / Data subsystem | Integration | ARCHITECTURE CONTRACT |
| SUB-INT-05 | Event / Flow | Integration Event / Flow subsystem | Integration | ARCHITECTURE CONTRACT |
| SUB-INT-06 | Integration / Interface | Integration Integration / Interface subsystem | Integration | ARCHITECTURE CONTRACT |
| SUB-INT-07 | Security / Authority | Integration Security / Authority subsystem | Integration | ARCHITECTURE CONTRACT |
| SUB-INT-08 | Testing / Evidence / Verification | Integration Testing / Evidence / Verification subsystem | Integration | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
