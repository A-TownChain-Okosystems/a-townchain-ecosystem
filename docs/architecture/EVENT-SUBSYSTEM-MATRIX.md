# Event — Subsystem Matrix

**Parent Matrix:** ATC-EVENT-001  
**Subsystem Matrix ID:** ATC-EVENT-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-EVT-01 | Core / Lifecycle | Event Core / Lifecycle subsystem | Event | ARCHITECTURE CONTRACT |
| SUB-EVT-02 | Contract / Schema | Event Contract / Schema subsystem | Event | ARCHITECTURE CONTRACT |
| SUB-EVT-03 | Source / Implementation | Event Source / Implementation subsystem | Event | ARCHITECTURE CONTRACT |
| SUB-EVT-04 | State / Data | Event State / Data subsystem | Event | ARCHITECTURE CONTRACT |
| SUB-EVT-05 | Event / Flow | Event Event / Flow subsystem | Event | ARCHITECTURE CONTRACT |
| SUB-EVT-06 | Integration / Interface | Event Integration / Interface subsystem | Event | ARCHITECTURE CONTRACT |
| SUB-EVT-07 | Security / Authority | Event Security / Authority subsystem | Event | ARCHITECTURE CONTRACT |
| SUB-EVT-08 | Testing / Evidence / Verification | Event Testing / Evidence / Verification subsystem | Event | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
