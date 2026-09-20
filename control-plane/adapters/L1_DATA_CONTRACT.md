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
