// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Global Descriptor Table + Task State Segment.

use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{Segment, CS, SS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const USER_CODE_SELECTOR_RPL3: u16 = 3;
pub const USER_DATA_SELECTOR_RPL3: u16 = 5;

const STACK_SIZE: usize = 4096 * 5;
const USER_ENTRY_STACK_SIZE: usize = 4096 * 2;

static mut USER_KERNEL_STACK: [u8; USER_ENTRY_STACK_SIZE] = [0; USER_ENTRY_STACK_SIZE];

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(core::ptr::addr_of!(STACK));
            stack_start + STACK_SIZE as u64
        };
        tss.privilege_stack_table[0] = unsafe {
            let start = core::ptr::addr_of!(USER_KERNEL_STACK) as *const u8;
            VirtAddr::from_ptr(start) + USER_ENTRY_STACK_SIZE as u64
        };
        tss
    };
}

pub struct Selectors {
    pub kernel_code_selector: SegmentSelector,
    pub user_code_selector: SegmentSelector,
    pub user_data_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code_selector = gdt.append(Descriptor::kernel_code_segment());
        let user_code_selector = gdt.append(Descriptor::user_code_segment());
        let user_data_selector = gdt.append(Descriptor::user_data_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                kernel_code_selector,
                user_code_selector,
                user_data_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kernel_code_selector);
        load_tss(GDT.1.tss_selector);
        SS::set_reg(SegmentSelector::NULL);
    }
}

pub fn user_code_selector() -> SegmentSelector {
    GDT.1.user_code_selector
}

pub fn user_data_selector() -> SegmentSelector {
    GDT.1.user_data_selector
}
