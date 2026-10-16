# ATC-ECOSYSTEM-LAYER-ARCHITECTURE-001 — Complete Layer Architecture for the A-TownChain Ecosystem

> **Document ID:** ATC-ECOSYSTEM-LAYER-ARCHITECTURE-001
> **Version:** 0.5.0 (DRAFT — reconciled to adopted L0–L9 + X1–X36 baseline)
> **Date:** 2026-10-16
> **Authority:** A-TownChain-Okosystems/a-townchain-ecosystem
> **Registry:** [ATC-LAYER-REGISTRY-001.yaml](layers/ATC-LAYER-REGISTRY-001.yaml)
> **Mapping:** [ATC-LAYER-MAPPING-001.yaml](layers/ATC-LAYER-MAPPING-001.yaml)
> **Catalog:** [ATC-COMPONENT-FUNCTION-CATALOG-001.json](ATC-COMPONENT-FUNCTION-CATALOG-001.json)

---

## Purpose

This document defines the canonical hierarchical layer and sub-layer structure for the entire A-TownChain ecosystem. It covers all systems, components, functions, interfaces, permissions, dependencies, and installation variants.

The goal is a complete architecture from hardware layer through kernel, operating system, blockchain, and AI to the user interface, supplemented by security, governance, development, and verification.

**Fundamental principle:** Each layer describes one responsibility. Every component has a unique layer assignment, a canonical repository as Single Source of Truth (SSOT), and traceable function contracts. The ecosystem catalog documents and controls these assignments rather than creating parallel implementations.

**Evidence classification (normative):** Architecture and status claims in the ATC ecosystem MUST be classified according to the four-tier evidence classification: `NORMATIVE` (approved standard, §9) → `IMPLEMENTED` (proven functional with tests/evidence) → `RESEARCH` (candidate, not a roadmap fact) → `LEGACY/UNVERIFIED` (old documentation claim without evidence).

---

## 1. Complete Layer Overview

### System Layers (L0-L9; adopted architecture baseline)

| ID | Layer | Responsibility | Components |
|----|-------|----------------|------------|
| L0 | Hardware & Compute | CPU, GPU, NPU, RAM, Storage, TPM, TEE, Firmware | COMP-COMPUTE |
| L1 | Boot, Firmware & HAL | UEFI, Secure Boot, ACPI, HAL, Drivers | (UNANALYZED) |
| L2 | ShivaCore Kernel & TCB | Memory, Scheduling, IPC, Capabilities, Isolation | COMP-SHIVA |
| L3 | GlobusOS OS | System services, FS, Network, Processes, Devices | COMP-GLOBUS |
| L4 | Desktop, WM & UI | Shell, Compositor, Windows, Launcher, Settings, A11y | (UNANALYZED) |
| L5 | A-TownChain Blockchain | Node, Consensus, Network, Tx, Ledger, ATC-Token | COMP-CHAIN, COMP-ALGO, COMP-NODE |
| L6 | ATC-VM & Smart Contracts | Bytecode, Execution, Gas, State, Storage, Contract APIs | COMP-VM, COMP-CONTRACTS |
| L7 | Developer Platform & ATCLang | Compiler, Lexer, Parser, Semantics, SDK, CLI, Toolchain | COMP-LANG, COMP-TOOLCHAIN, COMP-SDK, COMP-IDE |
| L8 | Aurora AI Platform | Control Plane, Agent Platform, Runtime, Intelligence Fabric, Models | COMP-AURORA |
| L9 | Applications & Ecosystem | Wallet, dApps, Explorer, Governance, System & AI Apps | COMP-GENESIS, COMP-CHRONICLES, COMP-GFF, COMP-MARKET, COMP-DEMO |

### Cross-Cutting Layers (X1-X36)

| ID | Layer | Responsibility |
|----|-------|----------------|
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

The adopted system architecture has exactly ten system layers (L0–L9) and 36 cross-cutting layers (X1–X36). Former L10 responsibilities are distributed by responsibility, not by numeric substitution: X9 covers platform compatibility; X35 covers installer/release/deployment/rollout governance; X36 covers composition, dependency closure and end-to-end compatibility; X14 covers configuration/secrets; X7 covers observability/reliability; X20/X21 cover incident response and recovery; X15 covers lifecycle/migration; X3 covers protocol/schema interoperability. This mapping is a responsibility crosswalk, not proof that the corresponding implementation contracts exist.

