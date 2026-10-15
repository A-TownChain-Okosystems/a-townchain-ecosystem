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

## 2. Important limitation: HEAD and root-tree SHAs

The manifest records repository names, archived state and roles, but does not store per-repository revision pins. The available connector operations returned recent commit-search results but did not expose a trustworthy default-branch ref object with both the exact `main` HEAD SHA and the corresponding root-tree SHA. A recent commit-search result is **not** accepted as proof that the commit is the current `main` HEAD.

Consequently:
- **Verified default-branch HEAD SHA coverage:** 0/33
- **Verified root-tree SHA coverage:** 0/33
- **Revision-pinned baseline:** incomplete
- No SHA has been promoted from a commit-search result to a claimed HEAD SHA.
- No root-tree SHA has been inferred from a commit SHA or file blob SHA.

The connector did return the following recent commit-search candidates. They are recorded only as discovery pointers, not as HEAD pins:

| Repository | Candidate commit SHA (not asserted as HEAD) |
|---|---|
| `.github` | `8106d7c060effd45050d29ba80bb0416866f1b01` |
| `a-townchain` | `a5e689671b1870699c7a1750f8a166e83590a41c` |
| `a-townchain-ecosystem` | `c83bf4e581f121f374f5d6619ceab20f97f9d1a1` |
| `a-townchain-os` | `a929fe77c65e3a6da4a2c5805f3e290291d47aaa` |
| `a-townchain-os-docs` | `1c4bfd2c9cd1c43deb0b88d3a4bfde94fc9d106c` |
| `atc-algorithm` | `f25b43383433766c43df6c5c494ecb70cdb4a4b7` |
| `atc-compute` | `89d833edf338085d6141b0c703ebf92a0db83a5e` |
| `atc-contracts` | `a7c8b417f482665ef30e94fcacae6469c28b1125` |
| `atc-engineering` | `b2ad23ee45a53de40ccfc1f1a1b4d37ca936d0e4` |
| `atc-explorer` | `c4da05304f214bb68bd942e7705c96c35d9a18ae` |
| `atc-ide` | `40a07acbbb2f240f0d409e97bd0262b8c63688b4` |
| `atc-indexer` | `7f3da49f30464471c8c1597ed59f6223050e79e4` |
| `atc-interop` | `5905fca02274704a00bdf708948180bc211a46d3` |
| `atc-launchpad` | `bd1a48c69983247a8428f90a77edd061ba79316b` |
| `atc-marketplace` | `5c3bab7da8a80d58af725b7dea2478ece2456f39` |
| `atc-mining` | `227f2d700307191f59596c5a78ccfd3a4b657aa9` |
| `atc-node` | `a817db43a089d172dd4688f366286c60dbf71a0e` |
| `atc-oracle` | `8087d34ddb8c2d40a60178e8f3b4c52ea67d52a8` |
| `atc-sdk` | `34eef345907935cdaf279e0546f230c03c86bb0c` |
| `atc-shivacore` | `21c99769654ff5915c421c3446e62c2a29c22b40` |
| `atc-standards` | `f2b9b95723bbf7cd4f85c386940e1130351c59c6` |
| `atc-storage` | `230746c1dc5e4c1355c2707368de50d6c1008660` |
| `atc-toolchain` | `aaed3daad2f6823349bb8c89bbeb996a7911317e` |
| `atc-vm` | `21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0` |
| `atc-wallet` | `812a31d292364304c72cae8be6f448c130d7428e` |
| `atc-zkp` | `df9e694772ffc955129739805fc1692753dbea84` |
| `atclang` | `7022d426d72a32b20905dc096ee49941d4e33989` |
| `aurora-ai` | `648e8bbbb94c0facb140a4613dcba29b47165072` |
| `demo-repository` | `6fca8f6356751e7e820e85ebf1984584392189b8` |
| `genesis-chronicles` | `9710bd9b314ca4bce836589f5fc8649619fc5604` |
| `genesis-engine` | `8b7100e8a88f3c9bb6e24ddda31706902f125bb5` |
| `genesis-franchise-factory` | `673fa560322b1449e8f23cc38bf839de1b246361` |
| `globus-os` | `3bf000143ec9bfad79d0bacc36b99b8111178b95` |

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
- [ ] Exact current default-branch HEAD SHA for every in-scope repository.
- [ ] Corresponding root-tree SHA for every in-scope repository.
- [ ] A reproducible validator comparing the manifest pins with the live GitHub refs.
- [ ] Run → Job → Step → exit code/logs and immutable artifact hashes for the validation run.
- [ ] Independent reviewer decision.

### Required next action
Use a GitHub API/connector operation that resolves each repository's `refs/heads/main` to its commit object and root tree. Update the versioned manifest only from those returned ref/commit/tree objects, then run and preserve a fail-closed comparison. Do not infer a PASS from this discovery report.

## 5. Evidence and status

This document records the observed state and the limitations of the available evidence. It is not a completed revision-pinned baseline, not a CI verification, and not normative approval.

- **WP-M0:** `BLOCKED`
- **CP0:** `BLOCKED`
- **Overall catalog:** `PROPOSED / NOT VERIFIED`
- **Implementation claim:** none
- **Normative change:** none
