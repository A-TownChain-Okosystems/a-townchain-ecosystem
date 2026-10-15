# ATC EVM Compatibility — Separate Architecture Decision 001

- **Decision ID:** `ATC-EVM-COMPATIBILITY-DECISION-001`
- **Version:** `0.1.0-draft`
- **Status:** `OPEN — DECISION REQUIRED`
- **Catalog date:** `2026-10-15`
- **Decision owner:** architecture governance in `a-townchain-ecosystem`; implementation ownership remains with the selected canonical runtime repository.
- **Normative status:** proposal only. This document does not approve EVM compatibility, define a production-ready EVM, or claim implementation/conformance.

## 1. Decision statement

Decide explicitly whether A-TownChain should support Ethereum Virtual Machine (EVM) compatibility, and if so, which compatibility level, hardfork, execution semantics, interfaces, and conformance obligations are required.

**This decision is separate from the existing ATC-VM specification.** Until an approved decision and versioned normative standard exist, the canonical `a-townchain/components/vm` remains the existing ATC-VM. Its stack-based execution model and ATCB/ATC1 bytecode conflict are not silently reinterpreted as EVM semantics. EVM support must not be inferred from Solidity support, JSON-RPC naming, an adapter stub, or a successful compile.

## 2. Current guardrails

1. **Standalone First, Ecosystem Second.** This repository records the decision and evidence contract; it must not become a competing VM implementation.
2. **Single implementation SSOT.** If approved, the selected canonical runtime/adapter implementation must live in its designated implementation repository. No second ATC-VM or shadow EVM implementation is created in the ecosystem catalog.
3. **No semantic leakage.** EVM compatibility code must be isolated behind a versioned adapter/runtime boundary. It must not change native ATC-VM opcode, bytecode, gas, storage, or state-transition semantics.
4. **Deterministic consensus execution.** All consensus-relevant behavior must be deterministic, versioned, bounded, and reproducible across supported platforms.
5. **Fail closed.** Unsupported opcodes, precompiles, transaction types, fork rules, malformed encodings, and unrecognized configuration must fail explicitly; no silent fallback to native ATC-VM semantics.
6. **No compatibility claim without differential evidence.** Source presence, Solidity compilation, or green general CI alone is insufficient.
7. **No Mainnet activation** until the decision, normative specification, implementation ownership, differential conformance, security review, migration/rollback plan, and activation governance are approved.

## 3. Decision options

### Option A — No EVM compatibility

Keep ATC-VM as the native ATC execution environment. Provide no EVM-bytecode execution or Ethereum semantic-compatibility claim. Tooling may offer source-level migration aids only if clearly labelled non-equivalent.

### Option B — Separate EVM execution environment / adapter (preferred for evaluation, not approved)

Introduce a separately versioned EVM-compatible execution component with an explicit adapter to chain transaction, state, resource accounting, and receipt interfaces. Keep native ATC-VM semantics unchanged. Define whether this is a separate runtime, a module selected by transaction type, or an external compatibility service before implementation begins.

### Option C — Make canonical ATC-VM EVM-compatible

Change the canonical VM's protocol contract to implement EVM semantics. This has the highest migration and consensus-risk surface and would require a separately versioned protocol migration, explicit compatibility break analysis, and proof that native ATC-VM behavior is preserved or intentionally replaced. This option must not be treated as an incremental opcode addition.

**No option is selected by this draft.** Option B is listed as the preferred option to evaluate because it preserves the existing VM boundary, not as an approved architecture decision.

## 4. Required compatibility profile

Any approved EVM support must pin a named Ethereum execution-specification release/hardfork and specify every supported item below. “EVM-compatible” without a profile is not an acceptable conformance statement.

| Area | Required normative decision |
|---|---|
| Bytecode | Exact EVM code format and validation rules; deployed bytecode vs initcode; code-size limits; invalid-opcode behavior |
| Opcode semantics | Full opcode set for the selected fork, stack effects, signed arithmetic, overflow/modular behavior, jumps, memory expansion, return/revert, exceptional halts |
| Gas | Fork-pinned opcode costs, dynamic gas, memory expansion, refunds, call/create costs, out-of-gas and refund caps; deterministic mapping to ATC resource accounting |
| State model | Account/code/storage semantics, zero-value and empty-account rules, storage read/write costs, deletion/self-destruct behavior for the selected fork |
| Calls and creation | CALL/CALLCODE/DELEGATECALL/STATICCALL semantics, CREATE/CREATE2, call depth, value transfer, revert propagation, precompile dispatch |
| Transactions | Explicit mapping or dedicated envelope for EVM transaction types, nonce, chain ID, signature recovery, fee fields, access lists, blob-related fields if applicable; no implicit equivalence to TX-V2 |
| Addresses and cryptography | 20-byte EVM address derivation and mapping to ATC identities; secp256k1 recovery semantics; collision, authorization, and namespace rules |
| Logs and receipts | Event/log encoding, bloom or alternative index contract, status, gas used, cumulative gas, transaction/receipt roots or their explicitly different ATC equivalents |
| RPC and tooling | Versioned JSON-RPC subset, error codes, quantity/data encoding, filters, subscriptions, trace/debug scope, wallet/provider behavior, unsupported methods |
| Fork/versioning | Activation rule, fork ID/version negotiation, historical replay, upgrade governance, backwards compatibility and deprecation |
| Host boundary | No filesystem, clock, network, randomness, or ambient host access from deterministic contract execution; all host calls are explicit and metered |
| Limits | Code, memory, stack, call depth, return data, logs, storage, execution time, and total transaction resource limits |
| Interop | Solidity compiler versions, metadata, ABI, libraries, deployment flows, and Ethereum tooling support are specified separately from bytecode execution conformance |

