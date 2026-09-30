# Dependency — Subsystem Matrix

**Parent Matrix:** ATC-DEPENDENCY-001  
**Subsystem Matrix ID:** ATC-DEPENDENCY-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-DEP-01 | Core / Lifecycle | Dependency Core / Lifecycle subsystem | Dependency | ARCHITECTURE CONTRACT |
| SUB-DEP-02 | Contract / Schema | Dependency Contract / Schema subsystem | Dependency | ARCHITECTURE CONTRACT |
| SUB-DEP-03 | Source / Implementation | Dependency Source / Implementation subsystem | Dependency | ARCHITECTURE CONTRACT |
| SUB-DEP-04 | State / Data | Dependency State / Data subsystem | Dependency | ARCHITECTURE CONTRACT |
| SUB-DEP-05 | Event / Flow | Dependency Event / Flow subsystem | Dependency | ARCHITECTURE CONTRACT |
| SUB-DEP-06 | Integration / Interface | Dependency Integration / Interface subsystem | Dependency | ARCHITECTURE CONTRACT |
| SUB-DEP-07 | Security / Authority | Dependency Security / Authority subsystem | Dependency | ARCHITECTURE CONTRACT |
| SUB-DEP-08 | Testing / Evidence / Verification | Dependency Testing / Evidence / Verification subsystem | Dependency | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
