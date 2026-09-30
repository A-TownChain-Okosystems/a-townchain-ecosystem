# AI Subsystem — Subsystem Matrix

**Parent Matrix:** ATC-AI-SUBSYSTEM-001  
**Subsystem Matrix ID:** ATC-AI-SUBSYSTEM-SUBSYSTEM-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT

## Purpose
Every canonical matrix is decomposed into explicit subsystems. A parent matrix entry is not a complete system definition until its owning subsystem, contract boundary, state/event behavior, security authority and evidence path are identifiable.

## Subsystem contract

| ID | Subsystem Domain | Subsystem | Parent Matrix | Status |
|---|---|---|---|---|
| SUB-AI-01 | Core / Lifecycle | AI Subsystem Core / Lifecycle subsystem | AI Subsystem | ARCHITECTURE CONTRACT |
| SUB-AI-02 | Contract / Schema | AI Subsystem Contract / Schema subsystem | AI Subsystem | ARCHITECTURE CONTRACT |
| SUB-AI-03 | Source / Implementation | AI Subsystem Source / Implementation subsystem | AI Subsystem | ARCHITECTURE CONTRACT |
| SUB-AI-04 | State / Data | AI Subsystem State / Data subsystem | AI Subsystem | ARCHITECTURE CONTRACT |
| SUB-AI-05 | Event / Flow | AI Subsystem Event / Flow subsystem | AI Subsystem | ARCHITECTURE CONTRACT |
| SUB-AI-06 | Integration / Interface | AI Subsystem Integration / Interface subsystem | AI Subsystem | ARCHITECTURE CONTRACT |
| SUB-AI-07 | Security / Authority | AI Subsystem Security / Authority subsystem | AI Subsystem | ARCHITECTURE CONTRACT |
| SUB-AI-08 | Testing / Evidence / Verification | AI Subsystem Testing / Evidence / Verification subsystem | AI Subsystem | ARCHITECTURE CONTRACT |

## Mandatory subsystem fields
Each subsystem MUST define: ownership, responsibility, inputs, outputs, state, events, interfaces, dependencies, security boundary, authority, capability, policy, approval, source, tests, workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification and Residual.

## Authority
Subsystem decomposition does not create runtime authority. The parent matrix and authoritative domain contract remain controlling. Documentation, UI, model, agent, event and orchestration outputs cannot independently authorize state mutation.

## Evidence
`Subsystem Contract → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Residual`
