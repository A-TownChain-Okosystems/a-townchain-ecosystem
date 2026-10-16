# A-TownChain Architecture Master Catalog — L0–L9 and X1–X36

**Catalog ID:** `ATC-ARCH-CATALOG-MASTER-X1-X36-L0-L9`  
**Schema version:** `0.1.0-draft`  
**Status:** `PROPOSED / NOT VERIFIED`  
**Scope:** L0–L9 system architecture layers and cross-cutting layers X1–X36, including sublayers, components, function contracts, interfaces, dependencies, canonical implementation ownership, and evidence. L0 naming is explicitly conflict-tracked until reconciled against existing architecture records; no competing definition is silently overwritten.

> This is a catalog baseline and reconciliation index, not a claim that every component/function is already inventoried, implemented, or verified. Entries without source-backed mapping remain `UNRESOLVED`; proposed requirements remain `PROPOSED`. Do not promote statuses from documentation alone.

## 1. Governing invariants

1. **Catalog First, Implementation Second.** Establish stable IDs, requirements, contracts, source mappings, and gap classifications before implementation work.
2. **Standalone First, Ecosystem Second.** Production code stays in its canonical owner repository. The ecosystem repository owns the architecture catalog, cross-repository mapping, integration governance, compliance, evidence aggregation, and drift controls only.
3. **Evidence before Acceptance.** Every implementation or verification claim must bind to the exact source revision and reproducible evidence.
4. **One implementation SSOT per capability.** Do not create a second VM, consensus implementation, kernel, compiler, runtime, installer, scheduler, or agent runtime in the ecosystem repository.
5. **No inferred source mapping.** A plausible repository or path is not evidence. Missing/uncertain ownership and paths are `UNRESOLVED`.
6. **Fail closed.** Missing mandatory evidence, unknown security/consensus semantics, stale source SHAs, failed required checks, and ambiguous ownership cannot pass acceptance.
7. `demo-repository` is excluded from the live repository drift gate.
8. Green CI alone is not verification. Preserve Run → Job → Step → exit code/logs → artifact hash and exact commit.
9. A layer is not `VERIFIED` unless every applicable required sublayer and contract has a resolved disposition and sufficient evidence.
10. Architecture/catalog completeness, implementation status, and verification status are reported separately.

## 2. Architecture hierarchy

The normative system architecture is **L0–L9** (ten system layers). The orthogonal cross-cutting architecture is **X1–X36** (36 cross-cutting layers). The applicability matrix contains exactly 360 possible relationships. A cross-cutting assignment does not create a duplicate runtime or replace a domain repository's implementation SSOT.

The catalog also has record levels (sublayers, components, function contracts, interfaces, source mappings, evidence and lifecycle metadata). Those are **record types**, not extra system-layer IDs. Do not interpret a catalog record level as L0, L1, etc.

Governing rules:
- Catalog First, Implementation Second.
- Standalone First, Ecosystem Second.
- One canonical implementation SSOT per capability.
- A component has one primary mapping in `ATC-LAYER-MAPPING-001.yaml` and may have additional X1–X36 mappings.
- Source mapping is not functional verification; statuses remain fail-closed without exact-SHA evidence.
- `demo-repository` remains excluded from the live repository drift gate.

## 3. L0–L9 scope and legacy L0 conflict reconciliation

**Adopted architecture baseline:** L0–L9 + X1–X36.

### L0 decision

**Authoritative definition for this baseline: L0 — Hardware & Compute.**

L0 covers physical compute capabilities and inventory: CPU, GPU, NPU/AI accelerators, memory, storage media, security hardware (TPM/TEE), connectivity/I/O, power/thermal capabilities and hardware identity/capability discovery. L0 provides capabilities but grants no permissions. Boot/firmware/HAL are L1; kernel/TCB is L2; GlobusOS system services are L3; desktop/UI is L4; blockchain core is L5; ATC-VM is L6; developer platform is L7; Aurora AI is L8; applications and ecosystem services are L9.

### Issue #71 crosswalk — historical “L0 System & Network”

Issue #71's former L0 definition grouped hardware, HAL, OS, P2P/transport, node runtime and ShivaCore/GlobusOS boundaries into one broad layer. That composite is **not retained as a single L0 ID**, because it crosses multiple trust boundaries. Preserve Issue #71 as historical provenance and map its responsibilities as follows:

| Issue #71 legacy concern | Adopted L0–L9 destination | Disposition |
|---|---|---|
| Physical hardware and compute resources | L0 | Hardware & Compute |
| Boot, firmware, HAL, device discovery | L1 | Boot/Firmware/HAL |
| ShivaCore kernel and TCB | L2 | Kernel/security boundary |
| GlobusOS services, OS APIs, networking/filesystem | L3 | System services and OS architecture |
| Desktop/window manager/UI | L4 | Desktop presentation |
| P2P transport, node runtime, blockchain networking/protocol | L5 | Blockchain Core, where blockchain-owned |
| Deterministic execution / ATC-VM | L6 | VM execution boundary |
| AI/agent/model capabilities | L8 | Aurora AI; no consensus authority |
| Applications and user experience | L9 | Applications and ecosystem services |
| Legacy cross-layer control plane | X1–X36 | Split into explicit cross-cutting concerns; no single catch-all X |

The old Issue #71 labels L1–L7 are also a separate legacy taxonomy and must not be joined by ID to the new L0–L9 IDs. Their responsibilities are crosswalked by meaning, not by numeric label. This is a taxonomy reconciliation decision, not an implementation or verification claim.

### Former L10 responsibility allocation

The former **L10 — Integration, Deployment & Operations** is retired as a system-layer ID in the adopted L0–L9 taxonomy. Its responsibilities are decomposed by function and trust boundary; they are not transferred wholesale to L9.

| Former L10 responsibility | Adopted owner / cross-cutting layer | Mapping rule |
|---|---|---|
| Installer, image building, native/host installation | Canonical installer/OS/application owner; X35 | Installer implementation must be assigned to an evidenced owning repository. No installer implementation is inferred from COMP-LAUNCHPAD. |
| Updates, package management, release manifest, staged rollout and rollback | X35 — Deployment & Release Engineering | Artifact provenance, approvals, rollout health gates and installed-version evidence are required. |
| Configuration and secrets | X14 — Configuration, Secrets & Runtime Environment Governance | Configuration schemas and secret handling remain in the canonical runtime/installer owner. |
| Observability, health, diagnostics | X7 — Observability, Reliability & Resilience | Telemetry and health evidence must be bound to concrete contracts and source revisions. |
| Incident handling and operational recovery | X20 — Incident Response; X21 — Continuity & Disaster Recovery | Incident and recovery contracts remain separate from release/composition status. |
| Lifecycle and migration | X15 — Update Management, Migration & Lifecycle Governance | Compatibility and rollback/rollforward are explicit lifecycle contracts. |
| Cross-component dependency closure, interface compatibility, end-to-end acceptance | X36 — Ecosystem Integration, System Composition & End-to-End Compatibility | Use the integration contract and exact dependency/version evidence; no duplicate runtime. |
| Protocol/schema compatibility across boundaries | X3 — Protocol & Interoperability | Must have concrete interface or protocol contract evidence. |

