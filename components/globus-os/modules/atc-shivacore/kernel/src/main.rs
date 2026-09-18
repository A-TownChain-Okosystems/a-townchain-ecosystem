// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Kernel-Einstiegspunkt.
// K-Sprint 5: ProcessManager -> Scheduler -> CR3 -> Ring3 -> Syscall.
#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![no_main]

extern crate alloc;

mod allocator;
mod ats1000;
mod capability;
mod context;
mod execution;
mod framebuffer;
mod gdt;
mod hal;
mod interrupts;
mod memory;
mod serial;
mod syscall;
mod user_transition;

use alloc::{boxed::Box, vec::Vec};
use bootloader_api::{config::{BootloaderConfig, Mapping}, entry_point, BootInfo};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

struct InitHandoff {
    address_space: *mut memory::AddressSpace,
    frame_allocator: *mut memory::BootInfoFrameAllocator,
    physical_memory_offset: u64,
}

unsafe impl Send for InitHandoff {}
unsafe impl Sync for InitHandoff {}

static mut INIT_HANDOFF: Option<InitHandoff> = None;

extern "C" fn globus_init_kernel_task() -> ! {
    serial_println!("ShivaCore: GlobusOS init task entered via CPU context switch.");
    let handoff = unsafe {
        INIT_HANDOFF.as_ref().expect("ShivaCore: missing init handoff state")
    };
    unsafe {
        user_transition::enter_init(
            &mut *handoff.address_space,
            &mut *handoff.frame_allocator,
            handoff.physical_memory_offset,
        )
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("ShivaCore: Kernel-Einstiegspunkt erreicht.");

    let cpu = hal::CpuInfo::detect();
    if !cpu.boot_compatible() {
        panic!("ShivaCore: CPU lacks required x86-64 boot features (SSE2/NX/APIC)");
    }

    if let Some(fb) = boot_info.framebuffer.as_mut() {
        framebuffer::init(fb);
        println!("ShivaCore Kernel v0.0.3 -- K-Sprint 5");
        println!("Boot: OK | Serial: OK | Framebuffer: OK");
    }

    gdt::init();
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();
    interrupts::init_pics();

    let phys_mem_offset = x86_64::VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("Bootloader physical_memory_offset missing"),
    );

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("ShivaCore: heap initialization failed");

    let mut kernel = match shivacore::kernel_init::KernelState::boot() {
        Ok(state) => state,
        Err(error) => panic!("ShivaCore: kernel subsystem initialization failed: {:?}", error),
    };
    kernel.smoke_test().expect("ShivaCore: kernel boot smoke test failed");

    let boxed = Box::new(41);
    serial_println!("ShivaCore: Box-Test -- Wert: {}", *boxed);

    let mut vec = Vec::new();
    for i in 0..10 {
        vec.push(i);
    }
    serial_println!("ShivaCore: Vec-Test -- Summe 0..10: {}", vec.iter().sum::<i32>());

    let init_pid = kernel
        .processes
        .spawn(shivacore::process::ProcessType::System, 255);
    assert!(kernel.processes.set_running(init_pid));

    let syscall_capabilities = kernel.processes.caps.clone();
    syscall::install_capability_state(syscall_capabilities);

    let mut init_address_space =
        unsafe { memory::AddressSpace::new(phys_mem_offset, &mut frame_allocator) };

    unsafe {
        INIT_HANDOFF = Some(InitHandoff {
            address_space: &mut init_address_space,
            frame_allocator: &mut frame_allocator,
            physical_memory_offset: phys_mem_offset.as_u64(),
        });
    }

    let init_task = context::BootstrapProcess::new(globus_init_kernel_task);
    let scheduled_init =
        execution::ScheduledProcess::new(init_pid, init_task, &init_address_space);

    let mut process_scheduler = execution::ProcessScheduler::new();
    process_scheduler.enqueue(scheduled_init);

    let mut current_context = context::Context::empty();

    println!("K-Sprint 5: ProcessManager -> Scheduler -> CR3 -> Ring3 -> Syscall");
    serial_println!(
        "ShivaCore: ready queue contains {} process.",
        process_scheduler.ready_len()
    );

    unsafe { process_scheduler.run_next(&mut current_context); }
}
