# Aurora / KI — Canonical Subsystem Matrix

**Document ID:** `ATC-AI-SUBSYSTEM-MATRIX-001`  
**Status:** ARCHITECTURE CONTRACT  
**Scope:** Aurora AI / KI-Plattform  
**Subsystems:** 267

## 1. Purpose

This document defines the canonical technical subsystem inventory for Aurora AI. It refines the existing AI domain matrix into implementable subsystem boundaries.

**Architecture presence does not imply implementation, testing, CI verification, integration, or E2E verification.**

## 2. Canonical Traceability

`Vision → AI Domain → Subsystem → Component → Function/API → Contract → Source → Test → Workflow → Exact-SHA Evidence → Verification → Residual`

Evidence separation is mandatory:

`Error Evidence ≠ Finding Evidence ≠ Verification Evidence`

`Source vorhanden ≠ Implementiert ≠ Verifiziert`

## 3. Subsystem Matrix

| ID | Domain | Subsystem |
|---|---|---|
| AI-SUB-001 | AI Core & Platform | AI Platform Core |
| AI-SUB-002 | AI Core & Platform | AI Configuration |
| AI-SUB-003 | AI Core & Platform | AI Lifecycle Manager |
| AI-SUB-004 | AI Core & Platform | AI Service Registry |
| AI-SUB-005 | AI Core & Platform | AI Component Registry |
| AI-SUB-006 | AI Core & Platform | AI Version Manager |
| AI-SUB-007 | AI Core & Platform | AI Environment Manager |
| AI-SUB-008 | AI Core & Platform | AI Dependency Manager |
| AI-SUB-009 | AI Core & Platform | AI Capability Registry |
| AI-SUB-010 | AI Core & Platform | AI Health Manager |
| AI-SUB-011 | Model Layer | Model Runtime |
| AI-SUB-012 | Model Layer | Model Loader |
| AI-SUB-013 | Model Layer | Model Registry |
| AI-SUB-014 | Model Layer | Model Metadata |
| AI-SUB-015 | Model Layer | Model Versioning |
| AI-SUB-016 | Model Layer | Model Routing |
| AI-SUB-017 | Model Layer | Model Selection |
| AI-SUB-018 | Model Layer | Model Fallback |
| AI-SUB-019 | Model Layer | Model Context |
| AI-SUB-020 | Model Layer | Model Adapter |
| AI-SUB-021 | Model Layer | Local Model Runtime |
| AI-SUB-022 | Model Layer | Remote Model Runtime |
| AI-SUB-023 | Model Layer | Multimodal Model Runtime |
| AI-SUB-024 | Model Layer | Model Isolation |
| AI-SUB-025 | ModelHub | ModelHub Core |
| AI-SUB-026 | ModelHub | Model Discovery |
| AI-SUB-027 | ModelHub | Model Registration |
| AI-SUB-028 | ModelHub | Model Validation |
| AI-SUB-029 | ModelHub | Model Provenance |
| AI-SUB-030 | ModelHub | Model Integrity |
| AI-SUB-031 | ModelHub | Model Compatibility |
| AI-SUB-032 | ModelHub | Model Policy |
| AI-SUB-033 | ModelHub | Model Deployment |
| AI-SUB-034 | ModelHub | Model Retirement |
| AI-SUB-035 | Inference & AI Runtime | Inference Engine |
| AI-SUB-036 | Inference & AI Runtime | Prompt Processor |
| AI-SUB-037 | Inference & AI Runtime | Context Builder |
| AI-SUB-038 | Inference & AI Runtime | Token Manager |
| AI-SUB-039 | Inference & AI Runtime | Batch Engine |
| AI-SUB-040 | Inference & AI Runtime | Streaming Engine |
| AI-SUB-041 | Inference & AI Runtime | Tool Context Engine |
| AI-SUB-042 | Inference & AI Runtime | Runtime Scheduler |
| AI-SUB-043 | Inference & AI Runtime | Resource Scheduler |
| AI-SUB-044 | Inference & AI Runtime | Runtime Isolation |
| AI-SUB-045 | Inference & AI Runtime | Runtime Limits |
| AI-SUB-046 | Inference & AI Runtime | Runtime Recovery |
| AI-SUB-047 | Agent System | Agent Core |
| AI-SUB-048 | Agent System | Agent Registry |
| AI-SUB-049 | Agent System | Agent Runtime |
| AI-SUB-050 | Agent System | Agent Planner |
| AI-SUB-051 | Agent System | Agent Executor |
| AI-SUB-052 | Agent System | Agent Loop |
| AI-SUB-053 | Agent System | Agent State |
| AI-SUB-054 | Agent System | Agent Context |
| AI-SUB-055 | Agent System | Agent Delegation |
| AI-SUB-056 | Agent System | Agent Coordination |
| AI-SUB-057 | Agent System | Agent Scheduling |
| AI-SUB-058 | Agent System | Agent Termination |
| AI-SUB-059 | Capability System | Capability Core |
| AI-SUB-060 | Capability System | Capability Registry |
| AI-SUB-061 | Capability System | Capability Resolver |
| AI-SUB-062 | Capability System | Capability Scope |
| AI-SUB-063 | Capability System | Capability Token |
| AI-SUB-064 | Capability System | Capability Delegation |
| AI-SUB-065 | Capability System | Capability Revocation |
| AI-SUB-066 | Capability System | Capability Expiration |
| AI-SUB-067 | Capability System | Capability Isolation |
| AI-SUB-068 | Capability System | Capability Audit |
| AI-SUB-069 | Policy & Authorization | Policy Engine |
| AI-SUB-070 | Policy & Authorization | Policy Registry |
| AI-SUB-071 | Policy & Authorization | Authorization Engine |
| AI-SUB-072 | Policy & Authorization | Scope Enforcement |
| AI-SUB-073 | Policy & Authorization | Approval Engine |
| AI-SUB-074 | Policy & Authorization | Risk Policy |
| AI-SUB-075 | Policy & Authorization | Resource Policy |
| AI-SUB-076 | Policy & Authorization | Data Policy |
| AI-SUB-077 | Policy & Authorization | Action Policy |
| AI-SUB-078 | Policy & Authorization | Policy Audit |
| AI-SUB-079 | Tool & Action System | Tool Registry |
| AI-SUB-080 | Tool & Action System | Tool Resolver |
| AI-SUB-081 | Tool & Action System | Tool Adapter |
| AI-SUB-082 | Tool & Action System | Tool Executor |
| AI-SUB-083 | Tool & Action System | Tool Sandbox |
| AI-SUB-084 | Tool & Action System | Tool Permission |
| AI-SUB-085 | Tool & Action System | Action Validator |
| AI-SUB-086 | Tool & Action System | Action Approval |
| AI-SUB-087 | Tool & Action System | Action Journal |
| AI-SUB-088 | Tool & Action System | Action Rollback |
| AI-SUB-089 | Memory | Memory Core |
| AI-SUB-090 | Memory | Short-Term Memory |
| AI-SUB-091 | Memory | Long-Term Memory |
| AI-SUB-092 | Memory | Episodic Memory |
| AI-SUB-093 | Memory | Semantic Memory |
| AI-SUB-094 | Memory | Working Memory |
| AI-SUB-095 | Memory | Memory Retrieval |
| AI-SUB-096 | Memory | Memory Ranking |
| AI-SUB-097 | Memory | Memory Update |
| AI-SUB-098 | Memory | Memory Deletion |
| AI-SUB-099 | Memory | Memory Permissions |
| AI-SUB-100 | Memory | Memory Provenance |
| AI-SUB-101 | RAG & Knowledge | RAG Core |
| AI-SUB-102 | RAG & Knowledge | Document Ingestion |
| AI-SUB-103 | RAG & Knowledge | Chunking |
| AI-SUB-104 | RAG & Knowledge | Embedding Engine |
| AI-SUB-105 | RAG & Knowledge | Vector Store |
| AI-SUB-106 | RAG & Knowledge | Retriever |
| AI-SUB-107 | RAG & Knowledge | Reranker |
| AI-SUB-108 | RAG & Knowledge | Knowledge Graph |
| AI-SUB-109 | RAG & Knowledge | Source Attribution |
| AI-SUB-110 | RAG & Knowledge | Knowledge Validation |
| AI-SUB-111 | RAG & Knowledge | Knowledge Versioning |
| AI-SUB-112 | RAG & Knowledge | Knowledge Provenance |
| AI-SUB-113 | Data Layer | AI Data Core |
| AI-SUB-114 | Data Layer | Data Ingestion |
| AI-SUB-115 | Data Layer | Data Validation |
| AI-SUB-116 | Data Layer | Data Normalization |
| AI-SUB-117 | Data Layer | Data Classification |
| AI-SUB-118 | Data Layer | Data Lineage |
| AI-SUB-119 | Data Layer | Data Versioning |
| AI-SUB-120 | Data Layer | Data Retention |
| AI-SUB-121 | Data Layer | Data Deletion |
| AI-SUB-122 | Data Layer | Data Access Control |
| AI-SUB-123 | Multimodal AI | Text AI |
| AI-SUB-124 | Multimodal AI | Vision AI |
| AI-SUB-125 | Multimodal AI | Audio AI |
| AI-SUB-126 | Multimodal AI | Speech-to-Text |
| AI-SUB-127 | Multimodal AI | Text-to-Speech |
| AI-SUB-128 | Multimodal AI | Video AI |
| AI-SUB-129 | Multimodal AI | Multimodal Fusion |
| AI-SUB-130 | Multimodal AI | Media Generation |
| AI-SUB-131 | Multimodal AI | Media Validation |
| AI-SUB-132 | AI ↔ Operating System | Aurora OS Gateway |
| AI-SUB-133 | AI ↔ Operating System | OS Capability Adapter |
| AI-SUB-134 | AI ↔ Operating System | Process Control Interface |
| AI-SUB-135 | AI ↔ Operating System | File System Interface |
| AI-SUB-136 | AI ↔ Operating System | Network Interface |
| AI-SUB-137 | AI ↔ Operating System | Device Interface |
| AI-SUB-138 | AI ↔ Operating System | Resource Interface |
| AI-SUB-139 | AI ↔ Operating System | System Management Interface |
| AI-SUB-140 | AI ↔ Operating System | OS Event Interface |
| AI-SUB-141 | AI ↔ Operating System | OS Audit Interface |
| AI-SUB-142 | AI ↔ A-TownChain | Blockchain Gateway |
| AI-SUB-143 | AI ↔ A-TownChain | Wallet Gateway |
| AI-SUB-144 | AI ↔ A-TownChain | Transaction Builder |
| AI-SUB-145 | AI ↔ A-TownChain | Transaction Simulation |
| AI-SUB-146 | AI ↔ A-TownChain | Signing Request |
| AI-SUB-147 | AI ↔ A-TownChain | Contract Interface |
| AI-SUB-148 | AI ↔ A-TownChain | Blockchain Query |
| AI-SUB-149 | AI ↔ A-TownChain | Event Listener |
| AI-SUB-150 | AI ↔ A-TownChain | Governance Interface |
| AI-SUB-151 | AI ↔ A-TownChain | Marketplace Interface |
| AI-SUB-152 | AI ↔ A-TownChain | NFT Interface |
| AI-SUB-153 | AI ↔ A-TownChain | Staking Interface |
| AI-SUB-154 | AI Security | AI Security Core |
| AI-SUB-155 | AI Security | Prompt Injection Defense |
| AI-SUB-156 | AI Security | Tool Injection Defense |
| AI-SUB-157 | AI Security | Context Isolation |
| AI-SUB-158 | AI Security | Model Isolation |
| AI-SUB-159 | AI Security | Agent Isolation |
| AI-SUB-160 | AI Security | Secret Protection |
| AI-SUB-161 | AI Security | Data Loss Prevention |
| AI-SUB-162 | AI Security | Output Validation |
| AI-SUB-163 | AI Security | Action Validation |
| AI-SUB-164 | AI Security | Sandbox |
| AI-SUB-165 | AI Security | Abuse Prevention |
| AI-SUB-166 | Privacy & Consent | Privacy Core |
| AI-SUB-167 | Privacy & Consent | Consent Manager |
| AI-SUB-168 | Privacy & Consent | Data Minimization |
| AI-SUB-169 | Privacy & Consent | PII Protection |
| AI-SUB-170 | Privacy & Consent | Memory Privacy |
| AI-SUB-171 | Privacy & Consent | Model Privacy |
| AI-SUB-172 | Privacy & Consent | Access Logging |
| AI-SUB-173 | Privacy & Consent | Data Retention Policy |
| AI-SUB-174 | Privacy & Consent | User Data Export |
| AI-SUB-175 | Privacy & Consent | User Data Deletion |
| AI-SUB-176 | AI Evaluation | Evaluation Core |
| AI-SUB-177 | AI Evaluation | Quality Evaluation |
| AI-SUB-178 | AI Evaluation | Safety Evaluation |
| AI-SUB-179 | AI Evaluation | Reliability Evaluation |
| AI-SUB-180 | AI Evaluation | Agent Evaluation |
| AI-SUB-181 | AI Evaluation | Tool Evaluation |
| AI-SUB-182 | AI Evaluation | RAG Evaluation |
| AI-SUB-183 | AI Evaluation | Regression Testing |
| AI-SUB-184 | AI Evaluation | Benchmarking |
| AI-SUB-185 | AI Evaluation | Human Evaluation |
| AI-SUB-186 | Training & Fine-Tuning | Training Manager |
| AI-SUB-187 | Training & Fine-Tuning | Dataset Manager |
| AI-SUB-188 | Training & Fine-Tuning | Dataset Validation |
| AI-SUB-189 | Training & Fine-Tuning | Fine-Tuning |
| AI-SUB-190 | Training & Fine-Tuning | Preference Training |
| AI-SUB-191 | Training & Fine-Tuning | RL/Feedback Pipeline |
| AI-SUB-192 | Training & Fine-Tuning | Training Evaluation |
| AI-SUB-193 | Training & Fine-Tuning | Model Export |
| AI-SUB-194 | Training & Fine-Tuning | Training Provenance |
| AI-SUB-195 | Training & Fine-Tuning | Training Security |
| AI-SUB-196 | Human Oversight | Approval Center |
| AI-SUB-197 | Human Oversight | Human-in-the-Loop |
| AI-SUB-198 | Human Oversight | Review Queue |
| AI-SUB-199 | Human Oversight | Action Confirmation |
| AI-SUB-200 | Human Oversight | Escalation Manager |
| AI-SUB-201 | Human Oversight | Override Manager |
| AI-SUB-202 | Human Oversight | Delegation Control |
| AI-SUB-203 | Human Oversight | Emergency Stop |
| AI-SUB-204 | Cost / Resource / Quota | Resource Manager |
| AI-SUB-205 | Cost / Resource / Quota | GPU Scheduler |
| AI-SUB-206 | Cost / Resource / Quota | NPU Scheduler |
| AI-SUB-207 | Cost / Resource / Quota | CPU Scheduler |
| AI-SUB-208 | Cost / Resource / Quota | Memory Manager |
| AI-SUB-209 | Cost / Resource / Quota | Token Budget |
| AI-SUB-210 | Cost / Resource / Quota | Compute Quota |
| AI-SUB-211 | Cost / Resource / Quota | Cost Manager |
| AI-SUB-212 | Cost / Resource / Quota | Priority Manager |
| AI-SUB-213 | Cost / Resource / Quota | Resource Accounting |
| AI-SUB-214 | Observability | AI Metrics |
| AI-SUB-215 | Observability | AI Logging |
| AI-SUB-216 | Observability | AI Tracing |
| AI-SUB-217 | Observability | Agent Trace |
| AI-SUB-218 | Observability | Tool Trace |
| AI-SUB-219 | Observability | Model Trace |
| AI-SUB-220 | Observability | Cost Telemetry |
| AI-SUB-221 | Observability | Error Telemetry |
| AI-SUB-222 | Observability | Audit Telemetry |
| AI-SUB-223 | AI Audit & Evidence | AI Audit Core |
| AI-SUB-224 | AI Audit & Evidence | Decision Evidence |
| AI-SUB-225 | AI Audit & Evidence | Action Evidence |
| AI-SUB-226 | AI Audit & Evidence | Tool Evidence |
| AI-SUB-227 | AI Audit & Evidence | Model Evidence |
| AI-SUB-228 | AI Audit & Evidence | Policy Evidence |
| AI-SUB-229 | AI Audit & Evidence | Approval Evidence |
| AI-SUB-230 | AI Audit & Evidence | Verification Evidence |
| AI-SUB-231 | AI Audit & Evidence | Evidence Integrity |
| AI-SUB-232 | AI Governance & Model Risk | AI Governance Core |
| AI-SUB-233 | AI Governance & Model Risk | Model Risk Manager |
| AI-SUB-234 | AI Governance & Model Risk | Model Approval |
| AI-SUB-235 | AI Governance & Model Risk | Model Policy |
| AI-SUB-236 | AI Governance & Model Risk | Agent Governance |
| AI-SUB-237 | AI Governance & Model Risk | Capability Governance |
| AI-SUB-238 | AI Governance & Model Risk | Tool Governance |
| AI-SUB-239 | AI Governance & Model Risk | AI Change Management |
| AI-SUB-240 | AI Governance & Model Risk | AI Incident Management |
| AI-SUB-241 | AI Supply Chain | Artifact Registry |
| AI-SUB-242 | AI Supply Chain | Model Provenance |
| AI-SUB-243 | AI Supply Chain | Dataset Provenance |
| AI-SUB-244 | AI Supply Chain | Dependency Scanner |
| AI-SUB-245 | AI Supply Chain | SBOM |
| AI-SUB-246 | AI Supply Chain | Artifact Signing |
| AI-SUB-247 | AI Supply Chain | Integrity Verification |
| AI-SUB-248 | AI Supply Chain | Reproducibility |
| AI-SUB-249 | AI Testing & Verification | Unit Testing |
| AI-SUB-250 | AI Testing & Verification | Integration Testing |
| AI-SUB-251 | AI Testing & Verification | Agent Testing |
| AI-SUB-252 | AI Testing & Verification | Tool Testing |
| AI-SUB-253 | AI Testing & Verification | RAG Testing |
| AI-SUB-254 | AI Testing & Verification | Security Testing |
| AI-SUB-255 | AI Testing & Verification | Adversarial Testing |
| AI-SUB-256 | AI Testing & Verification | Determinism Testing |
| AI-SUB-257 | AI Testing & Verification | Regression Testing |
| AI-SUB-258 | AI Testing & Verification | E2E AI Testing |
| AI-SUB-259 | AI Release & Verification | AI Build System |
| AI-SUB-260 | AI Release & Verification | AI Release Manager |
| AI-SUB-261 | AI Release & Verification | Model Release |
| AI-SUB-262 | AI Release & Verification | Agent Release |
| AI-SUB-263 | AI Release & Verification | Runtime Release |
| AI-SUB-264 | AI Release & Verification | Exact-SHA Verification |
| AI-SUB-265 | AI Release & Verification | Evidence Store |
| AI-SUB-266 | AI Release & Verification | Verification Gate |
| AI-SUB-267 | AI Release & Verification | Residual Registry |

