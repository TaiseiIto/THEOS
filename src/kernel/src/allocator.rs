extern crate alloc;

use {
    alloc::alloc::Layout,
    core::alloc::GlobalAlloc,
};

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator {};

pub struct Allocator {}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        panic!("The global allocator is unimplemented!");
    }

    unsafe fn dealloc(&self, _pointer: *mut u8, _: Layout) {
        panic!("The global allocator is unimplemented!");
    }
}

