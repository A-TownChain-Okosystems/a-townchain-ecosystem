# Repository Status — atc-l2

| Property | Value |
|---|---|
| Version | 0.1.0 |
| Status | development |
| Execution | deterministic Rust L2 state machine |
| Settlement | A-TownChain L1 commitment primitive |
| Proofs | not implemented; no ZK validity claim |
| Bridge | deposit/withdrawal primitives with replay protection |
| Tests | deterministic unit coverage included |
| Last Audit | 2026-09-18 |

ATC-L2 is the native second-layer execution layer for A-TownChain. L1 remains
the settlement and root-of-trust layer. The current implementation establishes
deterministic state, token balances, batches, state roots and settlement
commitment data. Sequencer networking, durable journal/recovery, L1 transaction
adapters and ZK proving/verification are subsequent integration stages.
