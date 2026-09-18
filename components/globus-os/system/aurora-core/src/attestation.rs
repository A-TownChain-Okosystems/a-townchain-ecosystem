//! Aurora execution-attestation contracts aligned with ATC-COMP-505.
//! Cryptographic signing/verification is injected until ATC-CRYPTO-001 is bound.

use crate::{AgentId, AuroraError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionAttestation {
    pub job_hash: [u8; 32],
    pub input_hash: [u8; 32],
    pub result_hash: [u8; 32],
    pub runtime_hash: [u8; 32],
    pub model_hash: Option<[u8; 32]>,
    pub env_hash: [u8; 32],
    pub agent_id: AgentId,
    pub worker_public_key: Vec<u8>,
    pub nonce: [u8; 32],
    pub executed_at_slot: u64,
    pub signature: Vec<u8>,
}

impl ExecutionAttestation {
    pub fn unsigned(
        job_hash: [u8; 32], input_hash: [u8; 32], result_hash: [u8; 32],
        runtime_hash: [u8; 32], model_hash: Option<[u8; 32]>, env_hash: [u8; 32],
        agent_id: AgentId, worker_public_key: Vec<u8>, nonce: [u8; 32],
        executed_at_slot: u64,
    ) -> Result<Self, AuroraError> {
        if worker_public_key.is_empty() || nonce == [0; 32] {
            return Err(AuroraError::InvalidRequest);
        }
        Ok(Self {
            job_hash, input_hash, result_hash, runtime_hash, model_hash, env_hash,
            agent_id, worker_public_key, nonce, executed_at_slot, signature: Vec::new(),
        })
    }

    pub fn signing_payload(&self) -> Vec<u8> {
        let mut out = b"atc-compute.attestation.v1".to_vec();
        for hash in [self.job_hash, self.input_hash, self.result_hash, self.runtime_hash, self.env_hash] {
            out.extend_from_slice(&hash);
        }
        match self.model_hash {
            Some(model) => { out.push(1); out.extend_from_slice(&model); }
            None => out.push(0),
        }
        append(&mut out, self.agent_id.as_str().as_bytes());
        append(&mut out, &self.worker_public_key);
        out.extend_from_slice(&self.nonce);
        out.extend_from_slice(&self.executed_at_slot.to_be_bytes());
        out
    }

    pub fn is_signed(&self) -> bool { !self.signature.is_empty() }
}

pub trait AttestationSigner {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, AuroraError>;
}

pub trait AttestationVerifier {
    fn verify(&self, payload: &[u8], signature: &[u8], public_key: &[u8]) -> bool;
}

pub fn sign_attestation(
    attestation: &mut ExecutionAttestation,
    signer: &dyn AttestationSigner,
) -> Result<(), AuroraError> {
    attestation.signature = signer.sign(&attestation.signing_payload())?;
    if attestation.signature.is_empty() {
        return Err(AuroraError::SecurityViolation);
    }
    Ok(())
}

pub fn verify_attestation(
    attestation: &ExecutionAttestation,
    verifier: &dyn AttestationVerifier,
) -> Result<(), AuroraError> {
    if !attestation.is_signed() || attestation.nonce == [0; 32] || attestation.worker_public_key.is_empty() {
        return Err(AuroraError::SecurityViolation);
    }
    if !verifier.verify(&attestation.signing_payload(), &attestation.signature, &attestation.worker_public_key) {
        return Err(AuroraError::SecurityViolation);
    }
    Ok(())
}

fn append(out: &mut Vec<u8>, value: &[u8]) {
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentId;

    struct TestSigner;
    impl AttestationSigner for TestSigner {
        fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, AuroraError> { Ok(payload[..8].to_vec()) }
    }
    struct TestVerifier;
    impl AttestationVerifier for TestVerifier {
        fn verify(&self, payload: &[u8], signature: &[u8], _: &[u8]) -> bool {
            signature == &payload[..8]
        }
    }

    fn sample() -> ExecutionAttestation {
        ExecutionAttestation::unsigned(
            [1;32], [2;32], [3;32], [4;32], Some([5;32]), [6;32],
            AgentId::new("agent-1").unwrap(), vec![7;32], [8;32], 42
        ).unwrap()
    }

    #[test]
    fn unsigned_attestation_is_rejected() {
        assert!(verify_attestation(&sample(), &TestVerifier).is_err());
    }

    #[test]
    fn signed_attestation_verifies() {
        let mut a = sample();
        sign_attestation(&mut a, &TestSigner).unwrap();
        assert!(verify_attestation(&a, &TestVerifier).is_ok());
    }
}
