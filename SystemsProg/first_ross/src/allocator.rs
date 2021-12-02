use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use bump::BumpAllocator;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags,
        Size4KiB,
    },
    VirtAddr,
};

pub mod bump;
pub mod linked_list;
pub struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

// use linked_list_allocator::LockedHeap;
// static ALLOCATOR: LockedHeap = LockedHeap::empty();
// static ALLOCATOR: DummyAllocator = DummyAllocator;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB;

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe { ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE) }
    Ok(())
}

/// A wrapper around spin::Mutex to permit trait implementations
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// Align the given address `addr` upwards to alignment `align`.
fn align_up(addr: usize, align: usize) -> usize {
    // - The `GlobalAlloc` trait guarantees that `align is always a power of
    // two. This means that `align - 1` has all the lower bits set (e.g. 0b0011)
    // - The bitwise `NOT` set all the bits expect the lower bits of `align`
    // - The bitwise `AND` align upwards the bits of address
    (addr + align - 1) & !(align - 1)
    //    let remainder = addr % align;
    //    if remainder == 0 {
    //        addr // addr already aligned
    //    } else {
    //        addr - remainder + align
    //    }
}
