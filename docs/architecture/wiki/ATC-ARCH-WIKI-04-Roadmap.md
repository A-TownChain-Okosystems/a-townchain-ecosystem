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
