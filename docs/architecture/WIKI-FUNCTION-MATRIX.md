# A-TownChain Ecosystem — Wiki Function Matrix

**Document ID:** ATC-WIKI-FUNCTION-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT  
**Canonical functions:** 64

## Purpose
The Wiki is the ecosystem's structured documentation and knowledge layer. It publishes architecture, specifications, implementation references and evidence references without becoming an authority over runtime state.

**Documentation ≠ implementation. Documentation ≠ authorization. Documentation ≠ verification.**

## Canonical knowledge flow
`Source/Repository → Documentation Contract → Wiki Page → Cross-Reference → Search/Knowledge Graph → Reader/Agent → Evidence Reference → Verification`

## Function matrix
| ID | Domain | Function | Layer | Authority | Status |
|---|---|---|---|---|---|
| WIKI-001 | Core & Information Architecture | Wiki shell | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-002 | Core & Information Architecture | Navigation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-003 | Core & Information Architecture | Information architecture | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-004 | Core & Information Architecture | Taxonomy | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-005 | Core & Information Architecture | Category management | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-006 | Core & Information Architecture | Page identity | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-007 | Core & Information Architecture | Page lifecycle | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-008 | Core & Information Architecture | Canonical page registry | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-009 | Knowledge Content | Architecture documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-010 | Knowledge Content | Protocol documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-011 | Knowledge Content | API documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-012 | Knowledge Content | OS documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-013 | Knowledge Content | Aurora documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-014 | Knowledge Content | Genesis documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-015 | Knowledge Content | ATCLang documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-016 | Knowledge Content | ATC-VM documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-017 | Reference & Specifications | Standards reference | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-018 | Reference & Specifications | Schema reference | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-019 | Reference & Specifications | Contract reference | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-020 | Reference & Specifications | CLI reference | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-021 | Reference & Specifications | Configuration reference | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-022 | Reference & Specifications | Error/reference catalog | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-023 | Reference & Specifications | Glossary | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-024 | Reference & Specifications | FAQ/How-to | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-025 | Source & Evidence | Source linking | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-026 | Source & Evidence | Commit linking | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-027 | Source & Evidence | PR linking | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-028 | Source & Evidence | Issue linking | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-029 | Source & Evidence | Workflow/run linking | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-030 | Source & Evidence | Exact-SHA evidence | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-031 | Source & Evidence | Implementation status | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-032 | Source & Evidence | Verification status | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-033 | Cross-System Knowledge | Integration documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-034 | Cross-System Knowledge | State-flow documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-035 | Cross-System Knowledge | Event-flow documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-036 | Cross-System Knowledge | Security-boundary documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-037 | Cross-System Knowledge | Dependency documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-038 | Cross-System Knowledge | API-orchestrator documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-039 | Cross-System Knowledge | Policy documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-040 | Cross-System Knowledge | Authority-boundary documentation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-041 | Search & Knowledge Graph | Full-text search | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-042 | Search & Knowledge Graph | Symbol search | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-043 | Search & Knowledge Graph | Cross-reference resolution | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-044 | Search & Knowledge Graph | Dependency graph view | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-045 | Search & Knowledge Graph | Knowledge graph | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-046 | Search & Knowledge Graph | Backlinks | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-047 | Search & Knowledge Graph | Related content | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-048 | Search & Knowledge Graph | Version comparison | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-049 | Governance & Change | Documentation ownership | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-050 | Governance & Change | Review workflow | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-051 | Governance & Change | Change history | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-052 | Governance & Change | Deprecation | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-053 | Governance & Change | Supersession | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-054 | Governance & Change | Approval metadata | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-055 | Governance & Change | Governance evidence | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-056 | Governance & Change | Audit trail | Documentation Knowledge Layer | Evidence/Governance | ARCHITECTURE CONTRACT |
| WIKI-057 | Publishing & Runtime | Wiki build | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-058 | Publishing & Runtime | Static generation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-059 | Publishing & Runtime | Preview | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-060 | Publishing & Runtime | Versioned publishing | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-061 | Publishing & Runtime | Search index generation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-062 | Publishing & Runtime | Broken-link validation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-063 | Publishing & Runtime | Schema validation | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |
| WIKI-064 | Publishing & Runtime | Publication verification | Documentation Knowledge Layer | Knowledge | ARCHITECTURE CONTRACT |

## Wiki authority rules
1. The Wiki is descriptive/documentary and never authoritative over runtime state.
2. Canonical source remains the owning repository, contract, schema, implementation or protocol authority.
3. Wiki statements distinguish ARCHITECTURE, SPECIFIED, IMPLEMENTED, TESTED, VERIFIED and RESIDUAL.
4. Verification claims identify exact source, test/workflow and SHA.
5. Historical documentation cannot establish current-main verification without current Exact-SHA evidence.
6. Documentation output cannot grant capabilities, approvals or runtime authorization.
7. Secrets never enter Wiki pages or evidence.
8. Error Evidence ≠ Finding Evidence ≠ Verification Evidence.

## Canonical documentation chain
`Architecture → Specification → Source → Test → Workflow → Exact-SHA → Run → Job → Step → Exit Code → Log → Verification → Wiki Evidence Reference`

## Integration
The Wiki indexes and explains, but does not replace the Function, API, Integration, State/Data, Event, Cross-Boundary State/Event, Security Boundary, Dependency, Policy, UI, OS, AI, VM, ATCLang, Genesis or Browser matrices.

## Residual
This matrix defines the Wiki capability contract. Existing pages, generators, search indexes, link validation and publication workflows require separate implementation and Exact-SHA verification.
