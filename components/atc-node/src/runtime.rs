use std::sync::Arc;
use atc_blockchain::{Block,Node};
pub const NUMERIC_CHAIN_ID:u64=658467;
pub struct Runtime{pub node:Arc<Node>}
impl Runtime{pub fn devnet(proposer:impl Into<String>)->Result<Self,String>{let node=Arc::new(Node::new(NUMERIC_CHAIN_ID,proposer.into()));node.create_genesis(0)?;Ok(Self{node})}pub fn submit(&self,tx:atc_blockchain::mempool::Transaction,now:u64)->Result<[u8;32],atc_blockchain::mempool::MempoolError>{self.node.submit(tx,now)}pub fn produce(&self,timestamp:u64,max:usize)->Result<Block,String>{self.node.produce(timestamp,max)}pub fn finalize(&self,block:&Block)->Result<bool,String>{self.node.finalize(block,1)}}
