//! End-to-end integration boundary for the A-TownChain ecosystem.
//! This crate owns no consensus logic; it composes the canonical node runtime and SDK.

use atc_node::runtime::{Runtime, NUMERIC_CHAIN_ID};
use atc_sdk::TransactionBuilder;
use ed25519_dalek::SigningKey;

pub const SYSTEM_CHAIN_ID: u64 = NUMERIC_CHAIN_ID;

pub fn build_signed_transfer(sender: &str, recipient: &str, amount: u64, nonce: u64) -> atc_blockchain::mempool::Transaction {
    let key = SigningKey::from_bytes(&[7u8; 32]);
    TransactionBuilder::transfer(SYSTEM_CHAIN_ID, sender, recipient, amount, 1, 2_000, nonce, 1).sign(&key)
}

pub fn boot_and_build_transaction() -> Result<([u8; 32], atc_blockchain::mempool::Transaction), String> {
    let runtime = Runtime::devnet("ecosystem-integration")?;
    let tx = build_signed_transfer("alice", "bob", 1, 0);
    let tx_id = runtime.submit(tx.clone(), 1).map_err(|e| format!("transaction rejected: {e:?}"))?;
    Ok((tx_id, tx))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_and_sdk_share_the_same_chain_contract() {
        let (tx_id, tx) = boot_and_build_transaction().expect("node must accept a canonical SDK transaction into its mempool");
        assert_eq!(tx.chain_id, SYSTEM_CHAIN_ID);
        assert_eq!(tx_id, tx.id);
        assert_ne!(tx.signature, [0u8; 64]);
    }
    #[test]
    fn transaction_id_is_deterministic() {
        let a = build_signed_transfer("alice", "bob", 1, 0);
        let b = build_signed_transfer("alice", "bob", 1, 0);
        assert_eq!(a.id, b.id);
        assert_eq!(a.payload, b.payload);
    }

    #[test]
    fn sdk_dao_payload_round_trips_through_persistent_state_codec() {
        let tx = TransactionBuilder::dao_create_proposal(
            SYSTEM_CHAIN_ID,
            "alice",
            42,
            1,
            10,
            "Ecosystem integration",
            "Verify DAO state persistence",
            None,
            0,
            1,
            10_000,
            0,
            1,
        );
        let state = atc_blockchain::mempool::StateDb::new();
        state.apply_dao_payload(&tx.payload, 1, "alice").expect("DAO proposal must enter canonical chain state");
        let snapshot = state.dao_snapshot();
        let root_before = state.root();
        state.restore_dao(&snapshot).expect("DAO snapshot must be decodable");
        assert_eq!(root_before, state.root());
    }
}
    #[test]
    fn node_produces_and_finalizes_sdk_transaction() {
        let runtime = Runtime::devnet("ecosystem-integration").expect("runtime must boot");
        let tx = build_signed_transfer("alice", "bob", 1, 0);
        runtime.submit(tx, 1).expect("transaction must enter mempool");
        let block = runtime.produce(2, 100).expect("node must produce a block from the pending transaction");
        assert_eq!(block.height, 1);
        assert_eq!(block.transactions.len(), 1);
        assert!(runtime.finalize(&block).expect("block finalization must execute"));
    }