The layer IDs and names in this section are sourced from [ATC-LAYER-REGISTRY-L1-L10-X1-X36-001.json](ATC-LAYER-REGISTRY-L1-L10-X1-X36-001.json). The registry remains proposed/unresolved and does not itself establish implementation or verification status.
---

## 2. Sub-Layers and Function Hierarchy

Each main layer has a deeper structure. The following hierarchy applies to the complete component and function catalog:

| Level | Content | Example |
|-------|---------|---------|
| 0 | Haupt-Layer | Hardware & Compute |
| 1 | Sub-Layer | GPU Compute |
| 2 | Komponente | GPU Driver Manager |
| 3 | Subkomponente | Device Discovery |
| 4 | Funktionsvertrag | `discover_devices()` |
| 5 | Schnittstellenvertrag | Inputs, Outputs, Errors, Permissions |
| 6 | Implementierung | Exact repository path and symbol |
| 7 | Verifikation | Tests, Workflow, Run/Job/Step, Exit-Code, Artifacts |

**Important:** These levels are catalog hierarchy and evidence chain, NOT seven additional operating system layers.

The legacy sub-layer registry at [ATC-LAYER-REGISTRY-001.yaml](layers/ATC-LAYER-REGISTRY-001.yaml) contains an earlier L0-L10/X1-X8 decomposition. It is historical input, not the authoritative definition of the adopted L0-L9/X1-X36 matrix; sub-layer reconciliation remains open.

---

## 3. Catalog Metadata Schema

Every component and function MUST have at least the following fields:

### Identity
- **Stable ID** (e.g., `COMP-SHIVA`, `FN-SHIVA-001`)
- **Name** (canonical name)
- **Description** (what it does)
- **Version** (semantic version)
- **Type** (component, sub-component, function, interface, implementation)
- **Status** (one of the binding status values below)

### Architecture
- **Primary Layer** (L0-L9 or X1-X36)
- **Sub-Layer** (e.g., `L2.4` for Capabilities & Sandboxing)
- **Parent-ID** (for sub-components)
- **Child-IDs** (for parent components)
- **Responsibility Scope** (what the component is responsible for)

### SSOT
- **Canonical Repository** (e.g., `A-TownChain-Okosystems/globus-os`)
- **Path** (e.g., `modules/atc-shivacore/kernel/`)
- **Symbol** (function/class name)
- **Commit-SHA** (exact commit)
- **Blob-SHA** (exact file blob)

### Contracts
- **Signature** (function signature)
- **Inputs** (expected inputs)
- **Outputs** (expected outputs)
- **Errors** (error conditions)
- **Invariants** (must-remain-true conditions)
- **Preconditions** (must-be-true-before)
- **Postconditions** (must-be-true-after)

### Security
- **Permissions** (required capabilities)
- **Capabilities** (granted capabilities)
- **Trust Boundary** (what trust level is required)
- **Threats** (relevant threats)
- **Audit Requirements** (required audits)

### Dependencies
- **Prerequisites** (what must be installed/running)
- **Dependencies** (other components this depends on)
- **Callers** (what calls this component)
- **API Versions** (protocol/API versions)
- **Protocol Versions** (communication protocol versions)

### Platform
- **Installation Models** (A/B/C/D, at least one)
- **GlobusOS Native** (yes/no)
- **Windows** (compatible as host application: yes/no)
- **Linux** (compatible as host application: yes/no)
- **Architecture** (x86_64, aarch64)
- **CPU/GPU/NPU** (hardware requirements)
- **Boot Mode** (native, hosted, container, VM)
- **Install Mode** (bare-metal, package, container, bundle)

### Evidence
- **Test References** (test files and test names)
- **CI Runs** (workflow run IDs, all bound to exact SHA)
- **Artifacts** (build artifacts and their hashes)
- **Verification Result** (pass/fail with evidence)
- **Open Gaps** (known gaps in verification)

---

## 4. Binding Status Values

