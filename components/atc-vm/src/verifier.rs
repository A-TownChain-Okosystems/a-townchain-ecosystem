// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical structural validation boundary for ATC-VM programs.
//!
//! The verifier operates on the parsed `.ops` representation currently used
//! by the blockchain adapter. No new wire encoding is invented here: the
//! parser is the canonical text-to-`Op` boundary for the current runtime.
//!
//! Validation is deliberately conservative. A program is rejected when its
//! control-flow graph cannot be proven stack-safe with a deterministic,
//! bounded resource envelope.

use crate::vm::Op;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationLimits {
    pub max_instructions: usize,
    pub max_stack_depth: usize,
    pub max_storage_slots: usize,
    pub max_gas: u64,
}

impl Default for ValidationLimits {
    fn default() -> Self {
        Self {
            max_instructions: 1_024,
            max_stack_depth: 256,
            max_storage_slots: 1_024,
            max_gas: 10_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    EmptyProgram,
    ProgramTooLarge { actual: usize, max: usize },
    InvalidJump { pc: usize, target: usize },
    StackUnderflow { pc: usize, required: usize, available: usize },
    StackDepthExceeded { pc: usize, depth: usize, max: usize },
    InconsistentStackHeight { pc: usize, expected: usize, found: usize },
    StorageSlotExceeded { pc: usize, slot: usize, max: usize },
    GasLimitExceeded { required: u64, max: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedProgram {
    ops: Vec<Op>,
    gas_limit: u64,
}

impl ValidatedProgram {
    pub fn ops(&self) -> &[Op] {
        &self.ops
    }

    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }
}

pub struct BytecodeVerifier {
    limits: ValidationLimits,
}

impl BytecodeVerifier {
    pub fn new(limits: ValidationLimits) -> Self {
        Self { limits }
    }

    pub fn verify(&self, program: Vec<Op>) -> Result<ValidatedProgram, VerifyError> {
        if program.is_empty() {
            return Err(VerifyError::EmptyProgram);
        }
        if program.len() > self.limits.max_instructions {
            return Err(VerifyError::ProgramTooLarge {
                actual: program.len(),
                max: self.limits.max_instructions,
            });
        }

        let static_gas = program.len() as u64;
        if static_gas > self.limits.max_gas {
            return Err(VerifyError::GasLimitExceeded {
                required: static_gas,
                max: self.limits.max_gas,
            });
        }

        let mut incoming: Vec<Option<usize>> = vec![None; program.len()];
        let mut work = vec![(0usize, 0usize)];

        while let Some((pc, depth)) = work.pop() {
            if pc >= program.len() {
                return Err(VerifyError::InvalidJump {
                    pc,
                    target: pc,
                });
            }

            if let Some(expected) = incoming[pc] {
                if expected != depth {
                    return Err(VerifyError::InconsistentStackHeight {
                        pc,
                        expected,
                        found: depth,
                    });
                }
                continue;
            }
            incoming[pc] = Some(depth);

            let (next_depth, successors) = self.validate_instruction(&program, pc, depth)?;

            for target in successors {
                if target >= program.len() {
                    return Err(VerifyError::InvalidJump { pc, target });
                }
                work.push((target, next_depth));
            }
        }

        Ok(ValidatedProgram {
            ops: program,
            gas_limit: self.limits.max_gas,
        })
    }

    fn validate_instruction(
        &self,
        program: &[Op],
        pc: usize,
        depth: usize,
    ) -> Result<(usize, Vec<usize>), VerifyError> {
        let op = &program[pc];

        let (next_depth, successors) = match op {
            Op::Push(_) | Op::Load(_) | Op::Caller => (depth + 1, vec![pc + 1]),
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Eq | Op::Lt => {
                (self.pop(pc, depth, 2)? + 1, vec![pc + 1])
            }
            Op::Dup => {
                self.pop(pc, depth, 1)?;
                (depth + 1, vec![pc + 1])
            }
            Op::Swap => {
                self.pop(pc, depth, 2)?;
                (depth, vec![pc + 1])
            }
            Op::Store(_) => (self.pop(pc, depth, 1)?, vec![pc + 1]),
            Op::Jump(target) => (depth, vec![*target]),
            Op::JumpIfZero(target) | Op::JumpIfNotZero(target) => {
                let after_pop = self.pop(pc, depth, 1)?;
                (after_pop, vec![*target, pc + 1])
            }
            Op::Halt => (depth, Vec::new()),
        };

        match op {
            Op::Load(slot) | Op::Store(slot) if *slot >= self.limits.max_storage_slots => {
                return Err(VerifyError::StorageSlotExceeded {
                    pc,
                    slot: *slot,
                    max: self.limits.max_storage_slots,
                });
            }
            _ => {}
        }

        if next_depth > self.limits.max_stack_depth {
            return Err(VerifyError::StackDepthExceeded {
                pc,
                depth: next_depth,
                max: self.limits.max_stack_depth,
            });
        }

        Ok((next_depth, successors))
    }

    fn pop(&self, pc: usize, depth: usize, required: usize) -> Result<usize, VerifyError> {
        if depth < required {
            return Err(VerifyError::StackUnderflow {
                pc,
                required,
                available: depth,
            });
        }
        Ok(depth - required)
    }
}

impl Default for BytecodeVerifier {
    fn default() -> Self {
        Self::new(ValidationLimits::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify(program: Vec<Op>) -> Result<ValidatedProgram, VerifyError> {
        BytecodeVerifier::default().verify(program)
    }

    #[test]
    fn validates_arithmetic_program() {
        let program = vec![
            Op::Push(7),
            Op::Push(3),
            Op::Add,
            Op::Push(2),
            Op::Mul,
            Op::Halt,
        ];
        let validated = verify(program.clone()).expect("program must validate");
        assert_eq!(validated.ops(), program.as_slice());
    }

    #[test]
    fn rejects_stack_underflow_before_execution() {
        let err = verify(vec![Op::Add, Op::Halt]).expect_err("must reject");
        assert_eq!(
            err,
            VerifyError::StackUnderflow {
                pc: 0,
                required: 2,
                available: 0
            }
        );
    }

    #[test]
    fn rejects_invalid_jump() {
        let err = verify(vec![Op::Push(1), Op::Jump(99)]).expect_err("must reject");
        assert_eq!(err, VerifyError::InvalidJump { pc: 1, target: 99 });
    }

    #[test]
    fn rejects_inconsistent_stack_height_at_merge() {
        let err = verify(vec![
            Op::Push(1),
            Op::JumpIfZero(4),
            Op::Push(2),
            Op::Jump(5),
            Op::Push(3),
            Op::Halt,
        ])
        .expect_err("merge with different stack heights must reject");
        assert!(matches!(
            err,
            VerifyError::InconsistentStackHeight { pc: 5, .. }
        ));
    }

    #[test]
    fn rejects_storage_slot_outside_limit() {
        let limits = ValidationLimits {
            max_storage_slots: 2,
            ..ValidationLimits::default()
        };
        let err = BytecodeVerifier::new(limits)
            .verify(vec![Op::Push(1), Op::Store(2), Op::Halt])
            .expect_err("must reject");
        assert_eq!(
            err,
            VerifyError::StorageSlotExceeded {
                pc: 1,
                slot: 2,
                max: 2
            }
        );
    }

    #[test]
    fn rejects_program_above_instruction_limit() {
        let limits = ValidationLimits {
            max_instructions: 2,
            ..ValidationLimits::default()
        };
        let err = BytecodeVerifier::new(limits)
            .verify(vec![Op::Halt, Op::Halt, Op::Halt])
            .expect_err("must reject");
        assert_eq!(
            err,
            VerifyError::ProgramTooLarge {
                actual: 3,
                max: 2
            }
        );
    }

    #[test]
    fn rejects_stack_depth_above_limit() {
        let limits = ValidationLimits {
            max_stack_depth: 2,
            ..ValidationLimits::default()
        };
        let err = BytecodeVerifier::new(limits)
            .verify(vec![Op::Push(1), Op::Push(2), Op::Push(3), Op::Halt])
            .expect_err("must reject");
        assert_eq!(
            err,
            VerifyError::StackDepthExceeded {
                pc: 2,
                depth: 3,
                max: 2
            }
        );
    }

    #[test]
    fn bounds_non_terminating_control_flow_with_gas() {
        let validated = verify(vec![Op::Push(1), Op::Jump(1)]).expect("structurally valid");
        assert_eq!(validated.gas_limit(), 10_000);
    }
}