The profile must distinguish at least these levels:

- **L0 — Source/tooling only:** source or ABI tools; no claim of EVM-bytecode execution.
- **L1 — EVM bytecode execution:** selected-fork opcode and execution semantics, with pinned differential tests.
- **L2 — Transaction/state compatibility:** L1 plus transaction admission, address/account mapping, state transition, logs and receipts.
- **L3 — Ethereum tooling/RPC compatibility:** L2 plus a declared JSON-RPC/provider/wallet/tooling surface.
- **L4 — Network/protocol equivalence:** Ethereum consensus, networking, state roots, and protocol equivalence. This is **out of scope by default** and must never be implied by L1–L3.

## 5. ATC integration contract

If EVM support is approved, the specification must define and test the following boundaries before implementation:

- **Runtime selection:** explicit, authenticated/versioned transaction or deployment metadata selects the EVM environment; ambiguous or unknown runtime identifiers are rejected.
- **State adapter:** every read/write is scoped to the transaction state overlay. Reverts and exceptional halts roll back the entire affected execution scope exactly as the selected EVM fork requires.
- **Resource adapter:** EVM gas remains a normative EVM quantity. Any conversion to ATC fees/resource units is explicit, deterministic, overflow-safe, and cannot make an underfunded execution succeed.
- **Transaction adapter:** EVM envelopes and native ATC TX-V2 are distinct types unless a normative mapping proves all signing, replay-protection, canonical-encoding, nonce, and authorization semantics.
- **Address/account adapter:** mappings are deterministic, collision-safe, versioned, and cannot silently confer authority over a native ATC account.
- **Receipts/events:** canonical ordering, encoding, indexing, and persistence are specified and covered by test vectors.
- **Failure model:** invalid transaction, invalid opcode, revert, out-of-gas, precompile error, adapter error, and host-resource exhaustion have distinct, deterministic outcomes.
- **Cross-platform invariance:** the same vectors must produce identical result, state diff, logs, receipt, and gas on every supported host and native deployment target.

## 6. Minimum conformance and security gates

A future implementation proposal must provide all of the following, bound to an exact 40-character source SHA:

1. A normative compatibility profile pinned to an exact Ethereum fork/specification revision.
2. Positive, negative, boundary, and malformed-input vectors for every supported opcode and precompile.
3. Differential execution tests against at least one pinned, independent reference client (for example, a pinned `revm` or Geth version), with version and configuration recorded.
4. State-transition, revert/rollback, gas, memory expansion, call/create, address derivation, logs/receipts, and transaction-envelope vectors.
5. Fuzzing and property tests for bytecode decoding, stack invariants, gas accounting, serialization, and adapter boundaries.
6. Cross-platform reproducibility tests; identical canonical outputs across supported architectures/build profiles.
7. Adversarial tests for resource exhaustion, denial of service, integer overflow, replay, address collisions, precompile abuse, and host-boundary escape.
8. Security review of the adapter, crypto/address mapping, transaction admission, state isolation, and upgrade activation.
9. Exact GitHub Actions evidence: workflow run ID → job → step → terminal conclusion → exit code → logs/artifacts, all bound to the same source SHA.
10. A migration, activation, deactivation/rollback, historical replay, and governance plan.
11. A complete unsupported-feature matrix; no untested or implicit “mostly compatible” claim.
12. Independent approval of the architecture decision and the relevant normative `atc-standards` entry before Mainnet activation.

A passing structural catalog check or generic CI run is not a substitute for these gates.

## 7. Proposed acceptance criteria

The decision may be marked **APPROVED** only when the architecture authority records one option, a named compatibility level, a pinned fork/profile, the canonical implementation owner, protocol/versioning strategy, migration implications, and explicit non-goals. Implementation readiness remains a separate status.

A compatibility implementation may be marked **VERIFIED** only when all required tests and security gates pass against the exact candidate SHA, all supported profile features have reproducible differential evidence, and unsupported features fail closed.

A release may be marked **PRODUCTION_READY** only after the standard production gates and activation governance pass. No status is inherited automatically from this architecture document.

## 8. Open questions that block approval

- Which option (A/B/C) is selected, and who is the approving architecture authority?
- Which exact Ethereum execution-specification fork/revision is the target?
- Is the desired level L0, L1, L2, or L3? L4 is excluded unless separately justified.
- Is EVM execution consensus-critical on the A-TownChain L1, or a non-consensus tooling/service feature?
- What is the canonical runtime implementation repository and module boundary?
- How are EVM addresses/accounts mapped to ATC identities and permissions?
- How are EVM gas and fee markets represented without changing native ATC fee semantics?
- Which JSON-RPC methods, transaction envelopes, precompiles, and tooling clients are in scope?
- What are the activation/fork, historical replay, upgrade, and rollback rules?

Until these questions are answered and approved, decision status remains `OPEN`, implementation status is not asserted, and Mainnet EVM compatibility is **NO-GO**.

## 9. Catalog and SSOT links

- Component/function inventory: [ATC Component and Function Catalog 001](ATC-COMPONENT-FUNCTION-CATALOG-001.md)
- Canonical native VM implementation: [`a-townchain/components/vm`](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/vm)
- Normative standards registry: [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards)
- Integration and evidence control plane: [`a-townchain-ecosystem`](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem)

This draft is an architecture decision record and specification proposal only. It does not create a second VM, modify canonical ATC-VM semantics, or assert EVM implementation/conformance.
