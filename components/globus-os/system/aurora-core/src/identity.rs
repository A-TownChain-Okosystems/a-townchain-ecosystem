//! Decentralized Aurora agent/device identity contracts.
//! Cryptography is injected at the trust boundary.

use crate::{AgentId, AuroraError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentIdentity {
    pub agent_id: AgentId,
    pub device_id: String,
    pub public_key: Vec<u8>,
    pub owner: String,
    pub key_algorithm: String,
}

impl AgentIdentity {
    pub fn new(
        agent_id: AgentId,
        device_id: impl Into<String>,
        public_key: Vec<u8>,
        owner: impl Into<String>,
        key_algorithm: impl Into<String>,
    ) -> Result<Self, AuroraError> {
        let device_id = device_id.into();
        let owner = owner.into();
        let key_algorithm = key_algorithm.into();
        if device_id.trim().is_empty() || owner.trim().is_empty()
            || key_algorithm.trim().is_empty() || public_key.is_empty()
        {
            return Err(AuroraError::InvalidRequest);
        }
        Ok(Self { agent_id, device_id, public_key, owner, key_algorithm })
    }

    pub fn signing_payload(&self) -> Vec<u8> {
        let mut out = b"atc.aurora.agent-identity.v1".to_vec();
        append(&mut out, self.agent_id.as_str().as_bytes());
        append(&mut out, self.device_id.as_bytes());
        append(&mut out, &self.public_key);
        append(&mut out, self.owner.as_bytes());
        append(&mut out, self.key_algorithm.as_bytes());
        out
    }
}

pub trait IdentityVerifier {
    fn verify_identity(&self, identity: &AgentIdentity, signature: &[u8]) -> bool;
}

fn append(out: &mut Vec<u8>, value: &[u8]) {
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentId;

    #[test]
    fn rejects_empty_identity_material() {
        assert!(AgentIdentity::new(
            AgentId::new("agent-1").unwrap(), "", vec![1], "owner", "ed25519"
        ).is_err());
        assert!(AgentIdentity::new(
            AgentId::new("agent-1").unwrap(), "device-1", Vec::new(), "owner", "ed25519"
        ).is_err());
    }

    #[test]
    fn signing_payload_is_stable() {
        let identity = AgentIdentity::new(
            AgentId::new("agent-1").unwrap(), "device-1", vec![1, 2, 3],
            "owner-1", "ed25519"
        ).unwrap();
        assert_eq!(identity.signing_payload(), identity.signing_payload());
        assert!(identity.signing_payload().starts_with(b"atc.aurora.agent-identity.v1"));
    }
}
