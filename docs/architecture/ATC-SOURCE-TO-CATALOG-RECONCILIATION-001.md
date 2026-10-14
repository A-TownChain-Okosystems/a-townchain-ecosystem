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


## Organization refresh attempt — 2026-10-13

**Result: UP-TO-DATE STATUS NOT VERIFIED.** This refresh queried the organization repository inventory and recent commit-search results. Repository metadata reports 33 repositories, all with default branch `main`: 26 active and 7 archived. The commit-search connector does not expose the resolved `refs/heads/main` SHA or its tree SHA. A commit-search hit is therefore recorded only as an observation, not as proof of the current default-branch tip. Do not use these SHAs as verified source bindings, build pins, or release evidence.

| Repository | Lifecycle from repository metadata | Latest commit-search hit (observation only) | Default-branch freshness |
|---|---|---|---|
| `a-townchain-os` | ACTIVE | `3f85a6e61b3cbf0a0b55590c90e2cd06bd328ed8` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atclang` | ACTIVE | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `a-townchain-os-docs` | ACTIVE | `f65992609ffcdd7e8eee4c29ac7a455da07fa947` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-shivacore` | ACTIVE | `21c99769654ff5915c421c3446e62c2a29c22b40` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `a-townchain` | ACTIVE | `c631249ed6ff450bf45338e78cb14e2bf5c65c74` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `globus-os` | ACTIVE | `7dbc7753032016cf6aeaf1a99807cd9c5173d090` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `aurora-ai` | ACTIVE | `ef880fc5b4469f9757839c3b0f3ef2ed368357c2` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `genesis-engine` | ACTIVE | `8326eb53e3e10cf08518e6769b44d4f034de1144` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-sdk` | ACTIVE | `34eef345907935cdaf279e0546f230c03c86bb0c` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-node` | ACTIVE | `a817db43a089d172dd4688f366286c60dbf71a0e` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-contracts` | ACTIVE | `a7c8b417f482665ef30e94fcacae6469c28b1125` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-wallet` | ARCHIVED | `812a31d292364304c72cae8be6f448c130d7428e` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-explorer` | ARCHIVED | `c4da05304f214bb68bd942e7705c96c35d9a18ae` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-indexer` | ARCHIVED | `7f3da49f30464471c8c1597ed59f6223050e79e4` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-mining` | ARCHIVED | `227f2d700307191f59596c5a78ccfd3a4b657aa9` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-interop` | ARCHIVED | `5905fca02274704a00bdf708948180bc211a46d3` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `genesis-chronicles` | ACTIVE | `a3f417b2350bb1cb920fd9a8f7b480ce7a67f5ab` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-oracle` | ARCHIVED | `8087d34ddb8c2d40a60178e8f3b4c52ea67d52a8` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-storage` | ARCHIVED | `230746c1dc5e4c1355c2707368de50d6c1008660` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-launchpad` | ACTIVE | `bd1a48c69983247a8428f90a77edd061ba79316b` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-marketplace` | ACTIVE | `5c3bab7da8a80d58af725b7dea2478ece2456f39` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-compute` | ACTIVE | `89d833edf338085d6141b0c703ebf92a0db83a5e` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-standards` | ACTIVE | `d6556168a21ed5a9b5d917262bd2a4ee196fdd46` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-vm` | ACTIVE | `21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-algorithm` | ACTIVE | `5a5628044ef4daacca943062eac3fd27a2eaec93` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-zkp` | ACTIVE | `ffa8ec1098c5ae0aa0145964843abe0c6806276f` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `.github` | ACTIVE | `af5d803dace369b83e4bffd2e4e6ba001d4aa000` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `demo-repository` | ACTIVE | `d9b0da9c966dea02586b6be64139e6b3cd669dfd` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-ide` | ACTIVE | `2a17feda7ed136ea2df8c01e2c4848d2cb9cac8a` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `genesis-franchise-factory` | ACTIVE | `673fa560322b1449e8f23cc38bf839de1b246361` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-engineering` | ACTIVE | `c2a4b93fb01178ef905f62d8a5fa6f42215d25a2` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `a-townchain-ecosystem` | ACTIVE | `2b6b885221416673333a49fb9327374d4deebf8c` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |
| `atc-toolchain` | ACTIVE | `82fe4a238603476d7c6c19d5c3044c88b71c5f2d` | NOT VERIFIED — search result only; default-branch ref/tree SHA not established |

### Findings and required next step

- Inventory count is confirmed at **33 repositories: 26 active, 7 archived**.
- The seven archived repositories are `atc-wallet`, `atc-explorer`, `atc-indexer`, `atc-mining`, `atc-interop`, `atc-oracle`, and `atc-storage`. They remain visible in the audit but are excluded from the active implementation/build graph unless a separately approved lifecycle decision changes that.
- **0/33 repositories are marked VERIFIED as current-main** by this refresh because the exact main-ref SHA + tree SHA pair was not available from the read surface. This is a verification limitation, not a claim that every repository is stale.
- Do not fast-forward or overwrite repositories merely because a commit-search result differs from a previous inventory. Resolve each repository's actual `main` ref and tree SHA, compare with the recorded source snapshot and integration/subtree pins, then inspect divergence and CI evidence.
- The next full audit must record, for each repository: `main` HEAD SHA, tree SHA, observation timestamp (UTC), ahead/behind relationship where a comparison base exists, open PRs affecting canonical paths, required workflow results, and documentation/catalog drift.
- Documentation and generated inventories must be refreshed from those resolved refs. Keep status `BLOCKED` or `RESIDUAL` where evidence is missing; do not imply that green repository-level CI proves cross-repository integration or production readiness.


