# Security Boundary — Subsystem Matrix

**Parent Matrix:** ATC-SECURITY-BOUNDARY-001  
**Subsystem Matrix ID:** ATC-SECURITY-BOUNDARY-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-SEC-01 | Core / Lifecycle | Security Boundary Core / Lifecycle subsystem | Security Boundary | ARCHITECTURE CONTRACT |
| SUB-SEC-02 | Contract / Schema | Security Boundary Contract / Schema subsystem | Security Boundary | ARCHITECTURE CONTRACT |
| SUB-SEC-03 | Source / Implementation | Security Boundary Source / Implementation subsystem | Security Boundary | ARCHITECTURE CONTRACT |
| SUB-SEC-04 | State / Data | Security Boundary State / Data subsystem | Security Boundary | ARCHITECTURE CONTRACT |
| SUB-SEC-05 | Event / Flow | Security Boundary Event / Flow subsystem | Security Boundary | ARCHITECTURE CONTRACT |
| SUB-SEC-06 | Integration / Interface | Security Boundary Integration / Interface subsystem | Security Boundary | ARCHITECTURE CONTRACT |
| SUB-SEC-07 | Security / Authority | Security Boundary Security / Authority subsystem | Security Boundary | ARCHITECTURE CONTRACT |
| SUB-SEC-08 | Testing / Evidence / Verification | Security Boundary Testing / Evidence / Verification subsystem | Security Boundary | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
