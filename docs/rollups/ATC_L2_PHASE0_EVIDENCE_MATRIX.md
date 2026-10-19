# ATC-L2 Rollup — Phase 0 Evidence Matrix

Status: **BLOCKED — feasibility inventory in progress**

This file is an evidence register, not an implementation claim. The ecosystem repository owns architecture, catalog, integration, and evidence coordination; canonical runtime implementation belongs in the relevant source repository.

## Decision boundary

Target architecture: ATC-L2 as a rollup settling to A-TownChain L1.

Not yet selected:
- Optimistic fraud-proof rollup vs. validity-proof rollup
- Batch commitment and canonical state-root format
- Data-availability mechanism and retention guarantees
- Sequencer failure handling, forced inclusion, and censorship recovery
- Withdrawal finality and challenge/verification windows

No production bridge transfers or security guarantees may be enabled or claimed until these gates have evidence.

## Source-backed observations

| Area | Source path | Blob SHA | Observation | Status |
|---|---|---|---|---|
| ZKP crate dependencies | `a-townchain/modules/atc-zkp/Cargo.toml` | `d0ee2f0bfc1f922e4ee6b71821f89b49384e5297` | Cargo `[dependencies]` is empty in the inspected file. | BLOCKED |
| ZKP verifier | `a-townchain/modules/atc-zkp/src/verifier.rs` | `808882b1d0492690abea0652e79f9bc2363d3d49` | Source explicitly labels the module planned / not implemented. | BLOCKED |
| SNARK module | `a-townchain/modules/atc-zkp/src/snarks.rs` | `ebf43b8dbd13ae660d111aed0c6828b12a419094` | Source explicitly labels the module planned / not implemented. | BLOCKED |
| Bridge state proofs | `a-townchain/components/interop/modules/atc-bridge/proofs/state_proof.atc` | `b5ec1883e810a398f68f22fbae4bd4dc6578a622` | Inspected verifier functions contain comments only; no executable verification logic shown. | BLOCKED |
| ZKP architecture | `a-townchain/components/zkp/docs/ZKP_ARCHITECTURE.md` | `631e3f110d84c8dea555621852218ac7d1d24319` | Architecture document describes a target layer; cited standards are candidate drafts. | RESIDUAL |
| L1 consensus candidate | `a-townchain/modules/atc-blockchain/consensus/consensus.atc` | `8ea2a39fa31073726e2dfde29111d4e231d7bd7c` | Block fields include `tx_count` and `merkle_root`; this alone does not establish a canonical rollup state root, batch commitment, DA, or L1 verifier. | RESIDUAL |
| Ecosystem layer model | `ARCHITECTURE.md` | `ea3a93cdb6f66eb33caf8fd7715e184fd524b27e` | Current model defines L2 as blockchain core/consensus, L3 as deterministic execution, L4 as scaling/rollups. Protocol L1/L2 must be distinguished from internal responsibility layers. | RESIDUAL |

Source links:
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-zkp/Cargo.toml
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-zkp/src/verifier.rs
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-zkp/src/snarks.rs
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/components/interop/modules/atc-bridge/proofs/state_proof.atc
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/components/zkp/docs/ZKP_ARCHITECTURE.md
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-blockchain/consensus/consensus.atc
- https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/blob/main/ARCHITECTURE.md

## Required evidence before implementation choice

1. Pin exact repository commit SHAs for `a-townchain`, `atc-standards`, and `atc-sdk`; blob SHAs above are file-content identifiers, not repository commit SHAs.
2. Locate and review canonical L1 transaction validation, deterministic state transition, state-root construction, block finality, and upgrade rules.
3. Identify existing tests and CI workflows for consensus determinism, state transition, cryptography, and bridge proofs; record run/job/step and artifact hashes.
4. Search all active source paths for existing rollup, batch, fraud-proof, validity-proof, DA, sequencer, forced-inclusion, and withdrawal implementations; classify legacy/archive references separately.
5. Produce a gap matrix with exact commit SHA + path + symbol + test/CI evidence for every claimed capability.
6. Only then select a proof model and open a separate implementation PR in the canonical source repository.

## Acceptance criteria

- [ ] Repository commit SHAs pinned and recorded.
- [ ] Canonical L1 state-transition and finality paths identified.
- [ ] State-root and batch-commitment semantics specified.
- [ ] Proof verification implementation and negative tests evidenced, or explicitly marked absent.
- [ ] Data availability and sequencer-failure recovery designed and tested.
- [ ] Deposit/withdrawal replay, double-spend, reorg/finality, and invalid-proof cases covered.
- [ ] Standards ownership and catalog layer mapping reconciled.
- [ ] CI evidence captured for the exact reviewed commit.

Until acceptance criteria are met, rollup runtime implementation and production settlement remain **BLOCKED**. Do not substitute stubs, comments, or architecture prose for working proof verification.
