extern crate embeddedsw_sys;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use embeddedsw_sys as esys;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[global_allocator]
static ALLOCATOR: XAllocator = XAllocator::new();

pub struct XAllocator {}

impl XAllocator {
    pub const fn new() -> Self {
        Self {}
    }
}

unsafe impl GlobalAlloc for XAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        esys::malloc(layout.size() as u32) as *mut _
    }

    unsafe fn dealloc(
        &self,
        ptr: *mut u8,
        _layout: Layout,
    ) {
        esys::free(ptr as *mut c_void)
    }
}
