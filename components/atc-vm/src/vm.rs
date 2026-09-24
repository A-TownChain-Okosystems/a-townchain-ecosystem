// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Stack machine. State-transition execution requires a validated program
//! and is separately gated by ATC-STD-600 chain context checks.

use crate::context::{execution_gate, ChainContext, ContextError};
use crate::verifier::{BytecodeVerifier, ValidatedProgram, VerifyError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Push(u64),
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    Jump(usize),
    JumpIfNotZero(usize),
    Eq,
    Lt,
    Load(usize),
    Store(usize),
    Caller,
    JumpIfZero(usize),
    Halt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    Verification(VerifyError),
    StackUnderflow,
    InvalidJump(usize),
    DivisionByZero,
    GasExhausted,
    Context(ContextError),
}

pub struct Vm {
    program: ValidatedProgram,
    stack: Vec<u64>,
    caller: u64,
    storage: Vec<u64>,
}

#[allow(dead_code)]
impl Vm {
    /// Constructs a VM only after the canonical bytecode verifier accepts the program.
    pub fn new(program: Vec<Op>) -> Result<Self, VmError> {
        let validated = BytecodeVerifier::default()
            .verify(program)
            .map_err(VmError::Verification)?;
        Ok(Self::from_validated(validated))
    }

    /// Constructs a VM from an already verified program.
    pub fn from_validated(program: ValidatedProgram) -> Self {
        Vm {
            program,
            stack: Vec::new(),
            caller: 0,
            storage: Vec::new(),
        }
    }

    pub fn with_context(
        program: Vec<Op>,
        caller: u64,
        storage: Vec<u64>,
    ) -> Result<Self, VmError> {
        let mut vm = Self::new(program)?;
        vm.caller = caller;
        vm.storage = storage;
        Ok(vm)
    }

    pub fn caller(&self) -> u64 {
        self.caller
    }

    pub fn state(&self) -> &[u64] {
        &self.storage
    }

    pub fn execute_state_transition(
        &mut self,
        context: &ChainContext,
        computed_genesis_id: &str,
        expected_protocol: &str,
        expected_vm: &str,
    ) -> Result<Vec<u64>, VmError> {
        execution_gate(context, computed_genesis_id, expected_protocol, expected_vm)
            .map_err(VmError::Context)?;
        self.run()
    }

    /// Executes only a program that has already crossed the verifier boundary.
    pub fn run(&mut self) -> Result<Vec<u64>, VmError> {
        let mut pc = 0usize;
        let mut gas_remaining = self.program.gas_limit();

        while pc < self.program.ops().len() {
            if gas_remaining == 0 {
                return Err(VmError::GasExhausted);
            }
            gas_remaining -= 1;

            match self.program.ops()[pc].clone() {
                Op::Push(v) => self.stack.push(v),
                Op::Add => self.binop(|a, b| a.wrapping_add(b))?,
                Op::Sub => self.binop(|a, b| a.wrapping_sub(b))?,
                Op::Mul => self.binop(|a, b| a.wrapping_mul(b))?,
                Op::Div => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack
                        .push(a.checked_div(b).ok_or(VmError::DivisionByZero)?);
                }
                Op::Eq => self.binop(|a, b| (a == b) as u64)?,
                Op::Lt => self.binop(|a, b| (a < b) as u64)?,
                Op::Dup => {
                    let v = *self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(v);
                }
                Op::Swap => {
                    let n = self.stack.len();
                    if n < 2 {
                        return Err(VmError::StackUnderflow);
                    }
                    self.stack.swap(n - 1, n - 2);
                }
                Op::Load(slot) => self
                    .stack
                    .push(self.storage.get(slot).copied().unwrap_or(0)),
                Op::Store(slot) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if slot >= self.storage.len() {
                        self.storage.resize(slot + 1, 0);
                    }
                    self.storage[slot] = v;
                }
                Op::Caller => self.stack.push(self.caller),
                Op::JumpIfZero(t) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v == 0 {
                        pc = self.valid_jump(t)?;
                        continue;
                    }
                }
                Op::Jump(t) => {
                    pc = self.valid_jump(t)?;
                    continue;
                }
                Op::JumpIfNotZero(t) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v != 0 {
                        pc = self.valid_jump(t)?;
                        continue;
                    }
                }
                Op::Halt => break,
            }
            pc += 1;
        }
        Ok(std::mem::take(&mut self.stack))
    }

    fn binop(&mut self, f: impl Fn(u64, u64) -> u64) -> Result<(), VmError> {
        let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
        self.stack.push(f(a, b));
        Ok(())
    }

    fn valid_jump(&self, t: usize) -> Result<usize, VmError> {
        if t < self.program.ops().len() {
            Ok(t)
        } else {
            Err(VmError::InvalidJump(t))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> ChainContext {
        ChainContext {
            chain_id: "atc".into(),
            network_id: "devnet".into(),
            genesis_id: "a".repeat(64),
            protocol_version: "1.0.0".into(),
            vm_version: "1.0.0".into(),
        }
    }

    #[test]
    fn arithmetik() {
        let mut vm = Vm::new(vec![
            Op::Push(2),
            Op::Push(3),
            Op::Add,
            Op::Push(4),
            Op::Mul,
            Op::Halt,
        ])
        .expect("program must validate");
        assert_eq!(vm.run(), Ok(vec![20]));
    }

    #[test]
    fn state_transition_requires_identity_gate() {
        let mut vm =
            Vm::with_context(vec![Op::Push(7), Op::Store(0), Op::Halt], 1, vec![])
                .expect("program must validate");
        assert!(vm
            .execute_state_transition(&context(), &"b".repeat(64), "1.0.0", "1.0.0")
            .is_err());
        assert!(
            vm.state().is_empty(),
            "invalid context darf keinen State mutieren"
        );
        assert!(vm
            .execute_state_transition(&context(), &"a".repeat(64), "1.0.0", "1.0.0")
            .is_ok());
        assert_eq!(vm.state(), &[7]);
    }

    #[test]
    fn verifier_rejects_invalid_program_without_constructing_vm() {
        let result = Vm::new(vec![Op::Add, Op::Halt]);
        assert!(matches!(
            result,
            Err(VmError::Verification(VerifyError::StackUnderflow { .. }))
        ));
    }

    #[test]
    fn verifier_rejects_invalid_jump_without_constructing_vm() {
        let result = Vm::new(vec![Op::Push(1), Op::Jump(99)]);
        assert!(matches!(
            result,
            Err(VmError::Verification(VerifyError::InvalidJump { .. }))
        ));
    }

    #[test]
    fn gas_bounds_non_terminating_valid_control_flow() {
        let mut vm = Vm::new(vec![Op::Push(1), Op::Jump(1)]).expect("program is structurally valid");
        assert_eq!(vm.run(), Err(VmError::GasExhausted));
    }
}