| Status | Meaning |
|--------|---------|
| `MAPPED` | Entry is assigned to a source or contract. |
| `IMPLEMENTED` | Implementation is proven by concrete sources. |
| `VERIFIED` | Defined verification criteria are met with traceable evidence. |
| `RESIDUAL` | Open gaps or incomplete requirements exist. |
| `UNRESOLVED` | Assignment or meaning is not yet clarified. |
| `BLOCKED` | Verification or implementation is blocked by a specific obstacle. |
| `EXCLUDED` | Deliberately excluded, with documented justification. |

**Fail-closed rule:** A source code entry alone MUST NOT receive the status `VERIFIED`.

---

## 5. Installation and Execution Models

The catalog MUST represent the same components across different deployment forms without duplicating their implementations.

| Model | Name | Description | Applies to |
|-------|------|-------------|------------|
| A | Native System | Hardware → Boot → ShivaCore → GlobusOS → Desktop → A-TownChain → Aurora AI | All system layers (L0-L9) |
| B | Windows Application | Native Windows integration, host-OS abstraction, process isolation, compatible runtime | L5+ only |
| C | Linux Application | Linux host adapters, process/FS integration, supported runtime | L5+ only |
| D | Development & Test | Cross-compilation, emulators, VMs, local nodes, test networks, CI | All layers |

**Critical distinction:** The native kernel architecture and the host application architecture MUST be specified separately. A kernel subsystem is NOT automatically executable as a normal Windows or Linux application.

**Validation rule (VAL-LR-005, fail-closed):** Components assigned to L0-L4 MUST NOT claim Windows or Linux installation models. Only L5+ components can run as host applications via OS abstraction.

---

## 6. Responsibilities and SSOT Assignment

| Architecture Area | Canonical Source |
|-------------------|-----------------|
| Blockchain Core | a-townchain-os |
| ATC-VM | a-townchain-os/components/vm |
| Consensus & Algorithms | a-townchain-os/components/algorithm |
| ShivaCore Kernel | globus-os/modules/atc-shivacore/kernel/ |
| ATCLang | atclang (canonical Rust implementation) |
| Aurora AI | aurora-ai (canonical; unassigned parts marked open) |
| Normative Standards | atc-standards |
| Ecosystem Catalog, Integration Control & Compliance | a-townchain-ecosystem |

For components without a proven canonical source, the catalog MUST declare `UNRESOLVED` or `BLOCKED`, not invent a new implementation location.

---

## 7. Implementation Sequence

1. **Legacy layer registry exists** — The earlier L0-L10/X1-X8 decomposition is retained as historical input; the adopted matrix is L0-L9/X1-X36 and is tracked in [ATC-LAYER-REGISTRY-L1-L10-X1-X36-001.json](ATC-LAYER-REGISTRY-L1-L10-X1-X36-001.json).

2. ✅ **Existing catalog mapped** — All 30 existing components assigned to layers without duplicating entries. See [ATC-LAYER-MAPPING-001.yaml](layers/ATC-LAYER-MAPPING-001.yaml).

3. **Gap analysis (NEXT)** — Identify missing components, function contracts, APIs, platform adapters, UI elements, and security controls per layer. Layers L1 (Boot/Drivers), L4 (Desktop/UI), X2 (Identity), X3 (Protocol), X7 (Observability), X8 (Build/Supply Chain) have no primary components yet.

4. **SSOT and evidence linkage** — Capture exact repositories, commit/blob SHAs, symbols, tests, and CI evidence for every entry (ongoing via functional verification passes).

5. **Dependency graph** — Document component, API, security, and build dependencies with explicit edges.

6. **Drift and completeness checks** — Fail-closed treatment of missing IDs, invalid parent-IDs, duplicate implementations, unresolved symbols, and missing evidence. Implemented in [validate_layer_registry.py](../../tools/validate_layer_registry.py).

**Acceptance criterion:** The catalog is complete only when every relevant layer and sub-layer is inventoried, every known component and function is assigned, every exception is justified, and every claimed implementation or verification is supported by exact evidence.

---

---

## 8. Layer Details

### L0 — Hardware & Compute (`ATC-LAYER-L0-HARDWARE-COMPUTE`)