## 4. Authority Architecture

```
User / Application
        ↓
Identity / Session
        ↓
Aurora Interface
        ↓
ModelHub / Memory / RAG
        ↓
AI Runtime
        ↓
Agent System
        ↓
Capability
        ↓
Policy
        ↓
Approval
        ↓
Tool / Action
        ↓
GlobusOS
        ↓
ShivaCore
        ↓
Hardware
```

The authoritative action chain is:

`Model → Agent → Capability → Policy → Approval → Tool → GlobusOS → ShivaCore → Hardware`

A model output, agent output, UI event, retrieved document, browser response, or external tool response is not by itself authorization.

## 5. Blockchain Boundary

`Aurora → Capability → Policy → Approval → Wallet → Explicit Signature → SDK/RPC → A-TownChain`

Aurora must not directly possess or silently use private signing keys.

## 6. Operating-System Boundary

`Aurora → OS Capability Gateway → GlobusOS Service → ShivaCore`

Aurora receives only explicitly exposed and policy-authorized capabilities. Direct kernel authority is forbidden.

## 7. Memory / Knowledge Boundary

Memory and RAG are data/knowledge subsystems, not authority subsystems.

`Source → Ingestion → Validation → Index/Memory → Retrieval → Context → Model`

Retrieved content cannot grant capabilities or permissions.

