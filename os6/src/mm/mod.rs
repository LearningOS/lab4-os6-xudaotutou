//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.


mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
pub use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use memory_set::{remap_test, kernel_token};
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE, mmap, munmap};
pub use page_table::{translated_byte_buffer, translated_refmut, translated_ref, translated_str, PageTableEntry};
pub use page_table::{PTEFlags, PageTable, UserBuffer};

use crate::task::current_user_token;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

pub fn get_slice_buffer<T: 'static>(start: usize) -> Option<&'static mut T> {
    let satp = current_user_token();
    let va = VirtAddr::from(start);
    let vpn = va.floor();
    let pt = PageTable::from_token(satp);
    if let Some(pte) = pt.translate(vpn) {
        let ppn = pte.ppn();
        let pa = PhysAddr::from(PhysAddr::from(ppn).0 | va.page_offset());
        Some(pa.get_mut::<T>())
    } else {
        None
    }
}