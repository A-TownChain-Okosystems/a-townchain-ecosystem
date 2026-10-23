// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical A-TownChain L1 transaction construction and signing.
//!
//! ATC economic amounts are u128 and are encoded as fixed-width 16-byte
//! big-endian values in the ATC-TX-DOMAIN-V2 signing preimage.

use crate::keys::{WalletKey, WalletKeyError};
use secp256k1::{ecdsa::Signature, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};

pub const NUMERIC_CHAIN_ID: u64 = 658467;
pub const TX_DOMAIN_V2: &[u8] = b"ATC-TX-DOMAIN-V2";
pub const TX_ID_V2: &[u8] = b"ATC-TX-ID-V2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TxType {
    Transfer = 0,
    Stake = 1,
    Unstake = 2,
    Contract = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub chain_id: u64,
    pub tx_type: TxType,
    pub sender_did: String,
    pub recipient_did: Option<String>,
    pub amount: u128,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    pub poh_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError {
    InvalidChainId,
    EmptySender,
    InvalidSignature,
    InvalidKey(WalletKeyError),
}

impl Transaction {
    pub fn signing_bytes(&self) -> Result<Vec<u8>, TxError> {
        if self.chain_id != NUMERIC_CHAIN_ID {
            return Err(TxError::InvalidChainId);
        }
        if self.sender_did.is_empty() {
            return Err(TxError::EmptySender);
        }

        let mut b = Vec::with_capacity(136 + self.payload.len());
        b.extend_from_slice(TX_DOMAIN_V2);
        append_canonical_fields(&mut b, self);
        Ok(b)
    }

    pub fn id(&self) -> Result<[u8; 32], TxError> {
        let mut b = Vec::with_capacity(148 + self.payload.len());
        b.extend_from_slice(TX_ID_V2);
        append_canonical_fields(&mut b, self);
        Ok(Sha256::digest(b).into())
    }

    fn digest(&self) -> Result<[u8; 32], TxError> {
        Ok(Sha256::digest(self.signing_bytes()?).into())
    }

    pub fn sign(&self, key: &WalletKey) -> Result<[u8; 64], TxError> {
        Ok(key.sign_digest(self.digest()?).serialize_compact())
    }

    pub fn verify(&self, public_key: &[u8; 33], signature: &[u8; 64]) -> Result<(), TxError> {
        let signature =
            Signature::from_compact(signature).map_err(|_| TxError::InvalidSignature)?;
        let mut normalized = signature;
        normalized.normalize_s();
        if normalized != signature {
            return Err(TxError::InvalidSignature);
        }
        let public_key =
            PublicKey::from_slice(public_key).map_err(|_| TxError::InvalidSignature)?;
        Secp256k1::verification_only()
            .verify_ecdsa(
                secp256k1::Message::from_digest(self.digest()?),
                &signature,
                &public_key,
            )
            .map_err(|_| TxError::InvalidSignature)
    }
}

fn append_canonical_fields(out: &mut Vec<u8>, tx: &Transaction) {
    out.extend_from_slice(&tx.chain_id.to_be_bytes());
    out.push(tx.tx_type as u8);
    put_bytes(out, tx.sender_did.as_bytes());
    match &tx.recipient_did {
        Some(value) => {
            out.push(1);
            put_bytes(out, value.as_bytes());
        }
        None => out.push(0),
    }
    out.extend_from_slice(&tx.amount.to_be_bytes());
    out.extend_from_slice(&tx.gas_price.to_be_bytes());
    out.extend_from_slice(&tx.gas_limit.to_be_bytes());
    out.extend_from_slice(&tx.nonce.to_be_bytes());
    out.extend_from_slice(&tx.timestamp.to_be_bytes());
    put_bytes(out, &tx.payload);
    out.extend_from_slice(&tx.poh_hash);
}

