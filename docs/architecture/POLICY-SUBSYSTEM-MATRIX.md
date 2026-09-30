# Policy — Subsystem Matrix

**Parent Matrix:** ATC-POLICY-001  
**Subsystem Matrix ID:** ATC-POLICY-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-POL-01 | Core / Lifecycle | Policy Core / Lifecycle subsystem | Policy | ARCHITECTURE CONTRACT |
| SUB-POL-02 | Contract / Schema | Policy Contract / Schema subsystem | Policy | ARCHITECTURE CONTRACT |
| SUB-POL-03 | Source / Implementation | Policy Source / Implementation subsystem | Policy | ARCHITECTURE CONTRACT |
| SUB-POL-04 | State / Data | Policy State / Data subsystem | Policy | ARCHITECTURE CONTRACT |
| SUB-POL-05 | Event / Flow | Policy Event / Flow subsystem | Policy | ARCHITECTURE CONTRACT |
| SUB-POL-06 | Integration / Interface | Policy Integration / Interface subsystem | Policy | ARCHITECTURE CONTRACT |
| SUB-POL-07 | Security / Authority | Policy Security / Authority subsystem | Policy | ARCHITECTURE CONTRACT |
| SUB-POL-08 | Testing / Evidence / Verification | Policy Testing / Evidence / Verification subsystem | Policy | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