COMP-OS-INTEGRATION remains an L9 integration-facing service with X3/X35/X36 projections, but its current FN-OSINT-002 source binding supports X36 applicability only. It does not by itself prove deployment/release implementation under X35. The current catalog has no source-backed installer component/function contract; that gap remains UNRESOLVED and must be filled by catalog-first reconciliation, not by relabeling Launchpad.

## 4. L0–L9 system architecture registry

| ID | Layer | Responsibility and boundary | Canonical ownership rule | Status |
|---|---|---|---|---|
| L0 | Hardware & Compute | Physical hardware resources, capability discovery and CPU/GPU/NPU/memory/storage/security/I/O/power capabilities. Grants no permissions. | Hardware contracts are specified at the HAL boundary; actual device implementation must be source-evidenced. | PROPOSED / UNRESOLVED |
| L1 | Boot, Firmware & Hardware Abstraction (HAL) | Boot chain, firmware interfaces, platform discovery, drivers/HAL and secure kernel handoff. | GlobusOS boot/HAL modules; exact paths must be evidenced. | PROPOSED / UNRESOLVED |
| L2 | ShivaCore Kernel / Trusted Computing Base | Capability microkernel, isolation, scheduling primitives, memory protection, secure IPC and kernel invariants. | `globus-os/modules/atc-shivacore/kernel/` is canonical kernel SSOT. | PROPOSED / UNRESOLVED |
| L3 | GlobusOS System Services & OS Architecture | Service lifecycle, filesystem/network services, process/device management and OS APIs. | `globus-os`; exact modules require evidence. | PROPOSED / UNRESOLVED |
| L4 | GlobusOS Desktop, Window Manager & UI | Compositor, window manager, launcher/taskbar, input, display and accessibility. | `globus-os`; exact module paths require evidence. | PROPOSED / UNRESOLVED |
| L5 | A-TownChain Blockchain Core | Node runtime, transactions, consensus integration, protocol networking and authoritative blockchain state transitions. | `a-townchain` canonical blockchain core; algorithm remains in its canonical component. | PROPOSED / UNRESOLVED |
| L6 | ATC-VM & Smart-Contract Execution | Bytecode validation, deterministic execution, gas/resource bounds, contract state and execution interfaces. | `a-townchain/components/vm` canonical ATC-VM; no duplicate production VM. | PROPOSED / UNRESOLVED |
| L7 | Developer Platform & ATCLang | Compiler/language semantics, artifacts, SDK/toolchain, diagnostics and developer workflows. | Map to canonical ATCLang/toolchain repositories; no inferred paths. | PROPOSED / UNRESOLVED |
| L8 | Aurora AI Platform | AI control plane, agent platform, policy/approval, runtime and model/tool routing. Not kernel or consensus authority. | `aurora-ai` canonical AI platform; exact module mappings require evidence. | PROPOSED / UNRESOLVED |
| L9 | Applications & Ecosystem Services | Wallets, clients, domain services, launchpad/marketplace, system applications and user-facing integrations. | Each application/service stays in its actual owning repository. | PROPOSED / UNRESOLVED |

**Former L10 responsibilities:** L10 is not a system-layer ID in this baseline. Deployment/release/rollout governance is represented by **X35**; integration, system composition and end-to-end compatibility by **X36**. These are cross-cutting contracts, not replacement implementation repositories. The former L10 × X3 matrix disposition is not transferred automatically; it remains unresolved unless separately supported by evidence.

### L-layer mandatory fields

Every L-layer record must contain: stable ID, title, version, status, owner, mission, in-scope/out-of-scope, sublayer IDs, requirements, component IDs, contract IDs, inbound/outbound dependencies, interface references, security boundaries, resource assumptions, canonical SSOT references, tests/acceptance criteria, evidence references, open conflicts, exclusions, lifecycle and approval decision.

## 5. X1–X36 cross-cutting layer registry

These layers govern capabilities across applicable L-layers. The applicability matrix must be populated during reconciliation; an empty/unknown relationship is not proof of non-applicability.

