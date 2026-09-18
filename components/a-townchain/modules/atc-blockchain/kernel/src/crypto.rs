//! Cryptographic boundary. Production deployments must inject a real signature provider.
pub trait SignatureVerifier:Send+Sync{fn verify(&self,message:&[u8],signature:&[u8;64],public_key:&[u8])->bool;}
pub struct RejectAllVerifier;impl SignatureVerifier for RejectAllVerifier{fn verify(&self,_:&[u8],_:&[u8;64],_:&[u8])->bool{false}}
pub fn signing_bytes(tx:&crate::mempool::Transaction)->Vec<u8>{let mut b=Vec::new();b.extend_from_slice(&tx.id);b.extend_from_slice(tx.sender_did.as_bytes());b.extend_from_slice(&tx.nonce.to_be_bytes());b.extend_from_slice(&tx.amount.to_be_bytes());b.extend_from_slice(&tx.gas_limit.to_be_bytes());b.extend_from_slice(&tx.gas_price.to_be_bytes());b}
