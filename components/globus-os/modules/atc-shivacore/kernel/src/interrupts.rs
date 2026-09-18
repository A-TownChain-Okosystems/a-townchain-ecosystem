// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems.
// ShivaCore — Interrupt Descriptor Table + PIC-Remapping.

use crate::gdt;
use crate::serial_println;
use crate::syscall::SyscallRequest;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::{PrivilegeLevel, VirtAddr};
use core::arch::global_asm;

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const SYSCALL_VECTOR: u8 = 0x80;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex { Timer = PIC_1_OFFSET, Keyboard }

impl InterruptIndex {
    fn as_u8(self) -> u8 { self as u8 }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct SyscallRegisters {
    rax: u64, rbx: u64, rcx: u64, rdx: u64, rsi: u64, rdi: u64, rbp: u64,
    r8: u64, r9: u64, r10: u64, r11: u64, r12: u64, r13: u64, r14: u64, r15: u64,
}

global_asm!(
    r#"
    .global shivacore_syscall_entry
    .type shivacore_syscall_entry, @function
shivacore_syscall_entry:
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rbp
    push rdi
    push rsi
    push rdx
    push rcx
    push rbx
    push rax
    mov rdi, rsp
    call {handler}
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
    iretq
    "#,
    handler = sym syscall_rust_handler,
);

unsafe extern "C" { fn shivacore_syscall_entry(); }

extern "C" fn syscall_rust_handler(frame: *mut SyscallRegisters) {
    let frame = unsafe { &mut *frame };
    let capability = (frame.rdi != 0).then_some(libshivacore::CapabilityHandle(frame.rdi));

    let request = SyscallRequest {
        abi_version: libshivacore::ABI_VERSION,
        syscall_id: frame.rax as u16,
        capability,
        arg0: frame.rsi,
        arg1: frame.rdx,
        payload_len: frame.r10 as usize,
    };

    let response = crate::syscall::dispatch_current(request);
    frame.rax = response.value;
    frame.rdx = response.error.map(|e| e as u32 as u64).unwrap_or(0);

    serial_println!(
        "ShivaCore: ring3 syscall pid={:?} id={} error={} value={}",
        crate::execution::current_pid(),
        request.syscall_id,
        frame.rdx,
        frame.rax
    );
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            idt[SYSCALL_VECTOR]
                .set_handler_addr(VirtAddr::new(shivacore_syscall_entry as usize as u64))
                .set_privilege_level(PrivilegeLevel::Ring3);
        }
        idt[InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_u8()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() { IDT.load(); }

pub fn init_pics() {
    unsafe { PICS.lock().initialize(); }
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    serial_println!("EXCEPTION: BREAKPOINT
{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT
{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    serial_println!("EXCEPTION: PAGE FAULT");
    serial_println!("Accessed Address: {:?}", Cr2::read());
    serial_println!("Error Code: {:?}", error_code);
    serial_println!("{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    crate::execution::on_timer_tick();
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let mut port: Port<u8> = Port::new(0x60);
    let _scancode: u8 = unsafe { port.read() };
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8()); }
}
