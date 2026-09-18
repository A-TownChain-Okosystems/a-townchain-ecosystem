use atc_blockchain::Node;
use atc_indexer::MemoryIndexer;
use atc_sdk::TransactionBuilder;
use ed25519_dalek::{Signer,SigningKey};
use std::sync::Arc;
#[test]
fn sdk_node_mempool_consensus_vm_state_storage_indexer(){
 let chain_id=658467;let node=Node::new(chain_id,"validator-1".into());let indexer=Arc::new(MemoryIndexer::new());node.set_indexer(indexer.clone());node.state.deposit("alice",1_000_000);node.create_genesis(1).unwrap();
 let key=SigningKey::from_bytes(&[7u8;32]);let tx=TransactionBuilder::transfer(chain_id,"alice","bob",100,1,1000,0,2).sign(&key);
 let txid=tx.id;node.submit(tx,2).unwrap();assert_eq!(node.pool.get_pending_batch(10).len(),1);
 let block=node.produce(3,10).unwrap();assert_eq!(block.transactions[0].id,txid);assert_eq!(node.state.balance("bob"),100);assert!(indexer.blocks().is_empty());
 let vote_key=SigningKey::from_bytes(&[6u8;32]);let voter="validator-1".to_string();let mut vote=atc_blockchain::consensus::Vote{block:block.id,voter:voter.clone(),approve:true,signature:[0;64],public_key:vote_key.verifying_key().to_bytes()};let mut vb=Vec::new();vb.extend_from_slice(b"ATC-VOTE-V1");vb.extend_from_slice(&chain_id.to_be_bytes());vb.extend_from_slice(&vote.block);vb.push(1);vb.extend_from_slice(&(voter.len()as u32).to_be_bytes());vb.extend_from_slice(voter.as_bytes());vote.signature=vote_key.sign(&vb).to_bytes();node.submit_vote(vote).unwrap();assert!(node.finalize(&block,1).unwrap());assert!(indexer.contains(block.id));assert_eq!(node.storage.block(1).unwrap().id,block.id);assert_eq!(node.storage.state_root(1),Some(node.state.root()));
}

#[test]
fn storage_restart_recovers_chain_and_state(){
 let path=std::env::temp_dir().join(format!("atc-e2e-{}-{}.journal",std::process::id(),658467u64));
 let _=std::fs::remove_file(&path);let _=std::fs::remove_file(path.with_extension("state"));
 let node=Node::open_storage(658467,"validator-1".into(),&path).unwrap();node.state.deposit("alice",1_000_000);node.create_genesis(1).unwrap();
 let key=SigningKey::from_bytes(&[8u8;32]);let tx=TransactionBuilder::transfer(658467,"alice","bob",25,1,1000,0,2).sign(&key);node.submit(tx,2).unwrap();let block=node.produce(3,10).unwrap();drop(node);
 let reopened=Node::open_storage(658467,"validator-1".into(),&path).unwrap();assert_eq!(reopened.chain.height(),1);assert_eq!(reopened.state.balance("bob"),25);assert_eq!(reopened.storage.block(1).unwrap().id,block.id);
 let _=std::fs::remove_file(&path);let _=std::fs::remove_file(path.with_extension("state"));
}
