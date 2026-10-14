# ATC Component and Function Catalog 001

**Catalog state:** `BASELINE_RESIDUAL`  
**Release decision:** `BLOCKED`  
**Machine-readable SSOT:** [ATC-COMPONENT-FUNCTION-CATALOG-001.json](ATC-COMPONENT-FUNCTION-CATALOG-001.json)

This catalog is the architecture/control-plane inventory. It does not replace component source code, normative standards, or repository-local API documentation.

**Current machine-readable count (2026-10-13):** 33 components · 210 function/capability contracts · 19 interfaces · 0 functionally verified. **Source reconciliation:** `RESIDUAL` until every runtime candidate is mapped or explicitly excluded; the strict completion gate is fail-closed. The last completed census is historical for its recorded exact HEAD; the current-head rerun must finish before its counts are treated as current.

## Rules

- Each capability has a stable component ID and function ID.
- Each component names one canonical owner; specifications and support repositories are not treated as duplicate implementation owners.
- Each function records purpose, inputs, outputs, failure modes, measurable acceptance criteria, test classes, and readiness.
- Cross-component edges are explicit and remain `RESIDUAL` until their exact API/ABI and compatibility evidence are verified.
- `IMPLEMENTED` does not mean `VERIFIED`; `PRODUCTION_READY` requires full exact-SHA evidence, required gates, logs/artifacts and review.
- Every component records a machine-readable execution model (`LIBRARY`, `RUNTIME`, `LIBRARY_AND_RUNTIME`, `TOOLING`, `NON_EXECUTABLE_REFERENCE`) with rationale; it is an inventory dimension, not implementation or verification evidence.
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

## 2026-10-13 catalog-first expansion — L1 blockchain contracts

The machine-readable catalog now contains **114 function contracts across 30 components**. This revision adds 11 explicit L1 chain contracts (`FN-CHAIN-004`–`FN-CHAIN-014`) and 5 consensus/algorithm decision contracts (`FN-ALGO-003`–`FN-ALGO-007`).

These records define required behavior before implementation changes. All 16 new contracts are `RESIDUAL` with `source_binding.status = NOT_SOURCE_VERIFIED`; this means source mapping is pending, **not** that the function is absent from code. Consensus selection, quorum/finality and fork-choice remain blocked pending a versioned normative decision. The catalog still does not claim completeness: source-symbol reconciliation, internal-function coverage, API/RPC/CLI/event coverage, `.atc` extraction, tests and exact-SHA evidence remain open. Release decision remains `BLOCKED`.

## Exact-HEAD source census — 2026-10-13

Latest successful source inventory: [run 38319690185](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38319690185), artifact [11778885556](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38319690185/artifacts/11778885556).

- 33 repositories in the manifest: 25 active repositories scanned at exact HEAD, 1 private active repository explicitly excluded for lack of CI access, 7 archived repositories classified as reference-only; 0 active repositories blocked.
- 1,363 source files scanned; 15,000 files excluded by documented scope rules for noncanonical implementation mirrors and archived/wiki code copies.
- 7,849 public candidates and 4,014 internal Rust function candidates: 11,863 source-symbol candidates in scope.
- 84 candidates bound to catalog records; 10,838 candidates remain unbound; 941 test/example candidates are explicitly excluded by policy with path, symbol and reason.
- 114 catalog records across 30 components: 82 exact path/symbol lexical matches, 4 related source surfaces, 2 test surfaces, 10 non-function artifacts, 9 source surfaces not identified, 3 consensus decisions blocked, 2 archived-reference functions and 2 private-repository functions blocked.
- 0 functions functionally verified. Lexical matches are source-location evidence only; the catalog remains `BASELINE_RESIDUAL` and release remains blocked.

## Updated reconciliation after kernel/OS contract expansion — 2026-10-13

The prior 114-record snapshot is superseded by the latest catalog revision: **139 records across 30 components**. The catalog gained 18 ShivaCore kernel contracts and 7 GlobusOS service/platform contracts.

