# Policy Matrix

**Document ID:** `ATC-POLICY-MATRIX-001`  
**Status:** `ARCHITECTURE CONTRACT`  
**Policy subsystem count:** 218

## Purpose

This matrix defines the canonical policy layer across Identity, Capability, Approval, Aurora, API, A-TownChain, Wallet, GlobusOS, ShivaCore, Data, Privacy, Security, Marketplace, Genesis, GateToHell, Governance and Evidence.

Policy is a decision layer. It does not replace identity, capability, approval, orchestration, execution or authoritative domain state.

## Canonical authority chain

```
Principal / Application / AI
        ↓
Identity / Authentication
        ↓
Capability
        ↓
Policy Evaluation
        ↓
Risk Evaluation
        ↓
DENY / ALLOW / ALLOW_WITH_CONDITIONS / REQUIRE_APPROVAL / DEFER
        ↓
Approval where required
        ↓
API Orchestrator
        ↓
Domain Service
        ↓
Authoritative State
        ↓
Audit / Evidence
```

## Canonical policy contract

Every policy contract must define:

`policy_id, domain, scope, version, owner, principal, resource, action, conditions, constraints, risk_level, required_capability, approval_requirement, decision, exceptions, expiration, dependencies, source, tests, workflow, exact_sha, evidence, verification, residual, status`

## Policy subsystem matrix