## 8. Security Invariants

1. Deny-by-default for external actions.
2. UI presence does not imply authorization.
3. Model output does not imply authorization.
4. Agent intent does not imply authorization.
5. Capability scope is explicit and bounded.
6. Approval is explicit where policy requires it.
7. Secrets/private keys never enter ordinary model context or evidence logs.
8. Tool execution is policy-gated and auditable.
9. OS/kernel authority remains outside the model boundary.
10. Blockchain signing remains outside the model boundary.
11. Error evidence, finding evidence, and verification evidence remain separate.
12. AI telemetry is evidence/diagnostic data, not authority.
13. Model/data/artifact provenance is explicit.
14. Resource, token, compute, and action quotas are enforceable.
15. Recovery and emergency-stop paths are independently controlled.

## 9. Verification Pipeline

```
Source
 ↓
Build
 ↓
Static / Type Checks
 ↓
Unit Tests
 ↓
Integration Tests
 ↓
Agent / Tool / RAG Tests
 ↓
Security / Adversarial Tests
 ↓
E2E Tests
 ↓
Exact-SHA CI
 ↓
Run → Job → Step → Exit Code → Log
 ↓
Verification Evidence
 ↓
Residual
```

## 10. Status Model

`UNANALYZED → ANALYZED → FIXED → RERUNNING → VERIFIED / RESIDUAL`

The subsystem matrix itself is an architecture contract. Runtime implementation status must be established independently for each subsystem.

## 11. Completeness Rule

Every AI capability must resolve to:

`Domain → Subsystem → Component → Function/API → Contract → Source → Test → Workflow → Exact-SHA Evidence → Verification → Residual`

A missing source, test, workflow, Exact-SHA result, or verification record remains a residual rather than being inferred from architecture coverage.
