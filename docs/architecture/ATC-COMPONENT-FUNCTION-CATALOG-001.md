# ATC Component and Function Catalog 001

**Catalog state:** `BASELINE_RESIDUAL`  
**Release decision:** `BLOCKED`  
**Machine-readable SSOT:** [ATC-COMPONENT-FUNCTION-CATALOG-001.json](ATC-COMPONENT-FUNCTION-CATALOG-001.json)

This catalog is the architecture/control-plane inventory. It does not replace component source code, normative standards, or repository-local API documentation.

**Related format registry:** [ATC-FILE-FORMAT-REGISTRY-001.md](ATC-FILE-FORMAT-REGISTRY-001.md) and [machine-readable registry](ATC-FILE-FORMAT-REGISTRY-001.json). The registry inventories formats and processing requirements; implementation function contracts remain in this catalog's JSON SSOT. Registry presence does not establish implementation or verification.

## Rules

- Each capability has a stable component ID and function ID.
- Each component names one canonical owner; specifications and support repositories are not treated as duplicate implementation owners.
- Each function records purpose, inputs, outputs, failure modes, measurable acceptance criteria, test classes, and readiness.
- Cross-component edges are explicit and remain `RESIDUAL` until their exact API/ABI and compatibility evidence are verified.
- `IMPLEMENTED` does not mean `VERIFIED`; `PRODUCTION_READY` requires full exact-SHA evidence, required gates, logs/artifacts and review.
- A skipped or pending required check is not a successful check. No automatic merge, repository archive, or source deletion is authorized by this catalog.

## Canonical domains in this baseline