| ID | Architecture layer | Required coverage |
|---|---|---|
| X1 | Security & Trust | Threat model, trust boundaries, secure defaults, cryptographic controls, security testing and risk disposition. |
| X2 | Identity, Access & Capabilities | Identity lifecycle, authentication, authorization, capability issuance/delegation/revocation and least privilege. |
| X3 | Protocol & Interoperability | Protocol schemas, versioning, negotiation, compatibility, canonical serialization and boundary validation. |
| X4 | Data, State & Storage | State ownership, persistence, consistency, migrations, integrity, retention and recovery. |
| X5 | Standards, Governance & Policy | Normative source, decision rights, policy versioning, exceptions, approval and conformance. |
| X6 | Testing, Verification & Evidence | Test strategy, conformance, reproducibility, evidence provenance and acceptance gates. |
| X7 | Observability, Reliability & Resilience | Logs/metrics/traces, health, fault handling, resilience and service-level evidence. |
| X8 | Performance, Capacity & Resource Optimization | Performance budgets, profiling, throughput/latency, capacity and optimization evidence. |
| X9 | Compatibility, Portability & Platform Support | Platform profiles, compatibility matrices, portability, hardware/software support and limits. |
| X10 | Supply Chain, Build Integrity & Artifact Provenance | Source/build provenance, dependency integrity, artifact identity, reproducibility and signing. |
| X11 | Privacy, Data Protection & Information Governance | Data classification, minimization, access, retention, deletion and privacy controls. |
| X12 | Resource Governance, Cost Control & Economic Sustainability | Resource budgets, cost attribution, economic constraints and sustainable resource use. |
| X13 | API Governance, Service Contracts & Developer Integration | API/ABI contracts, versioning, error semantics, developer integration and deprecation. |
| X14 | Configuration, Secrets & Runtime Environment Governance | Configuration schema, secret handling, environment isolation, defaults and change control. |
| X15 | Update Management, Migration & Lifecycle Governance | Upgrade, rollback/rollforward, migration safety, deprecation and lifecycle compatibility. |
| X16 | Accessibility, Internationalization & Inclusive Interaction | Accessibility requirements, locale, input/output modalities and inclusive interaction. |
| X17 | Documentation, Knowledge Management & Technical Information Governance | Source-linked docs, ownership, freshness, terminology and documentation quality. |
| X18 | Ecosystem Integrity, Catalog Drift & Architecture Observability | Catalog/source drift, ownership uniqueness, missing links, duplicate detection and reports. |
| X19 | Risk Management, Threat Analysis & Systemic Risk Governance | Risk register, threat analysis, systemic dependencies, risk owners and treatment. |
| X20 | Incident Response, Crisis Management & Operational Recovery | Detection, triage, containment, escalation, incident evidence and post-incident actions. |
| X21 | Business Continuity, Disaster Recovery & Mission-Critical Continuity | Recovery objectives, backups, failover, continuity exercises and recovery evidence. |
| X22 | Compliance, Legal Requirements & Regulatory Assurance | Applicable obligations, traceability, compliance evidence and review/exception records. |
| X23 | Product Lifecycle, Configuration Management & Portfolio Governance | Product baselines, configuration items, ownership, release scope and portfolio decisions. |
| X24 | Human Oversight, Decision Accountability & Organizational Readiness | Human approval boundaries, accountability, operator readiness and override auditing. |
| X25 | Organizational Interoperability, Ecosystem Collaboration & Federated Governance | Cross-team/repository ownership, federation boundaries, shared decisions and dispute handling. |
| X26 | Ecosystem Evolution, Strategic Innovation & Future Readiness | Evolution paths, migration strategy, extensibility, deprecation and future compatibility. |
| X27 | Ecosystem Knowledge, Semantic Integration & Architecture Intelligence | Semantic identity, ontology/metadata alignment, cross-layer relationships and knowledge quality. |
| X28 | Simulation, Digital Twin & System Modeling | Models, assumptions, simulation validity, scenario coverage and model-to-system correlation. |
| X29 | Digital Ethics, Responsible Innovation & Societal Impact | Ethical risk assessment, impact review, accountability and mitigation evidence. |
| X30 | Ecosystem Economics, Incentives & Value Flow Governance | Incentive models, value flows, accounting assumptions and economic attack analysis. |
| X31 | Ecosystem Data Governance, Information Quality & Data Lifecycle | Data ownership, quality, lineage, lifecycle, schemas and authoritative records. |
| X32 | Ecosystem API Security, Access Mediation & Zero-Trust Integration | Service identity, policy enforcement, boundary mediation, credentials and zero-trust checks. |
| X33 | Ecosystem Event Governance, Messaging & Distributed Workflow Coordination | Event identity/schema, ordering, delivery semantics, idempotency, retries, workflow state and audit. |
| X34 | Ecosystem Resource Scheduling, Capacity Allocation & Workload Governance | Workload classification, admission, reservations, fairness, quotas, isolation and safe release. |
| X35 | Ecosystem Deployment, Release Engineering & Rollout Governance | Artifact provenance, release manifest, approvals, staged rollout, health gates, recovery and installed-version evidence. |
| X36 | Ecosystem Integration, System Composition & End-to-End Compatibility | Component composition, ownership resolution, interface compatibility, dependency closure and E2E acceptance. |

### Cross-cutting layer boundaries

- X1/X2/X32 cover security and access from different scopes: core trust controls, identity/capability lifecycle, and service/API boundary mediation respectively.
- X3/X13/X36 cover protocol interoperability, API/service contracts, and composition/E2E compatibility respectively.
- X4/X11/X31 cover state/storage, privacy protection, and data governance/quality/lifecycle respectively.
- X6/X18 cover verification evidence and ecosystem catalog/drift observability respectively.
- X8/X12/X34 cover optimization, cost/economic governance, and scheduling/admission/capacity decisions respectively.
- X15/X23/X35 cover lifecycle/migration, product/configuration baselines, and release/deployment/rollout respectively.
- X33 coordinates event/workflow semantics; it does not replace canonical message brokers, runtimes or domain state machines.
- X36 coordinates integration contracts and evidence; it does not own duplicate implementations.

## 6. Sublayer decomposition baseline

Use the following reusable sublayer families where relevant. They are a coverage checklist, not a requirement to force every domain into exactly 17 sublayers. Domain-specific sublayers must be added; irrelevant families must be explicitly marked `EXCLUDED` with rationale.

1. Identity & Registry
2. Core Components
3. Data & State
4. Interfaces & Protocols
5. Execution & Orchestration
6. Security & Trust
7. Validation & Policy
8. Resource Management
9. Error Handling & Recovery
10. Observability & Audit
11. Lifecycle & Configuration
12. Interoperability
13. Testing & Simulation
14. Verification & Evidence
15. Governance & Compliance
16. Performance & Reliability
17. Integration & End-to-End

Each sublayer receives a stable ID and records parent layer, purpose, boundaries, requirements, components, function contracts, inputs/outputs, dependencies, security constraints, resource/error semantics, tests, evidence, exclusions and lifecycle.

Suggested ID convention: `ATC-ARCH-{L|X}{nn}-{SUBLAYER-SLUG}`. Existing established IDs take precedence; run a collision check before issuing new ones. Do not renumber existing contracts merely to fit this convention.

## 7. Component and function-contract model

### Component record

Required fields:

- `component_id`, `name`, `version`, `status`, `owner`
- `parent_layer_ids[]`, `sublayer_ids[]`
- `responsibility`, `in_scope`, `out_of_scope`
- `canonical_repository`, `canonical_path` (or explicit `UNRESOLVED`)
- `interface_ids[]`, `contract_ids[]`, `dependency_ids[]`
- `security_boundary`, `data_classification`, `resource_profile`
- `implementation_commit_sha`, `source_blob_sha`, `source_symbol` when evidenced
- `test_refs[]`, `evidence_refs[]`, `findings[]`, `exclusion_reason` if applicable

### Function contract record

Every function contract is independently addressable and testable. Required fields:

- Stable `contract_id`; parent layer, sublayer and component IDs.
- Name, purpose, accountable owner, normative/proposed status and version.
- Behavioral specification, typed inputs/outputs, units and serialization.
- Preconditions, postconditions, invariants, side effects and idempotency semantics.
- Error taxonomy, failure response, boundaries, limits, timeouts, retries and cancellation.
- Security/capability requirements and privacy constraints where applicable.
- Interface/API/ABI/IPC/event references and compatibility policy.
- Canonical repository, full 40-character commit SHA, exact path, symbol/entry point and blob SHA when mapped.
- Acceptance criteria; positive, negative, boundary, fault-injection and regression tests as applicable.
- Verification method, Run/Job/Step references, exit codes, logs and artifact hashes.
- Implementation status, verification status, lifecycle status, open findings and approved exceptions.