fn put_bytes(out: &mut Vec<u8>, value: &[u8]) {
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tx() -> Transaction {
        Transaction {
            chain_id: NUMERIC_CHAIN_ID,
            tx_type: TxType::Transfer,
            sender_did: "ATC-sender".into(),
            recipient_did: Some("ATC-recipient".into()),
            amount: 100,
            gas_price: 1,
            gas_limit: 1000,
            nonce: 7,
            timestamp: 1_700_000_000,
            payload: b"hello".to_vec(),
            poh_hash: [9u8; 32],
        }
    }

    #[test]
    fn l1_signature_roundtrip() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let tx = tx();
        let signature = tx.sign(&key).unwrap();
        assert!(tx.verify(&key.public_key(), &signature).is_ok());
    }

    #[test]
    fn signing_is_deterministic() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let tx = tx();
        assert_eq!(tx.sign(&key).unwrap(), tx.sign(&key).unwrap());
    }

    #[test]
    fn atc_tx_v2_u128_001_matches_shared_typescript_vector() {
        // Shared with atc-sdk/typescript/vectors/ATC-TX-V2-U128-001.json.
        // gas_price is u64 (8-byte BE); amount remains u128 (16-byte BE).
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let tx = tx();
        let expected_preimage = concat!(
            "4154432d54582d444f4d41494e2d5632",
            "00000000000a0c23",
            "00",
            "0000000a4154432d73656e646572",
            "01",
            "0000000d4154432d726563697069656e74",
            "00000000000000000000000000000064",
            "0000000000000001",
            "00000000000003e8",
            "0000000000000007",
            "000000006553f100",
            "0000000568656c6c6f",
            "0909090909090909090909090909090909090909090909090909090909090909"
        );
        let preimage = tx.signing_bytes().unwrap();
        let preimage_hex: String = preimage.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(preimage_hex, expected_preimage);
        assert_eq!(
            hex::encode(Sha256::digest(&preimage)),
            "2eb492404c03fadf9b7045768cfb5ec1e2a311ffd17d35ff7c92d97421104af8"
        );
        let signature = tx.sign(&key).unwrap();
        assert_eq!(
            hex::encode(signature),
            "845c864653f56193006c7a2044731000beff4003913e20e836c8053fbdf6f3693319abee3ee5b5214c880b8386b07c1e711beef13b3589d476099810c8acb1a0"
        );
        assert!(tx.verify(&key.public_key(), &signature).is_ok());
    }

    #[test]
    fn amount_is_fixed_width_u128_big_endian() {
        let mut tx = tx();
        tx.amount = u128::MAX;
        let bytes = tx.signing_bytes().unwrap();
        let amount_offset = TX_DOMAIN_V2.len()
            + 8
            + 1
            + 4
            + tx.sender_did.len()
            + 1
            + 4
            + tx.recipient_did.as_ref().unwrap().len();
        assert_eq!(&bytes[amount_offset..amount_offset + 16], &[0xff; 16]);
    }

    #[test]
    fn transaction_id_is_signature_independent() {
        let tx = tx();
        assert_eq!(
            tx.id().unwrap(),
            [
                0x04, 0x3e, 0xbe, 0x75, 0x45, 0xf4, 0x43, 0x20, 0x39, 0x4a, 0xf8, 0x21, 0x1d, 0xa3,
                0x23, 0xaf, 0xe8, 0xd2, 0x7f, 0xe9, 0xe9, 0x9b, 0xff, 0x5d, 0xa3, 0xaa, 0xfd, 0x2e,
                0x5c, 0x99, 0xb9, 0xdc,
            ]
        );
    }

    #[test]
    fn wrong_chain_id_is_rejected_before_signing() {
        let mut tx = tx();
        tx.chain_id = 1;
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        assert!(matches!(tx.sign(&key), Err(TxError::InvalidChainId)));
    }

    #[test]
    fn transaction_mutation_invalidates_signature() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let tx = tx();
        let signature = tx.sign(&key).unwrap();
        let mut altered = tx.clone();
        altered.amount += 1;
        assert!(altered.verify(&key.public_key(), &signature).is_err());
    }

    #[test]
    fn malformed_public_key_is_rejected() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let signature = tx().sign(&key).unwrap();
        assert!(tx().verify(&[0u8; 33], &signature).is_err());
    }
}
