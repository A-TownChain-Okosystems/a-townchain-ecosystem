# A-TownChain Ecosystem — Master Architecture

**Status:** CANONICAL ARCHITECTURE BASELINE
**Normative standards:** `A-TownChain-Okosystems/atc-standards`

## Architectural planes

### Trust / Consensus Plane

A-TownChain L1 is the authoritative trust and state-transition system. Consensus, economics, execution, ZKP and persistent state have canonical ownership and deterministic behavior.

Transaction -> Mempool/validation -> Consensus/proposer -> ATC-VM execution -> State transition -> Storage/state root -> Block validation/finality

### Language / Execution Plane

ATCLang -> ATC-IR/ABI -> ATC-VM -> canonical Rust runtime.

ATCLang is the language boundary for on-chain programs. The VM is the execution boundary. Production consensus execution must not depend on simulated or non-canonical reference behavior.

### Infrastructure Plane

ShivaCore -> GlobusOS -> userspace services. ShivaCore supplies the security-critical kernel boundary. GlobusOS owns operating-system integration and userspace platform. Kernel authority is not implicitly granted to AI or application code.

### AI Plane

Agent -> ToolRequest -> Policy/Capability validation -> Approval where required -> ToolExecutor -> Tool -> Audit.

Aurora AI operates as a userspace AI platform. Model inference, memory, tools and agents are subordinate to explicit capability and policy boundaries.

### Economic / Service Plane

L2, DeFi, Oracle, Interop, Compute, Marketplace and Launchpad extend the ecosystem. They integrate with the trust layer through explicit protocols and do not redefine L1 consensus or state-transition authority.

### Creation Plane

Genesis Engine is the canonical engine platform. Genesis Chronicles is an application/showcase built on the creation platform. Franchise Factory functionality is consolidated into Genesis Engine according to the repository contract.

## Canonical authority

| Responsibility | Canonical owner |
|---|---|
| Economics | `a-townchain/components/algorithm` |
| Consensus | `a-townchain/components/algorithm` |
| VM execution | `a-townchain/components/vm` |
| Language | `atclang` |
| ZKP | `a-townchain/components/zkp` |
| Node | `a-townchain/components/node` |
| Storage | `a-townchain/components/storage` |
| Wallet | `a-townchain/components/wallet` |
| SDK | `a-townchain/components/sdk` |
| Indexer | `a-townchain/components/indexer` |
| Explorer | `a-townchain/components/explorer` |
| OS kernel | `atc-shivacore` |
| OS | `globus-os` |
| AI | `aurora-ai` |
| Standards | `atc-standards` |
| Engineering governance | `atc-engineering` |
| Game engine | `genesis-engine` |
| Game | `genesis-chronicles` |

## Umbrella rule

`a-townchain-ecosystem` is the system-of-systems integration layer. It verifies cross-repository behavior and maintains imported integration state; it does not replace canonical source repositories. Where the standards registry records a monorepo migration, the canonical path is the registered monorepo path.

## Mainnet-0 boundary

Mainnet-0 contains the minimum production-capable trust and execution core: canonical L1 state transition, consensus, economics, verifier-gated VM execution, required cryptographic verification, durable persistence/recovery, node/network behavior, deterministic state transition, required wallet/SDK transaction path and release evidence.

Extended Explorer, Oracle, Compute, Interop, Marketplace, Launchpad, advanced AI providers and broader Genesis functionality may be Expansion unless explicitly required by a Mainnet release contract.

## Architectural invariants

- Invalid bytecode cannot enter state transition.
- Out-of-gas execution cannot commit state.
- Consensus state is deterministic.
- Chain identity is canonical and not duplicated as ad-hoc literals.
- Production cryptography is not simulated by reference stubs.
- AI has no implicit kernel, chain, wallet, governance or release authority.
- Evidence is bound to the exact revision it verifies.
- A component cannot claim COMPLIANT while required evidence is missing.