No field may silently disappear. If a field is inapplicable, record `NOT_APPLICABLE` plus a reason; if unknown, use `UNRESOLVED`.

## 8. Canonical SSOT registry

These mappings are architectural ownership rules, not proof that any specific function has already been found or verified.

| Capability | Canonical owner / rule | Forbidden catalog behavior |
|---|---|---|
| Blockchain core | `A-TownChain-Okosystems/a-townchain` | Do not create a second blockchain node/protocol implementation in ecosystem. |
| ATC-VM | `a-townchain/components/vm` | Do not treat a standalone `atc-vm` or ecosystem component as production SSOT. |
| Consensus/algorithm | `a-townchain/components/algorithm` | Do not infer consensus semantics or create duplicate algorithms. Unknown consensus decisions stay BLOCKED. |
| ShivaCore kernel | `globus-os/modules/atc-shivacore/kernel/` | `atc-shivacore` is specs/governance/support, not a second kernel implementation. |
| GlobusOS | `A-TownChain-Okosystems/globus-os`, exact module paths to verify | Do not duplicate OS services, window manager, installer or HAL in ecosystem. |
| ATCLang/toolchain | Canonical repo/path must be source-verified | Do not invent a second language, compiler, ATCB or runtime specification. |
| Aurora AI | Canonical repo/path must be source-verified | Do not create duplicate agent/model runtimes in ecosystem. AI does not acquire kernel/consensus authority. |
| Standards | `A-TownChain-Okosystems/atc-standards` | Proposed catalog contracts are not normative until registered and approved. |
| Ecosystem catalog | `A-TownChain-Okosystems/a-townchain-ecosystem` | Own catalog, cross-repo mapping, integration/compliance/evidence and drift control only. |
| Build/release/install | Per-component canonical owner; map exact workflow/artifact paths | Do not replace component-owned release pipeline with a parallel ecosystem implementation. |

## 9. Status model

Use separate fields for architecture, mapping, implementation, verification and lifecycle. A single overloaded status is insufficient.

- Architecture: `PROPOSED`, `ACCEPTED`, `DEPRECATED`, `REJECTED`.
- Mapping: `MAPPED`, `UNRESOLVED`, `EXCLUDED`.
- Implementation: `IMPLEMENTED`, `NOT_IMPLEMENTED`, `UNKNOWN`.
- Verification: `VERIFIED`, `NOT_VERIFIED`, `BLOCKED`, `RESIDUAL`.
- Catalog reconciliation: `MATCHED`, `MISSING`, `DUPLICATE`, `STALE`, `UNRESOLVED`, `EXCLUDED`.

Definitions:
- `MAPPED` means a source relationship is evidenced; it does not prove behavior.
- `IMPLEMENTED` means the implementation exists at the cited exact source revision; it does not prove conformance.
- `VERIFIED` means specified acceptance criteria passed for the exact tested source and scope.
- `BLOCKED` means a mandatory criterion or required evidence prevents acceptance.
- `RESIDUAL` means a known, explicit remaining gap; it is not equivalent to pass.
- `EXCLUDED` requires a rationale, scope and approval where governance requires one.
- Missing or inaccessible evidence is not a pass.

## 10. Evidence record

Minimum evidence fields for a verification claim:

```yaml
evidence_record:
  evidence_id: REQUIRED_STABLE_ID
  requirement_id: REQUIRED
  layer_id: REQUIRED
  sublayer_id: REQUIRED
  component_id: REQUIRED
  contract_id: REQUIRED
  repository: REQUIRED
  commit_sha: REQUIRED_FULL_40_HEX
  path: REQUIRED
  symbol: REQUIRED_OR_NOT_APPLICABLE_WITH_REASON
  blob_sha: REQUIRED_WHEN_AVAILABLE
  verification:
    workflow_run_id: REQUIRED
    job_id_or_name: REQUIRED
    step_id_or_name: REQUIRED
    exit_code: REQUIRED
    log_ref: REQUIRED
    artifact_refs: []
    artifact_hashes: []
  reviewer: REQUIRED
  reviewed_at: REQUIRED_ISO8601
  result: PASS_OR_FAIL_OR_BLOCKED
  rationale: REQUIRED
  exception_ref: OPTIONAL
```

Evidence must be inspectable, reproducible and tied to the same full commit SHA as the source mapping. A result from another branch/commit, a skipped test, missing log, absent exit code, or unverified artifact is not evidence of a pass.

## 11. Acceptance gates G0–G9

| Gate | Required decision |
|---|---|
| G0 Scope | Purpose, boundaries, non-goals, owner and applicability are explicit. |
| G1 Completeness | Requirements, sublayers, components, contracts and interfaces are inventoried or dispositioned. |
| G2 SSOT | Each production capability has one evidenced canonical owner; no unapproved duplicates. |
| G3 Contracts | Types, semantics, errors, invariants, versioning and compatibility are specified. |
| G4 Implementation | Claimed implementations map to exact source locations and revisions. |
| G5 Verification | Required positive/negative/boundary tests and evidence pass for the exact revision. |
| G6 Security | Threat model, trust boundaries, authorization, negative tests and risks are addressed. |
| G7 Integration | Dependency contracts and end-to-end behavior are evidenced. |
| G8 Operations | Observability, resource limits, lifecycle, upgrade/recovery and platform constraints are covered. |
| G9 Release | Findings are resolved or validly excepted; independent review and decision are recorded. |

Hard gates include security invariants, normative MUST requirements, unresolved consensus/protocol semantics, source ownership ambiguity for production code, and missing mandatory evidence. They cannot be offset by an average score.

## 12. Domain-specific acceptance profiles

### Blockchain, protocol and ATC-VM
- Normative transaction/signature/serialization/state-transition vectors; deterministic execution.
- Consensus and finality assumptions explicitly resolved; unknown consensus stays `BLOCKED`.
- VM bytecode format, opcode semantics, gas/resource ceilings, storage semantics and error behavior.
- Invalid inputs, replay, fork/reorg, network partitions and migration behavior tested where applicable.
- Cross-chain messages include source/destination chain identity, nonce/message identity, proof and finality semantics before acceptance.

