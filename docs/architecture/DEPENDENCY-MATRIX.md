# A-TownChain Ecosystem — Dependency Matrix

**Document ID:** ATC-DEPENDENCY-MATRIX-001  
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
| DEP-001 | Dependency Core | Dependency Identity | ARCHITECTURE CONTRACT |
| DEP-002 | Dependency Core | Dependency Owner | ARCHITECTURE CONTRACT |
| DEP-003 | Dependency Core | Dependency Type | ARCHITECTURE CONTRACT |
| DEP-004 | Dependency Core | Direct Dependency | ARCHITECTURE CONTRACT |
| DEP-005 | Dependency Core | Indirect Dependency | ARCHITECTURE CONTRACT |
| DEP-006 | Dependency Core | Optional Dependency | ARCHITECTURE CONTRACT |
| DEP-007 | Build | Build Dependency | ARCHITECTURE CONTRACT |
| DEP-008 | Build | Runtime Dependency | ARCHITECTURE CONTRACT |
| DEP-009 | Build | Test Dependency | ARCHITECTURE CONTRACT |
| DEP-010 | Build | Development Dependency | ARCHITECTURE CONTRACT |
| DEP-011 | Build | Protocol Dependency | ARCHITECTURE CONTRACT |
| DEP-012 | Runtime | API Dependency | ARCHITECTURE CONTRACT |
| DEP-013 | Runtime | Schema Dependency | ARCHITECTURE CONTRACT |
| DEP-014 | Runtime | Event Dependency | ARCHITECTURE CONTRACT |
| DEP-015 | Runtime | State Dependency | ARCHITECTURE CONTRACT |
| DEP-016 | Runtime | Storage Dependency | ARCHITECTURE CONTRACT |
| DEP-017 | API | Network Dependency | ARCHITECTURE CONTRACT |
| DEP-018 | API | Crypto Dependency | ARCHITECTURE CONTRACT |
| DEP-019 | API | Consensus Dependency | ARCHITECTURE CONTRACT |
| DEP-020 | API | VM Dependency | ARCHITECTURE CONTRACT |
| DEP-021 | API | Wallet Dependency | ARCHITECTURE CONTRACT |
| DEP-022 | API | SDK Dependency | ARCHITECTURE CONTRACT |
| DEP-023 | Data | OS Dependency | ARCHITECTURE CONTRACT |
| DEP-024 | Data | Kernel Dependency | ARCHITECTURE CONTRACT |
| DEP-025 | Data | HAL Dependency | ARCHITECTURE CONTRACT |
| DEP-026 | Data | Hardware Dependency | ARCHITECTURE CONTRACT |
| DEP-027 | Data | Aurora Dependency | ARCHITECTURE CONTRACT |
| DEP-028 | Security | Model Dependency | ARCHITECTURE CONTRACT |
| DEP-029 | Security | Agent Dependency | ARCHITECTURE CONTRACT |
| DEP-030 | Security | Tool Dependency | ARCHITECTURE CONTRACT |
| DEP-031 | Security | RAG Dependency | ARCHITECTURE CONTRACT |
| DEP-032 | Security | Data Dependency | ARCHITECTURE CONTRACT |
| DEP-033 | AI | Genesis Dependency | ARCHITECTURE CONTRACT |
| DEP-034 | AI | Marketplace Dependency | ARCHITECTURE CONTRACT |
| DEP-035 | AI | NFT Dependency | ARCHITECTURE CONTRACT |
| DEP-036 | AI | Browser Dependency | ARCHITECTURE CONTRACT |
| DEP-037 | AI | Version Constraint | ARCHITECTURE CONTRACT |
| DEP-038 | AI | Version Pin | ARCHITECTURE CONTRACT |
| DEP-039 | OS | Compatibility | ARCHITECTURE CONTRACT |
| DEP-040 | OS | ABI Compatibility | ARCHITECTURE CONTRACT |
| DEP-041 | OS | API Compatibility | ARCHITECTURE CONTRACT |
| DEP-042 | OS | Schema Compatibility | ARCHITECTURE CONTRACT |
| DEP-043 | OS | Event Compatibility | ARCHITECTURE CONTRACT |
| DEP-044 | Blockchain | License | ARCHITECTURE CONTRACT |
| DEP-045 | Blockchain | Provenance | ARCHITECTURE CONTRACT |
| DEP-046 | Blockchain | SBOM | ARCHITECTURE CONTRACT |
| DEP-047 | Blockchain | Vulnerability | ARCHITECTURE CONTRACT |
| DEP-048 | Blockchain | Integrity | ARCHITECTURE CONTRACT |
| DEP-049 | Release | Reproducibility | ARCHITECTURE CONTRACT |
| DEP-050 | Release | Lockfile | ARCHITECTURE CONTRACT |
| DEP-051 | Release | Artifact | ARCHITECTURE CONTRACT |
| DEP-052 | Release | Package Source | ARCHITECTURE CONTRACT |
| DEP-053 | Release | Registry | ARCHITECTURE CONTRACT |
| DEP-054 | Release | Mirror | ARCHITECTURE CONTRACT |
| DEP-055 | Governance | Dependency Update | ARCHITECTURE CONTRACT |
| DEP-056 | Governance | Update Policy | ARCHITECTURE CONTRACT |
| DEP-057 | Governance | Deprecation | ARCHITECTURE CONTRACT |
| DEP-058 | Governance | Migration | ARCHITECTURE CONTRACT |
| DEP-059 | Governance | Cycle Detection | ARCHITECTURE CONTRACT |
| DEP-060 | Verification | Impact Analysis | ARCHITECTURE CONTRACT |
| DEP-061 | Verification | Failure Propagation | ARCHITECTURE CONTRACT |
| DEP-062 | Verification | Security Boundary | ARCHITECTURE CONTRACT |
| DEP-063 | Verification | Test Coverage | ARCHITECTURE CONTRACT |
| DEP-064 | Verification | Exact-SHA Verification | ARCHITECTURE CONTRACT |

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
