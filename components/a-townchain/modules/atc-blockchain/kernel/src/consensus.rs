//! Deterministic validator-weighted finality boundary with signed-vote validation.
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
    validators: Mutex<BTreeMap<String, u64>>,
}

impl ConsensusEngine {
    pub fn new(chain_id: u64, proposer: String) -> Self {
        Self {
            chain_id,
            proposer,
            height: Mutex::new(0),
            votes: Mutex::new(BTreeMap::new()),
            validators: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn register_validator(&self, address: String, stake: u64) -> Result<(), String> {
        if address.is_empty() || stake == 0 {
            return Err("validator address and stake are required".into());
        }
        self.validators.lock().map_err(|_| "validator lock poisoned")?
            .insert(address, stake);
        Ok(())
    }

    pub fn unregister_validator(&self, address: &str) {
        self.validators.lock().unwrap().remove(address);
    }

    pub fn validator_stake(&self, address: &str) -> u64 {
        self.validators.lock().unwrap().get(address).copied().unwrap_or(0)
    }

    pub fn validators_snapshot(&self) -> BTreeMap<String, u64> {
        self.validators.lock().unwrap().clone()
    }

    pub fn total_validator_stake(&self) -> u64 {
        self.validators.lock().unwrap().values().copied().fold(0, u64::saturating_add)
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

        if !self.validators.lock().map_err(|_| "validator lock poisoned".to_string())?
            .contains_key(&v.voter)
        {
            return Err("voter is not an active validator".into());
        }
        let mut all = self.votes.lock().map_err(|_| "vote lock poisoned".to_string())?;
        let list = all.entry(v.block).or_default();
        if list.iter().any(|x| x.voter == v.voter) {
            return Err("duplicate voter".into());
        }
        list.push(v);
        Ok(())
    }

    /// Legacy count-based finality retained for compatibility.
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

    /// Canonical L1 finality: at least two thirds of registered validator stake.
    pub fn weighted_finality(&self, id: &[u8; 32]) -> bool {
        let validators = self.validators.lock().unwrap();
        let total = validators.values().copied().fold(0u64, u64::saturating_add);
        if total == 0 {
            return false;
        }
        let votes = self.votes.lock().unwrap();
        let Some(list) = votes.get(id) else {
            return false;
        };
        let mut seen = BTreeSet::new();
        let approved = list.iter()
            .filter(|v| v.approve && seen.insert(v.voter.as_str()))
            .filter_map(|v| validators.get(&v.voter).copied())
            .fold(0u64, u64::saturating_add);
        (approved as u128) * 3 >= (total as u128) * 2
    }

    pub fn set_height(&self, h: u64) {
        *self.height.lock().unwrap() = h
    }

    pub fn height(&self) -> u64 {
        *self.height.lock().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn signed_vote(engine: &ConsensusEngine, key: &SigningKey, voter: &str, block: [u8; 32], approve: bool) -> Vote {
        let mut vote = Vote {
            block,
            voter: voter.into(),
            approve,
            signature: [0; 64],
            public_key: key.verifying_key().to_bytes(),
        };
        let sig = key.sign(&vote_bytes(engine.chain_id, &vote));
        vote.signature = sig.to_bytes();
        vote
    }

    #[test]
    fn weighted_finality_requires_two_thirds_stake() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let a = SigningKey::from_bytes(&[1u8; 32]);
        let b = SigningKey::from_bytes(&[2u8; 32]);
        let c = SigningKey::from_bytes(&[3u8; 32]);
        engine.register_validator("a".into(), 40).unwrap();
        engine.register_validator("b".into(), 35).unwrap();
        engine.register_validator("c".into(), 25).unwrap();
        let block = [9u8; 32];
        engine.vote(signed_vote(&engine, &a, "a", block, true)).unwrap();
        engine.vote(signed_vote(&engine, &b, "b", block, true)).unwrap();
        assert!(engine.weighted_finality(&block));
        let other = [8u8; 32];
        engine.vote(signed_vote(&engine, &a, "a", other, true)).unwrap();
        assert!(!engine.weighted_finality(&other));
    }

    #[test]
    fn unregistered_votes_are_rejected() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let vote = signed_vote(&engine, &key, "unknown", [1u8; 32], true);
        assert_eq!(engine.vote(vote).unwrap_err(), "voter is not an active validator");
    }
}
