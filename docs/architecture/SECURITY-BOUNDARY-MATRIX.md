# A-TownChain Ecosystem — Security Boundary Matrix

**Document ID:** ATC-SECURITY-BOUNDARY-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT  
**Rule:** Architecture presence does not imply implementation, testing, CI verification, integration, or E2E verification.

## Canonical contract

Every entry is traced:

`Domain → Subsystem → Contract → Source → Test → Workflow → Exact-SHA → Evidence → Verification → Residual`

Evidence separation:

`Error Evidence ≠ Finding Evidence ≠ Verification Evidence`

## Matrix

| ID | Domain | Function / Contract | Status |
|---|---|---|---|
| SEC-001 | Trust Boundaries | Trust Boundary | ARCHITECTURE CONTRACT |
| SEC-002 | Trust Boundaries | Security Principal | ARCHITECTURE CONTRACT |
| SEC-003 | Trust Boundaries | Identity | ARCHITECTURE CONTRACT |
| SEC-004 | Trust Boundaries | Authentication | ARCHITECTURE CONTRACT |
| SEC-005 | Trust Boundaries | Authorization | ARCHITECTURE CONTRACT |
| SEC-006 | Trust Boundaries | Capability | ARCHITECTURE CONTRACT |
| SEC-007 | Identity | Capability Scope | ARCHITECTURE CONTRACT |
| SEC-008 | Identity | Policy Boundary | ARCHITECTURE CONTRACT |
| SEC-009 | Identity | Approval Boundary | ARCHITECTURE CONTRACT |
| SEC-010 | Identity | Authority Boundary | ARCHITECTURE CONTRACT |
| SEC-011 | Identity | Secret Boundary | ARCHITECTURE CONTRACT |
| SEC-012 | Identity | Key Boundary | ARCHITECTURE CONTRACT |
| SEC-013 | Capabilities | Wallet Boundary | ARCHITECTURE CONTRACT |
| SEC-014 | Capabilities | Signing Boundary | ARCHITECTURE CONTRACT |
| SEC-015 | Capabilities | Kernel Boundary | ARCHITECTURE CONTRACT |
| SEC-016 | Capabilities | ShivaCore Boundary | ARCHITECTURE CONTRACT |
| SEC-017 | Capabilities | HAL Boundary | ARCHITECTURE CONTRACT |
| SEC-018 | Capabilities | Hardware Boundary | ARCHITECTURE CONTRACT |
| SEC-019 | Secrets | Process Isolation | ARCHITECTURE CONTRACT |
| SEC-020 | Secrets | Sandbox | ARCHITECTURE CONTRACT |
| SEC-021 | Secrets | Container Isolation | ARCHITECTURE CONTRACT |
| SEC-022 | Secrets | VM Isolation | ARCHITECTURE CONTRACT |
| SEC-023 | Secrets | Network Boundary | ARCHITECTURE CONTRACT |
| SEC-024 | Secrets | Origin Boundary | ARCHITECTURE CONTRACT |
| SEC-025 | OS/Kernel | API Boundary | ARCHITECTURE CONTRACT |
| SEC-026 | OS/Kernel | RPC Boundary | ARCHITECTURE CONTRACT |
| SEC-027 | OS/Kernel | Contract Boundary | ARCHITECTURE CONTRACT |
| SEC-028 | OS/Kernel | VM Boundary | ARCHITECTURE CONTRACT |
| SEC-029 | OS/Kernel | Consensus Boundary | ARCHITECTURE CONTRACT |
| SEC-030 | OS/Kernel | State Boundary | ARCHITECTURE CONTRACT |
| SEC-031 | Blockchain | Storage Boundary | ARCHITECTURE CONTRACT |
| SEC-032 | Blockchain | Indexer Boundary | ARCHITECTURE CONTRACT |
| SEC-033 | Blockchain | Aurora Model Boundary | ARCHITECTURE CONTRACT |
| SEC-034 | Blockchain | Agent Boundary | ARCHITECTURE CONTRACT |
| SEC-035 | Blockchain | Tool Boundary | ARCHITECTURE CONTRACT |
| SEC-036 | Blockchain | Memory Boundary | ARCHITECTURE CONTRACT |
| SEC-037 | AI | RAG Boundary | ARCHITECTURE CONTRACT |
| SEC-038 | AI | Data Boundary | ARCHITECTURE CONTRACT |
| SEC-039 | AI | Prompt Boundary | ARCHITECTURE CONTRACT |
| SEC-040 | AI | Plugin Boundary | ARCHITECTURE CONTRACT |
| SEC-041 | AI | Extension Boundary | ARCHITECTURE CONTRACT |
| SEC-042 | AI | Browser Boundary | ARCHITECTURE CONTRACT |
| SEC-043 | Application | Genesis Mod Boundary | ARCHITECTURE CONTRACT |
| SEC-044 | Application | Marketplace Boundary | ARCHITECTURE CONTRACT |
| SEC-045 | Application | NFT Boundary | ARCHITECTURE CONTRACT |
| SEC-046 | Application | Identity Federation | ARCHITECTURE CONTRACT |
| SEC-047 | Application | Session Boundary | ARCHITECTURE CONTRACT |
| SEC-048 | Application | Privilege Transition | ARCHITECTURE CONTRACT |
| SEC-049 | Supply Chain | Least Privilege | ARCHITECTURE CONTRACT |
| SEC-050 | Supply Chain | Deny by Default | ARCHITECTURE CONTRACT |
| SEC-051 | Supply Chain | Fail Closed | ARCHITECTURE CONTRACT |
| SEC-052 | Supply Chain | Replay Protection | ARCHITECTURE CONTRACT |
| SEC-053 | Supply Chain | Injection Defense | ARCHITECTURE CONTRACT |
| SEC-054 | Supply Chain | DoS Protection | ARCHITECTURE CONTRACT |
| SEC-055 | Runtime Security | Rate Limit | ARCHITECTURE CONTRACT |
| SEC-056 | Runtime Security | Resource Isolation | ARCHITECTURE CONTRACT |
| SEC-057 | Runtime Security | Integrity | ARCHITECTURE CONTRACT |
| SEC-058 | Runtime Security | Provenance | ARCHITECTURE CONTRACT |
| SEC-059 | Runtime Security | Secure Update | ARCHITECTURE CONTRACT |
| SEC-060 | Runtime Security | SBOM | ARCHITECTURE CONTRACT |
| SEC-061 | Security Evidence | Artifact Signing | ARCHITECTURE CONTRACT |
| SEC-062 | Security Evidence | Dependency Trust | ARCHITECTURE CONTRACT |
| SEC-063 | Security Evidence | Audit | ARCHITECTURE CONTRACT |
| SEC-064 | Security Evidence | Security Telemetry | ARCHITECTURE CONTRACT |
| SEC-065 | Security Evidence | Incident Response | ARCHITECTURE CONTRACT |
| SEC-066 | Security Evidence | Recovery | ARCHITECTURE CONTRACT |
| SEC-067 | Testing | Secret Redaction | ARCHITECTURE CONTRACT |
| SEC-068 | Testing | Evidence Separation | ARCHITECTURE CONTRACT |
| SEC-069 | Testing | Threat Test | ARCHITECTURE CONTRACT |
| SEC-070 | Testing | Security Test | ARCHITECTURE CONTRACT |
| SEC-071 | Testing | Boundary Verification | ARCHITECTURE CONTRACT |
| SEC-072 | Testing | Security Residual | ARCHITECTURE CONTRACT |

## Authority rules

- Architecture does not create authority.
- UI, browser, model, agent, orchestrator and derived data never become authoritative merely by producing output.
- Authoritative state remains owned by its designated domain boundary.
- Cross-boundary mutation requires the applicable contract, capability, policy and approval path.
- Exact-SHA CI is required before an implementation claim becomes VERIFIED.
- Historical CI runs are not evidence for another SHA.
- Missing authoritative context fails closed where the contract requires it.

## Verification model

`Architecture → Contract → Source → Test → Integration → Exact-SHA CI → Run → Job → Step → Exit Code → Log → Verification → Residual`