Latest exact-HEAD evidence: [Source Symbol Inventory #38320206333](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38320206333), [machine-readable artifact](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38320206333/artifacts/11778097539).

- 107 exact path/symbol lexical matches; 0 functions functionally verified.
- 10,815 candidate symbols remain unbound; 941 test/example symbols are explicitly excluded by path/symbol policy with reasons.
- 32 catalog records remain residual or blocked, including 9 source surfaces not identified and 3 consensus decision gates blocked.
- Source census: 25 active repositories scanned, 1 private repository excluded for lack of CI access, 7 archived repositories reference-only, 0 active repositories blocked.
- Catalog remains `BASELINE_RESIDUAL`; implementation work must not begin until the remaining candidates are reconciled or explicitly excluded.

## Cross-component interface contracts — 2026-10-13

The machine-readable catalog now contains **13 cross-component interfaces**. Added explicit contracts for TX-V2 serialization/signing (`IF-008`), L1 node admission (`IF-009`), canonical persistence (`IF-010`), L1 standards (`IF-011`), ZKP verification (`IF-012`) and the ShivaCore/GlobusOS boot-service boundary (`IF-013`).

`IF-002` is `BLOCKED` while the ATCB-versus-ATC1 bytecode magic conflict remains unresolved. `IF-003` is `BLOCKED` while consensus algorithm, finality and fork-choice lack an authoritative versioned decision. `IF-010` is `BLOCKED` because the storage repository is archived and canonical persistence ownership is unresolved.

## Exact source-binding discipline — 2026-10-13

Every lexical source binding must carry an explicit `symbol_kind` (`function`, `type`, `value`, or another scanner-supported kind). Reconciliation matches `(repository, exact source SHA, source path, symbol kind, symbol name)`; a type/value with the same spelling cannot satisfy a function contract.

Bindings to supporting types/values are `RELATED_SOURCE_SURFACE_ONLY`, not runtime function matches. Symbols found only in test/conformance paths are `TEST_SURFACE_ONLY` and must be linked to test evidence, not counted as runtime implementation coverage. Unmatched symbol records retain the repository HEAD SHA and source-file SHA-256 so every residual is traceable.

This is a source-mapping integrity gate only. It does not upgrade any record to `VERIFIED`; source parsing remains lexical and functional verification remains `NOT_PERFORMED`.

## Interface contract expansion — 2026-10-13

The catalog now contains 16 cross-component interface records. Each record carries explicit fields for contract version, request schema, response schema, error/timeout semantics, authorization/capability boundary, compatibility policy, contract-test classes, exact-SHA evidence, and a resolution rule. Unresolved schema/evidence fields remain `RESIDUAL`; the three pre-existing normative conflicts remain `BLOCKED`.

New records: `IF-014` Node ↔ SDK RPC/client boundary; `IF-015` Genesis Engine ↔ canonical L1 core; `IF-016` Node peer-ingress ↔ canonical chain admission. No wire format, protocol version, or consensus semantics is assumed where the source contract is not yet proven.

## Canonical wallet source coverage — 2026-10-13

Added `COMP-WALLET` for `a-townchain/components/wallet`, distinct from the archived standalone `atc-wallet` repository. Five function contracts are source-bound to exact repository HEAD `c631249ed6ff450bf45338e78cb14e2bf5c65c74` and file blob `2693b7ba015456addcc8d129455b7e5a9e2f1860`: signing preimage, transaction ID, signature creation, signature verification, and canonical field encoding.

Added `IF-017` for wallet-to-chain TX-V2 interoperability. Source mapping confirms the TX-V2 domain, TX-ID domain, fixed-width amount encoding and compact-signature checks in the wallet file; canonical chain-side verification and shared cross-client vectors remain `RESIDUAL`. This does not imply production readiness.

## Current reconciliation state (2026-10-13)

Current count: **31 components, 146 function/capability contracts, 17 interfaces**. Of the 146 function/capability records, 112 are exact-HEAD source-mapped (lexical/source mapping only), 12 still have no identified satisfying source surface, 3 are blocked by normative consensus decisions, and 19 are explicitly classified as non-function/test/archived/private-reference exclusions. **0 are functionally verified.**

The current source inventory distinguishes runtime candidates from inline Rust test-only functions and records test-only symbols as reasoned exclusions. The strict reconciliation gate remains blocked until every runtime candidate is mapped or has a reviewed exclusion and every source binding resolves at the exact recorded SHA.


## Node runtime source contract expansion (2026-10-13)

Added exact-HEAD source contracts for NodeConfig construction/validation, PeerTable lifecycle, Devnet RPC state/dispatch/listener/handler/parsing, gossip listener/peer request/sync/block parsing, and the process entrypoint. The catalog now has 173 function/capability records; 139 have exact source mappings, but this remains lexical evidence only.

Critical finding: `FN-NODE-022` records `TransactionDomain::signing_bytes` in `atc-node` as a **BLOCKED legacy path** because it uses `ATC-TX-DOMAIN` and string-field encoding rather than the approved TX-V2 domain/encoding. `IF-017` is therefore `BLOCKED`; no code was changed. The canonical TX-V2 wallet side is source-bound, but chain-side verifier compatibility remains unresolved.

## Canonical storage ownership and security limits (2026-10-13)

Added `COMP-STORAGE-CORE` for active `a-townchain/components/storage`; the archived standalone storage service is now separately identified as `COMP-STORAGE-ARCHIVED`. Twenty storage functions are bound to exact source paths, blobs and the same a-townchain HEAD.

Two hard blockers are recorded: `ContentAddress::compute` claims BLAKE3 compatibility but actually computes an FNV-1a-derived 256-bit value, and the current store/blob service is in-memory only. `mvp_root_hash` is explicitly non-cryptographic. Therefore `COMP-STORAGE-CORE` and `IF-018` are `BLOCKED` for canonical production state storage; `FN-CHAIN-012` remains unresolved for durable block persistence.

## TypeScript TX-V2 parity inventory (2026-10-13)

Added `COMP-SDK-TS` with ten exact-HEAD executable function bindings and seven explicitly classified non-function type/value surfaces. `IF-019` is `BLOCKED`: `canonicalTransactionId` hashes `canonicalIdFields` without enforcing all the same preconditions as `canonicalSigningPreimage`. Cross-language vectors and invalid-input parity remain open. Current catalog count: 33 components, 210 function/capability contracts, 19 interfaces; 169 functions are source-mapped, 0 functionally verified.

## Exact-SHA reconciliation update — 2026-10-13 (PR #115 HEAD)

This section supersedes the preceding **Current reconciliation state** counts for the exact run listed below; earlier counts remain historical snapshots.

- **Source HEAD:** `6ad06513754c4a8b0c93e362935f4403a799fbe0`
- **Run:** [Source Symbol Inventory #38394483388](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38394483388)
- **Artifact:** [exact-SHA census, reconciliation and cross-repository hash report](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38394483388/artifacts/11810359144)
- Census: 33 repository records; 25 of 26 active repositories scanned, one private active repository explicitly excluded due to missing CI access, seven archived repositories reference-only; 1,368 source files scanned.
- Catalog reconciliation: 33 components, 210 function/capability contracts, 19 interfaces; 12,980 source-symbol candidates in scope; 164 bound, 6,429 excluded by current policy and **6,387 remain unbound without a reasoned exclusion**.
- Candidate classes: 6,555 runtime candidates, 3,998 test candidates and 2,427 non-function candidates. The strict `--require-complete` gate exited 2 and reports `BLOCKED`.
- Unresolved binding findings: `AMBIGUOUS_SOURCE_SYMBOL_BINDING=1`; `BOUND_SYMBOL_NOT_FOUND_IN_LEXICAL_SCAN=1`.
- Cross-repository identical-file report: 1,261 unique file hashes; 780 hashes with multiple candidate rows; 82 cross-repository hash groups and 492 candidate rows. These are triage-only signals, not proof of shared ownership or duplicate implementation.
- Functional verification remains `NOT_PERFORMED`; **0 functions are functionally verified**.

The repository-level routing summary is [ATC-UNBOUND-SYMBOL-CLASSIFICATION-001.md](../audit/ATC-UNBOUND-SYMBOL-CLASSIFICATION-001.md), with its machine-readable companion [ATC-UNBOUND-SYMBOL-CLASSIFICATION-001.json](../audit/ATC-UNBOUND-SYMBOL-CLASSIFICATION-001.json). These are triage records: no unbound candidate is automatically added as a function contract. Relevant candidates must be reviewed and added to the owning repository's local catalog; the central catalog references those contracts without taking implementation ownership. Unknown targets remain `ROUTE_REVIEW_REQUIRED`, and reviewed non-contract symbols require a reasoned exclusion.

**Current decision:** catalog completion `BLOCKED`; PR remains open and draft. Do not begin implementation work from this census alone.

## Execution model (Library and Runtime)

Machine-readable dimension `execution_model` per component, validated by the catalog validator and mirrored in the candidate triage (`execution_model_counts`). It classifies how implementation code is deployed/consumed; it is not implementation or verification evidence.

| Execution model | Count | Components |
|---|---|---|
| LIBRARY | 9 | A-TownChain Blockchain Core (COMP-CHAIN), ATC-Algorithm (COMP-ALGO), ATC-VM (COMP-VM), ATC SDK / CLI / Package Tooling (COMP-SDK), ATC Contracts and Registry (COMP-CONTRACTS), ATC Zero-Knowledge Proof Layer (COMP-ZKP), Canonical L1 Wallet and Transaction Signer (COMP-WALLET), Canonical Monorepo Storage Core (COMP-STORAGE-CORE), Canonical TypeScript L1 Protocol Primitives (COMP-SDK-TS) || RUNTIME | 11 | ShivaCore Kernel (COMP-SHIVA), GlobusOS (COMP-GLOBUS), Aurora AI (COMP-AURORA), ATC Node Runtime (COMP-NODE), ATC Compute (COMP-COMPUTE), ATC Launchpad (COMP-LAUNCHPAD), ATC Marketplace (COMP-MARKET), ATC IDE (COMP-IDE), Genesis Chronicles (COMP-CHRONICLES), Genesis Franchise Factory (COMP-GFF), A-TownChain OS Integration (COMP-OS-INTEGRATION) || LIBRARY_AND_RUNTIME | 3 | ATCLang (COMP-LANG), ATC Toolchain (COMP-TOOLCHAIN), Genesis Engine (COMP-GENESIS) || TOOLING | 4 | atc-engineering (COMP-ATC-ENG), ATC Standards (COMP-STANDARDS), a-townchain-ecosystem (COMP-ECOSYSTEM), Organization GitHub Governance (COMP-ORG-GOV) || NON_EXECUTABLE_REFERENCE | 6 | Archived ATC Storage Service (Reference Only) (COMP-STORAGE-ARCHIVED), ShivaCore Specification Support (COMP-SHIVA-SPEC), ATC-VM Specification Support (COMP-VM-SPEC), ATC-Algorithm Specification Support (COMP-ALGO-SPEC), Architecture Documentation Hub (COMP-DOCS), Demo Repository Fixture (COMP-DEMO) |
