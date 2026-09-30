# State & Data — Subsystem Matrix

**Parent Matrix:** ATC-STATE-DATA-001  
**Subsystem Matrix ID:** ATC-STATE-DATA-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-DAT-01 | Core / Lifecycle | State & Data Core / Lifecycle subsystem | State & Data | ARCHITECTURE CONTRACT |
| SUB-DAT-02 | Contract / Schema | State & Data Contract / Schema subsystem | State & Data | ARCHITECTURE CONTRACT |
| SUB-DAT-03 | Source / Implementation | State & Data Source / Implementation subsystem | State & Data | ARCHITECTURE CONTRACT |
| SUB-DAT-04 | State / Data | State & Data State / Data subsystem | State & Data | ARCHITECTURE CONTRACT |
| SUB-DAT-05 | Event / Flow | State & Data Event / Flow subsystem | State & Data | ARCHITECTURE CONTRACT |
| SUB-DAT-06 | Integration / Interface | State & Data Integration / Interface subsystem | State & Data | ARCHITECTURE CONTRACT |
| SUB-DAT-07 | Security / Authority | State & Data Security / Authority subsystem | State & Data | ARCHITECTURE CONTRACT |
| SUB-DAT-08 | Testing / Evidence / Verification | State & Data Testing / Evidence / Verification subsystem | State & Data | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
