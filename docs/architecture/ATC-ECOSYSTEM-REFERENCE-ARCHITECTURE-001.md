# ATC Ecosystem Reference Architecture 001

- **Status:** Architecture baseline proposal; implementation and conformance are not asserted.
- **Revision date:** 2026-10-15
- **Authority:** Integration/control-plane reference. This document does not supersede canonical implementation repositories or normative standards.
- **Release decision:** BLOCKED until the gates below have exact-commit evidence and all blocking decisions are resolved.

## 1. Purpose and governing rules

This document connects A-TownChain, GlobusOS, ShivaCore, Aurora AI, SDK, standards and tooling into one deployment and security model. It supplements the component/function catalog; it is not a second implementation specification.

1. **Standalone First, Ecosystem Second.** Each implementation must be independently usable and testable before ecosystem integration is considered complete.
2. **One implementation SSOT per capability.** Adapters may connect canonical owners but must not fork or replace them.
3. **Evidence before status.** Source presence proves neither behavior nor security. IMPLEMENTED, VERIFIED, RESIDUAL, BLOCKED and EXCLUDED are distinct states. Unknown behavior remains UNRESOLVED/BLOCKED.
4. **Fail closed at trust boundaries.** Missing identity, authorization, version, proof, finality, schema or test evidence cannot be interpreted as approval.
5. **No implicit protocol decisions.** Consensus, finality, VM bytecode format, EVM compatibility and cross-chain proof models require explicit normative decisions.
6. **No merge by implication.** Green unrelated CI, catalog counts, successful compilation or a single boot/test is not blanket acceptance evidence.

## 2. Canonical SSOT and ownership map

| Capability | Canonical owner / SSOT | Ecosystem repository role |
|---|---|---|
| Blockchain L2 core, node, transactions and chain integration | A-TownChain-Okosystems/a-townchain | Integration contracts, catalog, evidence and release gates |
| ATC-VM implementation | a-townchain/components/vm | Architecture records only; no second VM |
| Algorithm and consensus implementation | a-townchain/components/algorithm | Decision tracking and evidence; no invented consensus status |
| Wallet and transaction-signing implementation | a-townchain/components/wallet | Cross-component policy/evidence only |
| GlobusOS and ShivaCore kernel | A-TownChain-Okosystems/globus-os; kernel at modules/atc-shivacore/kernel/ | Integration contracts; no second kernel |
| Aurora AI agents, policy and runtime | A-TownChain-Okosystems/aurora-ai | Capability/approval contracts; no second agent runtime |
| SDK, CLI and client libraries | A-TownChain-Okosystems/atc-sdk | Contract/version compatibility tracking |
| Normative protocol and interface standards | A-TownChain-Okosystems/atc-standards | Decision/evidence links, not a competing normative registry |
| ATCLang compiler/language implementation | A-TownChain-Okosystems/atclang | Integration and conformance tracking |
| Toolchain and genesis/bootstrap utilities | atc-toolchain; genesis-engine | Pipeline contracts and exact-SHA evidence |
| Architecture, integration, catalog and compliance | A-TownChain-Okosystems/a-townchain-ecosystem | Control plane only |
| Standalone atc-vm, atc-algorithm and atc-shivacore repositories | Specification/support/migration evidence as applicable | Not duplicate production SSOTs without a reviewed ownership decision |

The repository manifest remains the authority for organization inventory and policy exclusions. demo-repository remains a manifest record but is excluded from the live drift gate by explicit policy; it must not be silently deleted or counted as an active production implementation.

## 3. System layers and dependency direction

1. **Hardware and firmware:** CPU, memory, storage, network, TPM/TEE where available, firmware and device discovery.
2. **Boot and platform trust:** verified boot chain, image/signature checks, recovery path and measured/attested state where implemented.
3. **ShivaCore TCB / OS kernel:** capability-based authority, memory/process isolation, secure IPC and explicit device/resource access.
4. **GlobusOS services and desktop:** session/window manager, device/settings services, monitoring, administration, updates, backup/recovery and guest-VM UX.
5. **Security and cryptographic providers:** key-provider interfaces, entropy source, signing/verifying, hashing, secret storage and policy checks; concrete algorithms/formats are pinned by standards.
6. **A-TownChain core:** transaction validation, block/state processing, node networking, canonical ATC-VM and algorithm implementation.
7. **Contracts and interoperability:** versioned contract interfaces, cross-chain message envelope, proof/finality validation and replay protection.
8. **Aurora AI control plane/runtime:** intent/planning, tool discovery, capability policy, sandboxed execution, approval, audit and local/remote model routing.
9. **SDK, CLI, IDE and developer tools:** canonical encoders, RPC clients, transaction vectors, reproducible build/test tools and diagnostics.
10. **Ecosystem integration and assurance:** component graph, repository manifest, source census, compatibility matrix, evidence, drift checks and release decisions.

