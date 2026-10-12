# ATC-SOURCE-TO-CATALOG-RECONCILIATION-001

- Schema: `ATC-SOURCE-TO-CATALOG-RECONCILIATION-001`
- Observation date: 2026-10-12 (GitHub commit search; timestamps are UTC)
- Related audit: [Issue #112](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/112)
- Baseline inventory: [ATC-SOURCE-INVENTORY-001.json](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/blob/audit/source-function-inventory-001-2026-10-11/docs/architecture/ATC-SOURCE-INVENTORY-001.json), observed 2026-10-11
- Catalog: [ATC-COMPONENT-FUNCTION-CATALOG-001.json](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/blob/audit/source-function-inventory-001-2026-10-11/docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json), blob SHA `ffda6f0622546bf245ae7acb6c801fe7d7fe0448`
- Catalog baseline: 30 components, 63 function contracts; status `BASELINE_RESIDUAL`; release decision `BLOCKED`

## Scope and interpretation

This is the first **commit-reconciliation matrix**, not a completed symbol/function audit. The SHA in “Latest observed commit” is the newest commit returned by GitHub commit search for that repository on 2026-10-12. Where it differs from the 2026-10-11 inventory, the default-branch tip and tree SHA must be re-read from the branch ref before treating the row as current-head verified. Tree SHAs and source-file counts in the baseline inventory are not silently carried forward as current for changed repositories.

No source symbol scan, API/ABI compatibility test, or terminal CI evidence is implied by a matching SHA. All function inventories remain `RESIDUAL` until source paths and symbols are extracted at a pinned SHA and reconciled to catalog IDs. This matrix does not close, combine, or downgrade the separate security audit or the 16 prioritized vulnerabilities.

## Soll-Ist matrix

| Repository | Canonical responsibility / role | 2026-10-11 inventory SHA | Latest observed commit SHA | Commit reconciliation | Function/source status |
|---|---|---|---|---|---|
| `a-townchain-os` | OS orchestration / integration; no duplicate component implementations | `3f85a6e61b3cbf0a0b55590c90e2cd06bd328ed8` | `3f85a6e61b3cbf0a0b55590c90e2cd06bd328ed8` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `atclang` | Canonical ATCLang implementation | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `a-townchain-os-docs` | Architecture documentation hub | `6b311f70dc12517d2e09b83297e96035aebed035` | `f65992609ffcdd7e8eee4c29ac7a455da07fa947` | DRIFT — re-pin main ref/tree | RESIDUAL — symbol scan pending |
| `atc-shivacore` | Kernel specs/governance/support only | `21c99769654ff5915c421c3446e62c2a29c22b40` | `21c99769654ff5915c421c3446e62c2a29c22b40` | MATCHES BASELINE | RESIDUAL — spec/function mapping pending |
| `a-townchain` | Canonical blockchain core; VM/algorithm SSOT under `components/vm` and `components/algorithm` | `0419b444dec4749cec7b146950d1304f5a406c25` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | DRIFT — re-pin main ref/tree | RESIDUAL — symbol scan pending |
| `globus-os` | Canonical GlobusOS and ShivaCore kernel source at `modules/atc-shivacore/kernel/` | `fef3af85ff9ecab90c25bc179e2a3284fa17f633` | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` | DRIFT — re-pin main ref/tree | RESIDUAL — symbol scan pending |
| `aurora-ai` | Canonical Aurora AI runtime; no kernel TCB, consensus, or signing authority | `b16a64fc09b295d7c96234cda4f334fb129d6790` | `1b5ff1824bb032958b9a2ae4e53365d7af01c3af` | DRIFT — re-pin main ref/tree | RESIDUAL — symbol scan pending |
| `genesis-engine` | Genesis Engine / application runtime | `c16b2898c67159a7a7d3b30309a503a11a268d67` | `8326eb53e3e10cf08518e6769b44d4f034de1144` | DRIFT — re-pin main ref/tree | RESIDUAL — symbol scan pending |
| `atc-sdk` | Developer SDK / CLI / package tooling | `34eef345907935cdaf279e0546f230c03c86bb0c` | `34eef345907935cdaf279e0546f230c03c86bb0c` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `atc-node` | Node runtime and protocol boundary | `1ee3a3aee6d66681debca7934cd9aaa9e1051ee2` | `a817db43a089d172dd4688f366286c60dbf71a0e` | DRIFT — re-pin main ref/tree | RESIDUAL — symbol scan pending |
| `atc-contracts` | Contract tooling/runtime; reconcile with canonical VM/registry ownership | `a7c8b417f482665ef30e94fcacae6469c28b1125` | `a7c8b417f482665ef30e94fcacae6469c28b1125` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `genesis-chronicles` | Genesis history / chronicles | `a3f417b2350bb1cb920fd9a8f7b480ce7a67f5ab` | `a3f417b2350bb1cb920fd9a8f7b480ce7a67f5ab` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `atc-launchpad` | Launchpad application/service | `bd1a48c69983247a8428f90a77edd061ba79316b` | `bd1a48c69983247a8428f90a77edd061ba79316b` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `atc-marketplace` | Marketplace application/service | `5c3bab7da8a80d58af725b7dea2478ece2456f39` | `5c3bab7da8a80d58af725b7dea2478ece2456f39` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `atc-compute` | Compute service boundary | `d0831dfb72590537d8b0ca4d435a3649e71cdb61` | `d0831dfb72590537d8b0ca4d435a3649e71cdb61` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `atc-standards` | Normative standards and protocol contracts | `01368510be5d9325a8f051c881650161ee11f9fd` | `dc0ff69d8f1abb4bc4ba7a409d5bad20cf5e8f41` | DRIFT — re-pin main ref/tree | RESIDUAL — standard-to-code/test trace pending |
| `atc-vm` | Secondary/spec/support repository; canonical production VM remains `a-townchain/components/vm` | `21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0` | `21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0` | MATCHES BASELINE | RESIDUAL — canonical-source parity pending |
| `atc-algorithm` | Algorithm specification/support; canonical implementation remains `a-townchain/components/algorithm` | `5a5628044ef4daacca943062eac3fd27a2eaec93` | `5a5628044ef4daacca943062eac3fd27a2eaec93` | MATCHES BASELINE | RESIDUAL — canonical-source parity pending |
| `atc-zkp` | Zero-knowledge proof layer | `ffa8ec1098c5ae0aa0145964843abe0c6806276f` | `ffa8ec1098c5ae0aa0145964843abe0c6806276f` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `.github` | Organization-wide GitHub policy/configuration | `eeacedf33fd4b81046a3ee56eb1c815836bd9833` | `af5d803dace369b83e4bffd2e4e6ba001d4aa000` | DRIFT — re-pin main ref/tree | RESIDUAL — policy/workflow inventory pending |
| `demo-repository` | Demo/test fixture, not a production component | `d9b0da9c966dea02586b6be64139e6b3cd669dfd` | `d9b0da9c966dea02586b6be64139e6b3cd669dfd` | MATCHES BASELINE | RESIDUAL — explicit fixture exclusion required |
| `atc-ide` | Developer IDE | `2a17feda7ed136ea2df8c01e2c4848d2cb9cac8a` | `2a17feda7ed136ea2df8c01e2c4848d2cb9cac8a` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `genesis-franchise-factory` | Genesis franchise/factory application | `673fa560322b1449e8f23cc38bf839de1b246361` | `673fa560322b1449e8f23cc38bf839de1b246361` | MATCHES BASELINE | RESIDUAL — symbol scan pending |
| `atc-engineering` | Engineering governance and evidence controls | `470e6a9f500fdade0ea33065d30dd76b58d6492b` | `2e91ec61ac6680566cbe9b525406a9752c31e539` | DRIFT — re-pin main ref/tree | RESIDUAL — symbol scan pending |
| `a-townchain-ecosystem` | Architecture/integration/evidence control plane only | `4bc2c63868ee0b4df2456929d3a7d310bc7f4588` | `5a66f276a10ff02ac470d561d4ea260012229cec` | DRIFT — re-pin main ref/tree | RESIDUAL — catalog/integration controls pending |
| `atc-toolchain` | Build/compiler/toolchain | `82fe4a238603476d7c6c19d5c3044c88b71c5f2d` | `82fe4a238603476d7c6c19d5c3044c88b71c5f2d` | MATCHES BASELINE | RESIDUAL — symbol scan pending |

## Initial reconciliation results

- Organization census: 33 repositories; 26 active, 7 archived.
- Active repositories represented here: 26/26.
- Latest observed commit equals the 2026-10-11 inventory SHA: 16 repositories.
- Latest observed commit differs from the inventory SHA: 10 repositories. These rows require default-branch-tip and tree-SHA refresh before the inventory is accepted as current.
- Source-function completeness: **NOT VERIFIED**. The inventory's file counts are discovery metadata, not function counts.
- Component catalog remains 30 components / 63 function contracts, `BASELINE_RESIDUAL`; release remains `BLOCKED`.
- Security audit and the 16 prioritized vulnerabilities remain separate workstreams; this matrix is not their disposition.

## Next fail-closed steps

1. Resolve each DRIFT row against the actual `main` ref and recapture commit SHA + tree SHA at the same observation time.
2. Run language-aware symbol extraction against every pinned active-repository SHA; include public/exported functions, methods, services, syscalls, RPC routes, CLI commands, event handlers, workflows/config entrypoints and declarative `.atc` constructs as applicable.
3. Map each symbol to a catalog component/function ID or a documented exclusion. Detect missing, duplicate, stale and unmapped symbols.
4. Reconcile the catalog's 63 contracts against extracted symbols; do not infer functionality from repository names or file counts.
5. Record API/ABI compatibility tests and terminal CI evidence as Run → Job → Step → Exit Code → Log/Artifact. Pending/skipped is not pass.
6. Keep every unsupported claim `RESIDUAL`/ `BLOCKED`; no merge, gate relaxation, archive, delete or reactivation is authorized by this matrix.
