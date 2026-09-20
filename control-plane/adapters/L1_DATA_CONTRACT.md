# L1 Data Adapter Contract

The L1 panel consumes only explicit adapter payloads. It does not infer runtime state from source-code presence.

## Adapter endpoint contract

A runtime adapter MUST expose:

- `GET /status` — node health, chain ID, version, commit
- `GET /chain/head` — current and finalized height
- `GET /consensus` — validator/quorum/finality state
- `GET /mempool` — pending transaction count
- `GET /evidence` — evidence/slashing state
- `GET /state/recovery` — persistence and restart-recovery status

The existing `atc-node` RPC currently exposes `status`, `block`, `state_root`, `balance`, transaction submission and block production. The adapter contract deliberately separates currently exposed RPC data from future runtime endpoints.

## Required L1 panel payload

```json
{
  "component_id": "atc-node",
  "status": "NOT_READY",
  "chain_id": null,
  "height": null,
  "finalized_height": null,
  "validators": {"active": null, "quorum": null},
  "mempool": {"pending": null},
  "consensus": {"state": "UNKNOWN"},
  "persistence": {"state": "UNKNOWN"},
  "recovery": {"state": "UNKNOWN"},
  "evidence": {"ready": false, "ref": null},
  "commit_sha": null,
  "version": null,
  "observed_at": null
}
```

Unknown values remain `null`; the dashboard MUST NOT replace them with zero or healthy defaults.

## Evidence requirements

A positive L1 state requires:

1. CI run reference
2. commit SHA
3. build result
4. integration result
5. multi-process E2E result
6. finality result
7. persistence/restart recovery result
8. evidence artifact/reference

The adapter is observational by default. Node-control actions remain behind the Control Matrix and explicit authorization.


## Cross-component evidence gates

The L1 panel consumes these connected execution gates:

- **L1 runtime:** `cargo check -p atc-node` and `cargo test -p atc-node --all-targets`
- **Multi-node network:** `cargo test -p atc-blockchain --test multi_process_network_e2e`
- **Wallet path:** `cargo test -p atc-node --test wallet_node_process_e2e` proving Wallet → Node → Block → Balance.
- **SDK/persistence:** `cargo test -p integration` proving canonical SDK transaction construction and restart/state persistence paths.
- **VM boundary:** block production executes transactions through the canonical `AtcVmExecutor`; a successful production test is required before this connection is considered verified.

A gate is **not verified** unless the corresponding GitHub Actions job completes successfully for the same commit SHA.

- **Indexer/finality:** `cargo test -p atc-sdk --test e2e` verifies finalized blocks reach the indexer and survive storage restart.
- **ATC-VM:** `cargo test --manifest-path components/atc-vm/Cargo.toml` verifies the VM independently; L1 VM integration additionally requires the L1 E2E gates above.
- **Wallet library:** `cargo test --manifest-path components/atc-wallet/Cargo.toml` verifies wallet primitives independently; Wallet→Node integration requires the process E2E gate.
