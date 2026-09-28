# AURORA-AI-COMPUTE-PLATFORM-001

**Status:** ARCHITECTURE_ONLY  
**Authority:** `a-townchain-ecosystem` system architecture  
**Normative standards authority:** `atc-standards`  
**Implementation SSOTs:** `aurora-ai`, `a-townchain-os`, `atc-shivacore` / canonical kernel source in `globus-os`

## Purpose

Define the canonical hardware-to-AI architecture for Aurora without binding the system to a manufacturer, product family, host operating system, or external AI software stack.

The canonical hardware target is **ATC AI Compute Platform**.

This document is an architectural contract. It does not claim implementation, test, CI, security, or E2E verification.

## Canonical Layering

```
AURORA AI
  |
  +-- Aurora Authority Plane
  |     Capability / Policy / Approval / Identity / Audit / Trust
  |
  +-- Aurora Runtime
  |     Scheduler / Context / Agent Execution / Model Lifecycle
  |
  +-- ATC AI Runtime
  |     Model ABI / Graph / Tensor / Compiler / Execution / Quantization
  |
  +-- CPU / GPU / NPU backends
  |
  +-- GlobusOS AI Services
  |     Memory / RAG / Tool Runtime / AI IPC / Model Service / Storage / Network
  |
  +-- ShivaCore
  |     Kernel / IPC / Scheduler / Memory / Capabilities / Isolation / Security
  |
  +-- ATC Hardware HAL
  |     CPU HAL / GPU HAL / NPU HAL / Security HAL
  |
  +-- ATC AI Compute Platform
        CPU / GPU / NPU / Memory / Storage / TPM / Secure Processor / Firmware
```

## Hard Architecture Boundaries

### Aurora != Runtime

Aurora is the intelligent system. Runtime provides controlled execution.

```
Aurora decides
  -> Authority verifies
  -> Runtime executes
```

### Runtime != Kernel

```
ATC AI Runtime
  -> GlobusOS service
  -> ShivaCore IPC
```

AI services do not receive implicit kernel privileges.

### NPU != Trust Domain

NPU is a compute accelerator. Trust is established through the applicable secure-boot, measured-boot, hardware-security, TEE, attestation, and ShivaCore capability boundaries.

### Hardware != Manufacturer

The canonical architecture describes capabilities, interfaces, contracts, and evidence. Manufacturer and product names are not architectural dependencies.

## Canonical AI Execution Contract

```
Aurora Agent
    |
    v
Capability Request
    |
    v
Aurora Authority Plane
    |
    v
ATC Model ABI
    |
    v
Model Verification
    |
    v
ATC AI Runtime
    |
    v
Execution Planner
    +---- CPU
    +---- GPU
    +---- NPU
    |
    v
Verified Execution
    |
    v
Result Validation
    |
    v
Aurora
```

## Security Chain

```
Hardware Root of Trust
        |
        v
Secure Boot
        |
        v
Measured Boot
        |
        v
ShivaCore
        |
        v
GlobusOS
        |
        v
Aurora Runtime
        |
        v
Model Verification
        |
        v
Capability / Policy
        |
        v
AI Execution
```

## ATC AI Compute Platform

Machine-readable capability model:

```
ATC-AICP
├── CPU
├── GPU
├── NPU
├── MEMORY
├── STORAGE
├── SECURITY
│   ├── TPM
│   ├── SECURE_PROCESSOR
│   └── TEE
├── BOOT
│   ├── SECURE_BOOT
│   └── MEASURED_BOOT
└── FIRMWARE
```

A declared capability is not evidence of implementation or verification.

## Repository / SSOT Mapping

| Repository | Responsibility |
|---|---|
| `atc-standards` | Canonical contracts, governance, schemas, conformance rules |
| `aurora-ai` | Aurora Authority Plane, Runtime, ModelHub, Agent, Memory, RAG, Multimodal, Tools |
| `a-townchain-os` | GlobusOS AI services and system integration |
| `atc-shivacore` | ShivaCore kernel/TCB, capability, IPC, memory, isolation and HAL boundary |
| `globus-os` | Canonical active ShivaCore kernel implementation and hardware/boot integration |
| `a-townchain-ecosystem` | System architecture, integration and cross-repository evidence |

**Standalone First, Ecosystem Second** remains mandatory.

## Evidence Rule

```
ARCHITECTURE_ONLY
    ->
SPECIFIED
    ->
IMPLEMENTED
    ->
TESTED
    ->
CI_VERIFIED
    ->
INTEGRATED
    ->
E2E_VERIFIED
```

A file, device, driver, interface, or documented design is not by itself evidence for a higher state.

**No Evidence, No Trust.**
