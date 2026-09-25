//! ATC-VM execution adapter.
use crate::mempool::{Transaction, TxType};
use atc_vm::{
    ops::parse_ops,
    verifier::{BytecodeVerifier, ValidationLimits},
    vm::Vm,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub tx_id: [u8; 32],
    pub success: bool,
    pub gas_used: u64,
    pub return_data: Vec<u8>,
}

pub trait VmExecutor: Send + Sync {
    fn execute(&self, tx: &Transaction, state_root: [u8; 32]) -> Result<ExecutionReceipt, String>;
}

pub struct AtcVmExecutor {
    pub protocol: String,
    pub vm_version: String,
    pub genesis_id: String,
}

impl VmExecutor for AtcVmExecutor {
    fn execute(&self, tx: &Transaction, _: [u8; 32]) -> Result<ExecutionReceipt, String> {
        if tx.tx_type != TxType::Contract || tx.payload.starts_with(crate::dao_state::MAGIC) {
            return Ok(ExecutionReceipt {
                tx_id: tx.id,
                success: true,
                gas_used: tx.gas_cost(),
                return_data: Vec::new(),
            });
        }

        let program = parse_ops(
            std::str::from_utf8(&tx.payload).map_err(|_| "contract payload is not UTF-8")?,
        )
        .map_err(|e| format!("invalid ATC-VM program: {e:?}"))?;

        // The transaction gas limit is an execution resource ceiling. The VM
        // verifier also keeps a protocol-level hard ceiling to prevent an
        // unbounded transaction from expanding the trust boundary.
        let defaults = ValidationLimits::default();
        let limits = ValidationLimits {
            max_gas: tx.gas_limit.min(defaults.max_gas),
            ..defaults
        };
        let validated = BytecodeVerifier::new(limits)
            .verify(program)
            .map_err(|e| format!("ATC-VM verification rejected program: {e:?}"))?;

        let mut vm = Vm::from_validated(validated);
        let context = atc_vm::context::ChainContext {
            chain_id: "atc".into(),
            network_id: "devnet".into(),
            genesis_id: self.genesis_id.clone(),
            protocol_version: self.protocol.clone(),
            vm_version: self.vm_version.clone(),
        };
        let out = vm
            .execute_state_transition(
                &context,
                &self.genesis_id,
                &self.protocol,
                &self.vm_version,
            )
            .map_err(|e| format!("ATC-VM execution: {e:?}"))?;

        Ok(ExecutionReceipt {
            tx_id: tx.id,
            success: true,
            gas_used: tx.gas_cost(),
            return_data: out.into_iter().flat_map(|v| v.to_be_bytes()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn executor() -> AtcVmExecutor {
        AtcVmExecutor {
            protocol: "1.0.0".into(),
            vm_version: "1.0.0".into(),
            genesis_id: "a".repeat(64),
        }
    }

    #[test]
    fn rejects_invalid_program_before_vm_execution() {
        let tx = Transaction::new_with_chain_id(
            0,
            TxType::Contract,
            "did:atc:sender".into(),
            None,
            0,
            1,
            5_000,
            0,
            0,
            b"Add\nHalt\n".to_vec(),
            [0; 64],
            [0; 32],
            [0; 32],
        );

        let result = executor().execute(&tx, [0; 32]);
        assert!(result
            .expect_err("invalid bytecode must be rejected")
            .contains("verification rejected program"));
    }
}
