# A-TownChain Ecosystem — System Boundaries

## Trust boundary

The highest-trust execution path is:

ATCLang -> ATC-IR/ABI -> ATC-VM -> State Transition -> Consensus / Persistence

This path requires deterministic, reproducible and fail-closed behavior.

## Platform boundary

ShivaCore -> GlobusOS -> userspace services.

Kernel authority ends at the defined capability boundary. Userspace services, including Aurora AI, use explicit interfaces and capabilities.

## AI boundary

Aurora AI may reason, invoke approved tools and use authorized memory/model services. It must not acquire implicit authority over kernel operations, undeclared filesystem resources, chain state transitions, wallet keys, governance decisions, or release/merge authority.

## Application boundary

Genesis, Marketplace, Launchpad, Explorer and other applications consume stable platform protocols. They must not become alternate sources of truth for consensus, economics, VM execution or governance.

## On-chain vs off-chain

On-chain: consensus-critical state, deterministic state transitions, canonical execution semantics, commitments/proofs and data explicitly required by protocol rules.

Off-chain: UI, indexing, model inference, most orchestration, analytics, content pipelines and services whose results are not consensus-critical.

A capability crossing the boundary requires an explicit protocol, authorization model, deterministic serialization where required, and evidence.

## Duplicate-authority rule

If two components implement the same consensus-critical responsibility, the architecture is DUPLICATED until one implementation is explicitly canonical and the other is reduced to a consumer, adapter, test oracle, or retired source.
