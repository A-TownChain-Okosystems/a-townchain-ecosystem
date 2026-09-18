use atc_blockchain::Node;
use atc_indexer::MemoryIndexer;
use atc_sdk::TransactionBuilder;
use ed25519_dalek::SigningKey;
use std::sync::Arc;
#[test]
fn sdk_node_mempool_consensus_vm_state_storage_indexer(){
 let chain_id=658467;let node=Node::new(chain_id,"validator-1".into());let indexer=Arc::new(MemoryIndexer::new());node.set_indexer(indexer.clone());node.state.deposit("alice",1_000_000);node.create_genesis(1).unwrap();
 let key=SigningKey::from_bytes(&[7u8;32]);let tx=TransactionBuilder::transfer(chain_id,"alice","bob",100,1,1000,0,2).sign(&key);
 let txid=tx.id;node.submit(tx,2).unwrap();assert_eq!(node.pool.get_pending_batch(10).len(),1);
 let block=node.produce(3,10).unwrap();assert_eq!(block.transactions[0].id,txid);assert_eq!(node.state.balance("bob"),100);assert!(indexer.blocks().is_empty());
 assert!(node.finalize(&block,1).unwrap());assert!(indexer.contains(block.id));assert_eq!(node.storage.block(1).unwrap().id,block.id);assert_eq!(node.storage.state_root(1),Some(node.state.root()));
}
