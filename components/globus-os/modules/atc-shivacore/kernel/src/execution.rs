// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Kernel execution bridge: Process -> CPU context -> address space.
//!
//! The scheduler owns the authoritative current PID used by the syscall boundary.
//! This keeps syscall identity tied to the scheduler rather than a userspace value.

use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::ats1000::Pid;
use crate::context::{self, BootstrapProcess, Context};
use crate::memory::AddressSpace;
use x86_64::registers::control::Cr3;

static CURRENT_PID: AtomicU32 = AtomicU32::new(0);

pub fn current_pid() -> Option<Pid> {
    match CURRENT_PID.load(Ordering::Acquire) {
        0 => None,
        pid => Some(Pid(pid)),
    }
}

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

    pub unsafe fn run_next(&mut self, current_context: &mut Context) -> ! {
        let process = self
            .ready
            .pop_front()
            .expect("ShivaCore: scheduler run requested with empty ready queue");

        self.current = Some(process.pid);
        CURRENT_PID.store(process.pid.0, Ordering::Release);
        serial_println!("ShivaCore: scheduler selected pid={}.", process.pid.0);

        process.activate_address_space();
        context::switch(current_context, process.context());
        panic!("ShivaCore: scheduled process returned unexpectedly");
    }

    pub fn current_pid(&self) -> Option<Pid> {
        self.current
    }
}
