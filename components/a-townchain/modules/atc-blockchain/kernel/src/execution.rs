//! VM execution boundary. ATCLang execution remains behind the VM.
#[derive(Clone,Debug,PartialEq,Eq)]pub struct ExecutionReceipt{pub tx_id:[u8;32],pub success:bool,pub gas_used:u64,pub return_data:Vec<u8>}
pub trait VmExecutor:Send+Sync{fn execute(&self,tx:&crate::mempool::Transaction,state_root:[u8;32])->Result<ExecutionReceipt,String>;}
pub struct NoopVmExecutor;impl VmExecutor for NoopVmExecutor{fn execute(&self,tx:&crate::mempool::Transaction,_:[u8;32])->Result<ExecutionReceipt,String>{Ok(ExecutionReceipt{tx_id:tx.id,success:true,gas_used:tx.gas_cost(),return_data:Vec::new()})}}
