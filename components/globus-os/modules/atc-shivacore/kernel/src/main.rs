// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Kernel-Einstiegspunkt.
// K-Sprint 0: Boot protocol + diagnostics.
// K-Sprint 1: GDT/TSS + IDT/PIC.
// K-Sprint 2: Paging/frame allocator/heap.
// K-Sprint 3: kernel context switch + controlled ring-3 init handoff.
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
mod user_transition;

use alloc::{boxed::Box, vec::Vec};
use bootloader_api::{config::{BootloaderConfig, Mapping}, entry_point, BootInfo};
use core::panic::PanicInfo;

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
    serial_println!("ShivaCore: kernel execution context switch OK.");

    let handoff = unsafe {
        INIT_HANDOFF
            .as_ref()
            .expect("ShivaCore: missing init handoff state")
    };

    serial_println!("ShivaCore: entering controlled GlobusOS ring-3 init.");
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
    serial_println!(
        "ShivaCore: CPU vendor={} family={} model={} stepping={} apic={} x2apic={} nx={} invariant_tsc={}",
        cpu.vendor_name(), cpu.family, cpu.model, cpu.stepping,
        cpu.features.apic, cpu.features.x2apic, cpu.features.nx, cpu.features.invariant_tsc
    );
    if !cpu.boot_compatible() {
        panic!("ShivaCore: CPU lacks required x86-64 boot features (SSE2/NX/APIC)");
    }

    if let Some(fb) = boot_info.framebuffer.as_mut() {
        framebuffer::init(fb);
        println!("ShivaCore Kernel v0.0.3 -- K-Sprint 3");
        println!("Boot: OK | Serial: OK | Framebuffer: OK");
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

    let mut kernel = match shivacore::kernel_init::KernelState::boot() {
        Ok(state) => state,
        Err(error) => panic!("ShivaCore: kernel subsystem initialization failed: {:?}", error),
    };
    serial_println!("ShivaCore: kernel subsystem initialization OK.");

    kernel.smoke_test().expect("ShivaCore: kernel boot smoke test failed");
    serial_println!("ShivaCore: kernel smoke test OK.");

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
    serial_println!("ShivaCore: GlobusOS init process registered pid={}.", init_pid.0);

    // Keep the boot-owned mapper/allocator alive while the init task performs
    // its final page mappings and IRETQ transition.
    let mut init_address_space = unsafe {
        memory::AddressSpace::new(phys_mem_offset, &mut frame_allocator)
    };
    serial_println!(
        "ShivaCore: isolated init address space created root_frame={:?}.",
        init_address_space.root_frame()
    );

    unsafe {
        INIT_HANDOFF = Some(InitHandoff {
            address_space: &mut init_address_space as *mut _,
            frame_allocator: &mut frame_allocator as *mut _,
            physical_memory_offset: phys_mem_offset.as_u64(),
        });
    }

    let init_task = context::BootstrapProcess::new(globus_init_kernel_task);
    let mut current_context = context::Context::empty();

    println!("K-Sprint 3: kernel context switch -> GlobusOS init -> ring3");
    serial_println!("ShivaCore: switching CPU context to GlobusOS init task.");

    unsafe {
        context::switch(&mut current_context, init_task.context());
    }

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
