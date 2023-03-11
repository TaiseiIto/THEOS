extern crate alloc;

use {
    alloc::{
        alloc::Layout,
        collections::btree_map,
    },
    core::{
        alloc::GlobalAlloc,
        cell,
    },
    super::uefi::{
        services::boot::memory_allocation,
        tables::system,
        types::{
            status,
            void,
        },
    },
};

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator {
    system: None,
    provided_to_allocated: cell::UnsafeCell::new(btree_map::BTreeMap::new()),
};

pub struct Allocator {
    system: Option<system::System<'static>>,
    provided_to_allocated: cell::UnsafeCell<btree_map::BTreeMap<usize, usize>>,
}

impl Allocator {
    pub fn set_system(system: system::System<'static>) {
        unsafe {
            ALLOCATOR.system = Some(system);
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let requested_size: usize = layout.size();
        let align: usize = layout.align();
        let memory_type = memory_allocation::MemoryType::LoaderData;
        let allocated_size: usize = align + requested_size - 1;
        let allocated = void::Void::new();
        let mut allocated = &allocated;
        match &self.system {
            Some(system) => match system.boot_services.allocate_pool(
                memory_type,
                allocated_size,
                &mut allocated,
            ) {
                status::SUCCESS => Ok(()),
                _ => Err(()),
            },
            None => Err(()),
        }.expect("Can't allocate memory!");
        let allocated = allocated as *const void::Void;
        let allocated = allocated as usize;
        let provided = ((allocated + align - 1) / align) * align;
        self.provided_to_allocated
            .get()
            .as_mut()
            .expect("Can't allocate memory!")
            .insert(provided, allocated);
        let provided = provided as *mut u8;
        provided
    }

    unsafe fn dealloc(&self, pointer: *mut u8, _: Layout) {
        let provided = pointer as usize;
        let allocated: usize = *self.provided_to_allocated
            .get()
            .as_ref()
            .expect("Can't free memory!")
            .get(&provided)
            .expect("Can't free memory!");
        let allocated = allocated as *const void::Void;
        let allocated = &*allocated;
        match &self.system {
            Some(system) => match system.boot_services.free_pool(allocated) {
                status::SUCCESS => Ok(()),
                _ => Err(()),
            },
            None => Err(()),
        }.expect("Can't free memory!");
    }
}