Dependencies flow toward lower-level contracts, not around them. UI and AI call owner-controlled APIs; they must not access kernel internals, wallet secrets, VM state, validator authority or privileged RPC paths directly. The ecosystem control plane describes and verifies edges but is not a runtime dependency required to boot or execute a standalone component.

## 4. Deployment models (distinct profiles)

### Profile A — Native GlobusOS installation
GlobusOS is installed as the host OS on supported physical hardware. The install/update path must define boot mode, disk/partition handling, signed artifacts, rollback, recovery media, hardware support and preservation of unrelated disks/boot entries. The installer must not modify disks or boot configuration outside explicit user-approved targets.

### Profile B — Ecosystem applications on Windows or Linux
Individual components or an approved packaged distribution run as host applications/services. Host-specific services are isolated behind adapters. This profile does not prove that GlobusOS is installed as a native OS or that ShivaCore kernel features exist on the host.

### Profile C — Windows/Linux guest inside GlobusOS
GlobusOS remains the physical host and runs a separately isolated guest VM. Backend, machine profile, guest matrix, virtual firmware/devices, quotas, image validation, snapshot/restore, console integration and isolation must be selected and tested. QEMU is an evaluation candidate, not a selected/proven backend. Linux KVM acceleration is Linux-host-specific and is not a native GlobusOS hypervisor by itself.

### Profile D — Native dual boot (future, separate decision)
A second OS is installed to a physical disk/partition and selected at boot. Only one OS runs at a time. This is not a VM, CPU emulation or part of the initial guest-VM acceptance gate.

### Profile E — Full-system emulation (optional fallback)
CPU/system emulation may allow other architecture combinations but can have substantial performance costs. It is specified separately from hardware-assisted virtualization and must not be advertised as equivalent in performance or compatibility.

Every distribution artifact must carry version, target OS/architecture, dependency/license inventory, integrity/signature metadata, provenance, install/removal instructions and rollback/recovery expectations. Unsupported combinations fail closed rather than silently selecting an untested path.

## 5. Security architecture and trust boundaries

Treat downloaded packages, guest operating systems, contract bytecode, network peers, RPC inputs, external model outputs, plugins/tools and imported developer projects as untrusted.

- **Boot/update:** verify signatures and provenance before activation; reject unsigned, malformed, downgraded or wrong-target artifacts unless an explicit recovery policy authorizes recovery.
- **Kernel/OS:** no ambient authority. Privileged actions require explicit capabilities and auditable policy decisions.
- **Process/VM isolation:** guest memory, devices, files, clipboard, USB, physical disks and networking are denied by default. Grants are explicit, scoped, revocable and logged.
- **Wallet/key material:** private keys and seed material are not exposed to UI, logs, model prompts, guest OS, arbitrary plugins, build scripts or general-purpose RPC. Signing is a narrow owner-controlled operation with transaction display, policy checks and explicit approval as required.
- **Aurora/tool execution:** model output is data, not authority. Tool invocation requires independent policy evaluation, a scoped capability, argument validation, sandboxing, an audit event and human approval for sensitive operations. Wallet/transaction actions are approval-gated.
- **Blockchain/network:** validate sizes, encodings, signatures, chain IDs, nonces, replay domains, state preconditions and resource budgets. Peer/RPC failure must not bypass consensus or authorization checks.
- **Cross-chain:** never accept a message solely because a remote endpoint claims success. Validate the selected proof/finality model, source/destination chain IDs, domain, nonce/message ID, replay rules and destination authorization.
- **Telemetry:** minimize sensitive content, redact secrets, bound log size/access and make diagnostic export explicit.
- **Recovery:** verify backup/restore integrity and authorization; snapshot restore must not silently roll back anti-replay state, key policy or trusted update metadata.

Threat model, key lifecycle, entropy source, cryptographic suite, attestation semantics and privileged-operation policy require normative specifications and independent review before production readiness.

## 6. Cryptography and identity

Each cryptographic operation must specify purpose, algorithm/profile, input encoding, domain separation, key type/size, canonical signature rules, error behavior, versioning, test vectors and migration policy.

Recorded TX-V2 constraints include secp256k1 ECDSA with RFC6979/SHA-256, low-S signatures, compressed 33-byte SEC1 public keys, compact 64-byte r||s signatures and domain ATC-TX-DOMAIN-V2. Legacy ATC-TX-DOMAIN and atc-tx.v1 must not be silently accepted as TX-V2. Canonical transaction/wallet implementation remains in a-townchain; normative encoding belongs in atc-standards. Exact interoperability vectors and end-to-end signing/verification evidence remain required.

