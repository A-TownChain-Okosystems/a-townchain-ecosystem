// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Minimal x86_64 kernel context switching.
//!
//! This is intentionally limited to the kernel execution domain. Ring-3
//! transition and per-process page tables are implemented separately so that
//! no unsafe userspace entry is implied by a kernel-only context switch.

#![cfg(target_arch = "x86_64")]

use alloc::boxed::Box;
use alloc::vec;
use core::arch::asm;

const STACK_SIZE: usize = 16 * 1024;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub rsp: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl Context {
    pub const fn empty() -> Self {
        Self { rsp: 0, rbx: 0, rbp: 0, r12: 0, r13: 0, r14: 0, r15: 0 }
    }
}

pub struct BootstrapProcess {
    stack: Box<[u8]>,
    context: Context,
}

impl BootstrapProcess {
    pub fn new(entry: extern "C" fn() -> !) -> Self {
        let mut stack = vec![0u8; STACK_SIZE].into_boxed_slice();
        let top = stack.as_mut_ptr() as usize + stack.len();
        let stack_top = (top & !0xFusize).saturating_sub(8);
        unsafe { (stack_top as *mut u64).write(entry as usize as u64); }

        Self {
            stack,
            context: Context { rsp: stack_top as u64, ..Context::empty() },
        }
    }

    pub fn context(&self) -> &Context { &self.context }
    pub fn context_mut(&mut self) -> &mut Context { &mut self.context }
    pub fn stack_size(&self) -> usize { self.stack.len() }
}

/// Switches from the current kernel context to next.
///
/// Safety:
/// - current and next must point to valid contexts.
/// - next.rsp must point at a stack prepared for the epilogue below.
/// - both contexts must remain alive for the duration of the switch.
/// - this primitive does not change privilege level or address space.
#[inline(never)]
pub unsafe fn switch(current: &mut Context, next: &Context) {
    asm!(
        "push r15",
        "push r14",
        "push r13",
        "push r12",
        "push rbp",
        "push rbx",
        "mov [rdi + 0x00], rsp",
        "mov rsp, [rsi + 0x00]",
        "pop rbx",
        "pop rbp",
        "pop r12",
        "pop r13",
        "pop r14",
        "pop r15",
        "ret",
        in("rdi") current,
        in("rsi") next,
        options(preserves_flags)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn never_returns() -> ! { loop {} }

    #[test]
    fn bootstrap_context_has_stack_and_entry() {
        let process = BootstrapProcess::new(never_returns);
        assert_eq!(process.stack_size(), STACK_SIZE);
        assert_ne!(process.context().rsp, 0);
    }
}
