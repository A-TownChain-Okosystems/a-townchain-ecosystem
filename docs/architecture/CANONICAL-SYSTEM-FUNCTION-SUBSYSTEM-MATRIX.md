# Master System Function — Subsystem Matrix

**Parent Matrix:** ATC-CANONICAL-SYSTEM-FUNCTION-001  
**Subsystem Matrix ID:** ATC-CANONICAL-SYSTEM-FUNCTION-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-SYS-01 | Core / Lifecycle | Master System Function Core / Lifecycle subsystem | Master System Function | ARCHITECTURE CONTRACT |
| SUB-SYS-02 | Contract / Schema | Master System Function Contract / Schema subsystem | Master System Function | ARCHITECTURE CONTRACT |
| SUB-SYS-03 | Source / Implementation | Master System Function Source / Implementation subsystem | Master System Function | ARCHITECTURE CONTRACT |
| SUB-SYS-04 | State / Data | Master System Function State / Data subsystem | Master System Function | ARCHITECTURE CONTRACT |
| SUB-SYS-05 | Event / Flow | Master System Function Event / Flow subsystem | Master System Function | ARCHITECTURE CONTRACT |
| SUB-SYS-06 | Integration / Interface | Master System Function Integration / Interface subsystem | Master System Function | ARCHITECTURE CONTRACT |
| SUB-SYS-07 | Security / Authority | Master System Function Security / Authority subsystem | Master System Function | ARCHITECTURE CONTRACT |
| SUB-SYS-08 | Testing / Evidence / Verification | Master System Function Testing / Evidence / Verification subsystem | Master System Function | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