The custom ATC hash is not audited by virtue of source presence. Independent cryptographic analysis is a Mainnet gate. Entropy/randomness APIs must distinguish production entropy from deterministic test-only generators; deterministic PRNGs must never silently supply production key-generation entropy.

Key-provider APIs distinguish generate/import/derive/sign/verify/rotate/revoke/export policy and state whether secret bytes can leave the provider. Key export is denied by default. TPM/TEE capabilities are optional only where a declared profile says so; absence must be visible, not represented as attestation success.

## 7. Blockchain, VM and consensus boundaries

- a-townchain/components/vm is the only canonical ATC-VM implementation.
- ATC-VM is not assumed EVM-compatible. EVM compatibility remains a separate open decision; no option is approved by this baseline.
- The ATCB-versus-ATC1 bytecode magic/version conflict is a blocking specification decision. Decoders fail closed on unknown, conflicting, truncated or unsupported formats.
- VM execution binds program validation, caller/context authority, state transitions, gas/resource limits and deterministic failure semantics to versioned contracts.
- Consensus/finality must not be inferred from PoH, block production, gossip or a passing unit test. Normative protocol, adversarial assumptions, fork choice, validator rules, reorganization handling and finality query semantics require explicit decisions and tests.
- Transaction validation, node submission, block inclusion and finality are separate lifecycle states. An API must not report finality merely because a transaction was accepted or included.
- Genesis/configuration and network identity must be versioned and bound to the correct chain ID/environment. Mainnet, testnet and devnet data/keys must not be confused.

## 8. Cross-chain interoperability

Transport and cross-chain trust are separate. Transport may deliver a message; only a canonical verifier for the chosen source-chain proof/finality model can authorize execution.

A versioned envelope must define source/destination chain IDs, protocol/profile and destination interface versions, nonce and message ID, payload schema/hash, proof reference/data, explicit finality evidence, expiry/replay rule, destination handler and authorization context.

Normative decisions remain required for supported chains, finality assumptions, proof verifier ownership, light-client versus external-validator/trusted-relayer model, key rotation, upgrades, pause/recovery, rate limits, relayer incentives and incident response. No bridge is trust-minimized until assumptions and verifier are explicit and adversarial tests cover forged proofs, replay, reordering, duplicate delivery, reorgs, verifier upgrades and destination failure.

The ecosystem repository owns decision/evidence records; implementation belongs in a selected canonical interop SSOT only after an architecture decision assigns it. No second bridge implementation is to be added here.

## 9. API, ABI and interface contracts

Every public or cross-repository API needs a stable interface ID, owner, version, schema, compatibility promise, authentication/authorization, resource limits, timeout/retry semantics, error taxonomy, idempotency rules and conformance vectors.

Required interface families: OS capability IPC; crypto/key provider and identity/attestation; chain RPC for lifecycle/submission/status/receipts/finality; ATC-VM invocation/state; wallet signing/approval; cross-chain proof/finality; Aurora tool execution/approval/audit; SDK/provider adapters; installer/update/provenance/recovery; guest lifecycle/quota/console/device grants.

Rules:
1. Wire schemas and normative versions live in atc-standards or the explicitly assigned normative SSOT.
2. Implementation code remains in the canonical owner repository.
3. SDKs and UIs are clients/adapters, not alternate protocol authorities.
4. Unknown fields/versions follow a specified compatibility policy; security-sensitive ambiguity fails closed.
5. Retries are idempotent or use idempotency keys; signing, submission and cross-chain execution must not duplicate side effects after ambiguous timeouts.
6. Errors distinguish invalid input, unauthorized, unsupported version, resource exhaustion, transient transport failure, rejected transaction, included transaction and finalized transaction.
7. Never log secrets, seed material, privileged capability tokens or sensitive guest data.

## 10. Aurora AI control plane/runtime

Aurora is not the kernel, consensus engine, VM or wallet authority. Its layers are Experience → Control Plane → Fabric → Runtime → Intelligence. Local or external model selection cannot weaken capability, privacy, audit or approval policy.

- Prompts, retrieved documents, tool results and model output are untrusted data.
- Separate planner decisions from authorization; policy independently authorizes every tool action.
- Sandbox tools with bounded time, memory, network and filesystem access.
- Store external API keys in a protected secret provider, not source files, model context, logs or diagnostics.
- Define offline-state conflict handling, synchronization and audit continuity.
- Model/version changes require regression vectors for routing, policy enforcement, data leakage and sensitive-action approval.
- Wallet operations use canonical wallet APIs, user-visible transaction details, policy checks and explicit approval where required. Model confidence is not authorization.

## 11. Developer and release toolchain

The developer path must be reproducible from source checkout through format/lint, unit/property/fuzz tests, build, artifact generation, signing, installation, integration and release evidence. Rust production code and Python governance/research tools may coexist under explicit boundaries. Generated artifacts must be reproducible or carry provenance.