### GlobusOS and ShivaCore
- Least privilege, capability boundaries, IPC validation and negative authorization tests.
- Boot chain, firmware requirements, trust anchors and platform profile evidence.
- Memory isolation, service/kernel boundary, HAL and driver contracts.
- Install, upgrade, interrupted install, boot failure, recovery and data migration paths.
- Intel/AMD/QEMU or other platform claims limited to profiles actually tested.

### ATCLang and build/toolchain
- Language feature matrix tied to parser, semantic analysis, diagnostics, code generation and artifact format.
- Positive and negative conformance tests; deterministic outputs where required.
- Bytecode/runtime ABI compatibility; no duplicate competing ATCB format.
- Unknown semantics and incomplete feature coverage block a claim of full language conformance.

### Aurora AI
- Model/tool calls are policy-, capability- and approval-gated; privilege boundaries cannot be bypassed through prompt injection or tool chaining.
- Model, dataset, tool, runtime and backend versions have provenance.
- Sandbox isolation, output validation, cancellation, offline/local/external backend behavior and fallback are tested.
- Wallet and other privileged actions follow explicit authorization/confirmation and audit requirements.
- Audit does not unnecessarily expose prompts, secrets or private data.

### Ecosystem governance
- Stable IDs, no duplicate canonical owners, referential integrity and stale mapping detection.
- Exclusions and exceptions carry rationale and approval evidence.
- Live drift inventory uses the registered repository manifest; `demo-repository` excluded.
- Aggregated verification status must never overstate component-level evidence.

## 13. Reconciliation work queue

This catalog must be completed through evidence-backed reconciliation, not by assuming the architecture tables are exhaustive.

1. **Registry reconciliation:** compare all existing L0–L9 and X1–X36 records with existing catalog files and issues; preserve established IDs and identify duplicates/conflicts.
2. **Sublayer reconciliation:** enumerate each layer's actual/proposed sublayers; mark each `MAPPED`, `MISSING`, `UNRESOLVED` or justified `EXCLUDED`.
3. **Component census:** inspect canonical source repositories and manifests; map component IDs to exact repository/path/symbol/blob/full commit SHA.
4. **Function census:** inventory public functions, internal runtime contracts, APIs, IPC, events, build/runtime interfaces and protocol operations; exclude tests/docs/generated/CI from runtime inventory only with explicit reasons and maintain their evidence links separately.
5. **Contract normalization:** reconcile signatures, types, units, errors, invariants, compatibility and versioning against source and normative standards.
6. **Dependency graph:** record directed dependency edges, edge type, interface/contract, version constraints and forbidden/cyclic edges.
7. **Evidence linkage:** attach exact run/job/step, exit code, logs, artifacts and hashes to each claim.
8. **Gap and duplicate triage:** produce counts and stable IDs for missing, duplicate, stale and unresolved records; do not silently drop unresolved entries.
9. **Validator coverage:** validate schema, IDs, parent-child links, SSOT uniqueness, reference integrity, source SHA format, status transitions and evidence completeness.
10. **Layer acceptance:** run G0–G9 per contract → component → sublayer → layer → system; record independent decision.
11. **Change governance:** register normative rules in `atc-standards` through approved change control; this catalog references rather than duplicates normative standards.
12. **Reporting:** publish inventory totals and residuals by status, layer, repository and risk; every count must be reproducible from machine-readable artifacts.


- [ ] Close former L10 migration: X35 owns deployment/release/rollout governance; X36 owns integration/system composition/end-to-end compatibility. Do not create an L10 alias.
- [ ] Reconcile Issue #71 composite “L0 System & Network” through the approved semantic crosswalk; preserve historical provenance and do not reuse legacy numeric IDs.

## 14. Definition of catalog completeness

The catalog is complete for a defined revision only when all the following are true:

- Every L0–L9 and X1–X36 record exists with stable ID, owner, scope, boundary, status and applicability.
- Every applicable sublayer is present or explicitly excluded with rationale.
- Every discovered component and function/interface contract is represented or has a traceable exclusion.
- Every required contract has typed semantics, errors, invariants, versioning and acceptance criteria.
- Every production capability has one canonical SSOT mapping or an explicit `UNRESOLVED` record; no unresolved record is hidden by aggregate counts.
- Every dependency edge and relevant cross-layer relationship is represented.
- Each implementation/verification status is supported by source/evidence appropriate to that status.
- Schema and graph validators run against the exact catalog revision and their logs/results are retained.
- Counts reconcile across the machine-readable registry, human-readable views and generated reports.
- Review decision names the exact catalog commit SHA and remaining accepted residuals.

**Important:** a catalog can be structurally complete while many components remain `NOT_IMPLEMENTED`, `UNRESOLVED` or `NOT_VERIFIED`. These are separate dimensions.

## 15. Current disposition

- Layer scope index: recorded here for L0–L9 and X1–X36.
- Full sublayer/component/function census: `NOT VERIFIED`; requires comparison with the live catalog and canonical source trees.
- Canonical source mapping: ownership rules recorded; per-function source references require verification.
- Contract conformance and CI evidence: `NOT VERIFIED`.
- Normative status: this file is a proposed catalog artifact; it does not replace an approved `atc-standards` change.
- Acceptance: `BLOCKED` until reconciliation, validators, evidence review and approval are complete.

## 16. Related work

- Issue #182 — consolidate the full layer, sublayer, component and function architecture tree.
- Issue #183 — define acceptance criteria for all architecture layers and sublayers.
- Issue #112 — full source-code comparison for component/function catalog.
- Issues #146–#181 — X1–X36 layer-specific work items; reconcile each rather than duplicating its scope.
- L0–L9 work items #136–#145 — reconcile against the system layer registry.

This document is an organizing artifact for the above work, not evidence that the issues are completed.


## 17. Implementation roadmap — milestones, checkpoints, sprints and TODOs

**Roadmap ID:** `ATC-ROADMAP-CATALOG-X1-X36-L0-L9`  
**Roadmap status:** `PROPOSED / NOT VERIFIED`  
**Planning cadence:** two-week sprints are a planning assumption, not a delivery commitment. Re-estimate after the baseline census.  
**Execution rule:** catalog/governance work may be performed in this repository; production implementation TODOs must be dispatched to the canonical owner repository. No production implementation is to be copied into the ecosystem repository.

### 16.1 Delivery strategy and milestone sequence

