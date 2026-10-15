# WP-M0 Baseline Checkpoint — Repository Scope and Revision Evidence

- **Checkpoint ID:** `ATC-WP-M0-CP0-001`
- **Assessment status:** `BLOCKED`
- **Overall work-package status:** `PROPOSED / NOT VERIFIED`
- **Assessment date:** 2026-10-15
- **Target catalog PR:** [#184](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/184)
- **Coordinating issue:** [#185](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/185)
- **Existing manifest:** [ATC-ORG-REPOSITORY-MANIFEST-001.json](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/blob/main/docs/audit/ATC-ORG-REPOSITORY-MANIFEST-001.json)
- **Manifest blob SHA observed on `main`:** `5400a61343e33a4d1816f305a7d22a34fecf2a6c`

## 1. Scope and inventory result

A live organization repository listing returned **33 repositories**, all with default branch `main`. The current manifest also contains 33 entries. Cross-checking the live listing with the manifest yielded the same repository-name set in the retrieved page.

| Classification | Count | Result |
|---|---:|---|
| Repositories in live listing | 33 | Observed |
| Manifest entries | 33 | Observed |
| Archived repositories | 7 | Matches manifest |
| Non-archived repositories | 26 | Matches manifest |
| `demo-repository` | 1 | Private; explicitly excluded from live drift gate |

The repository listing is an observation of the connected GitHub account and organization at the time of this assessment, not a signed or immutable organization snapshot.

## 2. Revision-pinned baseline — API objects resolved

The previous limitation is superseded by the direct GitHub REST reads performed on 2026-10-15. The live organization listing returned 33 repositories and reported each default branch as `main`. For each repository, the branch ref endpoint `GET /repos/{owner}/{repo}/git/ref/heads/main` returned a commit object SHA; the Git database commit endpoint for that exact SHA returned its root `tree.sha`.

- **Default-branch HEAD SHA coverage:** 33/33
- **Root-tree SHA coverage:** 33/33
- **Valid 40-character lowercase SHA checks:** 33/33 HEAD and 33/33 tree values
- **Live drift-gate scope:** 32 repositories
- **Explicit live drift-gate exclusion:** `demo-repository` (still inventoried and pinned; exclusion does not remove it from the 33-repository inventory)
- **Automated validator execution:** pending
- **CI Run → Job → Step / exit-code logs / artifact SHA:** pending
- **Independent reviewer decision:** pending

The revision pins below are captured observations, not permanent assertions. Any movement of a default-branch HEAD after this snapshot is expected to fail the comparison until the baseline is deliberately refreshed and reviewed.

| Repository | Default branch | HEAD commit SHA | Root tree SHA | Drift gate |
|---|---|---|---|---|
| `.github` | `main` | `8106d7c060effd45050d29ba80bb0416866f1b01` | `2e468fc0c302926e50a7756df816bb8d7e86edc6` | included |
| `a-townchain` | `main` | `a5e689671b1870699c7a1750f8a166e83590a41c` | `a6bd467b900560cc2c6fca88ce633afb8401c864` | included |
| `a-townchain-ecosystem` | `main` | `c83bf4e581f121f374f5d6619ceab20f97f9d1a1` | `e6b2fc6950cde47d0b36b376459a8afc083cbd29` | included |
| `a-townchain-os` | `main` | `a929fe77c65e3a6da4a2c5805f3e290291d47aaa` | `b9ab44e7d283ce9b9ba2906b2e5d1853391782ef` | included |
| `a-townchain-os-docs` | `main` | `1c4bfd2c9cd1c43deb0b88d3a4bfde94fc9d106c` | `b4b1595a40eba29ce9059f8c332999fc4874d7d1` | included |
| `atc-algorithm` | `main` | `f25b43383433766c43df6c5c494ecb70cdb4a4b7` | `bce31cbd81b9c4a9dfc65b0b77fcd29727ea4c13` | included |
| `atc-compute` | `main` | `89d833edf338085d6141b0c703ebf92a0db83a5e` | `e140d5a5bc884bd56d037296397c0de879adea94` | included |
| `atc-contracts` | `main` | `a7c8b417f482665ef30e94fcacae6469c28b1125` | `026851749bdff7ff5a686dfc9fab6beacd9fce72` | included |
| `atc-engineering` | `main` | `b2ad23ee45a53de40ccfc1f1a1b4d37ca936d0e4` | `707d8bca64ef8adbdee53971873b0505988b1782` | included |
| `atc-explorer` | `main` | `c4da05304f214bb68bd942e7705c96c35d9a18ae` | `9aa7f3c6f2f90c33c0a92646a5ff5ce9964347ff` | included |
| `atc-ide` | `main` | `40a07acbbb2f240f0d409e97bd0262b8c63688b4` | `93d4d6353588e04d1ca81ae35a580e5b50fbae63` | included |
| `atc-indexer` | `main` | `7f3da49f30464471c8c1597ed59f6223050e79e4` | `382a3bb7e4d0ab12338344a01890fe5a38b1655d` | included |
| `atc-interop` | `main` | `5905fca02274704a00bdf708948180bc211a46d3` | `1d0dbc1c6ce05ad26b0808ba4e663758a9fa5b51` | included |
| `atc-launchpad` | `main` | `bd1a48c69983247a8428f90a77edd061ba79316b` | `b67a4f6a50ab578a8e82ac82748aecaf5bba003c` | included |
| `atc-marketplace` | `main` | `5c3bab7da8a80d58af725b7dea2478ece2456f39` | `c86fcc7f48d478a460419cbb075c7dea05a1fabb` | included |
| `atc-mining` | `main` | `227f2d700307191f59596c5a78ccfd3a4b657aa9` | `84535a74f93d5cd674b7d54195dacf741f1e9033` | included |
| `atc-node` | `main` | `a817db43a089d172dd4688f366286c60dbf71a0e` | `3bb0a666e8e7a4de08aaa31cb82189fdd35cf8d4` | included |
| `atc-oracle` | `main` | `8087d34ddb8c2d40a60178e8f3b4c52ea67d52a8` | `2fce137a779ca392551c1e9d2ee44027ea17ae8a` | included |
| `atc-sdk` | `main` | `34eef345907935cdaf279e0546f230c03c86bb0c` | `e8e5979cc795aa587ef182e831ecc4765bfee120` | included |
| `atc-shivacore` | `main` | `21c99769654ff5915c421c3446e62c2a29c22b40` | `7110f2aac94aaccdb1f0dfc8c4c9140db29c7456` | included |
| `atc-standards` | `main` | `f2b9b95723bbf7cd4f85c386940e1130351c59c6` | `d8a4f86c953d803c229937c9f840353d40dd0739` | included |
| `atc-storage` | `main` | `230746c1dc5e4c1355c2707368de50d6c1008660` | `307d3d2a179de1ed5d82618dfbe78f072ba6d4c0` | included |
| `atc-toolchain` | `main` | `aaed3daad2f6823349bb8c89bbeb996a7911317e` | `c3c16423e35f882308e0fa4af44cc9ef65c871de` | included |
| `atc-vm` | `main` | `21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0` | `d60af19553baf4ff88a3098c81a683d77ba7096e` | included |
| `atc-wallet` | `main` | `812a31d292364304c72cae8be6f448c130d7428e` | `2f624447b95a9ad723bf2e1369bf8e541bc427b5` | included |
| `atc-zkp` | `main` | `df9e694772ffc955129739805fc1692753dbea84` | `08290da2d511c7e2ec1f54a9b25c993315173ac9` | included |
| `atclang` | `main` | `7022d426d72a32b20905dc096ee49941d4e33989` | `6cbadf84b8c5b6051e232a631fcab78d6d86d091` | included |
| `aurora-ai` | `main` | `648e8bbbb94c0facb140a4613dcba29b47165072` | `1a942325ede62527a1fc1f7b010cb4ac5fefa3e8` | included |
| `demo-repository` | `main` | `6fca8f6356751e7e820e85ebf1984584392189b8` | `b35b4576f1732acc665283893c8214989df0829a` | **excluded** |
| `genesis-chronicles` | `main` | `9710bd9b314ca4bce836589f5fc8649619fc5604` | `19fdb0a75dda524b635829c95c95ff1324816f55` | included |
| `genesis-engine` | `main` | `8b7100e8a88f3c9bb6e24ddda31706902f125bb5` | `f85fbbee198c7561ff438b2bc26cfd29d1dd757c` | included |
| `genesis-franchise-factory` | `main` | `673fa560322b1449e8f23cc38bf839de1b246361` | `cbc461e067f5d46d0d5b7a378b80867673f92dd3` | included |
| `globus-os` | `main` | `3bf000143ec9bfad79d0bacc36b99b8111178b95` | `66413b78b5a9c09bdb2988ad33e6bcf8e71bc1fa` | included |

### Corrected automated validation — PASS

- Workflow run: [38811855932](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38811855932)
- Exact tested revision: `88722afe8f1ad2b91a6d094857eb4690bc284732`
- Job: `116686774782`, `Validate live repository refs and root trees`; job completed `success`.
- Validator step: completed `success`; validator process exit code `0`.
- Report status: `PASS`; `errors=[]`; inventory entries `33`; live inventory `32`; explicit exclusion `demo-repository`; revision pins checked `32/32`.
- Report JSON SHA-256 (recomputed from downloaded artifact and matching the log): `d143d52568ab16fd49b222e6fa53b43b3d1dcef2cea3955f5a4b39e3af04eade`.
- Artifact: [wp-m0-baseline-validation-38811855932-1](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38811855932/artifacts/12008074502)
- Artifact ZIP digest (GitHub upload digest): `sha256:ac3997db5616922dbfc5b2e14e3bc22ff4d108bcf2e0a6d1c8d8e5df31d86ac6`.
- The downloaded ZIP was inspected: it contains the JSON report and its `.sha256` file; the embedded JSON digest matches the report bytes.
- Decision boundary: this PASS applies to the automated WP-M0 manifest/live-ref validation at the exact tested revision only. Independent review and checkpoint approval are still pending; CP0 remains `BLOCKED` until that review records its decision.

### First automated validation run — failed closed as intended

- Workflow run: [38811486409](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38811486409)
- Run head: `dff79bcb812a018be17cb4d216800923a3ceecf8`
- Job: `116685536408`, `Validate live repository refs and root trees`
- Validator step: failed; process exit code `1`
- Result: `BLOCKED`, inventory manifest `33`, repositories visible to the workflow token `32`, revision pins compared `32/32`
- Report JSON SHA-256 (from job log): `9dd8506f51961a403c6427ebd600797efeab5b4d0d267ace5091f6d5f7a65975`
- Uploaded artifact: [wp-m0-baseline-validation-38811486409-1](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38811486409/artifacts/12009300863)
- Uploaded artifact ZIP digest: `sha256:9f3c42bfd94253949aa7f59ec582084f32e74c94924aa2c09e39651a6ed662a6`
- Failure detail: `demo-repository` was absent from the workflow token's organization listing. This repository is explicitly excluded from the live drift gate, so treating its absence from that listing as a baseline mismatch was an exclusion-handling defect in the validator—not evidence that a tracked in-scope repository was missing.
- Corrective commit: `88722afe8f1ad2b91a6d094857eb4690bc284732` updates the validator to exempt explicitly excluded repositories from the live-listing comparison and to validate the captured default-branch value dynamically rather than hard-coding `main`.

The first CI run is **not a pass**. The corrected validator must run again and its new report, exit code, artifact digest, and independent review must be checked before CP0 can be decided.

### Reproducible validator added

- Validator: `scripts/validate_wp_m0_baseline.py`
- Workflow: `.github/workflows/wp-m0-baseline-validation.yml`
- Manifest updated on the WP-M0 branch to schema version `1.1.0`, including capture timestamp, default branch, HEAD SHA, root-tree SHA, and explicit exclusion metadata.
- Validator acceptance is fail-closed: inventory additions/removals, unavailable API data, branch/archive drift, invalid/missing SHA pins, HEAD drift, or root-tree drift produce a non-zero exit. The workflow preserves a JSON report and SHA-256 sidecar as an artifact.
- The workflow requires organization-wide repository read access to enumerate the complete inventory. If `GITHUB_TOKEN` cannot see the complete organization inventory, configure the read-only `ATC_ORG_READ_TOKEN` secret; the validator must remain blocked rather than silently accept a partial listing.

## 3. Scope rules

1. Preserve all 33 organization repositories in the inventory; archived repositories remain visible as historical records.
2. Classify 26 non-archived and 7 archived entries from the live listing; reconcile future changes instead of silently overwriting history.
3. Exclude `demo-repository` from the live drift gate regardless of its GitHub archived flag.
4. Keep `a-townchain-ecosystem` limited to catalog, integration governance, compliance, evidence aggregation and drift controls; production implementation remains in canonical owner repositories.
5. Do not treat the repository list or this report as proof of source-census completeness or implementation verification.

## 4. CP0 decision

**Decision: `BLOCKED`**

### Passed observations
- [x] Live repository listing retrieved.
- [x] Manifest fetched; exact manifest blob SHA recorded.
- [x] Manifest count and live listing count both equal 33.
- [x] Archived/non-archived counts reconcile (7/26).
- [x] `demo-repository` explicitly identified for live drift-gate exclusion.

### Blocking acceptance criteria
- [x] Exact default-branch HEAD SHA captured for all 33 inventory repositories.
- [x] Corresponding root-tree SHA captured for all 33 inventory repositories.
- [x] Fail-closed validator and workflow added to the WP-M0 branch.
- [ ] Successful validator run with Run → Job → Step → exit code/logs and artifact hashes.
- [ ] Independent reviewer decision.

### Required next action
Run the new WP-M0 workflow against the updated manifest; inspect the live-inventory permissions and the exact validator report/artifact. Resolve any drift or access failures, then obtain independent review. Do not infer a PASS from the captured revision pins alone.

## 5. Evidence and status

The revision-pinned inventory is captured from direct ref and commit objects. The first automated comparison failed closed because the validator incorrectly treated the explicitly excluded private `demo-repository` as a missing in-scope repository. The corrected rerun passed at exact revision `88722afe8f1ad2b91a6d094857eb4690bc284732` with exit code 0 and a retained report artifact. CP0 remains blocked until an independent review is recorded. No normative approval is implied.

- **WP-M0:** `BLOCKED`
- **CP0:** `BLOCKED`
- **Overall catalog:** `PROPOSED / NOT VERIFIED`
- **Implementation claim:** none
- **Normative change:** none
