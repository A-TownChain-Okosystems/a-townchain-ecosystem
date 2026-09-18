// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Kernel execution bridge: Process -> CPU context -> address space.
//!
//! This is the first concrete scheduler bridge. It deliberately keeps the
//! scheduling policy small: a ready queue selects one kernel execution context
//! at a time, activates that process' CR3, then enters the existing context
//! switch primitive. It does not pretend to implement SMP or time slicing yet.

use alloc::collections::VecDeque;

use crate::context::{self, BootstrapProcess, Context};
use crate::memory::AddressSpace;
use crate::ats1000::Pid;
use x86_64::registers::control::Cr3;

pub struct ScheduledProcess {
    pub pid: Pid,
    process: BootstrapProcess,
    root_frame: x86_64::structures::paging::PhysFrame,
    cr3_flags: x86_64::registers::control::Cr3Flags,
}

impl ScheduledProcess {
    pub fn new(pid: Pid, process: BootstrapProcess, address_space: &AddressSpace) -> Self {
        Self {
            pid,
            process,
            root_frame: address_space.root_frame(),
            cr3_flags: address_space.cr3_flags(),
        }
    }

    pub fn context(&self) -> &Context {
        self.process.context()
    }

    /// Activates the process address space before CPU context entry.
    ///
    /// Safety: the root frame must refer to a valid page table and all kernel
    /// mappings required by the next context must remain present.
    pub unsafe fn activate_address_space(&self) {
        Cr3::write(self.root_frame, self.cr3_flags);
    }
}

pub struct ProcessScheduler {
    ready: VecDeque<ScheduledProcess>,
    current: Option<Pid>,
}

impl ProcessScheduler {
    pub const fn new() -> Self {
        Self { ready: VecDeque::new(), current: None }
    }

    pub fn enqueue(&mut self, process: ScheduledProcess) {
        self.ready.push_back(process);
    }

    pub fn ready_len(&self) -> usize {
        self.ready.len()
    }

    /// Selects the next process and enters it through the kernel context switch.
    ///
    /// The caller owns the current context and must not return from the selected
    /// process unless that process explicitly transfers control back.
    pub unsafe fn run_next(&mut self, current_context: &mut Context) -> ! {
        let process = self
            .ready
            .pop_front()
            .expect("ShivaCore: scheduler run requested with empty ready queue");
        self.current = Some(process.pid);
        serial_println!("ShivaCore: scheduler selected pid={}.", process.pid.0);
        process.activate_address_space();
        context::switch(current_context, process.context());
        panic!("ShivaCore: scheduled process returned unexpectedly");
    }

    pub fn current_pid(&self) -> Option<Pid> {
        self.current
    }
}
