extern crate alloc;

use {
    alloc::alloc::Layout,
    core::alloc::GlobalAlloc,
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
};

pub struct Allocator {
    system: Option<system::System<'static>>
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
        let allocated = ((allocated + align - 1) / align) * align;
        let allocated = allocated as *mut u8;
        allocated
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
    }
}

#[derive(Clone, Copy)]
struct AddressPair {
    allocated: usize,
    provided: usize,
}

impl AddressPair {
    fn new(allocated: usize, provided: usize) -> Self {
        Self {
            allocated,
            provided,
        }
    }
}

struct AddressMap {
    pairs: [Option<AddressPair>; ADDRESS_PAIRS],
}

const ADDRESS_PAIRS: usize = 0x400;

impl AddressMap {
    fn new() -> Self {
        let pairs: [Option<AddressPair>; ADDRESS_PAIRS] = [None; ADDRESS_PAIRS];
        Self {
            pairs,
        }
    }

    fn insert(&mut self, allocated: usize, provided: usize) {
        *self.pairs
            .iter_mut()
            .find(|pair| pair.is_none())
            .expect("Can't insert an address pair to an address map!")
            = Some(AddressPair::new(allocated, provided));
    }

    fn find(&self, provided: usize) -> Option<usize> {
        self.pairs
            .iter()
            .filter_map(|pair| *pair)
            .find_map(|pair| if let AddressPair {
                allocated,
                provided,
            } = pair {
                Some(allocated)
            } else {
                None
            })
    }
}

