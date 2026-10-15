# ATC Component and Function Catalog 001

**Catalog state:** `BASELINE_RESIDUAL`  
**Release decision:** `BLOCKED`  
**Machine-readable SSOT:** [ATC-COMPONENT-FUNCTION-CATALOG-001.json](ATC-COMPONENT-FUNCTION-CATALOG-001.json)

This catalog is the architecture/control-plane inventory. It does not replace component source code, normative standards, or repository-local API documentation.

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
