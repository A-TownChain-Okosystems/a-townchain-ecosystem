# PR #184 Layer Audit Remediation Ledger

**Baseline:** `88691eae6b06d93ca83c1ba10e2a510eacc44f4b`  
**Disposition:** `OPEN / DRAFT / NOT VERIFIED`  
**Purpose:** Track concrete corrections from the first L0–L9 × X1–X36 audit without inventing applicability or promoting unverified claims.

## Immediate corrections

1. **Layer ID integrity (L1):** reject any `target_layer` outside `L0`–`L9` in system-layer dependency records. The reported `L10` reference is invalid under the adopted taxonomy. Do not silently remap it to L9; determine its actual responsibility and map deployment/release concerns to X35 or integration/E2E concerns to X36 only when the contract supports that mapping.
2. **Aurora authorization boundary (L8):** `AgentPool::execute` is not sufficient evidence for the catalog's policy/capability authorization claim. Keep `FN-AURORA-002` residual/blocked until the actual execution boundary is identified and the authorization path plus deny-path tests are evidenced. Do not assume authorization is absent elsewhere; trace the call path.
3. **Layer coverage (L3/L4):** add stable function contracts for GlobusOS services and desktop/UI only after source inventory. L4 must explicitly cover compositor, window manager, launcher/taskbar, input, context menu, settings, and accessibility. These are catalog contracts, not permission to create duplicate implementations.
4. **Matrix reconciliation:** preserve all 344 currently unresolved pairs as `UNRESOLVED` until each pair has a contract-grounded decision. The 16 current `APPLICABLE` entries remain mapping claims, not automatically verified implementations. Add `NOT_APPLICABLE` only with a documented boundary rationale and evidence reference.
5. **Source integrity:** repair invalid/missing blob SHAs only by fetching the exact file at the recorded full commit SHA and computing/reading the actual Git blob SHA. Never infer or copy a SHA from a neighboring symbol or current HEAD.
6. **L2 security evidence:** keep IOMMU/DMA isolation, TCB documentation and recovery contracts residual until exact source bindings and appropriate tests/evidence are attached.

## Acceptance gates

- [ ] Validator rejects unknown system-layer IDs, including L10.
- [ ] Every matrix pair has a stable pair ID and exactly one disposition: `APPLICABLE`, `NOT_APPLICABLE`, or `UNRESOLVED`.
- [ ] Every resolved disposition has a rationale; every evidence claim points to an immutable source/evidence reference.
- [ ] L3/L4 source inventory and stable function contracts are complete for the declared scope.
- [ ] Aurora execution-boundary authorization is traced and tested, including denial cases.
- [ ] Exact commit SHA, path, symbol, blob SHA, test run/job/step/exit code and artifact hashes are preserved where applicable.
- [ ] No function or matrix relationship is marked `VERIFIED` from documentation or green CI alone.
- [ ] `demo-repository` remains excluded from the live drift gate.

## Current disposition

This ledger does not claim that the underlying source defects have already been repaired. It converts the audit findings into explicit fail-closed acceptance gates. PR #184 must remain Draft until the applicable gates are evidenced.