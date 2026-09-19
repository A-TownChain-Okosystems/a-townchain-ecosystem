# Persistent ATC Node Network

The normal `atc-node` process now owns the persistent L1 network loop.

## Runtime path

`RPC transaction -> mempool -> scheduled proposer -> block broadcast -> block validation/state transition -> validator vote -> weighted quorum finality -> durable storage -> restart recovery`

## Node configuration

- `ATC_NODE_ID`: canonical validator/node identity
- `ATC_RPC_ADDR`: JSON-RPC listen address
- `ATC_P2P_ADDR`: authenticated peer transport listen address
- `ATC_PEERS`: comma-separated peer addresses
- `ATC_DATA_DIR`: durable chain storage directory
- `ATC_VALIDATORS`: comma-separated `id:stake:64hexseed` validator configuration
- `ATC_BLOCK_INTERVAL_SECS`: proposer interval; defaults to the protocol 360-second block interval

All nodes in one network use the same deterministic genesis commitment. Validators are registered before the consensus loop starts, and a configured local validator signs votes for imported and locally produced blocks.