The foundation of the entire A-TownChain ecosystem: physical compute resources, hardware detection, hardware capabilities, and secure provision of CPU, GPU, NPU, memory, and security resources for ShivaCore, GlobusOS, A-TownChain, and Aurora AI.

**Scope boundary:** Firmware, boot processes, driver management, and hardware abstraction belong to L1 as higher-level responsibilities. L0 defines the physical resources and their capabilities.

**Binding principle:** L0 provides capabilities but grants NO permissions. Control of device access, DMA, memory isolation, and privileged operations is governed by the responsible kernel (L2), HAL (L1), and security (X1) contracts.

**Sub-layers (9, owner specification 2026-10-15):**

| ID | Sub-Layer | Scope |
|----|-----------|-------|
| L0.1 | CPU | x86-64, ARM64, cores, SIMD, virtualization |
| L0.2 | GPU | Graphics, parallel compute, VRAM, acceleration |
| L0.3 | NPU & AI Accelerators | Inference, matrix compute, AI acceleration |
| L0.4 | Memory | RAM, VRAM, cache, NUMA, ECC |
| L0.5 | Storage | NVMe, SSD, HDD, flash, persistent media |
| L0.6 | Security Hardware | TPM, TEE, hardware cryptography, secure elements |
| L0.7 | Connectivity & I/O | PCIe, USB, Ethernet, Wi-Fi, DMA, IOMMU |
| L0.8 | Power & Thermal | Energy, temperature, power limits, sensors |
| L0.9 | Hardware Identity & Capability Discovery | Hardware identity, device capabilities, resource inventory, capability metadata |

**Function contracts (35, Sollstruktur):** All 35 proposed function contracts (ATC-L0-CPU-001 … ATC-L0-DISC-004) are defined in [ATC-L0-FUNCTION-CATALOG-001.yaml](layers/ATC-L0-FUNCTION-CATALOG-001.yaml). Every function starts as `UNRESOLVED` — these are proposed stable catalog IDs defining function contracts, NOT existing implementations. Sub-layer coverage: L0.1 (6), L0.2 (6), L0.3 (5), L0.4 (3), L0.5 (3), L0.6 (3), L0.7 (3), L0.8 (2), L0.9 (4).

**Layer dependencies:** L0 capability dependencies to system layers L1-L8 are documented in the function catalog's `layer_dependencies` section (8 draft dependency contracts). Former L10-related concerns are not system-layer dependencies and are split across X9, X35 and X36; the exact X-layer relationships remain unresolved pending evidence.

**Acceptance criteria (7):** All nine sub-layers registered; stable IDs and defined contracts; traceable hardware dependencies to ShivaCore, GlobusOS, ATC-VM, and Aurora AI; vendor-specific characteristics separated from cross-architecture contracts; non-existent hardware classified as optional/unsupported/not-detected (never as present); security-critical functions with concrete trust boundary and verification requirement; no function receives `VERIFIED` without traceable evidence.

---

### L1 — Boot, Firmware & Hardware Abstraction (`ATC-LAYER-L1-BOOT-FIRMWARE-HAL`)