| Milestone | Target outcome | Exit checkpoint | Blocking conditions |
|---|---|---|---|
| M0 — Baseline and scope lock | Establish exact catalog baseline, source revisions, owners, existing IDs and scope. | **CP0:** baseline commit and inventory inputs are recorded; no silent ID replacement. | Missing repository manifest, unpinned source revisions or scope conflicts. |
| M1 — Registry reconciliation | Reconcile existing L0–L9 and X1–X36 records, Issues #136–#183, and existing catalog artifacts. | **CP1:** every layer has a disposition; duplicate/conflicting IDs have explicit decisions. | Unresolved ID collisions or overwritten prior work. |
| M2 — Sublayer completeness | Enumerate domain-specific sublayers and layer applicability. | **CP2:** every layer/sublayer is mapped, missing, unresolved or excluded with rationale. | Unexplained empty applicability or generic template presented as exhaustive inventory. |
| M3 — Component and function census | Inventory components, public/internal functions, APIs, ABIs, IPC, events and protocol contracts from canonical source trees. | **CP3:** reproducible counts and source-backed mappings exist for each inspected revision. | Unpinned source tree, unexplained exclusions, hidden unresolved items or duplicate SSOT. |
| M4 — Contract normalization | Define behavior, types, errors, invariants, limits, versioning and acceptance tests. | **CP4:** each in-scope contract has a complete contract record or explicit blocked/unresolved disposition. | Ambiguous normative semantics, especially security, consensus, serialization or VM behavior. |
| M5 — Dependency and interface graph | Build cross-layer, component, interface and dependency edges. | **CP5:** graph validates referential integrity, ownership, version constraints and prohibited cycles. | Missing edge endpoint, inferred ownership or unresolved critical dependency. |
| M6 — Validators and evidence pipeline | Implement catalog schema/graph/evidence validators and reproducible reports. | **CP6:** validators run on a pinned catalog SHA; logs, exit codes and artifact hashes are retained. | Missing fail-closed behavior, untraceable artifacts or checks that can pass on missing data. |
| M7 — Canonical implementation waves | Dispatch prioritized contract gaps to their canonical repositories and track implementation PRs. | **CP7:** each claimed implementation links to owner-repo PR/commit and required tests; catalog tracks residuals. | Duplicate implementations, source drift, unsafe scope, missing owner or absent acceptance criteria. |
| M8 — End-to-end acceptance and release decision | Run layer/system acceptance, independent review and residual-risk disposition. | **CP8:** G0–G9 decisions are recorded against exact SHAs; accepted residuals and blocked items are explicit. | Any hard gate unresolved, missing mandatory evidence, or aggregate status overstating component evidence. |

Milestones are sequential where a dependency is hard; independent layer censuses can run in parallel after M0, provided they use pinned source revisions and shared stable-ID rules.

### 16.2 Sprint plan

Sprint numbers are sequencing labels. Sprint length is provisionally two weeks; do not attach calendar dates or promise completion until capacity, repository access and dependencies are confirmed.

| Sprint | Focus | Definition of done |
|---|---|---|
| S0 — Setup and baseline | M0; pin catalog branch/commit, source manifests, current registry exports, ID policy and owner map. | Reproducible baseline manifest exists; all inputs have source/revision metadata; exclusions are recorded. |
| S1 — Layer and issue reconciliation | M1; compare L0–L9 and X1–X36 with current issues/catalog files. | Reconciliation table includes existing ID, canonical record, conflict, disposition, owner and evidence. |
| S2 — Sublayer and applicability matrix | M2; enumerate sublayers and cross-cutting applicability per L-layer. | No blank status is interpreted as N/A; every absence has a reason or remains unresolved. |
| S3 — Source census wave A | M3; prioritize L1–L4 and their applicable X-layer contracts. | Pinned repository snapshots, component/function inventories and reproducible totals published. |
| S4 — Source census wave B | M3; prioritize L5–L7, including blockchain, ATC-VM and ATCLang. | Canonical mappings and explicit unresolved protocol/language/VM semantics recorded; no consensus assumptions invented. |
| S5 — Source census wave C | M3; prioritize L8–L9, Aurora AI and applications; track install/release/operations through X35/X36. | Model/tool/approval boundaries and install/release ownership are mapped or explicitly unresolved. |
| S6 — Contract normalization | M4; normalize high-risk contracts first, then remaining API/ABI/IPC/event/state contracts. | Contract schema passes; testable acceptance criteria and negative/error cases exist. |
| S7 — Dependency graph | M5; add dependency edges, interface versions, security boundaries and compatibility links. | Graph validator identifies no untriaged dangling edge, duplicate owner or prohibited cycle. |
| S8 — Validator implementation | M6; implement schema, ID, referential integrity, SSOT uniqueness, stale SHA and evidence completeness checks. | Pinned run includes exact logs, exit codes and artifacts; missing mandatory data fails closed. |
| S9 — Priority implementation wave | M7; select P0/P1 gaps from evidence-backed risk ranking and create owner-repo work items. | Every work item has canonical repository, contract IDs, acceptance criteria, dependency links and test plan. |
| S10 — Integration and acceptance | M8; run cross-layer and end-to-end checks, independent review, residual triage and release decision. | G0–G9 decisions are evidence-linked; blocked items remain blocked; no unsupported VERIFIED status. |

**Sprint boundary rule:** at sprint close, report planned/done/blocked/deferred items, exact catalog/source SHAs, changed IDs, test evidence, residuals, and next-sprint dependencies. A sprint is not complete solely because its documentation was written.

### 16.3 Checkpoints and required review packet

Each checkpoint must produce a review packet containing:

- checkpoint ID, date, accountable reviewer and decision;
- exact catalog commit SHA and each source repository's full commit SHA;
- changed/new/retired IDs and before/after inventory counts;
- sublayer/component/contract coverage and explicit exclusions;
- dependency changes and SSOT ownership conflicts;
- validator/test Run → Job → Step references, exit codes, logs and artifact hashes;
- unresolved items, risk severity, owner, blocker and next action;
- decision: `PASS`, `PASS_WITH_ACCEPTED_RESIDUALS`, or `BLOCKED`, with rationale.

Checkpoint approval does not promote unrelated records. Status promotion is scoped to the exact IDs, source revisions and evidence reviewed.

### 16.4 Prioritization and work-item rules

Prioritize work in this order, subject to evidence-backed risk review:

1. **P0 — Safety/security/correctness blockers:** capability bypass, trust-boundary failure, consensus/protocol ambiguity, nondeterminism, unsafe install/update/recovery, signing/provenance gaps, privileged AI actions without policy/approval, or missing hard-gate evidence.
2. **P1 — SSOT and contract integrity:** duplicate ownership, incorrect source mappings, broken API/ABI/IPC/event contracts, serialization/version conflicts, state consistency and critical dependency gaps.
3. **P2 — Verification and operations:** missing negative/fault-injection tests, observability, recovery, portability matrix, reproducible builds and operational readiness.
4. **P3 — Optimization and completeness improvements:** performance tuning, non-blocking documentation, ergonomics and low-risk maintainability work.

