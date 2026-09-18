// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Physical memory and paging initialization for the x86-64 boot path.

use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (frame, _) = x86_64::registers::control::Cr3::read();
    let virt = physical_memory_offset + frame.start_address().as_u64();
    &mut *virt.as_mut_ptr()
}

pub struct BootInfoFrameAllocator {
    memory_regions: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_regions: &'static MemoryRegions) -> Self {
        Self {
            memory_regions,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        self.memory_regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            .flat_map(|r| {
                let start = (r.start + 0xFFF) & !0xFFF;
                let end = r.end & !0xFFF;
                (start..end)
                    .step_by(0x1000)
                    .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
            })
    }
}

pub struct AddressSpace {
    root_frame: PhysFrame,
    mapper: OffsetPageTable<'static>,
    cr3_flags: x86_64::registers::control::Cr3Flags,
}

impl AddressSpace {
    /// Creates a new process root and inherits only the kernel half.
    pub unsafe fn new(
        physical_memory_offset: VirtAddr,
        frame_allocator: &mut BootInfoFrameAllocator,
    ) -> Self {
        let (active_frame, cr3_flags) = x86_64::registers::control::Cr3::read();
        let active_ptr = physical_memory_offset + active_frame.start_address().as_u64();
        let active_table = &*active_ptr.as_mut_ptr::<PageTable>();

        let root_frame = frame_allocator
            .allocate_frame()
            .expect("ShivaCore: no frame available for process page table");
        let root_ptr = physical_memory_offset + root_frame.start_address().as_u64();
        let root = &mut *root_ptr.as_mut_ptr::<PageTable>();
        root.zero();

        for index in 256..512 {
            root[index] = active_table[index];
        }

        let mapper = OffsetPageTable::new(root, physical_memory_offset);
        Self { root_frame, mapper, cr3_flags }
    }

    pub fn mapper(&mut self) -> &mut OffsetPageTable<'static> {
        &mut self.mapper
    }

    pub fn root_frame(&self) -> PhysFrame {
        self.root_frame
    }

    pub fn cr3_flags(&self) -> x86_64::registers::control::Cr3Flags {
        self.cr3_flags
    }

    pub unsafe fn activate(&self) {
        x86_64::registers::control::Cr3::write(self.root_frame, self.cr3_flags);
    }
}

unsafe impl Send for BootInfoFrameAllocator {}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next = self.next.saturating_add(1);
        frame
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn page_alignment_contract_is_4k() {
        assert_eq!(0x1000usize, 4096);
    }
}