| ID | Domain | Policy subsystem | Canonical responsibility |
|---|---|---|---|
| POL-001 | Policy Core | Policy Engine | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-002 | Policy Core | Policy Registry | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-003 | Policy Core | Policy Definition | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-004 | Policy Core | Policy Parser | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-005 | Policy Core | Policy Validator | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-006 | Policy Core | Policy Compiler | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-007 | Policy Core | Policy Runtime | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-008 | Policy Core | Policy Versioning | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-009 | Policy Core | Policy Lifecycle | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-010 | Policy Core | Policy Dependency | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-011 | Policy Core | Policy Priority | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-012 | Policy Core | Policy Conflict Resolver | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-013 | Identity & Access | Identity Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-014 | Identity & Access | Authentication Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-015 | Identity & Access | Session Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-016 | Identity & Access | Authorization Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-017 | Identity & Access | Role Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-018 | Identity & Access | Permission Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-019 | Identity & Access | Capability Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-020 | Identity & Access | Delegation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-021 | Identity & Access | Impersonation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-022 | Identity & Access | Account Recovery Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-023 | Identity & Access | Device Trust Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-024 | Identity & Access | Federation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-025 | Capability | Capability Scope | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-026 | Capability | Capability Lifetime | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-027 | Capability | Capability Issuance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-028 | Capability | Capability Delegation | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-029 | Capability | Capability Revocation | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-030 | Capability | Capability Renewal | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-031 | Capability | Capability Isolation | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-032 | Capability | Capability Resource Limit | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-033 | Capability | Capability Action Limit | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-034 | Capability | Capability Audit | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-035 | Approval | Approval Requirement | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-036 | Approval | Human Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-037 | Approval | Multi-Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-038 | Approval | Risk-Based Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-039 | Approval | Transaction Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-040 | Approval | Financial Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-041 | Approval | System Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-042 | Approval | AI Action Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-043 | Approval | Administrative Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-044 | Approval | Emergency Approval | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-045 | Approval | Approval Expiration | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-046 | Approval | Approval Revocation | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-047 | AI / Aurora | AI Usage Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-048 | AI / Aurora | Model Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-049 | AI / Aurora | Agent Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-050 | AI / Aurora | Tool Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-051 | AI / Aurora | Action Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-052 | AI / Aurora | Prompt Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-053 | AI / Aurora | Context Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-054 | AI / Aurora | Memory Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-055 | AI / Aurora | RAG Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-056 | AI / Aurora | Data Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-057 | AI / Aurora | AI Resource Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-058 | AI / Aurora | AI Cost Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-059 | AI / Aurora | AI Safety Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-060 | AI / Aurora | AI Privacy Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-061 | AI / Aurora | AI Governance Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-062 | API | API Access Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-063 | API | Endpoint Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-064 | API | Method Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-065 | API | Rate Limit Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-066 | API | Quota Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-067 | API | Request Size Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-068 | API | Timeout Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-069 | API | Retry Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-070 | API | Idempotency Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-071 | API | Version Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-072 | API | Deprecation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-073 | Blockchain | Transaction Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-074 | Blockchain | Signing Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-075 | Blockchain | Nonce Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-076 | Blockchain | Fee Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-077 | Blockchain | Gas Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-078 | Blockchain | Contract Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-079 | Blockchain | VM Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-080 | Blockchain | Staking Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-081 | Blockchain | Validator Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-082 | Blockchain | Mining Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-083 | Blockchain | Governance Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-084 | Blockchain | Treasury Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-085 | Blockchain | Asset Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-086 | Blockchain | NFT Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-087 | Blockchain | Bridge Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-088 | Wallet & Key | Wallet Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-089 | Wallet & Key | Key Generation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-090 | Wallet & Key | Key Storage Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-091 | Wallet & Key | Key Export Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-092 | Wallet & Key | Key Import Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-093 | Wallet & Key | Signing Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-094 | Wallet & Key | Hardware Signer Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-095 | Wallet & Key | Multisig Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-096 | Wallet & Key | Recovery Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-097 | Wallet & Key | Key Rotation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-098 | Wallet & Key | Key Revocation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-099 | Operating System | Process Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-100 | Operating System | Resource Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-101 | Operating System | Memory Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-102 | Operating System | File Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-103 | Operating System | Storage Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-104 | Operating System | Network Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-105 | Operating System | Device Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-106 | Operating System | Driver Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-107 | Operating System | Update Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-108 | Operating System | Firmware Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-109 | Operating System | Recovery Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-110 | Operating System | Power Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-111 | Operating System | Security Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-112 | Operating System | Privacy Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-113 | Operating System | Application Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-114 | ShivaCore / TCB | TCB Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-115 | ShivaCore / TCB | Kernel Access Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-116 | ShivaCore / TCB | Capability Enforcement Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-117 | ShivaCore / TCB | Isolation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-118 | ShivaCore / TCB | Privilege Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-119 | ShivaCore / TCB | Secure IPC Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-120 | ShivaCore / TCB | Hardware Access Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-121 | ShivaCore / TCB | Memory Protection Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-122 | ShivaCore / TCB | Secure Boot Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-123 | ShivaCore / TCB | Trusted Context Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-124 | ShivaCore / TCB | Kernel Audit Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-125 | Data | Data Access Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-126 | Data | Data Classification | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-127 | Data | Data Minimization | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-128 | Data | Data Retention | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-129 | Data | Data Deletion | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-130 | Data | Data Export | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-131 | Data | Data Residency | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-132 | Data | Data Lineage | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-133 | Data | Data Provenance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-134 | Data | Data Integrity | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-135 | Data | Schema Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-136 | Data | Data Migration Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-137 | Privacy | Privacy Core | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-138 | Privacy | Consent Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-139 | Privacy | PII Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-140 | Privacy | Memory Privacy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-141 | Privacy | AI Context Privacy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-142 | Privacy | Logging Privacy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-143 | Privacy | Telemetry Privacy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-144 | Privacy | Analytics Privacy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-145 | Privacy | User Data Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-146 | Privacy | Deletion Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-147 | Security | Security Baseline | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-148 | Security | Threat Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-149 | Security | Trust Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-150 | Security | Certificate Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-151 | Security | Encryption Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-152 | Security | Secret Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-153 | Security | Integrity Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-154 | Security | Provenance Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-155 | Security | Supply Chain Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-156 | Security | Dependency Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-157 | Security | Vulnerability Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-158 | Security | Incident Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-159 | Security | Emergency Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-160 | Marketplace / NFT | Marketplace Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-161 | Marketplace / NFT | Listing Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-162 | Marketplace / NFT | Purchase Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-163 | Marketplace / NFT | Seller Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-164 | Marketplace / NFT | Buyer Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-165 | Marketplace / NFT | Escrow Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-166 | Marketplace / NFT | Payment Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-167 | Marketplace / NFT | Royalty Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-168 | Marketplace / NFT | License Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-169 | Marketplace / NFT | NFT Mint Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-170 | Marketplace / NFT | NFT Transfer Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-171 | Marketplace / NFT | NFT Burn Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-172 | Marketplace / NFT | Asset Provenance Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-173 | Marketplace / NFT | Dispute Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-174 | Genesis | Game Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-175 | Genesis | World Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-176 | Genesis | Multiplayer Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-177 | Genesis | Mod Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-178 | Genesis | Asset Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-179 | Genesis | Script Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-180 | Genesis | AI NPC Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-181 | Genesis | Quest AI Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-182 | Genesis | Economy Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-183 | Genesis | Anti-Cheat Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-184 | Genesis | Save Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-185 | Browser / GateToHell | Browser Security Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-186 | Browser / GateToHell | Origin Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-187 | Browser / GateToHell | Site Isolation Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-188 | Browser / GateToHell | Permission Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-189 | Browser / GateToHell | Extension Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-190 | Browser / GateToHell | Download Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-191 | Browser / GateToHell | Upload Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-192 | Browser / GateToHell | Cookie Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-193 | Browser / GateToHell | Storage Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-194 | Browser / GateToHell | Wallet Connection Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-195 | Browser / GateToHell | gth:// Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-196 | Browser / GateToHell | Web AI Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-197 | Browser / GateToHell | Tracking Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-198 | Governance | Protocol Governance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-199 | Governance | Economic Governance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-200 | Governance | Parameter Governance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-201 | Governance | Upgrade Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-202 | Governance | Release Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-203 | Governance | Change Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-204 | Governance | Emergency Governance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-205 | Governance | Delegation Governance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-206 | Governance | Evidence Governance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-207 | Governance | Verification Governance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-208 | Evidence & Verification | Evidence Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-209 | Evidence & Verification | Error Evidence Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-210 | Evidence & Verification | Finding Evidence Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-211 | Evidence & Verification | Verification Evidence Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-212 | Evidence & Verification | Evidence Provenance | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-213 | Evidence & Verification | Evidence Integrity | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-214 | Evidence & Verification | Exact-SHA Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-215 | Evidence & Verification | CI Evidence Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-216 | Evidence & Verification | Run Evidence Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-217 | Evidence & Verification | Residual Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |
| POL-218 | Evidence & Verification | Release Evidence Policy | Policy definition, evaluation, enforcement boundary and evidence requirements |