| Component | Canonical implementation owner | Principal responsibilities |
|---|---|---|
| atc-engineering | [atc-engineering](https://github.com/A-TownChain-Okosystems/atc-engineering) | Fleet governance and exact-SHA assurance |
| ATC Standards | [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | Normative specifications and conformance contracts |
| ATCLang | [atclang](https://github.com/A-TownChain-Okosystems/atclang) | Language syntax, parsing and frontend |
| ShivaCore Kernel | [globus-os/modules/atc-shivacore/kernel](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel) | no_std capability microkernel and TCB |
| A-TownChain | [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) | Canonical L1 transaction, block and state implementation |
| ATC-Algorithm | [a-townchain/components/algorithm](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/algorithm) | Canonical consensus/algorithm implementation |
| ATC-VM | [a-townchain/components/vm](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/vm) | Deterministic bytecode execution |
| GlobusOS | [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) | Userspace services and OS integration above ShivaCore |
| Aurora AI | [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) | AI runtime, agent and swarm coordination; outside kernel TCB and consensus authority |
| ATC Toolchain | [atc-toolchain](https://github.com/A-TownChain-Okosystems/atc-toolchain) | Compiler/build/test/package tooling |
| Genesis Engine | [genesis-engine](https://github.com/A-TownChain-Okosystems/genesis-engine) | Genesis configuration and runtime/bootstrap orchestration |
| Ecosystem control plane | [a-townchain-ecosystem](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem) | Architecture, integration, compliance and evidence only |

## Function contract analysis (schema 1.1.0)

Every function record carries a machine-readable `contract_analysis` object in the JSON SSOT. It extends the existing function fields rather than creating a parallel catalog:

- `purpose`, `inputs`, `outputs`, `preconditions`, and `failure_semantics`
- `behavior` including invariants, side-effect and trust-boundary assessment
- `architecture_placement` with owning component, layer, canonical owner, path and symbol where known
- `consumers` and `reuse` for direct/transitive callers, delegation, adapters and duplicate/conflicting implementations
- `source_evidence` with repository, full 40-character commit SHA, path, symbol and file blob SHA

The migration marks unproven behavior, consumers, reuse, error contracts and recovery as `UNRESOLVED`; it does not infer them from names or prose. `preconditions` may be a status-bearing object or a non-empty array of explicit condition strings; the array form is descriptive only and never implies verification. A source mapping is not functional verification. The validator rejects malformed structures, mismatched component placement and incomplete or malformed SHA evidence. Reconciliation only accepts a source binding when repository HEAD, path/symbol and file blob SHA agree; missing or mismatched blob evidence remains residual. The source inventory is lexical evidence, not a complete call graph.

## Production gates

1. **Ownership:** exactly one canonical implementation per capability.
2. **Interface:** versioned inputs, outputs, error semantics and compatibility rules.
3. **Correctness:** positive, negative, boundary, regression and deterministic-vector tests.
4. **Security:** threat model, trust boundary, invalid-input and resource-exhaustion behavior.
5. **Build integrity:** required formatting, lint, build, test and security checks have terminal conclusions.
6. **Operations:** diagnostics, compatibility, recovery/rollback and artifact provenance where applicable.
7. **Evidence:** exact 40-character source SHA, workflow run, job, step, exit code 0, logs/artifacts and review reference.

## Validate locally or in CI

```sh
python3 -m json.tool docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json >/dev/null
python3 tools/validate_component_function_catalog.py \
  --input docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json \
  --report component-function-catalog-report.json
```

The validator's `PASS_STRUCTURAL_ONLY` means the registry is structurally consistent; it is not a certification of production readiness. A component or function cannot be marked `PRODUCTION_READY` without complete evidence fields. The release decision remains `BLOCKED` until implementation and integration gates are independently verified.

## Known protocol-sensitive acceptance criteria

- A-TownChain: chain ID `658467`, supply `360000000 ATC`, 18 decimals, u128 amounts as 16-byte big-endian and u64 counters as 8-byte big-endian.
- TX-V2: secp256k1 ECDSA, RFC6979/SHA-256, low-S, 33-byte compressed SEC1 public key, 64-byte compact signature and `ATC-TX-DOMAIN-V2`; legacy signing domains must be rejected.
- ATC-VM: normative `ATCB` magic, u16 big-endian version, u32 big-endian instruction count; resolve `ATCB` versus `ATC1` drift against the normative specification. Enforce the documented operation, stack and storage limits and gas behavior.
- ShivaCore/GlobusOS: kernel authority remains capability-bounded; AI, networking and contract execution remain outside the kernel TCB unless a formally approved architecture change says otherwise.
- Consensus: do not claim production readiness until the consensus algorithm and parameters are formally frozen and deterministic/adversarial evidence passes.

## ATC-VM source reconciliation (2026-10-12)

Canonical implementation owner: [`a-townchain/components/vm`](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/vm), source HEAD `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9`.

The machine-readable `COMP-VM` entry now contains **29 function contracts**:

- `FN-VM-001` — textual `.ops` parser (`parse_ops`)
- `FN-VM-002` — gated state transition with explicit gas budget
- `FN-VM-003`–`FN-VM-010` — VM constructors, caller/state getters, legacy transition gate, structural verifier, gas-bounded transactional interpreter and raw local interpreter
- `FN-VM-011`–`FN-VM-013` — chain context validation, computed-genesis comparison and fail-closed execution gate

**Important correction:** `parse_ops` parses line-oriented `.ops` text. It is not a binary `ATCB` decoder. No binary `ATCB` magic/version/instruction-count decoder was evidenced in the inspected canonical source surface. The prior binary-decoding claim is therefore not treated as implemented.

Source-level limits recorded from `vm.rs`: `MAX_PROGRAM_OPS=65,536`, `MAX_STACK_ITEMS=4,096`, `MAX_STORAGE_SLOTS=65,536`. The public raw `Vm::run()` path is unmetered and does not enforce all bounds; it is documented as local/unit-only, not the consensus state-transition entrypoint. The catalog records this as a residual risk.

The inventory also records `assembler.rs`, `main.rs` and `tests/exec_chain_e2e.rs` as inspected surfaces, but does **not** claim their function contracts are complete. All 29 VM function records remain `RESIDUAL`; exact-SHA workflow Run/Job/Step/exit/log evidence and functional verification are still required. Release remains `BLOCKED`.

## ATC-VM assembler and runner reconciliation (2026-10-12)

The `COMP-VM` catalog now maps **29 function contracts** from the canonical VM source surface at `a-townchain` HEAD `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` (refreshed after the VM runner resource-bounds fixes merged via PR #71). The organization-wide catalog now lists **98 functions across 30 components**.

Additional contracts cover:

- `assemble`: restricted EXEC-GATE source/vector to textual `.ops` generation, with local simulation before writing output.
- Parser helpers: `Parser::new`, `peek`, `take`, `expr`, `term`, `factor`, `tokenize`.
- Source/vector helpers: `find_function`, `json_u64_for_key`, `simulate`.
- Runner helpers: `parse_slot_pair`, `parse_args`, `run_ops_file`, `main`, and `usage`.

### Newly documented residuals

- `find_function` locates source constructs by substring scanning; it is not a complete language parser and does not robustly handle nested braces.
- `json_u64_for_key` scans text rather than parsing standards-compliant JSON; duplicate/nested key ambiguity remains.
- The local CLI calls raw unmetered `Vm::run()`. Its `--set-slot` path may resize storage based on a user-supplied index without a maximum bound. This requires a defensive limit before allocation.
- The assembler emits textual `.ops`, not serialized `ATCB` bytecode. No canonical binary `ATCB` decoder was found in the inspected source surface.

These findings are cataloged as `RESIDUAL`, not silently promoted to implementation or verification. No cargo test, clippy, schema-validation or runtime result is claimed by this catalog edit. The release decision remains `BLOCKED`.

## Organization-wide coverage

The machine-readable catalog currently defines **30 components**, **98 functions**, **7 cross-component interfaces** and **7 production gates**. It also includes a `repository_coverage` entry for all **33 observed repositories** from the organization integration registry: **26 active** and **7 archived**. The validator checks unique full repository names, archive-state totals and the required fail-closed status of each entry.

This is deliberately not equivalent to 33 complete function inventories. The current catalog defines 30 component records and 98 function contracts across core, developer tooling, node/service and application domains. Every remaining active repository is still `RESIDUAL` / `AUDIT_REQUIRED` until its actual source tree, README/specs, exported APIs, tests, dependencies and exact-SHA CI evidence are inspected. Archived repositories are `REFERENCE_ONLY`, not active production targets. The audit must not invent functions from repository names.

Next audit passes must add repository-specific function IDs and interface contracts for the remaining active domains, including SDK, node, contracts, ZKP, storage/compute services, oracle/interop, applications, organization governance and repository orchestration. Migration candidates require source-to-canonical path/history evidence before KEEP/SYNC/ARCHIVE decisions.


## Completeness supplement 2026-10-12

A full organization census (26/26 active repositories scanned, 4,917 source files, 35,550 candidate public symbols) was reconciled against this catalog. Results:

- **All function contracts now carry explicit source records** (63 at the first supplement, 71 after the ATCLang pass, 98 after the component and specification pass). 58 previously unbound functions were bound conservatively to census-verified files at exact SHAs (`SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED`); 46 of these were census-verified lexical matches (`LEXICAL_MATCH_ONLY`, functional verification pending).
- **12 bindings target specification, documentation, data or ATCLang (`.atc`) files** the census scanner cannot lexically verify. These remain explicit residuals, not silent omissions.
- **5 records are explicit blocked entries:** 2 archived-repository functions (atc-storage, `BLOCKED_ARCHIVED_REFERENCE_ONLY`), 2 demo-fixture functions (`NO_SOURCE_SYMBOLS_IN_CENSUS`) and 1 atc-toolchain function with no identified compile/package surface (`SOURCE_SURFACE_NOT_IDENTIFIED`).
- **Component and repository census SHAs refreshed** to the 2026-10-12 census; 0 SHA mismatches remain between the catalog and the census.
- Known scanner gap recorded: the census cannot yet extract symbols from ATCLang (`.atc`) sources (atc-contracts scanned only 3 non-`.atc` files). An ATCLang symbol extractor is a follow-up requirement, not a blocker for lexical bindings elsewhere.

### Private repository exclusion 2026-10-12

`demo-repository` is the only private repository in the manifest (verified against the GitHub API on 2026-10-12). The CI census cannot clone it because the workflow credential lacks private-repository access (Issue #112). The census tool now records such checkouts as `EXCLUDED_PRIVATE_NO_CI_ACCESS` and counts them in `active_excluded_private` instead of failing the whole run:

- CI census: 25 active scanned + 1 excluded private, status `RESIDUAL`.
- Local census with credentials: 26 active scanned, 0 excluded, unchanged.
- The exclusion is documented evidence of reduced CI coverage, not a silent omission. The 2 demo-fixture functions (`FN-DEMO-001`, `FN-DEMO-002`) remain explicit blocked records (`NO_SOURCE_SYMBOLS_IN_CENSUS`).
- Once a census credential with private-repository access is provisioned, the checkout is scanned normally again and the exclusion disappears; no catalog change is required.

The workflow assertion still hard-codes `active_scanned == 26` until a 2-line edit in `.github/workflows/source-symbol-inventory.yml` is applied (agent push blocked by the `workflow` OAuth scope, GH013). Until then `scanner-tests` stays red despite the census itself exiting 0; the exact replacement is documented in the audit record (R-1).

Lexical mapping is not functional verification. Readiness states remain `RESIDUAL`; the release decision remains `BLOCKED`.

### Source binding states

Reconciliation against the 2026-10-12 census (`tools/reconcile_component_function_catalog.py`, functional verification `NOT_PERFORMED`):

| Binding state | First supplement (63) | ATCLang pass (71) | VM/spec pass (98) |
|---|---|---|---|
| Census-verified lexical match (functional verification pending) | 46 | 53 | 66 |
| Bound file/symbol not found by the lexical scan (spec/doc/data/`.atc`/test files and non-public symbols) | 12 | 13 | 27 |
| Explicit blocked record (archived/fixture/no surface) | 5 | 5 | 5 |
| **Total functions** | **63** | **71** | **98** |

Catalog-side `source_binding.status` values: 66 `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED`, 15 `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS`, 2 `BLOCKED_ARCHIVED_REFERENCE_ONLY`, 2 `NO_SOURCE_SYMBOLS_IN_CENSUS`, 1 `SOURCE_SURFACE_NOT_IDENTIFIED`.

Of the 27 not-found bindings, 12 target specification, documentation, data or ATCLang (`.atc`) files the lexical scanner does not symbol-scan, and 15 are ATC-VM bindings to non-public symbols (`Parser` methods, private helpers in `assembler.rs`, binary-crate functions in `main.rs`). The 15 are marked `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS`: the symbols exist at the exact SHA but are not public API candidates, so the public-symbol census cannot verify them. Both groups are documented residuals, not silent omissions.


### ATCLang completeness pass 2026-10-12

The ATCLang component (`COMP-LANG`) was audited against the full atclang repository source at census HEAD `21c595dc`. The previous 2-function entry (parsing, native directive validation) covered only the frontend; the repository implements a complete compiler pipeline. `COMP-LANG` now defines **10 functions**, all source-bound:

| Function | Binding | Census-verified |
|---|---|---|
| FN-LANG-003 AST-to-bytecode lowering | `crates/atc-core/src/lower.rs` (`lower_program`) | yes |
| FN-LANG-004 Canonical bytecode encoding and verification | `crates/atc-core/src/bytecode.rs` (`verify`) | yes |
| FN-LANG-005 Artifact envelope with hash binding (ATCA) | `crates/atc-core/src/artifact.rs` (`decode_and_verify`) | yes |
| FN-LANG-006 ABI value and protocol type boundary | `crates/atc-core/src/value.rs` (`AbiType`, plus `ir.rs` IR boundary) | yes |
| FN-LANG-007 Deterministic reference execution (reference only) | `crates/atc-core/src/vm.rs` (`execute_verified`) | yes |
| FN-LANG-008 Differential parser and corpus conformance | `crates/atc-core/tests/differential.rs` | no (test file not symbol-scanned; counted in the 13 above) |
| FN-LANG-009 Standard library reference conformance | `crates/atc-core/tests/stdlib_reference_conformance.rs` (`encode_be`) | yes |
| FN-LANG-010 Determinism gate tooling | `tools/determinism_check.py` (`main`) | yes |

Scope guardrails kept: the reference VM (`FN-LANG-007`) and stdlib reference (`FN-LANG-009`) are explicitly **reference-only** (ATC-RUNTIME-CLASSIFICATION-001); the canonical L1 execution engine remains `a-townchain/components/vm`. ABI wire encoding stays deferred to the ATC-ABI-001 conformance gate. Deep conformance of the language surface against `specs/` remains `RESIDUAL`.

Note: interface `IF-002` currently attributes the bytecode wire contract to the toolchain producer; with the canonical encoder/verifier now enumerated in atclang, a follow-up interface review should confirm the producing component for the normative bytecode contract.
## Updating the catalog

Update this catalog only from reviewed source/interface evidence. Every change should include a reason, exact source refs and test coverage. Unknown or inaccessible facts stay `RESIDUAL`/ `BLOCKED`. Never infer migration completeness from root/subtree tree-SHA equality alone, and never archive or delete a source repository based on this catalog alone.


### Current coverage counts

- Component records: **30**
- Function contracts: **98**
- Cross-component interface records: **7**
- Production gates: **7**
- Repository coverage: **33 total / 26 active / 7 archived**

The catalog includes support/governance records for the standalone ATC-VM and ATC-Algorithm repositories, but their canonical production implementations remain in `a-townchain/components/vm` and `a-townchain/components/algorithm`. The standalone `atc-shivacore` repository is specification/support only; the canonical kernel remains under GlobusOS. Archived repositories are retained as reference records and are not silently reactivated.


## Missing capability contracts — cross-platform deployment and function gaps (2026-10-15)

This additive revision records missing capabilities identified in the dual-deployment architecture. These are **catalog contracts only**; they do not claim that the implementation exists or has passed verification.

### New component

- `COMP-PLATFORM-DISTRIBUTION` — cross-platform distribution and deployment control-plane contracts.

### New function contracts

- Deployment: `FN-PLAT-DIST-001` through `FN-PLAT-DIST-007` — package matrix, native boot/install path, Windows/Linux host-process packaging, lifecycle/rollback, signed updates/provenance, cross-platform VM/protocol conformance, and host capability boundaries.
- GlobusOS: `FN-GLOBUS-004` through `FN-GLOBUS-006` — native desktop/window management, host-edition lifecycle, and OS service capability mediation.
- Node: `FN-NODE-003` through `FN-NODE-005` — lifecycle/synchronization status, RPC transport/error contract, and a finality query explicitly blocked until normative finality semantics exist.
- SDK: `FN-SDK-003` and `FN-SDK-004` — cross-language canonical transaction vectors and RPC/provider compatibility.
- Aurora AI: `FN-AURORA-005` and `FN-AURORA-006` — policy/capability-gated tool execution and approval-gated wallet/transaction actions.
- Blockchain core: `FN-CHAIN-004` and `FN-CHAIN-005` — canonical transaction validation/submission and explicit consensus/finality decision tracking.
- ATC-VM: `FN-VM-030` — bytecode artifact decode/validation, **BLOCKED** until the ATCB/ATC1 conflict is resolved normatively.

### Evidence and status

- All new functions are `RESIDUAL` or `BLOCKED` with explicit `UNBOUND_RESIDUAL` source bindings; no implementation or exact-SHA verification is implied.
- The ecosystem repository remains the architecture/evidence control plane. Canonical implementations remain in `a-townchain`, `globus-os`, `aurora-ai`, `atc-sdk`, and `atc-standards` as applicable.
- Source-to-catalog reconciliation must bind each function to exact repository HEAD SHA, path, symbol/entry point, and blob SHA—or record a reproducible exclusion.
- Unknown consensus/finality semantics and the ATCB-versus-ATC1 bytecode conflict remain blocked. Release remains `BLOCKED`.


## Canonical blockchain source mapping — exact HEAD addendum (2026-10-15)

Repository inspected: [`a-townchain` exact HEAD `a5e689671b1870699c7a1750f8a166e83590a41c`](https://github.com/A-TownChain-Okosystems/a-townchain/commit/a5e689671b1870699c7a1750f8a166e83590a41c). The source mappings below bind exact source paths and file blob SHAs. They establish **source mapping only**, not functional verification.

### Added canonical component and function contracts

- New component `COMP-WALLET`: canonical wallet key and transaction signing code under `a-townchain/components/wallet`.
- `FN-ALGO-003`: `MonetaryPolicy::{epoch,raw_subsidy,subsidy}` in `components/algorithm/src/economics.rs` (blob `f415428712ad99302ff54d6e090cb72e600570b6`).
- `FN-ALGO-004`: `atc_hash` / `atc_hash_hex` in `components/algorithm/src/hash.rs` (blob `97d3bee54f95caf2439765383041b0e120d1ab46`); **BLOCKED for Mainnet** pending independent cryptographic analysis.
- `FN-ALGO-005`: `PohChain::{genesis,tick,verify,ticks}` in `components/algorithm/src/poh.rs` (blob `75a62238e2a3e4083caa6e420c2de3f215e28450`); **BLOCKED** as a production consensus/finality claim.
- `FN-VM-031`: explicit-gas, context-gated VM state-transition path in `components/vm/src/vm.rs` (blob `57a919d911ae7900f740bab739bb20fabf0d4313`).
- `FN-CHAIN-006`: Devnet block production/chain validation in `components/node/src/chain.rs` (blob `9876cda188a353195686ded9de533770d5c5fe94`).
- `FN-NODE-006`: pull-based gossip synchronization in `components/node/src/gossip.rs` (blob `2d7c0538c0aa5bff9c524691c285e7705554405a`); TCP MVP without TLS/auth/signatures.
- `FN-WALLET-001`: `WalletKey::{from_seed,public_key,sign_digest,verify_digest}` in `components/wallet/src/keys.rs` (blob `0279290bcf68fd4471125deac20e208b29a67756`).
- `FN-WALLET-002`: `Transaction::{signing_bytes,id,digest,sign,verify}` in `components/wallet/src/tx.rs` (blob `2693b7ba015456addcc8d129455b7e5a9e2f1860`).

### Status and boundaries

- All entries remain `RESIDUAL` or `BLOCKED`, with `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` source bindings.
- The evidence manifest in the source repository records a partial, not-audited, not-conformance-verified development state. Its recorded test PASS applies only to its bound commit/run; it is not blanket evidence for every function or the current HEAD.
- The custom ATC-HASH-001 implementation remains unaudited; PoH, Devnet block/gossip, and wallet/TX-V2 end-to-end claims remain bounded by the source comments and open gates.
- The update does not claim complete source census, complete wallet security, consensus/finality completion, or Mainnet readiness. Release remains `BLOCKED`.


## Exact-head source-binding reconciliation — 2026-10-15

A follow-up audit checked the selected `a-townchain` source paths against exact HEAD `a5e689671b1870699c7a1750f8a166e83590a41c`. Function bindings and observed-symbol inventories were updated to the current commit and the file blob SHA returned for each exact path. Paths whose blobs changed since the previous census (including `components/vm/src/assembler.rs` and `components/vm/src/main.rs`) were checked at the current HEAD before rebinding.

`COMP-NODE` now identifies `a-townchain/components/node` as the canonical implementation path. The separate `atc-node` repository remains recorded as a migration-source candidate for `FN-NODE-001..002`; it is not a second canonical implementation. The selected-path inventory remains `RESIDUAL`, exported-symbol reconciliation is still incomplete, and no functional verification is inferred from source presence.

Wallet source inventory now records the verified repository root tree SHA; this is not a wallet subtree SHA. Full source census, test evidence, interface conformance, and security gates remain open. Release remains `BLOCKED`.

## Separate architecture decision — EVM compatibility (2026-10-15)

Specification: [`ATC-EVM-COMPATIBILITY-DECISION-001.md`](ATC-EVM-COMPATIBILITY-DECISION-001.md).

This is a **separate, open architecture decision**, not an implicit feature of ATC-VM. Decision status is `OPEN — DECISION REQUIRED`; implementation/conformance status is not asserted.

### Catalog additions

- `COMP-EVM-COMPAT` — architecture decision and adapter contract registry only; it is **not** a VM implementation owner.
- `FN-EVM-COMPAT-001` — decision/profile registry and pinned EVM fork.
- `FN-EVM-COMPAT-002` — isolated EVM runtime/adapter boundary and explicit runtime selection.
- `FN-EVM-COMPAT-003` — transaction, address, state, gas/fee, revert, logs and receipts mapping.
- `FN-EVM-COMPAT-004` — exact-SHA differential conformance gate against a pinned independent reference client.
- `FN-EVM-COMPAT-005` — explicit JSON-RPC/provider/wallet/compiler/tooling compatibility matrix.
- `IF-008` — proposed chain-to-EVM integration boundary, currently `BLOCKED`.

All five function contracts are `BLOCKED` with residual/unbound evidence. They are architecture contracts only; no source implementation, EVM compatibility, or exact-SHA conformance is claimed.

### Decision options and guardrails

- **Option A:** no EVM compatibility; keep native ATC-VM only.
- **Option B:** separate, versioned EVM runtime/adapter with isolated semantics — preferred for evaluation, **not approved**.
- **Option C:** make canonical ATC-VM EVM-compatible — requires a separate protocol migration and explicit compatibility-break analysis.

No option is selected in this draft. Until a decision is approved, the canonical `a-townchain/components/vm` remains the native ATC-VM; its bytecode, opcode, gas and state semantics must not be silently reinterpreted as EVM. The native `ATCB` versus `ATC1` conflict remains an independent blocker.

The specification requires a fork-pinned compatibility profile, explicit levels (L0 tooling through L3 RPC/tooling; L4 Ethereum network/protocol equivalence out of scope by default), deterministic transaction/address/state/gas mapping, fail-closed unsupported features, independent differential vectors, fuzz/property tests, cross-platform reproducibility, exact-SHA Run→Job→Step evidence, security review, and activation/rollback governance.

Catalog total after this addition: **33 components / 131 function contracts**. This is an inventory count, not proof of implementation completeness or verification. Release remains `BLOCKED`; Mainnet EVM compatibility remains `NO-GO`.

## GlobusOS guest operating system virtualization (2026-10-15)

Specification: [`GLOBUSOS-GUEST-OS-VIRTUALIZATION-001.md`](GLOBUSOS-GUEST-OS-VIRTUALIZATION-001.md).

Goal: allow a user to install Windows or Linux as a separately managed guest OS inside GlobusOS, start it in a window/full-screen session, and switch between the guest and the GlobusOS desktop without rebooting the physical host.

### Catalog additions

- `COMP-GUEST-OS-VIRT` — canonical implementation ownership assigned to `A-TownChain-Okosystems/globus-os`; the ecosystem repository only records architecture, interfaces and evidence.
- `FN-GUEST-VIRT-001` — guest VM installation and lifecycle.
- `FN-GUEST-VIRT-002` — virtualization backend and versioned machine-profile adapter.
- `FN-GUEST-VIRT-003` — guest resource quotas and capability isolation.
- `FN-GUEST-VIRT-004` — guest console/input integration with the GlobusOS desktop/window manager.
- `FN-GUEST-VIRT-005` — host/guest compatibility matrix and exact-SHA verification.
- `IF-009` — proposed GlobusOS-to-guest-VM lifecycle/console interface, currently `BLOCKED`.

All five function contracts are `BLOCKED` / `UNBOUND_RESIDUAL`. No backend is selected; QEMU is an evaluation candidate only. No Windows/Linux guest implementation or compatibility is claimed.

### Important distinction: VM vs. dual boot

- **Hosted VM (primary target):** GlobusOS remains running; Windows/Linux runs in an isolated virtual machine with its own virtual disk, virtual firmware/devices, quotas and guest lifecycle. Switching between host and guest does not require rebooting the physical machine.
- **Native dual boot (future, separately approved):** a second OS is installed to a physical disk/partition and selected before boot. Only one OS runs at a time; this is not emulation and is outside the initial VM milestone.
- **Full-system CPU emulation (fallback):** may support some cross-architecture combinations, with a significant potential performance cost. It is not interchangeable with hardware-assisted virtualization.

Security defaults: guests are untrusted; host files, clipboard, physical disks, USB, GPU, bridged networking, host wallet keys, validator secrets and privileged ATC interfaces are not exposed automatically. Resource limits, capability grants, backend isolation, image validation, and explicit host/guest input handling are mandatory.

Catalog totals after this addition: **34 components / 136 function records**. This is a contract inventory, not proof of implementation completeness. Release remains `BLOCKED` until a backend and support matrix are selected and exact-head installation, isolation, recovery, UI integration and security evidence passes.

## User-facing controls, control panels and system areas — 2026-10-15

This catalog extension records ten UI/system-area components and twenty function contracts. It is architecture/control-plane metadata only: no UI, privileged service, wallet signing, consensus, virtualization backend or model runtime is implemented by this repository.

### Canonical ownership

| Component | Canonical owner | Scope |
|---|---|---|
| `COMP-UI-DESKTOP-SHELL` | `globus-os` | Desktop shell, launcher, taskbar, window/workspace management and session locking |
| `COMP-UI-CONTROL-PANEL` | `globus-os` | Display/audio/input/accessibility/network/storage/privacy settings via owner APIs |
| `COMP-SYSTEM-MONITOR` | `globus-os` | Process/resource telemetry, logs and bounded diagnostic reports |
| `COMP-SYSTEM-ADMIN` | `globus-os` | Service administration, updates, backup, restore and recovery controls |
| `COMP-DEVICE-MANAGEMENT` | `globus-os` | Device inventory, health and capability-gated access policy |
| `COMP-SECURITY-CENTER` | `globus-os` | Evidence-backed security posture, identity/key-provider status and policy controls |
| `COMP-ATC-DASHBOARD` | `a-townchain` | Node/peer/sync visibility and wallet/cross-chain operation status through canonical APIs |
| `COMP-AURORA-CONTROL-PANEL` | `aurora-ai` | Model/agent/runtime administration and exact-action approval/denial |
| `COMP-VIRTUALIZATION-CONTROL` | `globus-os` | Guest lifecycle, resource quotas and host integration policy for the proposed hosted-VM capability |
| `COMP-DEVELOPER-TOOLS` | `atc-sdk` | API/SDK diagnostics and exact-SHA build/test/artifact visibility |

### Function contracts added

- `FN-UI-WINDOW-001`, `FN-UI-SESSION-001`
- `FN-CTRL-SETTINGS-001`, `FN-CTRL-DEVICE-001`
- `FN-SYS-MONITOR-001`, `FN-SYS-DIAG-001`
- `FN-SYS-ADMIN-001`, `FN-SYS-RECOVERY-001`
- `FN-DEVICE-INVENTORY-001`, `FN-DEVICE-POLICY-001`
- `FN-SEC-POLICY-001`, `FN-SEC-POSTURE-001`
- `FN-ATC-DASH-001`, `FN-ATC-OPS-001`
- `FN-AURORA-APPROVAL-001`, `FN-AURORA-MODEL-001`
- `FN-VIRT-GUEST-001`, `FN-VIRT-POLICY-001`
- `FN-DEV-TOOLS-001`, `FN-DEV-TOOLS-002`

All 20 new function contracts have `source_binding.status = UNBOUND_RESIDUAL`; exact source paths, exported symbols and exact-head verification have not yet been established. Sensitive operations remain `BLOCKED` where authorization, host isolation or security evidence is a prerequisite. Other contracts remain `RESIDUAL`. The dashboard may not present unknown telemetry as healthy, infer consensus/finality from display state, expose key material, or grant implicit host/device access to guest VMs. UI approval does not create unrestricted or reusable authority.

### Verification boundary

The canonical implementation owners remain responsible for the actual UI, APIs and privileged behavior. Before any new contract can be marked implemented or verified, reconcile the owner repository's exact HEAD, source path, symbol/blob SHA, versioned interface, negative/security tests and terminal workflow Run→Job→Step/exit evidence. Source presence or a rendered screen is not proof of functional correctness. Release remains `BLOCKED`.

Catalog total after this extension: **44 components / 156 function contracts**. Inventory growth is not a completeness or verification claim.

## Ecosystem reference architecture — 2026-10-15

Reference architecture: [ATC-ECOSYSTEM-REFERENCE-ARCHITECTURE-001.md](ATC-ECOSYSTEM-REFERENCE-ARCHITECTURE-001.md).

The reference architecture connects existing component contracts across native GlobusOS installation, Windows/Linux host-app deployment, guest virtualization, security/capability boundaries, cryptographic providers, blockchain/VM semantics, cross-chain proof/finality, versioned APIs, Aurora tool execution and reproducible developer workflows. It is an architecture/control-plane document and does not create a second implementation SSOT.

Canonical implementation ownership is unchanged: a-townchain owns blockchain/node/ATC-VM/algorithm/wallet; globus-os owns GlobusOS and ShivaCore kernel; aurora-ai owns Aurora; atc-sdk owns SDK/CLI; atc-standards owns normative protocol/interface standards; this repository owns only catalog, integration, architecture decisions and evidence.

The document explicitly keeps the following unresolved gates visible: consensus/finality semantics, ATCB-versus-ATC1 bytecode format, EVM compatibility decision, independent custom-hash review, TX-V2 end-to-end vectors, cross-chain proof/finality trust model, native installer support matrix, guest-VM backend/isolation, API version/error/idempotency contracts, Aurora capability/approval tests, and bidirectional source reconciliation.

These additions do not change any implementation status. The catalog remains an inventory of mixed source-mapped and residual contracts; exact-HEAD workflow evidence is still required before any function or release gate can be marked VERIFIED.


## API Orchestrator ownership audit — 2026-10-15

### Audit result

The catalog now records `COMP-API-ORCHESTRATOR`, eleven residual function contracts (`FN-API-ORCH-001..011`) and proposed interface `IF-010`. This is an additive update to the existing JSON SSOT, not a second catalog.

**Canonical implementation owner: UNRESOLVED.** The active repository search found no executable `APIOrchestrator` implementation in `a-townchain-os`. The located Python file is in `a-townchain-os-docs` at commit `1c4bfd2c9cd1c43deb0b88d3a4bfde94fc9d106c`, path `docs/wiki/kai-os/code/backend/api/orchestrator/orchestrator.py`, blob `06a23d546da81b44cdcb035cadab71046c89d114`; the ATCLang artifact is at `wiki/kai-os/backend/api/orchestrator/orchestrator.atc`, blob `f68761008b8ec349ee0a4d24b5c2907fbef2ad86`. These are documentation-repository artifacts, and archived monorepo copies are reference-only. They do not establish a production runtime, owner, compiled ATCLang artifact, or successful tests.

The `a-townchain-os` integration/validation role must not be conflated with ownership of a backend API runtime. Ownership remains explicitly unassigned until an active executable source, exact repository HEAD, path/symbol, and test entry point are identified.

### Contract inventory

- `FN-API-ORCH-001`: service registry and health
- `FN-API-ORCH-002`: typed routing and version negotiation
- `FN-API-ORCH-003`: capability and policy authorization
- `FN-API-ORCH-004`: bounded queue, concurrency and backpressure
- `FN-API-ORCH-005`: deadlines, cancellation, retry and idempotency
- `FN-API-ORCH-006`: load balancing and circuit-breaker recovery
- `FN-API-ORCH-007`: event delivery, correlation and replay semantics
- `FN-API-ORCH-008`: structured errors, tracing and redacted audit
- `FN-API-ORCH-009`: graceful shutdown, recovery and resource bounds
- `FN-API-ORCH-010`: secure external service adapters
- `FN-API-ORCH-011`: standalone Windows/Linux and GlobusOS runtime integration
- `IF-010`: proposed API-Orchestrator-to-chain service-dispatch contract

Every function remains `RESIDUAL` with `UNBOUND_RESIDUAL` source binding. No function is `VERIFIED`. No gateway, Aurora, wallet, kernel, chain, or VM implementation is duplicated. Normative contract work remains tracked in [atc-standards Issue #111](https://github.com/A-TownChain-Okosystems/atc-standards/issues/111).

### Required follow-up

1. Identify the active executable owner and record exact repository HEAD, source path, symbol and blob SHA.
2. Bind only source-backed functions; keep unresolved records residual or blocked.
3. Run the catalog validator and source reconciliation against the updated branch.
4. Execute authorization, routing, deadline/idempotency, overload, breaker recovery, event replay, redaction, shutdown and Windows/Linux tests.
5. Attach exact-SHA Run→Job→Step, terminal conclusion, exit code, logs and artifact hashes before any status promotion.

Catalog totals after this additive update: **45 components / 167 function contracts / 10 cross-component interfaces**. These are inventory counts, not implementation-completeness or verification claims. Release remains `BLOCKED`.


## Maintenance System — ATC-STD-MAINT-000 (2026-10-15)

Normative parent-standard work is tracked in [atc-standards Issue #112](https://github.com/A-TownChain-Okosystems/atc-standards/issues/112). The existing catalog has been extended with `COMP-MAINTENANCE-LIFECYCLE`, `FN-MAINT-000..024`, and residual interfaces `IF-011` and `IF-012`.

### Lifecycle

`DETECT → ASSESS → CLASSIFY → PLAN → APPROVE → IMPLEMENT → TEST → VALIDATE → DEPLOY → MONITOR → EVIDENCE → CLOSE`

Risk classes: `M0` routine; `M1` operational; `M2` security-sensitive; `M3` critical/trust-boundary changes requiring independent review, explicit human approval, recovery planning and post-deployment verification.

### Ownership boundary

- `atc-standards` owns the normative lifecycle, risk, approval and evidence contract.
- Each implementation repository remains the SSOT for its own runtime, package, wallet, VM, node, kernel, Aurora or OS behavior.
- `a-townchain-ecosystem` owns integration/catalog/compliance/evidence orchestration only; it must not become a second updater or domain implementation.
- Maintenance actions must delegate to canonical owner APIs and must not bypass owner authorization or capability checks.

### Initial state and evidence rule

All 25 maintenance function contracts are `RESIDUAL` with `UNBOUND_RESIDUAL` source bindings. The component verification gate is `BLOCKED`. No implementation or verification is claimed. A maintenance run may be called `VERIFIED` only with exact-SHA terminal Run→Job→Step, conclusion, exit code, logs and artifact hashes; green CI alone is insufficient.

Catalog inventory after the additive update: **47 components / 227 function contracts / 12 cross-component interfaces**. These are inventory counts, not proof of implementation completeness or release readiness.


## Ecosystem Connection Matrix — what must connect where (2026-10-15)

The machine-readable matrix is stored in the existing JSON catalog under `connection_matrix` (schema `1.0.0`). It records **115 unique provider→consumer connections** derived from catalog dependencies plus explicit critical paths. These are declared integration requirements, not proof of working interfaces.

### Integration map and required order

| Priority | Provider → consumer | What must be integrated | Gate before treating as available |
|---|---|---|---|
| P0 | `COMP-STANDARDS → component owners` | Normative schemas, versioning, encodings, error semantics, conformance vectors | Standard registry entry + owner/source bindings |
| P0 | `COMP-SHIVA → COMP-GLOBUS` | Kernel syscall/capability/IPC ABI | ABI version, privilege tests, IPC/error tests |
| P0 | `COMP-CHAIN ↔ COMP-ALGO` | Consensus inputs, canonical encodings and transition/error semantics | Consensus decision and exact conformance vectors; unknown finality stays BLOCKED |
| P0 | `COMP-CHAIN ↔ COMP-VM` | Execution context, state access, deterministic results, resource/gas accounting | Bytecode/runtime compatibility vectors and deterministic replay tests |
| P0 | `COMP-TOOLCHAIN → COMP-VM` and `COMP-LANG → COMP-TOOLCHAIN` | Compiler/build artifact to VM input, diagnostics and bytecode format | Canonical ATCB decision, versioned schema and end-to-end vectors |
| P0 | `COMP-NODE → COMP-SDK / COMP-WALLET / COMP-ATC-DASHBOARD` | RPC, transaction submission, typed errors, receipt and status reporting | Versioned RPC schema, negative tests, exact source and CI evidence |
| P0 | `COMP-WALLET → COMP-CHAIN` | Canonically encoded signed transactions and transaction result reconciliation | TX-V2 vectors, signature/domain/replay tests; chain remains authoritative |
| P0 | `COMP-GLOBUS → COMP-AURORA` | Capability-scoped OS/service API, quotas, policy and audit | Deny-by-default and privilege-boundary tests |
| P0 | `COMP-AURORA → approval-gated COMP-WALLET path` | AI-requested wallet actions with explicit user approval | No direct key/seed access; approval-state and audit-redaction tests |
| P1 | `COMP-PLATFORM-DISTRIBUTION → native GlobusOS + Windows/Linux host targets` | Package/install/update/rollback delegation, platform support matrix | Exact host/hardware tests, signed artifacts and recovery evidence |
| P1 | `COMP-GLOBUS / COMP-SHIVA → COMP-GUEST-OS-VIRT` | Guest lifecycle, isolation, device/display/input boundary and resource quotas | Backend decision, host/guest matrix, isolation and recovery tests; currently BLOCKED |
| P1 | `COMP-MAINTENANCE-LIFECYCLE → canonical owners → release gate` | Drift/risk/approval, owner-delegated action, rollback and evidence collection | Per-target exact SHA, run/job/step, exit code/logs/artifact hashes; missing evidence blocks release |
| P1 | `COMP-STANDARDS → COMP-ECOSYSTEM / COMP-ATC-ENG` | Normative registry, requirement-to-contract mapping and policy validation | Schema/registry validator output tied to exact commit |
| P1 | `COMP-SDK → COMP-NODE / COMP-CHAIN` | Provider compatibility, transaction vectors, chain identity and typed RPC errors | Cross-version compatibility tests and reproducible vectors |
| P2 | `COMP-IDE → COMP-TOOLCHAIN / COMP-VM` | Canonical compile/run/diagnostic delegation and source mapping | No second compiler/VM; reproducible build and integration tests |
| P2 | `COMP-CONTRACTS → COMP-CHAIN / COMP-VM` | Contract registry, deployment references and runtime interface | Schema/version, authorization and deterministic execution tests |
| P2 | `COMP-LAUNCHPAD / COMP-MARKET → COMP-CONTRACTS / COMP-CHAIN` | Product/domain workflows against canonical contract and chain APIs | Auth, idempotency, typed errors and end-to-end tests |
| P2 | `COMP-STORAGE → COMP-SHIVA` | Capability-scoped storage, integrity metadata and recovery | Access-boundary, integrity and restore tests |
| P2 | `COMP-ZKP → COMP-CHAIN` | Proof/public-input encoding and verification result contract | Normative approval and positive/negative vectors; no security property inferred |
| P2 | `COMP-GENESIS → COMP-CHAIN / COMP-GLOBUS / COMP-TOOLCHAIN` | Genesis workflow, chain state, OS service and build artifact contracts | Versioned schemas, reproducibility and end-to-end evidence |

### Matrix rules

- **Provider** supplies the contract or service; **consumer** depends on it. Existing `cross_component_interfaces` remain cross-referenced; their direction is not silently rewritten.
- `RESIDUAL` means required but not source-bound/conformance-tested; `BLOCKED` means the architecture/implementation decision must be resolved first; `EXCLUDED` means no production connection is implied.
- Every production edge must carry canonical repository, exact full commit SHA, source path/symbol, interface version, failure semantics, security boundary, compatibility tests and terminal CI evidence.
- No edge becomes `VERIFIED` merely because it exists in `depends_on`, a catalog row or a green CI summary. The live drift gate must continue to exclude `demo-repository`.
- Ownership remains unchanged: implementation SSOTs stay in their canonical repositories; `a-townchain-ecosystem` maintains the integration matrix, compliance mapping and evidence only.

**Current matrix inventory:** 115 matrix connections; 116 declared dependency edges; 112 unique dependency pairs; 3 additional critical paths; 2 blocked; 1 excluded; 0 verified. These counts are planning state, not a completeness or release-readiness claim.


## Shared UI Controls and Interaction Contracts (2026-10-15)

The existing catalog now includes `COMP-UI-PRIMITIVES` for shared GlobusOS UI controls. It is a catalog/control contract, not a claim that a toolkit has been implemented or source-verified. Canonical UI source path and symbols remain unresolved; all 35 new function contracts are `RESIDUAL` / `UNBOUND_RESIDUAL`.

### Controls covered

| Group | Catalog contracts | Required integration |
|---|---|---|
| Settings and options | settings pages, category navigation, option panels | Delegate each setting to its owning system/service API; validate schema, scope and policy |
| Menus and navigation | application menus, context menus, tabs, accordions | Stable action IDs, keyboard navigation, stale-state checks and execution-time authorization |
| Input controls | dropdowns, sliders, switches, buttons, radio groups, checkboxes, text/numeric inputs and forms | Typed values, min/max/step/units, validation, pending/error states and duplicate-submit protection |
| Physical/device controls | lever/actuator widgets, device controls | Commands are requests, not proof of observed physical state; device owner enforces interlocks, limits and authorization |
| Layout and positioning | window/control position, docking, responsive layout, drag/drop/resize | Valid display/workspace bounds, constraints, keyboard alternatives and restoration |
| Action feedback | dialogs/confirmation, status indicators, notifications, progress, empty/error views | Distinguish requested, pending, observed, failed and completed states; confirmation binds to exact action |
| Accessibility and presentation | focus/keyboard/shortcuts, accessibility semantics, themes/scaling, localization | Accessible names/roles/states, keyboard operation, locale-independent IDs and policy-respecting appearance |
| State and event lifecycle | persistence/restoration, event routing, undo/redo, apply/save/cancel/reset, permission-aware state | Versioned state, no restoration of trusted authorization, typed owner API calls and fail-closed privileged actions |

### UI integration boundaries

- `COMP-UI-PRIMITIVES` is a shared component owned by the GlobusOS implementation repository, subject to confirmation of the canonical source path. It is consumed by desktop shell, system settings, monitor, administration/recovery, device management, security center, ATC dashboard, Aurora control panel, virtualization control panel and developer tools.
- UI controls emit typed action requests. **The owning service** validates inputs, permissions, policy, interlocks and state transitions. A disabled button, hidden menu item, slider position or switch state is never an authorization boundary.
- For a switch or lever, distinguish the *requested value*, *accepted command*, and *observed effective state*. Never display success solely because the user moved the control.
- Position, window geometry and docking belong to UI/window-management services; they must not confer access to the represented process, file, device or guest OS.
- Persistent UI state is versioned and limited to non-sensitive preferences/layout. Secrets, capabilities, approvals and authentication state must not be restored as trusted state.
- Each control requires keyboard/focus behavior, accessibility semantics, validation, loading/disabled/unavailable/error states, and negative tests. Security-sensitive actions additionally require owner-side authorization and audit/redaction tests.

### Updated matrix inventory

**125 matrix connections; 129 declared dependency edges; 125 unique dependency pairs; 3 additional critical paths; 2 BLOCKED; 1 EXCLUDED; 0 VERIFIED.** These are catalog counts only. No implementation or accessibility/security compliance is inferred.


## GlobusOS UI source reconciliation — exact HEAD 2026-10-15

Repository: [`globus-os`](https://github.com/A-TownChain-Okosystems/globus-os/commit/3bf000143ec9bfad79d0bacc36b99b8111178b95)  
Commit SHA: `3bf000143ec9bfad79d0bacc36b99b8111178b95` · root tree SHA: `66413b78b5a9c09bdb2988ad33e6bcf8e71bc1fa`

A partial source reconciliation binds six existing UI-adjacent contracts to exact files and blob SHAs. These are source mappings only; readiness remains `RESIDUAL` and no function is marked `VERIFIED`.

| Function contract | Exact source path | Source blob SHA | Symbol / scope |
|---|---|---|---|
| `FN-UI-POSITION-001` | `system/graphics/src/wm.rs` | `eeee511cffe49b2d70915975926f29a3c911855b` | `WindowManager::{open,set_state,placements}` — window state/placement only |
| `FN-UI-LAYOUT-001` | `system/graphics/src/compositor.rs` | `2c1e1001ab2b7734cc4e55c89ba6ec16b4660014` | `Rect`, `SurfacePlacement`, `Compositor::{create,focus}` |
| `FN-UI-FOCUS-001` | `system/graphics/src/input.rs` | `1b4c03b8a3abe6116bd6092dc98b6f72abc3a72f` | `InputRouter::{set_pointer_focus,set_keyboard_focus,route}` |
| `FN-UI-EVENT-001` | `system/graphics/src/input.rs` | `1b4c03b8a3abe6116bd6092dc98b6f72abc3a72f` | `InputRouter::route` |
| `FN-UI-STATE-001` | `system/graphics/src/desktop.rs` | `ee3e839dae95e2b8552d7c94f3b690b3f321b808` | `DesktopSession::{mode,lock,suspend,resume,shutdown}` |
| `FN-UI-NOTIFICATION-001` | `system/graphics/src/shell.rs` | `392a6aaa3968116748be970d901c383b59fa482e` | `DesktopShell::{notify,dismiss_notification,notifications}` |

Additional inspected candidates include `system/settings/src/lib.rs` (capability-gated settings core) and the ATCLang draft modules under `modules/atc-globus-desktop/src/`. The latter include TODO/planned functions and are not treated as completed UI widgets. No generic reusable dropdown, slider, switch, button, menu or form toolkit was established in this pass.

All six bindings use `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED`; the six function readiness values remain `RESIDUAL`. The component source inventory remains incomplete and exported-symbol reconciliation is false. Required next evidence: full source census, owner-service interface mapping, exact-head UI/API and negative-path tests, accessibility/security tests, and Run → Job → Step terminal results with exit codes/logs/artifact hashes.
