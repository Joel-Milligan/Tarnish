use bootloader_api::info::{MemoryRegionKind, MemoryRegions, Optional};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{self, OffsetPageTable, PageTable, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};

pub mod allocator;

/// Initialize a new OffsetPageTable.
///
/// # Safety
///
/// Caller must guarantee that the complete physical memory is
/// mapped to virtual memory at the passed `physical_memory_offset`.
/// Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init_paging(physical_memory_offset: Optional<u64>) -> OffsetPageTable<'static> {
    let phys_mem_offset = VirtAddr::new(physical_memory_offset.into_option().unwrap());
    let level_4_table = active_level_4_table(phys_mem_offset);
    OffsetPageTable::new(level_4_table, phys_mem_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub struct BootInfoFrameAllocator {
    memory_regions: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the passed memory map is valid.
    /// The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_regions: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_regions,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable) // get usable regions from memory map
            .map(|r| r.start..r.end) // map each region to its address range
            .flat_map(|r| r.step_by(4096)) // transform to an iterator of frame start addresses
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr))) // create `PhysFrame` types from the start addresses
    }
}

unsafe impl paging::FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
