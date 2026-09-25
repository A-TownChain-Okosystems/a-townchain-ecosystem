# Repository Architecture Contracts — Baseline

This file maps all repositories in the organization to one canonical system responsibility. Normative governance remains in `atc-standards`; this file is an architectural contract and audit index.

| Repository | Layer | Canonical responsibility | Mainnet-0 | Status baseline |
|---|---|---|---|---|
| .github | Governance | Organization-wide automation/governance hub | Required | UNVERIFIED |
| a-townchain-ecosystem | Integration | System-of-systems integration, cross-repo gates and evidence orchestration | Required | INCOMPLETE |
| a-townchain | Core | L1 orchestration and canonical blockchain monorepo | Required | UNVERIFIED |
| a-townchain-os | Integration | Cross-domain OS/L7 integration | Required where integrated | UNVERIFIED |
| a-townchain-os-docs | Governance | Architecture/documentation knowledge base | Required | UNVERIFIED |
| atclang | Core | ATCLang language/compiler boundary | Required | UNVERIFIED |
| atc-shivacore | Core | Rust/no_std kernel and TCB | Required for GlobusOS deployment | UNVERIFIED |
| atc-standards | Governance | Normative standards, registry and conformance SSOT | Required | UNVERIFIED |
| atc-engineering | Governance | Engineering control plane, fleet audit and evidence tooling | Required | INCOMPLETE |
| atc-node | Core | Historical node source; canonical runtime migrated to a-townchain/components/node | Migration source | UNVERIFIED |
| atc-vm | Core | Historical VM source; canonical runtime migrated to a-townchain/components/vm | Migration source | UNVERIFIED |
| atc-sdk | Platform | Historical SDK source; canonical implementation migrated to a-townchain/components/sdk | Migration source | UNVERIFIED |
| atc-wallet | Platform | Historical wallet source; canonical implementation migrated to a-townchain/components/wallet | Migration source | UNVERIFIED |
| atc-contracts | Core | Historical contract source; canonical implementation migrated to a-townchain/components/contracts | Migration source | UNVERIFIED |
| atc-zkp | Core | Historical ZKP source; canonical implementation migrated to a-townchain/components/zkp | Migration source | UNVERIFIED |
| atc-algorithm | Core | Historical consensus/economics source; canonical implementation migrated to a-townchain/components/algorithm | Migration source | UNVERIFIED |
| atc-storage | Core | Historical storage source; canonical implementation migrated to a-townchain/components/storage | Migration source | UNVERIFIED |
| atc-indexer | Platform | Historical indexer source; canonical implementation migrated to a-townchain/components/indexer | Migration source | UNVERIFIED |
| atc-explorer | Application | Historical explorer source; canonical implementation migrated to a-townchain/components/explorer | Expansion | UNVERIFIED |
| atc-interop | Core/Service | Historical interoperability source; canonical implementation migrated to a-townchain/components/interop | Expansion | UNVERIFIED |
| atc-oracle | Core/Service | Historical oracle source; canonical implementation migrated to a-townchain/components/oracle | Expansion | UNVERIFIED |
| atc-compute | Service | Verifiable/authorized compute platform | Expansion | UNVERIFIED |
| atc-mining | Core/Service | Historical mining/reward source; canonical implementation migrated to a-townchain/components/mining | Required where mining applies | UNVERIFIED |
| atc-marketplace | Application | Marketplace/DEX application layer | Expansion | UNVERIFIED |
| atc-launchpad | Application | Launchpad application layer | Expansion | UNVERIFIED |
| aurora-ai | Platform | AI runtime, agents, tools, memory and policy boundary | Expansion / platform | UNVERIFIED |
| globus-os | Platform | Operating system and userspace platform | Platform | UNVERIFIED |
| genesis-engine | Application/Platform | General-purpose creation engine/editor/runtime/SDK | Expansion | UNVERIFIED |
| genesis-chronicles | Application | Flagship Genesis game/application | Expansion | UNVERIFIED |
| genesis-franchise-factory | Application | Historical franchise factory source; target capability consolidated into Genesis Engine | Expansion | UNVERIFIED |
| atc-ide | Developer | ATCLang/ecosystem developer environment | Expansion | UNVERIFIED |
| demo-repository | Demo | Demonstration/test repository; not a production authority | No | UNVERIFIED |

## Contract rule

Each repository must have exactly one primary responsibility. Secondary integrations must consume canonical interfaces rather than create competing protocol authority.

## Migration rule

The standards registry is authoritative for migrations. A repository marked as migrated is not automatically a second implementation authority. Its canonical target path must be used for production ownership, and retirement/archive work must be tracked separately.

## Audit status

The initial contract baseline deliberately uses UNVERIFIED where the current implementation, tests, CI and evidence have not yet been audited against this contract. COMPLIANT must never be inferred from repository existence or documentation.

## Required per-repository audit fields

- responsibility
- dependencies
- public interfaces
- security boundary
- mainnet relevance
- implementation revision
- tests
- CI
- evidence
- definition of done
- status: MISSING / INCOMPLETE / DIVERGENT / DUPLICATED / UNVERIFIED / COMPLIANT / EXPANSION
