# API Orchestrator Matrix — A-TownChain Ecosystem

**Document ID:** `ATC-API-ORCHESTRATOR-MATRIX-001`  
**Status:** `ARCHITECTURE CONTRACT`  
**Orchestrator subsystem count:** 183

## Purpose
The API Orchestrator is the coordination layer between API Gateway/API contracts and domain subsystems. It plans and executes workflows, composes APIs, handles resilience and collects evidence without acquiring domain authority.

## Orchestrator matrix
| ID | Domain | Subsystem | Responsibility |
|---|---|---|---|
| ORC-001 | Orchestrator Core | Orchestrator Core | Coordination/workflow responsibility |
| ORC-002 | Orchestrator Core | Workflow Engine | Coordination/workflow responsibility |
| ORC-003 | Orchestrator Core | Workflow Registry | Coordination/workflow responsibility |
| ORC-004 | Orchestrator Core | Workflow Versioning | Coordination/workflow responsibility |
| ORC-005 | Orchestrator Core | Execution Engine | Coordination/workflow responsibility |
| ORC-006 | Orchestrator Core | Dependency Resolver | Coordination/workflow responsibility |
| ORC-007 | Orchestrator Core | Service Registry | Coordination/workflow responsibility |
| ORC-008 | Orchestrator Core | Endpoint Registry | Coordination/workflow responsibility |
| ORC-009 | Orchestrator Core | Route Resolver | Coordination/workflow responsibility |
| ORC-010 | Orchestrator Core | API Composer | Coordination/workflow responsibility |
| ORC-011 | Orchestrator Core | API Aggregator | Coordination/workflow responsibility |
| ORC-012 | Orchestrator Core | Request Coordinator | Coordination/workflow responsibility |
| ORC-013 | Workflow & Process Orchestration | Workflow Definition | Coordination/workflow responsibility |
| ORC-014 | Workflow & Process Orchestration | Workflow Parser | Coordination/workflow responsibility |
| ORC-015 | Workflow & Process Orchestration | Workflow Validator | Coordination/workflow responsibility |
| ORC-016 | Workflow & Process Orchestration | Workflow Planner | Coordination/workflow responsibility |
| ORC-017 | Workflow & Process Orchestration | Step Scheduler | Coordination/workflow responsibility |
| ORC-018 | Workflow & Process Orchestration | Parallel Executor | Coordination/workflow responsibility |
| ORC-019 | Workflow & Process Orchestration | Sequential Executor | Coordination/workflow responsibility |
| ORC-020 | Workflow & Process Orchestration | Conditional Executor | Coordination/workflow responsibility |
| ORC-021 | Workflow & Process Orchestration | Loop Executor | Coordination/workflow responsibility |
| ORC-022 | Workflow & Process Orchestration | Branch Executor | Coordination/workflow responsibility |
| ORC-023 | Workflow & Process Orchestration | Join Coordinator | Coordination/workflow responsibility |
| ORC-024 | Workflow & Process Orchestration | Dependency Graph | Coordination/workflow responsibility |
| ORC-025 | Workflow & Process Orchestration | Workflow State | Coordination/workflow responsibility |
| ORC-026 | Workflow & Process Orchestration | Workflow Resume | Coordination/workflow responsibility |
| ORC-027 | Workflow & Process Orchestration | Workflow Cancellation | Coordination/workflow responsibility |
| ORC-028 | Workflow & Process Orchestration | Workflow Timeout | Coordination/workflow responsibility |
| ORC-029 | API Composition | API Composition Engine | Coordination/workflow responsibility |
| ORC-030 | API Composition | Request Fan-Out | Coordination/workflow responsibility |
| ORC-031 | API Composition | Response Fan-In | Coordination/workflow responsibility |
| ORC-032 | API Composition | Data Mapper | Coordination/workflow responsibility |
| ORC-033 | API Composition | Schema Transformer | Coordination/workflow responsibility |
| ORC-034 | API Composition | Protocol Adapter | Coordination/workflow responsibility |
| ORC-035 | API Composition | Version Adapter | Coordination/workflow responsibility |
| ORC-036 | API Composition | Compatibility Adapter | Coordination/workflow responsibility |
| ORC-037 | API Composition | Response Normalizer | Coordination/workflow responsibility |
| ORC-038 | API Composition | Error Aggregator | Coordination/workflow responsibility |
| ORC-039 | Service Orchestration | Service Discovery | Coordination/workflow responsibility |
| ORC-040 | Service Orchestration | Service Selection | Coordination/workflow responsibility |
| ORC-041 | Service Orchestration | Service Health | Coordination/workflow responsibility |
| ORC-042 | Service Orchestration | Service Failover | Coordination/workflow responsibility |
| ORC-043 | Service Orchestration | Load Distribution | Coordination/workflow responsibility |
| ORC-044 | Service Orchestration | Service Dependency | Coordination/workflow responsibility |
| ORC-045 | Service Orchestration | Service Lifecycle | Coordination/workflow responsibility |
| ORC-046 | Service Orchestration | Service Activation | Coordination/workflow responsibility |
| ORC-047 | Service Orchestration | Service Deactivation | Coordination/workflow responsibility |
| ORC-048 | Service Orchestration | Service Recovery | Coordination/workflow responsibility |
| ORC-049 | Resilience Orchestration | Retry Manager | Coordination/workflow responsibility |
| ORC-050 | Resilience Orchestration | Backoff Manager | Coordination/workflow responsibility |
| ORC-051 | Resilience Orchestration | Circuit Breaker | Coordination/workflow responsibility |
| ORC-052 | Resilience Orchestration | Timeout Manager | Coordination/workflow responsibility |
| ORC-053 | Resilience Orchestration | Bulkhead | Coordination/workflow responsibility |
| ORC-054 | Resilience Orchestration | Failover Manager | Coordination/workflow responsibility |
| ORC-055 | Resilience Orchestration | Recovery Manager | Coordination/workflow responsibility |
| ORC-056 | Resilience Orchestration | Compensation Engine | Coordination/workflow responsibility |
| ORC-057 | Resilience Orchestration | Saga Coordinator | Coordination/workflow responsibility |
| ORC-058 | Resilience Orchestration | Dead Letter Handler | Coordination/workflow responsibility |
| ORC-059 | Transaction Orchestration | Transaction Coordinator | Coordination/workflow responsibility |
| ORC-060 | Transaction Orchestration | Transaction Planner | Coordination/workflow responsibility |
| ORC-061 | Transaction Orchestration | Transaction Validator | Coordination/workflow responsibility |
| ORC-062 | Transaction Orchestration | Transaction Simulation | Coordination/workflow responsibility |
| ORC-063 | Transaction Orchestration | Transaction Ordering | Coordination/workflow responsibility |
| ORC-064 | Transaction Orchestration | Transaction Dependency | Coordination/workflow responsibility |
| ORC-065 | Transaction Orchestration | Transaction Status | Coordination/workflow responsibility |
| ORC-066 | Transaction Orchestration | Confirmation Coordinator | Coordination/workflow responsibility |
| ORC-067 | Transaction Orchestration | Finality Coordinator | Coordination/workflow responsibility |
| ORC-068 | Transaction Orchestration | Transaction Recovery | Coordination/workflow responsibility |
| ORC-069 | Aurora Orchestration | AI Workflow Orchestrator | Coordination/workflow responsibility |
| ORC-070 | Aurora Orchestration | Agent Orchestrator | Coordination/workflow responsibility |
| ORC-071 | Aurora Orchestration | Model Orchestrator | Coordination/workflow responsibility |
| ORC-072 | Aurora Orchestration | Model Routing | Coordination/workflow responsibility |
| ORC-073 | Aurora Orchestration | Tool Orchestrator | Coordination/workflow responsibility |
| ORC-074 | Aurora Orchestration | Capability Orchestrator | Coordination/workflow responsibility |
| ORC-075 | Aurora Orchestration | Policy Orchestrator | Coordination/workflow responsibility |
| ORC-076 | Aurora Orchestration | Approval Orchestrator | Coordination/workflow responsibility |
| ORC-077 | Aurora Orchestration | Memory Orchestrator | Coordination/workflow responsibility |
| ORC-078 | Aurora Orchestration | RAG Orchestrator | Coordination/workflow responsibility |
| ORC-079 | Aurora Orchestration | Context Orchestrator | Coordination/workflow responsibility |
| ORC-080 | Aurora Orchestration | Agent Handoff | Coordination/workflow responsibility |
| ORC-081 | Aurora Orchestration | Human Handoff | Coordination/workflow responsibility |
| ORC-082 | Aurora Orchestration | AI Recovery | Coordination/workflow responsibility |
| ORC-083 | OS Orchestration | OS Service Orchestrator | Coordination/workflow responsibility |
| ORC-084 | OS Orchestration | Process Orchestrator | Coordination/workflow responsibility |
| ORC-085 | OS Orchestration | Device Orchestrator | Coordination/workflow responsibility |
| ORC-086 | OS Orchestration | Driver Orchestrator | Coordination/workflow responsibility |
| ORC-087 | OS Orchestration | Storage Orchestrator | Coordination/workflow responsibility |
| ORC-088 | OS Orchestration | Network Orchestrator | Coordination/workflow responsibility |
| ORC-089 | OS Orchestration | Update Orchestrator | Coordination/workflow responsibility |
| ORC-090 | OS Orchestration | Recovery Orchestrator | Coordination/workflow responsibility |
| ORC-091 | OS Orchestration | Backup Orchestrator | Coordination/workflow responsibility |
| ORC-092 | OS Orchestration | Security Orchestrator | Coordination/workflow responsibility |
| ORC-093 | OS Orchestration | Power Orchestrator | Coordination/workflow responsibility |
| ORC-094 | OS Orchestration | Resource Orchestrator | Coordination/workflow responsibility |
| ORC-095 | Cross-System Orchestration | Identity | Coordination/workflow responsibility |
| ORC-096 | Cross-System Orchestration | Wallet | Coordination/workflow responsibility |
| ORC-097 | Cross-System Orchestration | Blockchain | Coordination/workflow responsibility |
| ORC-098 | Cross-System Orchestration | Staking | Coordination/workflow responsibility |
| ORC-099 | Cross-System Orchestration | Mining | Coordination/workflow responsibility |
| ORC-100 | Cross-System Orchestration | NFT | Coordination/workflow responsibility |
| ORC-101 | Cross-System Orchestration | Marketplace | Coordination/workflow responsibility |
| ORC-102 | Cross-System Orchestration | Genesis | Coordination/workflow responsibility |
| ORC-103 | Cross-System Orchestration | GateToHell | Coordination/workflow responsibility |
| ORC-104 | Cross-System Orchestration | OS | Coordination/workflow responsibility |
| ORC-105 | Cross-System Orchestration | Aurora | Coordination/workflow responsibility |
| ORC-106 | Cross-System Orchestration | Storage | Coordination/workflow responsibility |
| ORC-107 | Cross-System Orchestration | Identity Federation | Coordination/workflow responsibility |
| ORC-108 | Cross-System Orchestration | Event Bus | Coordination/workflow responsibility |
| ORC-109 | Event Orchestration | Event Router | Coordination/workflow responsibility |
| ORC-110 | Event Orchestration | Event Filter | Coordination/workflow responsibility |
| ORC-111 | Event Orchestration | Event Correlator | Coordination/workflow responsibility |
| ORC-112 | Event Orchestration | Event Sequencer | Coordination/workflow responsibility |
| ORC-113 | Event Orchestration | Event Aggregator | Coordination/workflow responsibility |
| ORC-114 | Event Orchestration | Event Replay | Coordination/workflow responsibility |
| ORC-115 | Event Orchestration | Event Retry | Coordination/workflow responsibility |
| ORC-116 | Event Orchestration | Event Dead Letter | Coordination/workflow responsibility |
| ORC-117 | Event Orchestration | Event Audit | Coordination/workflow responsibility |
| ORC-118 | Event Orchestration | Event Recovery | Coordination/workflow responsibility |
| ORC-119 | Security Orchestration | Security Policy Coordinator | Coordination/workflow responsibility |
| ORC-120 | Security Orchestration | Capability Coordinator | Coordination/workflow responsibility |
| ORC-121 | Security Orchestration | Authorization Coordinator | Coordination/workflow responsibility |
| ORC-122 | Security Orchestration | Approval Coordinator | Coordination/workflow responsibility |
| ORC-123 | Security Orchestration | Secret Boundary Manager | Coordination/workflow responsibility |
| ORC-124 | Security Orchestration | Trust Coordinator | Coordination/workflow responsibility |
| ORC-125 | Security Orchestration | Integrity Coordinator | Coordination/workflow responsibility |
| ORC-126 | Security Orchestration | Provenance Coordinator | Coordination/workflow responsibility |
| ORC-127 | Security Orchestration | Security Event Coordinator | Coordination/workflow responsibility |
| ORC-128 | Security Orchestration | Incident Orchestrator | Coordination/workflow responsibility |
| ORC-129 | Data Orchestration | Data Pipeline | Coordination/workflow responsibility |
| ORC-130 | Data Orchestration | Data Ingestion | Coordination/workflow responsibility |
| ORC-131 | Data Orchestration | Data Validation | Coordination/workflow responsibility |
| ORC-132 | Data Orchestration | Data Transformation | Coordination/workflow responsibility |
| ORC-133 | Data Orchestration | Data Enrichment | Coordination/workflow responsibility |
| ORC-134 | Data Orchestration | Data Routing | Coordination/workflow responsibility |
| ORC-135 | Data Orchestration | Data Synchronization | Coordination/workflow responsibility |
| ORC-136 | Data Orchestration | Data Replication | Coordination/workflow responsibility |
| ORC-137 | Data Orchestration | Data Migration | Coordination/workflow responsibility |
| ORC-138 | Data Orchestration | Data Recovery | Coordination/workflow responsibility |
| ORC-139 | Resource Orchestration | Resource Scheduler | Coordination/workflow responsibility |
| ORC-140 | Resource Orchestration | CPU Orchestrator | Coordination/workflow responsibility |
| ORC-141 | Resource Orchestration | GPU Orchestrator | Coordination/workflow responsibility |
| ORC-142 | Resource Orchestration | NPU Orchestrator | Coordination/workflow responsibility |
| ORC-143 | Resource Orchestration | Memory Orchestrator | Coordination/workflow responsibility |
| ORC-144 | Resource Orchestration | Storage Orchestrator | Coordination/workflow responsibility |
| ORC-145 | Resource Orchestration | Network Resource Orchestrator | Coordination/workflow responsibility |
| ORC-146 | Resource Orchestration | Compute Quota | Coordination/workflow responsibility |
| ORC-147 | Resource Orchestration | Cost Orchestrator | Coordination/workflow responsibility |
| ORC-148 | Resource Orchestration | Priority Manager | Coordination/workflow responsibility |
| ORC-149 | Observability Orchestration | Telemetry Coordinator | Coordination/workflow responsibility |
| ORC-150 | Observability Orchestration | Metrics Aggregator | Coordination/workflow responsibility |
| ORC-151 | Observability Orchestration | Log Aggregator | Coordination/workflow responsibility |
| ORC-152 | Observability Orchestration | Trace Coordinator | Coordination/workflow responsibility |
| ORC-153 | Observability Orchestration | Health Coordinator | Coordination/workflow responsibility |
| ORC-154 | Observability Orchestration | Diagnostic Coordinator | Coordination/workflow responsibility |
| ORC-155 | Observability Orchestration | Alert Coordinator | Coordination/workflow responsibility |
| ORC-156 | Observability Orchestration | Evidence Coordinator | Coordination/workflow responsibility |
| ORC-157 | Observability Orchestration | Audit Coordinator | Coordination/workflow responsibility |
| ORC-158 | Observability Orchestration | Incident Correlator | Coordination/workflow responsibility |
| ORC-159 | Orchestrator Governance | Contract Registry | Coordination/workflow responsibility |
| ORC-160 | Orchestrator Governance | Schema Registry | Coordination/workflow responsibility |
| ORC-161 | Orchestrator Governance | Workflow Governance | Coordination/workflow responsibility |
| ORC-162 | Orchestrator Governance | Policy Governance | Coordination/workflow responsibility |
| ORC-163 | Orchestrator Governance | Version Governance | Coordination/workflow responsibility |
| ORC-164 | Orchestrator Governance | Change Management | Coordination/workflow responsibility |
| ORC-165 | Orchestrator Governance | Dependency Governance | Coordination/workflow responsibility |
| ORC-166 | Orchestrator Governance | Provenance Registry | Coordination/workflow responsibility |
| ORC-167 | Orchestrator Governance | Evidence Registry | Coordination/workflow responsibility |
| ORC-168 | Orchestrator Governance | Residual Registry | Coordination/workflow responsibility |
| ORC-169 | Orchestrator Testing | Workflow Unit Tests | Coordination/workflow responsibility |
| ORC-170 | Orchestrator Testing | API Composition Tests | Coordination/workflow responsibility |
| ORC-171 | Orchestrator Testing | Service Integration Tests | Coordination/workflow responsibility |
| ORC-172 | Orchestrator Testing | Failure Tests | Coordination/workflow responsibility |
| ORC-173 | Orchestrator Testing | Retry Tests | Coordination/workflow responsibility |
| ORC-174 | Orchestrator Testing | Recovery Tests | Coordination/workflow responsibility |
| ORC-175 | Orchestrator Testing | Saga Tests | Coordination/workflow responsibility |
| ORC-176 | Orchestrator Testing | Security Tests | Coordination/workflow responsibility |
| ORC-177 | Orchestrator Testing | Authorization Tests | Coordination/workflow responsibility |
| ORC-178 | Orchestrator Testing | Capability Tests | Coordination/workflow responsibility |
| ORC-179 | Orchestrator Testing | Approval Tests | Coordination/workflow responsibility |
| ORC-180 | Orchestrator Testing | Determinism Tests | Coordination/workflow responsibility |
| ORC-181 | Orchestrator Testing | Load Tests | Coordination/workflow responsibility |
| ORC-182 | Orchestrator Testing | E2E Tests | Coordination/workflow responsibility |
| ORC-183 | Orchestrator Testing | Chaos Tests | Coordination/workflow responsibility |

