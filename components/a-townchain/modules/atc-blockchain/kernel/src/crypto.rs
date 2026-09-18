//! Cryptographic verification for transaction signatures.
use ed25519_dalek::{Signature,Verifier,VerifyingKey};
use crate::mempool::Transaction;
pub trait SignatureVerifier:Send+Sync{fn verify(&self,message:&[u8],signature:&[u8;64],public_key:&[u8])->bool;}
pub struct Ed25519Verifier;
impl SignatureVerifier for Ed25519Verifier{fn verify(&self,message:&[u8],signature:&[u8;64],public_key:&[u8])->bool{let Ok(pk)=<[u8;32]>::try_from(public_key)else{return false};let Ok(key)=VerifyingKey::from_bytes(&pk)else{return false};key.verify(message,&Signature::from_bytes(signature)).is_ok()}}
pub fn signing_bytes(tx:&Transaction)->Vec<u8>{let mut b=Vec::new();b.extend_from_slice(b"ATC-TX-V1");b.extend_from_slice(&tx.id);b.extend_from_slice(tx.sender_did.as_bytes());b.extend_from_slice(&tx.nonce.to_be_bytes());b.extend_from_slice(&tx.amount.to_be_bytes());b.extend_from_slice(&tx.gas_limit.to_be_bytes());b.extend_from_slice(&tx.gas_price.to_be_bytes());b.extend_from_slice(&tx.timestamp.to_be_bytes());b.extend_from_slice(&tx.poh_hash);b}
