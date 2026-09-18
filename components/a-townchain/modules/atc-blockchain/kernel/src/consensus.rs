//! Deterministic finality boundary with unique-voter and signed-vote validation.
use crate::security::simple_hash;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Mutex,
};
#[derive(Clone, Debug)]
pub struct Vote {
    pub block: [u8; 32],
    pub voter: String,
    pub approve: bool,
    pub signature: [u8; 64],
    pub public_key: [u8; 32],
}
fn vote_bytes(chain: u64, v: &Vote) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(b"ATC-VOTE-V1");
    b.extend_from_slice(&chain.to_be_bytes());
    b.extend_from_slice(&v.block);
    b.push(v.approve as u8);
    b.extend_from_slice(&(v.voter.len() as u32).to_be_bytes());
    b.extend_from_slice(v.voter.as_bytes());
    b
}
pub struct ConsensusEngine {
    pub chain_id: u64,
    pub proposer: String,
    height: Mutex<u64>,
    votes: Mutex<BTreeMap<[u8; 32], Vec<Vote>>>,
}
impl ConsensusEngine {
    pub fn new(chain_id: u64, proposer: String) -> Self {
        Self {
            chain_id,
            proposer,
            height: Mutex::new(0),
            votes: Mutex::new(BTreeMap::new()),
        }
    }
    pub fn propose_id(&self, h: u64, parent: [u8; 32], state: [u8; 32], tx: [u8; 32]) -> [u8; 32] {
        let mut b = Vec::new();
        b.extend_from_slice(&self.chain_id.to_be_bytes());
        b.extend_from_slice(&h.to_be_bytes());
        b.extend_from_slice(&parent);
        b.extend_from_slice(&state);
        b.extend_from_slice(&tx);
        b.extend_from_slice(self.proposer.as_bytes());
        simple_hash(&b)
    }
    pub fn vote(&self, v: Vote) -> Result<(), String> {
        let pk = VerifyingKey::from_bytes(&v.public_key)
            .map_err(|_| "invalid vote public key".to_string())?;
        pk.verify(
            &vote_bytes(self.chain_id, &v),
            &Signature::from_bytes(&v.signature),
        )
        .map_err(|_| "invalid vote signature".to_string())?;
        let mut all = self
            .votes
            .lock()
            .map_err(|_| "vote lock poisoned".to_string())?;
        let list = all.entry(v.block).or_default();
        if list.iter().any(|x| x.voter == v.voter) {
            return Err("duplicate voter".into());
        }
        list.push(v);
        Ok(())
    }
    pub fn finality(&self, id: &[u8; 32], quorum: usize) -> bool {
        self.votes
            .lock()
            .unwrap()
            .get(id)
            .map(|v| {
                v.iter()
                    .filter(|x| x.approve)
                    .map(|x| x.voter.as_str())
                    .collect::<BTreeSet<_>>()
                    .len()
                    >= quorum
            })
            .unwrap_or(false)
    }
    pub fn set_height(&self, h: u64) {
        *self.height.lock().unwrap() = h
    }
    pub fn height(&self) -> u64 {
        *self.height.lock().unwrap()
    }
}
