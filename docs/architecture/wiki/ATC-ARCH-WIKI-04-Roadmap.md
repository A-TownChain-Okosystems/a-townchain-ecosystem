# Chapter 04 — Implementation roadmap

**Roadmap status:** `PROPOSED / NOT VERIFIED`  
**Cadence:** two-week sprints are a planning assumption only.

## Milestones

| Milestone | Outcome |
|---|---|
| M0 | Pin baseline, manifest and source revisions |
| M1 | Reconcile layer registry, IDs and issues |
| M2 | Complete sublayer/applicability dispositions |
| M3 | Source census of components and functions |
| M4 | Normalize function contracts |
| M5 | Build and validate dependency/interface graph |
| M6 | Implement fail-closed validators and evidence pipeline |
| M7 | Dispatch prioritized implementation work to canonical repositories |
| M8 | End-to-end acceptance and independent release decision |

## Sprint sequence

- S0: setup and baseline.
- S1: layer and issue reconciliation.
- S2: sublayer and applicability matrix.
- S3: source census for L1–L4.
- S4: source census for L5–L7, including blockchain, ATC-VM and ATCLang.
- S5: source census for L8–L10, including Aurora AI, applications and operations.
- S6: contract normalization.
- S7: dependency graph.
- S8: validator implementation.
- S9: priority implementation wave.
- S10: integration and acceptance.

## Checkpoints

CP0–CP8 align with M0–M8. Each checkpoint requires exact catalog/source SHAs, changed IDs, inventory counts, dependency/SSOT findings, Run → Job → Step references, exit codes, logs, artifact hashes, unresolved risks, reviewer and explicit decision.

## Initial TODOs

| ID | Work |
|---|---|
| TODO-CAT-001 | Pin baseline and repository manifest; exclude demo repository from live drift |
| TODO-CAT-002 | Reconcile existing records and ID collisions |
| TODO-CAT-003 | Build L1–L10 × X1–X36 applicability matrix |
| TODO-CAT-004 | Enumerate domain-specific sublayers |
| TODO-CAT-005 | Run canonical source census |
| TODO-CAT-006 | Triage unknown security/protocol/consensus/VM/language semantics |
| TODO-CAT-007 | Normalize contracts and test criteria |
| TODO-CAT-008 | Generate dependency/interface graph |
| TODO-CAT-009 | Implement fail-closed validators |
| TODO-CAT-010 | Generate reproducible evidence packets |
| TODO-CAT-011 | Create canonical-owner implementation issues |
| TODO-CAT-012 | Run cross-layer acceptance and residual triage |
| TODO-CAT-013 | Reconcile machine/human/generated counts |
| TODO-CAT-014 | Independent review of exact catalog revision |

## Sprint close definition

Report planned/done/blocked/deferred tasks, exact SHAs, changed IDs, test evidence, residuals and dependencies. Documentation written is not by itself sprint completion or implementation verification.


## Implementation order and dependency gates

The detailed sequence is authoritative for planning only; it does not indicate completion.

1. **Baseline first:** pin catalog/source revisions and repository manifest; preserve `demo-repository` exclusion from live drift checks.
2. **Registry second:** reconcile all L1–L10 and X1–X36 records, existing issues, IDs and boundaries.
3. **Structure third:** complete sublayer inventory and L-layer × X-layer applicability matrix with explicit dispositions.
4. **Source census fourth:** map canonical source candidates to components, symbols, interfaces and functions; classify every candidate.
5. **Ownership/risk fifth:** resolve SSOT and block on unknown critical protocol, consensus, security, VM, language or AI-runtime semantics.
6. **Contracts sixth:** normalize function contracts and testable acceptance criteria.
7. **Graph seventh:** build source-backed dependency, interface, data-flow and trust-boundary graphs.
8. **Evidence tooling eighth:** implement deterministic validators, drift/duplication checks and fail-closed evidence generation.
9. **Implementation waves ninth:** dispatch tasks to canonical owner repositories, following Wave A–G in the master catalog's Section 18.
10. **End-to-end acceptance last:** verify integration, portability, security, lifecycle and recovery; obtain an explicit independent decision.

### Dependency rule

Parallel work is allowed only after ownership, interface contracts and acceptance criteria are stable. Do not implement a guessed contract to unblock a dependent task. Mark the dependency `BLOCKED` or `UNRESOLVED`, document the decision owner and resume only when evidence resolves it.

### Wave summary

- Wave A: boot/HAL, identity/capabilities, trust and evidence foundation.
- Wave B: ShivaCore/GlobusOS substrate and system services.
- Wave C: blockchain core and canonical ATC-VM.
- Wave D: ATCLang/toolchain.
- Wave E: Aurora AI with policy, sandboxing and explicit approval boundaries.
- Wave F: desktop/UI, applications and ecosystem services.
- Wave G: installation, release composition, recovery and end-to-end compatibility.

See [Section 18 in the master catalog](../ATC-ARCH-CATALOG-MASTER-X1-X36-L1-L10.md) for the complete ordered table, checkpoint exit conditions and stop/go rules.
