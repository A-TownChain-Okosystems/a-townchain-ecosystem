# API Orchestrator — Subsystem Matrix

**Parent Matrix:** ATC-API-ORCHESTRATOR-001  
**Subsystem Matrix ID:** ATC-API-ORCHESTRATOR-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-ORC-01 | Core / Lifecycle | API Orchestrator Core / Lifecycle subsystem | API Orchestrator | ARCHITECTURE CONTRACT |
| SUB-ORC-02 | Contract / Schema | API Orchestrator Contract / Schema subsystem | API Orchestrator | ARCHITECTURE CONTRACT |
| SUB-ORC-03 | Source / Implementation | API Orchestrator Source / Implementation subsystem | API Orchestrator | ARCHITECTURE CONTRACT |
| SUB-ORC-04 | State / Data | API Orchestrator State / Data subsystem | API Orchestrator | ARCHITECTURE CONTRACT |
| SUB-ORC-05 | Event / Flow | API Orchestrator Event / Flow subsystem | API Orchestrator | ARCHITECTURE CONTRACT |
| SUB-ORC-06 | Integration / Interface | API Orchestrator Integration / Interface subsystem | API Orchestrator | ARCHITECTURE CONTRACT |
| SUB-ORC-07 | Security / Authority | API Orchestrator Security / Authority subsystem | API Orchestrator | ARCHITECTURE CONTRACT |
| SUB-ORC-08 | Testing / Evidence / Verification | API Orchestrator Testing / Evidence / Verification subsystem | API Orchestrator | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
