// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Kernel-Einstiegspunkt.
// K-Sprint 0: Boot (BIOS+UEFI via bootloader 0.11), serielle Debug-Konsole,
// Framebuffer-Textausgabe.
// K-Sprint 1: GDT + TSS, IDT, PIC.
// K-Sprint 2: Paging, Frame-Allocator, Heap.
// K-Sprint 3: kernel-mode execution context switch + init task bootstrap.
#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![no_main]

extern crate alloc;

mod allocator;
mod ats1000;
mod context;
mod framebuffer;
mod gdt;
mod hal;
mod interrupts;
mod memory;
mod serial;

use alloc::{boxed::Box, vec::Vec};
use bootloader_api::{
    config::{BootloaderConfig, Mapping},
    entry_point, BootInfo,
};
use core::panic::PanicInfo;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

extern "C" fn globus_init_kernel_task() -> ! {
    serial_println!("ShivaCore: GlobusOS init task entered via CPU context switch.");
    serial_println!("ShivaCore: kernel execution context switch OK.");
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("ShivaCore: Kernel-Einstiegspunkt erreicht.");

    let cpu = hal::CpuInfo::detect();
    serial_println!(
        "ShivaCore: CPU vendor={} family={} model={} stepping={} apic={} x2apic={} nx={} invariant_tsc={}",
        cpu.vendor_name(), cpu.family, cpu.model, cpu.stepping,
        cpu.features.apic, cpu.features.x2apic, cpu.features.nx, cpu.features.invariant_tsc
    );
    if !cpu.boot_compatible() {
        panic!("ShivaCore: CPU lacks required x86-64 boot features (SSE2/NX/APIC)");
    }
    serial_println!("ShivaCore: CPU HAL compatibility check OK.");

    if let Some(fb) = boot_info.framebuffer.as_mut() {
        framebuffer::init(fb);
        println!("ShivaCore Kernel v0.0.3 -- K-Sprint 3");
        println!("Boot: OK | Serial: OK | Framebuffer: OK");
    } else {
        serial_println!("ShivaCore: WARNUNG -- kein Framebuffer vom Bootloader erhalten.");
    }

    gdt::init();
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();
    interrupts::init_pics();
    serial_println!("ShivaCore: GDT/IDT/PIC OK.");

    let phys_mem_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("Bootloader hat physical_memory_offset nicht gesetzt (Config fehlt?)");
    let phys_mem_offset = x86_64::VirtAddr::new(phys_mem_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap-Initialisierung fehlgeschlagen");
    serial_println!("ShivaCore: Paging-Mapper + Heap initialisiert (100 KiB).");

    // kernel_init belongs to the library crate. The binary must consume the
    // exported library module instead of redeclaring it as a binary module.
    let mut kernel = match shivacore::kernel_init::KernelState::boot() {
        Ok(state) => state,
        Err(error) => panic!("ShivaCore: kernel subsystem initialization failed: {:?}", error),
    };
    serial_println!("ShivaCore: kernel subsystem initialization OK.");

    kernel
        .smoke_test()
        .expect("ShivaCore: kernel boot smoke test failed");
    serial_println!("ShivaCore: kernel smoke test OK.");

    let boxed = Box::new(41);
    serial_println!("ShivaCore: Box-Test -- Wert: {}", *boxed);

    let mut vec = Vec::new();
    for i in 0..10 {
        vec.push(i);
    }
    serial_println!(
        "ShivaCore: Vec-Test -- Summe 0..10: {}",
        vec.iter().sum::<i32>()
    );

    // Register the first system process in the kernel process table before
    // transferring CPU execution to its dedicated kernel stack.
    let init_pid = kernel
        .processes
        .spawn(shivacore::process::ProcessType::System, 255);
    serial_println!("ShivaCore: GlobusOS init process registered pid={}.", init_pid.0);

    let mut init_task = context::BootstrapProcess::new(globus_init_kernel_task);
    let mut current_context = context::Context::empty();

    println!("K-Sprint 3: kernel context switch -> GlobusOS init");
    serial_println!("ShivaCore: switching CPU context to GlobusOS init task.");

    unsafe {
        context::switch(&mut current_context, init_task.context());
    }

    // The bootstrap task is deliberately non-returning. Reaching this point
    // means the context-switch contract was violated.
    let _ = init_task.context_mut();
    panic!("ShivaCore: init task unexpectedly returned");
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allokation fehlgeschlagen: {:?}", layout)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("ShivaCore: KERNEL PANIC -- {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
