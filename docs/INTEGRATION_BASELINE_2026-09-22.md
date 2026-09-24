# A-TownChain Ecosystem Integration Baseline

Date: 2026-09-22

## Ownership
The ecosystem repository is the integration/release orchestrator. Canonical implementation remains in source repositories.

## Integrated architecture
Experience -> Application -> Platform -> Runtime -> Host ABI -> Syscall ABI -> ShivaCore -> HAL -> Hardware.

Blockchain:
Wallet/SDK -> API -> Mempool -> Proposer -> Block -> Validation -> Consensus/Finality -> State -> ATC-VM -> Persistence -> Restart recovery.

AI:
Aurora -> Agent Runtime -> Policy -> Capability -> Host ABI -> ShivaCore.

## Canonical repositories
- globus-os: OS/platform and active ShivaCore kernel source
- atc-shivacore: ShivaCore contracts/specifications/support
- atc-standards: normative standards SSOT
- atclang: language/compiler
- atc-vm: VM source where applicable
- a-townchain: L1 source
- aurora-ai: AI platform
- genesis-engine: engine source
- a-townchain-os-docs: documentation hub

## Readiness
This baseline is not a production-readiness claim. Every implementation status must be backed by code, tests and CI evidence.

## Required cross-repository gates
1. ABI compatibility
2. capability/policy compatibility
3. deterministic VM host boundary
4. chain-ID/signing-domain consistency
5. state/persistence/restart consistency
6. integration tests
7. security and dependency gates
8. reproducible build/evidence

## Source-of-truth rule
Do not create duplicate canonical implementations in the umbrella repository. Changes to component behavior belong in the component SSOT; the umbrella records compatibility and release evidence.
