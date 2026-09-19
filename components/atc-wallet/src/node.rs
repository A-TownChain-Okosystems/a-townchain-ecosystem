//! L1 node transport boundary.
//!
//! This module intentionally does not invent an RPC protocol. It defines the
//! typed boundary the wallet uses once the canonical ATC node RPC is frozen.

use crate::tx::Transaction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeError {
    Transport(String),
    Rejected(String),
    Unsupported,
}

pub trait NodeClient {
    fn submit_transaction(&self, tx: &Transaction, signature: &[u8; 64])
        -> Result<[u8; 32], NodeError>;

    fn balance(&self, address: &str) -> Result<u64, NodeError>;
}

pub struct UnconfiguredNode;

impl NodeClient for UnconfiguredNode {
    fn submit_transaction(
        &self,
        _tx: &Transaction,
        _signature: &[u8; 64],
    ) -> Result<[u8; 32], NodeError> {
        Err(NodeError::Unsupported)
    }

    fn balance(&self, _address: &str) -> Result<u64, NodeError> {
        Err(NodeError::Unsupported)
    }
}
