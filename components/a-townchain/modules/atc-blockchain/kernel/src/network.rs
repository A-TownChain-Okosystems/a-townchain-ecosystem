//! P2P protocol boundary; transport is intentionally outside consensus state.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkMessage {
    Transaction([u8; 32]),
    Block([u8; 32]),
    Vote([u8; 32]),
}

pub trait PeerTransport: Send + Sync {
    fn broadcast(&self, message: NetworkMessage) -> Result<(), String>;
}

pub struct NullTransport;

impl PeerTransport for NullTransport {
    fn broadcast(&self, _message: NetworkMessage) -> Result<(), String> {
        Ok(())
    }
}