Minimum contracts:
- pin compiler/toolchain and dependency versions; record license and vulnerability findings;
- use canonical SDK encoders and shared transaction vectors, not per-language reimplementations;
- verify bytecode/artifact format and target version before execution/deployment;
- distinguish local test networks/devnets from Mainnet;
- preserve machine-readable results with exact commit and artifact hashes;
- keep secrets out of build/test output; declare network access in reproducibility checks;
- classify test helpers, generated code, docs and CI as support/exclusion candidates unless evidence justifies counting them as runtime functions;
- require independent review for cryptography, privileged OS paths, consensus/finality, guest isolation and bridge verifiers.

## 12. Interface ownership and producer-consumer edges

An edge is valid only when it identifies producer, interface ID/version, consumer, edge type (direct call, protocol, adapter, wrapper/delegation or transitive dependency) and source evidence. Naming/directory similarity is not proof.

- UI → owner API; never direct access to private subsystem internals.
- Aurora → capability broker → owner API; policy/approval is separate from execution.
- SDK/CLI → versioned RPC/interface → node/chain; no private duplicate encoders without shared conformance vectors.
- Guest manager → VMM adapter → virtual devices; default-deny host access and resource limits.
- Wallet → key provider/crypto implementation → transaction verifier; no key-material bypass through UI, AI or SDK.
- Interop handler → proof/finality verifier → destination handler; transport is not trust.
- Ecosystem catalog → canonical repositories/CI artifacts; evidence links only, not runtime authority.

If a mapping/edge lacks exact commit, path, symbol and blob SHA, record it as residual/unresolved. Test claims require the exact tested SHA plus workflow run, job, step, terminal conclusion, exit code and logs/artifacts.

## 13. Mandatory architecture decisions and blockers

| Gate | Required decision/evidence | Until complete |
|---|---|---|
| Consensus/finality | Normative protocol, threat assumptions, implementation and adversarial tests | BLOCKED for production finality claims |
| ATC bytecode | Resolve ATCB vs ATC1; pin format and negative vectors | BLOCKED for ambiguous decode/artifacts |
| EVM compatibility | Approve/reject a separate profile and adapter boundary | OPEN; no compatibility claim |
| Cryptographic hash | Independent review, vectors, differential/property/fuzz tests and migration plan | BLOCKED for Mainnet security claim |
| TX-V2 | Exact encoding vectors and end-to-end wallet-to-node evidence | RESIDUAL/BLOCKED as applicable |
| Cross-chain | Select proof/finality trust model and canonical verifier owner | BLOCKED for trust-minimized bridge claim |
| Native installation | Supported hardware/boot/disk matrix and signature/update/rollback/recovery evidence | RESIDUAL |
| Guest OS virtualization | Select backend/matrix; prove isolation, quotas, restore and UI integration | BLOCKED |
| Kernel/OS security | Capability boundary, secure boot/key provider and hardware profiles reviewed | BLOCKED where evidence is missing |
| Aurora tool execution | Capability/policy, sandbox, approval, audit and secret-handling tests | RESIDUAL/BLOCKED |
| API compatibility | Normative interface versions, error/idempotency semantics and cross-language vectors | RESIDUAL |
| Catalog completeness | Bidirectional source census, explicit exclusions and duplicate/delegation reconciliation | RESIDUAL |
| Release evidence | Exact-SHA Run → Job → Step → terminal conclusion → exit code → logs/artifacts | BLOCKED until complete |

## 14. Verification and acceptance sequence

1. Freeze architecture decision records and identify decision owners; open proposals do not become defaults.
2. Bind contracts to canonical repository HEAD SHA, exact source path, symbol/entry point and file blob SHA.
3. Reconcile both directions: every in-scope runtime symbol is mapped or explicitly excluded, and every catalog function is source-bound or has a reasoned residual/blocked state.
4. Validate component/function IDs, dependencies, interface versions, producer-consumer edges and duplicate/delegation relationships.
5. Run validators/tests on the exact PR head and record terminal logs/exit codes. Pending, skipped, inaccessible or unrelated checks are not passes.
6. Run targeted security/conformance tests for cryptography, wallet, VM, consensus/finality, interop, install/update, guest isolation and Aurora policy.
7. Review residuals/conflicts with canonical owners. Catalog expansion does not mark implementation complete.
8. Keep the PR Draft and release BLOCKED until mandatory gates have exact-head evidence and authorized reviewers approve relevant decisions.

## 15. Status statement

This is a reference architecture and gate map. It does not assert that all components are implemented, that the system currently boots as a complete OS, that Windows/Linux guests work, that EVM compatibility is available, that consensus/finality is finalized, that the custom hash is audited or that a cross-chain bridge is trust-minimized. Implementation, source mapping, tests and security status must be read from the component/function catalog and its exact-SHA evidence.