Do not assign severity based on layer number or convenience. Every priority needs a reason, impact, likelihood/uncertainty, owner and acceptance test.

Every TODO must be a trackable item with:
- `todo_id`, title, priority and status;
- parent milestone/sprint/checkpoint and affected layer/sublayer/component/contract IDs;
- canonical repository and exact path when known, otherwise `UNRESOLVED`;
- accountable owner or `UNASSIGNED`;
- prerequisite/dependency IDs and blocker;
- implementation scope and explicit non-goals;
- acceptance criteria and required positive/negative/boundary tests;
- evidence expected, exact source revision after execution, and reviewer;
- estimate only when a responsible team has assessed it;
- closure rule and linked PR/commit/issue.

### 16.5 Initial TODO backlog

These are proposed backlog entries, not assertions that the work has started or completed.

| TODO ID | Priority | Work item | Depends on | Completion evidence |
|---|---|---|---|---|
| TODO-CAT-001 | P0 | Pin catalog baseline, repository manifest and source SHAs; exclude `demo-repository` from live drift gate. | M0 | Manifest and exact revisions retained; exclusion test proves demo repo is not a live gate input. |
| TODO-CAT-002 | P1 | Reconcile existing layer records, issue references and stable-ID collisions without deleting established IDs. | TODO-CAT-001 | Machine-readable reconciliation report and reviewer decision. |
| TODO-CAT-003 | P1 | Produce complete L0–L9 × X1–X36 applicability matrix with rationale and owners. | TODO-CAT-002 | Every relationship has applicable/not-applicable/unresolved disposition and provenance. |
| TODO-CAT-004 | P1 | Enumerate domain-specific sublayers for every layer; distinguish template families from actual exhaustive inventory. | TODO-CAT-003 | Sublayer registry with stable IDs, parent links, boundaries and dispositions. |
| TODO-CAT-005 | P1 | Run source census for each canonical repository and record component/function/API/ABI/IPC/event inventories. | TODO-CAT-001, TODO-CAT-004 | Reproducible per-repository reports tied to full commit SHAs and paths. |
| TODO-CAT-006 | P0 | Triage unknown security, consensus, transaction, serialization, VM and ATCLang semantic decisions. | TODO-CAT-005 | Decision records or explicit `BLOCKED` contracts; no semantics inferred from names/docs alone. |
| TODO-CAT-007 | P1 | Normalize contract schema and acceptance criteria; link requirements to source and tests. | TODO-CAT-005, TODO-CAT-006 | Schema-valid contract records and requirement-to-test traceability. |
| TODO-CAT-008 | P1 | Generate dependency/interface graph and detect dangling edges, duplicate SSOT and cycles. | TODO-CAT-003, TODO-CAT-007 | Reproducible graph artifact and triaged validator findings. |
| TODO-CAT-009 | P0 | Implement fail-closed schema, ID, source-SHA, SSOT and evidence validators in the appropriate catalog/governance location. | TODO-CAT-007, TODO-CAT-008 | Pinned workflow evidence with non-zero failure tests for deliberately invalid fixtures. |
| TODO-CAT-010 | P0 | Define and execute evidence packet generation with Run/Job/Step, exit code, logs and artifact hashes. | TODO-CAT-009 | Evidence records validate and can be independently reproduced. |
| TODO-CAT-011 | P0 | Create owner-repository implementation issues for accepted P0/P1 gaps; avoid production code in ecosystem. | TODO-CAT-006, TODO-CAT-009 | Each issue links contract IDs, canonical owner, acceptance tests and dependencies. |
| TODO-CAT-012 | P1 | Run cross-layer integration and domain-specific acceptance profiles; document residual risks. | TODO-CAT-010, TODO-CAT-011 | G0–G9 results, exact SHAs, reviewer decisions and explicit residuals. |
| TODO-CAT-013 | P1 | Reconcile human-readable catalog counts with machine-readable registries and generated reports. | TODO-CAT-005, TODO-CAT-009 | Count parity report and deterministic regeneration instructions. |
| TODO-CAT-014 | P1 | Prepare independent release/acceptance review for the exact catalog revision. | TODO-CAT-012, TODO-CAT-013 | Signed/attributed review decision; no unresolved hard gate presented as accepted. |

### 16.6 Roadmap reporting dashboard

At each sprint review, publish these measures with numerator, denominator, scope, source revision and generation method:

- layer registry coverage: reconciled layers / 46 total layer IDs;
- sublayer disposition coverage: dispositioned sublayers / discovered or proposed sublayers;
- component mapping coverage: source-mapped components / discovered components;
- function-contract coverage: represented contracts / discovered in-scope functions and interfaces;
- SSOT resolution: capabilities with evidenced canonical owner / in-scope capabilities;
- dependency graph coverage: validated edges / discovered edges;
- contract acceptance coverage: contracts passing required gates / contracts in acceptance scope;
- evidence completeness: claims with complete evidence packet / claims requiring verification;
- validator coverage: required invariants checked / required invariants;
- open blockers and accepted residuals by priority, layer, repository and owner.

Counts must never conflate `UNRESOLVED`, `EXCLUDED`, `NOT_IMPLEMENTED` and `VERIFIED`. Publish raw totals and the exact query/script or artifact used to compute them; do not report a percentage without its denominator.

### 16.7 Roadmap governance

- This roadmap is a planning baseline, not a delivery guarantee or normative standard.
- Dates, estimates, assignees and sprint capacity remain uncommitted until explicitly agreed.
- Changes to scope, acceptance gates, normative semantics or canonical ownership require traceable review and, where applicable, approved `atc-standards` change control.
- Work must remain in Draft until the relevant review and evidence criteria are met; no merge or release is implied by a milestone table.
- The current disposition remains `PROPOSED / NOT VERIFIED`; all milestones are open until checkpoint evidence is reviewed.


## 18. Catalog Wiki — chapter index and navigation

**Wiki ID:** `ATC-ARCH-CATALOG-WIKI-X1-X36-L0-L9`  
**Status:** `PROPOSED / NOT VERIFIED`  
**Canonical source:** this master catalog and the chapter files linked below. The Wiki is a navigation/documentation view, not a second implementation SSOT or a normative standards registry.

### Chapter index