Platform initialization, boot chain of trust, firmware/hardware detection, standardized hardware contracts, and secure kernel handoff between platform and ShivaCore. L1 does not replace the kernel and implements no second driver/kernel SSOT. Firmware tables and addresses are untrusted inputs requiring validation; signature verification, measured boot, and policy enforcement are separate functions. Source: [Issue #136](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/136).

**Sub-layers (9):** L1.1 Boot Manager, L1.2 Secure Boot & Trust Chain, L1.3 Firmware Interface, L1.4 Platform Discovery, L1.5 Hardware Abstraction Layer (HAL), L1.6 Bus & Device Interface, L1.7 Boot Storage & Loading, L1.8 Platform Diagnostics, L1.9 Kernel Handoff & Platform Contract.

**Function contracts (56):** [ATC-L1-FUNCTION-CATALOG-001.yaml](layers/ATC-L1-FUNCTION-CATALOG-001.yaml). Inventory at globus-os main `3bf0001`: **14 IMPLEMENTED** (inline test refs + workspace suite 1298/1298, exit 0 — e.g. BootSequence, TrustAnchor/LoadPolicy, RSDP validation, MmioRegion, PciBus, entry_point, MemoryMap handoff, DriverFaultManager), **16 MAPPED** (source exists, test evidence unconfirmed — e.g. ModuleSignature model, version pinning, IDT/PIC init, BootInfo), **26 UNRESOLVED** with evidence-based reasons (no UEFI service consumption, no measured boot, no DMA/IOMMU capture, no kernel-image verification). No contract VERIFIED: CI evidence at binding SHA pending. Intel/AMD/QEMU test targets from Issue #136 remain pending.

---

### L2 — ShivaCore Kernel & Trusted Computing Base (`ATC-LAYER-L2-SHIVACORE-KERNEL`)

Rust `no_std` capability microkernel with minimal TCB. Security model: least privilege, no ambient authority, isolation by default, explicit IPC, fail-closed. Outside the kernel: A-TownChain-Node, ATC-VM, GlobusOS services, drivers, and Aurora AI unless a strictly privileged kernel task requires inclusion. Kernel SSOT: `globus-os/modules/atc-shivacore/kernel/`. Source: [Issue #137](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/137).

**Sub-layers (12):** L2.1 Kernel Core, L2.2 Capability Security, L2.3 Memory Management, L2.4 Scheduling & Execution, L2.5 IPC & Communication, L2.6 Interrupts & Timers, L2.7 Architecture & CPU, L2.8 Device & DMA Isolation, L2.9 Trusted Computing Base, L2.10 Faults & Recovery, L2.11 Identity & Secure Services, L2.12 Audit & Verification Hooks.

**Function contracts (75):** [ATC-L2-FUNCTION-CATALOG-001.yaml](layers/ATC-L2-FUNCTION-CATALOG-001.yaml). Inventory at globus-os main `3bf0001` (kernel SSOT directory): **47 IMPLEMENTED** (inline test refs + workspace suite 1298/1298, exit 0 — e.g. capability attenuation/revocation, VMM W^X and half-split invariants, page fault classification, deadline-aware scheduler, capability-checked IPC and syscall dispatch, fault isolation with capability revocation), **17 MAPPED** (source exists, test evidence unconfirmed — e.g. IDT/PIC init without test region, SavedRegs context structures, GDT selectors, trust anchor model), **11 UNRESOLVED** (no synchronization primitives, no shared-memory IPC, no DMA/IOMMU lifecycle, no TCB registry/documentation, no kernel panic contract). No contract VERIFIED: CI evidence at binding SHA pending; Intel/AMD/QEMU targets from Issue #137 remain pending.

---

## 9. Architecture Reference Model

This document is the layer architecture companion to:
- **ATC_ARCHITECTURE_REFERENCE_MODEL.md v0.1.0** (a-townchain-os-docs, Commit 20bcb4d) — Layer comparison model targeting the total stack (a-townchain-os = L7 integration, no core implementation)

The architecture presented here is the **proposed overall plan**. It is not yet proof that all layers are already registered in the GitHub catalog or fully implemented. Status claims follow the binding status values in Section 4 and the evidence classification defined at the top of this document.

---

## Change Log

| Version | Date | Change |
|---------|------|--------|
| 0.1.0 | 2026-10-15 | Initial draft: 11 main layers + 8 cross-cutting layers, 30 components mapped, validation tool created |
| 0.2.0 | 2026-10-15 | L0 detail added per owner spec: 9 sub-layers (L0.1-L0.9), 35 function contracts (ATC-L0-*), layer dependency matrix, capability-not-permission principle; sub-layer ID format normalized to dot notation |
| 0.4.0 | 2026-10-15 | L2 detail added per Issue #137: 12 sub-layers (L2.1-L2.12), 75 function contracts (ATC-L2-*), kernel inventory at globus-os 3bf0001 (47 IMPLEMENTED, 17 MAPPED, 11 UNRESOLVED); validator covers L0+L1+L2 catalogs (31 checks) |
| 0.3.0 | 2026-10-15 | L1 detail added per Issue #136: 9 sub-layers (L1.1-L1.9), 56 function contracts (ATC-L1-*), inventory at globus-os 3bf0001 (14 IMPLEMENTED, 16 MAPPED, 26 UNRESOLVED); validator generalized to per-layer function catalogs |