## Exact default-branch ref snapshot — 2026-10-13

The previous refresh limitation is narrowed: direct GitHub REST reads of `refs/heads/main` succeeded for all 33 repositories in the organization inventory. The values below are **observed branch HEAD commit SHAs**, not commit-search hits. This establishes ref identity at observation time; it does not by itself establish CI health, tree-level reproducibility, integration compatibility, or that a repository needs a content sync.

| Repository | Lifecycle | Default branch | Observed `main` HEAD |
|---|---|---|---|
| `a-townchain-os` | active | `main` | `3f85a6e61b3cbf0a0b55590c90e2cd06bd328ed8` |
| `atclang` | active | `main` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` |
| `a-townchain-os-docs` | active | `main` | `f65992609ffcdd7e8eee4c29ac7a455da07fa947` |
| `atc-shivacore` | active | `main` | `21c99769654ff5915c421c3446e62c2a29c22b40` |
| `a-townchain` | active | `main` | `c631249ed6ff450bf45338e78cb14e2bf5c65c74` |
| `globus-os` | active | `main` | `7dbc7753032016cf6aeaf1a99807cd9c5173d090` |
| `aurora-ai` | active | `main` | `ef880fc5b4469f9757839c3b0f3ef2ed368357c2` |
| `genesis-engine` | active | `main` | `8326eb53e3e10cf08518e6769b44d4f034de1144` |
| `atc-sdk` | active | `main` | `34eef345907935cdaf279e0546f230c03c86bb0c` |
| `atc-node` | active | `main` | `a817db43a089d172dd4688f366286c60dbf71a0e` |
| `atc-contracts` | active | `main` | `a7c8b417f482665ef30e94fcacae6469c28b1125` |
| `atc-wallet` | archived | `main` | `812a31d292364304c72cae8be6f448c130d7428e` |
| `atc-explorer` | archived | `main` | `c4da05304f214bb68bd942e7705c96c35d9a18ae` |
| `atc-indexer` | archived | `main` | `7f3da49f30464471c8c1597ed59f6223050e79e4` |
| `atc-mining` | archived | `main` | `227f2d700307191f59596c5a78ccfd3a4b657aa9` |
| `atc-interop` | archived | `main` | `5905fca02274704a00bdf708948180bc211a46d3` |
| `genesis-chronicles` | active | `main` | `a3f417b2350bb1cb920fd9a8f7b480ce7a67f5ab` |
| `atc-oracle` | archived | `main` | `8087d34ddb8c2d40a60178e8f3b4c52ea67d52a8` |
| `atc-storage` | archived | `main` | `230746c1dc5e4c1355c2707368de50d6c1008660` |
| `atc-launchpad` | active | `main` | `bd1a48c69983247a8428f90a77edd061ba79316b` |
| `atc-marketplace` | active | `main` | `5c3bab7da8a80d58af725b7dea2478ece2456f39` |
| `atc-compute` | active | `main` | `89d833edf338085d6141b0c703ebf92a0db83a5e` |
| `atc-standards` | active | `main` | `d6556168a21ed5a9b5d917262bd2a4ee196fdd46` |
| `atc-vm` | active | `main` | `21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0` |
| `atc-algorithm` | active | `main` | `5a5628044ef4daacca943062eac3fd27a2eaec93` |
| `atc-zkp` | active | `main` | `ffa8ec1098c5ae0aa0145964843abe0c6806276f` |
| `.github` | active | `main` | `af5d803dace369b83e4bffd2e4e6ba001d4aa000` |
| `demo-repository` | active | `main` | `d9b0da9c966dea02586b6be64139e6b3cd669dfd` |
| `atc-ide` | active | `main` | `2a17feda7ed136ea2df8c01e2c4848d2cb9cac8a` |
| `genesis-franchise-factory` | active | `main` | `673fa560322b1449e8f23cc38bf839de1b246361` |
| `atc-engineering` | active | `main` | `c2a4b93fb01178ef905f62d8a5fa6f42215d25a2` |
| `a-townchain-ecosystem` | active | `main` | `2b6b885221416673333a49fb9327374d4deebf8c` |
| `atc-toolchain` | active | `main` | `82fe4a238603476d7c6c19d5c3044c88b71c5f2d` |

### Synchronization decision

- Repository inventory: **33/33 refs resolved**; 26 active and 7 archived.
- Archived repositories were read only; no writes were made to them.
- This snapshot does **not** declare all repositories synchronized. No repository has been advanced, reset, force-pushed, or merged as part of this pass.
- Remaining gates before any source synchronization: resolve each commit's tree SHA; compare canonical source repositories with integration/catalog pins; inspect ahead/behind and open PR divergence; collect required CI status for each exact HEAD; then create narrowly scoped PRs for confirmed drift.
- Status: `REF_SNAPSHOT=VERIFIED`; `TREE_AND_CI_AUDIT=RESIDUAL`; `CROSS_REPOSITORY_SYNC=BLOCKED_PENDING_DIFF_AND_GATE_EVIDENCE`.