| Chapter | Subject | Intended reader / use |
|---|---|---|
| [00 — Home and navigation](wiki/ATC-ARCH-WIKI-00-Home.md) | Purpose, principles, status, navigation and contribution rules | All contributors and reviewers |
| [01 — Architecture layers](wiki/ATC-ARCH-WIKI-01-Layers.md) | L0–L9 system layers and X1–X36 cross-cutting layers | Architecture owners |
| [02 — Sublayers, components and contracts](wiki/ATC-ARCH-WIKI-02-Contracts.md) | Hierarchy, metadata, stable IDs, function-contract schema and exclusions | Catalog/source-census contributors |
| [03 — SSOT and ownership](wiki/ATC-ARCH-WIKI-03-SSOT.md) | Canonical repositories, ownership boundaries and duplicate prevention | Implementers and maintainers |
| [04 — Roadmap and delivery](wiki/ATC-ARCH-WIKI-04-Roadmap.md) | Milestones M0–M8, checkpoints CP0–CP8, sprints S0–S10 and TODO-CAT-001–014 | Workstream leads |
| [05 — Verification and evidence](wiki/ATC-ARCH-WIKI-05-Evidence.md) | Status model, evidence packets, G0–G9 and hard gates | Reviewers and release decision owners |
| [06 — Governance and change control](wiki/ATC-ARCH-WIKI-06-Governance.md) | Approval, normative standards, exceptions, residuals and release discipline | Governance owners |

### Wiki maintenance rules

1. Keep the master catalog as the architecture registry and source of cross-layer scope; chapter pages explain and navigate that registry.
2. Do not copy production implementation details into the ecosystem repository as a competing implementation.
3. Link every claim of implementation or verification to exact source revisions and evidence.
4. Treat missing facts as `UNRESOLVED`; use `EXCLUDED` only with a reason and required approval.
5. Update chapter links and this index in the same pull request when a chapter is added, renamed or retired.
6. Do not label the Wiki complete until all chapter links resolve and the chapter-to-registry consistency check passes.
7. Normative requirements become authoritative only through approved `atc-standards` change control.

The files are stored as version-controlled Markdown chapters in the catalog repository so changes can be reviewed with the same pull-request and commit evidence. They are a **Wiki-style chapter set**, not proof that GitHub's separate repository Wiki feature has been published or enabled.


## 19. Implementation order — dependency-driven execution sequence

**Status:** `PROPOSED / NOT VERIFIED`. This sequence defines prerequisite order, not calendar commitments or proof that any task has completed. The roadmap is dependency-driven: do not start broad production implementation while scope, ownership or acceptance criteria remain ambiguous.

### Ordered execution

| Order | Stage | Required work | Exit condition / checkpoint |
|---:|---|---|---|
| 0 | Freeze baseline | Pin catalog branch/commit, repository manifest, active/archived scope and exclusion rules; exclude `demo-repository` from live drift | CP0: baseline reproducible; source revisions recorded |
| 1 | Reconcile architecture registry | Reconcile L0–L9 and X1–X36 IDs, existing issues, duplicates, names, boundaries and ownership | CP1: every registry record has an explicit disposition |
| 2 | Build applicability and sublayer map | Construct L-layer × X-layer applicability matrix; enumerate sublayers; document N/A rationales and unresolved cells | CP2: no blank cells masquerade as non-applicability |
| 3 | Run canonical source census | Inventory components, symbols, interfaces, functions, tests and generated/runtime distinctions from canonical repositories | CP3: every candidate mapped, excluded with rationale, or unresolved |
| 4 | Resolve SSOT and risk blockers | Confirm canonical owner/path and flag ambiguous protocol, consensus, security, VM, language and AI-runtime semantics | CP4: no guessed production ownership; hard blockers visible |
| 5 | Normalize contracts | Define typed inputs/outputs, behavior, invariants, errors, side effects, limits, compatibility and acceptance tests | CP5: contracts meet the catalog schema and have testable criteria |
| 6 | Build dependency/interface graph | Map calls, interfaces, data flows, trust boundaries, version constraints and cross-layer dependencies | CP6: graph edges are source/evidence-backed or explicitly unresolved |
| 7 | Implement validation and evidence tooling | Add deterministic catalog validators, duplicate/SSOT/drift checks, evidence schema and fail-closed reporting | CP7: negative tests prove missing/stale evidence is rejected |
| 8 | Execute implementation waves | Create prioritized tasks in canonical owner repositories; implement in dependency order; no duplicate ecosystem implementation | Per-wave gate: exact source SHA, tests, artifacts and review |
| 9 | Integrate and accept end-to-end | Run cross-layer compatibility, security, portability, lifecycle and recovery checks; triage residuals | CP8: independent PASS, PASS_WITH_ACCEPTED_RESIDUALS, or BLOCKED decision |

### Implementation wave ordering

1. **Wave A — foundation and trust:** boot/HAL, identity/capabilities, security boundaries, build provenance and core test/evidence plumbing (L1–L2 with relevant X-layers).
2. **Wave B — OS substrate:** kernel integration, OS services, storage, IPC, configuration, update/lifecycle and resource controls (L2–L3).
3. **Wave C — blockchain and execution:** protocol/cryptography/state, blockchain core and canonical ATC-VM, with security and deterministic-execution gates (L5–L6).
4. **Wave D — developer toolchain:** ATCLang and build/artifact pipeline, only after its source inventory and language semantics are reconciled (L7).
5. **Wave E — AI platform:** Aurora control plane, policy/capability mediation, sandboxed runtime, model adapters and approval/audit paths; AI must not acquire kernel, consensus or wallet authority implicitly (L8).
6. **Wave F — user-facing integration:** desktop/window manager, applications and ecosystem services against accepted OS/service contracts (L4 and L9).
7. **Wave G — release composition:** installer and platform profiles, upgrade/rollback, supply-chain provenance, observability, disaster recovery and full-system compatibility (X35/X36 and X1–X36).

Wave order is a default dependency sequence, not a claim that every feature in a layer is blocked on every prior feature. Independent work may proceed in parallel only after its contract, owner, interfaces and acceptance criteria are stable. Security, identity, evidence and resource governance remain cross-cutting gates throughout.

### Stop/go rules

- Stop if canonical owner, interface semantics, security invariants or normative requirement is unresolved and affects correctness.
- No implementation task is marked complete from a documentation change alone.
- No gate passes without exact revision, reproducible test/evidence references and explicit result.
- Failed validators, missing artifacts, stale SHAs and empty required evidence fail closed.
- Re-plan only through recorded changes to dependencies, scope and checkpoint decision; do not silently reorder gates.
