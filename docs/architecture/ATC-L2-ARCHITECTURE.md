# A-TownChain Layer-2 Architecture

## Purpose

ATC-L2 is a native second-layer execution environment for A-TownChain. It is
not an Ethereum compatibility layer and does not replace L1 consensus.

## Canonical flow

ATCLang -> ATC-VM -> ATC-L2 execution -> L2 state root -> L1 settlement.

L1 remains authoritative for consensus, final settlement, L2 registration,
deposit/withdrawal settlement and recovery policy.

## L2 state

The deterministic state contains:

- L2 chain identifier and height
- token registry
- token balances
- consumed L1 deposit references
- finalized withdrawal references

Canonical state collections use BTreeMap. Integer arithmetic uses checked
operations for token supply and balances.

## Token lifecycle

The initial token runtime supports:

- CreateToken
- Mint
- Burn
- Transfer
- Deposit
- Withdraw

Deposit and withdrawal references are replay protected. L1 release of a
withdrawal remains an L1 settlement responsibility; the L2 crate does not
pretend that a local withdrawal is sufficient for asset release.

## Batches and settlement

Each batch records:

- parent state root
- resulting state root
- transaction list
- execution receipts
- deterministic batch hash

The L1SettlementCommitment binds the L2 chain identifier, batch number,
state root and batch hash. This is a settlement data primitive, not yet an L1
consensus instruction or ZK validity proof.

## Security boundary

The sequencer, once implemented, only orders transactions. It is not the
security root. Canonical batches and L1 commitments must remain sufficient for
state reconstruction.

## Integration roadmap

1. deterministic L2 state/execution — implemented
2. token runtime — implemented
3. batch/state-root commitment — implemented
4. L1 settlement adapter — next
5. durable L2 journal and restart recovery — next
6. sequencer, mempool and RPC — next
7. ATC-ZKP proving/verifier integration — next
8. ATC-Wallet, GlobusOS and Aurora integration — next