## Decision semantics

- `DENY`: action is not permitted.
- `ALLOW`: action is permitted under the evaluated contract.
- `ALLOW_WITH_CONDITIONS`: action is permitted only while explicit conditions remain true.
- `REQUIRE_APPROVAL`: policy evaluation requires an explicit approval step before execution.
- `DEFER`: decision requires additional authoritative context; it is not an implicit allow.

## Security invariants

1. UI presence is not authorization.
2. Model output is not authorization.
3. Agent intent is not authorization.
4. Orchestrator execution is not authorization.
5. Capability scope cannot be expanded by policy evaluation.
6. Policies cannot create private keys or signatures.
7. Wallet signing remains an explicit wallet/user boundary.
8. Blockchain canonical state remains authoritative in the L1 protocol.
9. GlobusOS cannot bypass ShivaCore security boundaries.
10. AI actions follow Model → Agent → Capability → Policy → Approval → Tool.
11. Secrets are never placed into logs or evidence.
12. Error Evidence ≠ Finding Evidence ≠ Verification Evidence.
13. Historical CI does not verify a different source SHA.
14. Policy exceptions are explicit, bounded, auditable and time-limited.
15. Policy evaluation must fail closed where required authoritative context is unavailable.

## Cross-system policy boundaries

### Aurora
`Model → Agent → Capability → Policy → Approval → Tool → API Orchestrator → Domain Service`

### Blockchain
`Application/Aurora → Capability → Policy → Approval → Wallet → Explicit Signature → SDK/RPC → Node → Canonical State`

### Operating system
`Application/Aurora → Capability → Policy → OS Service → ShivaCore → Hardware`

### Browser
`Page/Extension/AI → Browser Capability → Policy → User Approval where required → Browser Service`

## Policy verification pipeline

```
Policy Source
 → Schema Validation
 → Policy Static Validation
 → Unit Tests
 → Policy Engine Tests
 → Integration Tests
 → Security Tests
 → Negative / Deny Tests
 → Exact-SHA CI
 → Run
 → Job
 → Step
 → Exit Code
 → Log
 → Evidence
 → Verification
 → Residual
```

## Status model

`UNANALYZED → ANALYZED → FIXED → RERUNNING → VERIFIED`

`RESIDUAL` remains explicit whenever an implementation, integration or verification gap remains.

## Architecture ≠ implementation ≠ evidence

Presence of a policy entry does not prove:

- policy engine implementation;
- enforcement at the actual security boundary;
- integration with the target service;
- successful tests;
- Exact-SHA CI;
- verification.

## Ownership

- Policy architecture aggregation: `a-townchain-ecosystem`
- Normative governance/policy SSOT: `atc-standards`
- Aurora policy enforcement: `aurora-ai`
- OS policy enforcement: `a-townchain-os` / ShivaCore
- Blockchain policy enforcement: responsible L1 repositories
- Wallet authorization: wallet/SDK boundary

## Traceability

`Vision → Policy Domain → Policy Subsystem → Rule → Contract → Source → Test → Workflow → Exact-SHA Evidence → Verification → Residual`