## Canonical orchestration contract
`API Request → Authentication → Authorization → Capability Resolution → Policy Evaluation → Approval → Workflow Planning → Dependency Resolution → Service Selection → API Execution → State/Event Processing → Validation → Commit/Compensation → Audit/Evidence → Response`

## Blockchain boundary
`Aurora/Application → API Orchestrator → Capability → Policy → Approval → Wallet → Explicit Signature → SDK/RPC → A-TownChain`

The orchestrator never owns wallet private keys and never authorizes a signature.

## Aurora boundary
`Model → Agent → Capability → Policy → Approval → Tool → API Orchestrator → GlobusOS/A-TownChain`

## OS update workflow
`Discovery → Metadata Validation → Signature Verification → Provenance/SBOM → Compatibility → Recovery Point → Staged Installation → Reboot → Health Check → Commit/Recovery/Rollback`

## Allowed
- API coordination
- workflow planning
- service invocation
- data transformation
- retries
- timeout/failover/recovery
- compensation
- event correlation
- evidence collection

## Forbidden
- bypassing policy
- inventing capabilities
- simulating approval
- storing wallet private keys
- replacing canonical blockchain state
- bypassing ShivaCore
- treating UI/AI output as authorization
- deriving verification evidence from execution alone

## Evidence contract
Each orchestrated workflow records:
`Workflow ID, Execution ID, Version, Principal, Capability, Policy Decision, Approval, Input, Steps, Dependencies, Service Calls, State Transitions, Errors, Retries, Compensations, Output, Audit Events, Source, Test, CI, Exact SHA, Run, Job, Step, Exit Code, Log, Verification, Residual`

## Verification
`Source → Contract → Unit → Integration → Failure/Recovery → Security → Determinism → E2E → Exact-SHA CI → Evidence → Verification → Residual`