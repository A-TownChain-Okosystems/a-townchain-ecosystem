# ATC-L2 Protocol v0.1

## Batch lifecycle

1. accept signed L2 transactions
2. establish deterministic transaction order
3. execute against the previous L2 state
4. emit receipts
5. compute the new state root
6. compute the batch hash
7. submit a settlement commitment to A-TownChain L1
8. wait for L1 settlement/finality

The current Rust implementation covers steps 3–6.

## Deposits

An L1 deposit supplies a unique L1 reference. ATC-L2 rejects a duplicate
reference. The future L1 adapter must derive this reference from a canonical
L1 event or transaction identity.

## Withdrawals

An L2 withdrawal consumes the L2 balance and records a unique reference.
The future L1 settlement adapter must release assets only after the relevant
L2 commitment is finalized and the withdrawal is authorized by L1 rules.

## Determinism

No wall-clock, random, host filesystem or network state participates in state
transition execution. Canonical ordering and hashing are explicit.

## Non-goals for v0.1

- Ethereum JSON-RPC compatibility
- EVM compatibility
- permissionless sequencer networking
- unverified bridge releases
- claiming ZK security before a verifier exists
