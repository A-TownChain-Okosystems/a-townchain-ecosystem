// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Rust trusted key boundary for the ATC wallet.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use getrandom::fill;
use zeroize::Zeroize;

#[derive(Debug)]
pub enum KeyError {
    Randomness,
}

pub struct WalletKey {
    signing_key: SigningKey,
}

impl WalletKey {
    pub fn generate() -> Result<Self, KeyError> {
        let mut seed = [0u8; 32];
        fill(&mut seed).map_err(|_| KeyError::Randomness)?;
        let key = SigningKey::from_bytes(&seed);
        seed.zeroize();
        Ok(Self { signing_key: key })
    }

    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self { signing_key: SigningKey::from_bytes(&seed) }
    }

    pub fn public_key(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_seed_derives_same_public_key() {
        let a = WalletKey::from_seed([7u8; 32]);
        let b = WalletKey::from_seed([7u8; 32]);
        assert_eq!(a.public_key(), b.public_key());
    }

    #[test]
    fn signature_roundtrip() {
        let key = WalletKey::from_seed([7u8; 32]);
        let sig = key.sign(b"ATC wallet test");
        assert!(key.verifying_key().verify_strict(b"ATC wallet test", &sig).is_ok());
    }
}
