extern crate alloc;

use {
    alloc::alloc::Layout,
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
    address_map: cell::UnsafeCell::new(INITIAL_ADDRESS_MAP),
};

pub struct Allocator {
    address_map: cell::UnsafeCell<AddressMap>,
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let requested_size: usize = layout.size();
        let align: usize = layout.align();
        let memory_type = memory_allocation::MemoryType::LoaderData;
        let allocated_size: usize = align + requested_size - 1;
        let allocated = void::Void::new();
        let mut allocated = &allocated;
        match system::system()
            .boot_services
            .allocate_pool(
                memory_type,
                allocated_size,
                &mut allocated,
            ) {
                status::SUCCESS => (),
                _ => panic!("Can't allocate memory!"),
        }
        let allocated = allocated as *const void::Void;
        let allocated = allocated as usize;
        let provided = ((allocated + align - 1) / align) * align;
        self.address_map
            .get()
            .as_mut()
            .expect("Can't allocate memory!")
            .insert(allocated, provided);
        let provided = provided as *mut u8;
        provided
    }

    unsafe fn dealloc(&self, pointer: *mut u8, _: Layout) {
        let provided = pointer as usize;
        let allocated: usize = self.address_map
            .get()
            .as_ref()
            .expect("Can't free memory!")
            .find(provided)
            .expect("Can't free memory!");
        let allocated = allocated as *const void::Void;
        let allocated = &*allocated;
        match system::system()
            .boot_services
            .free_pool(allocated) {
            status::SUCCESS => (),
            _ => panic!("Can't free memory!"),
        }
    }
}

struct AddressMap {
    pairs: [Option<AddressPair>; ADDRESS_PAIRS],
}

const ADDRESS_PAIRS: usize = 0x400;
const INITIAL_ADDRESS_MAP: AddressMap = AddressMap {
    pairs: [None; ADDRESS_PAIRS],
};

impl AddressMap {
    fn insert(&mut self, allocated: usize, provided: usize) {
        *self.pairs
            .iter_mut()
            .find(|pair| pair.is_none())
            .expect("Can't insert an address pair to an address map!")
            = Some(AddressPair::new(allocated, provided));
    }

    fn find(&self, provided: usize) -> Option<usize> {
        let key: usize = provided;
        self.pairs
            .iter()
            .filter_map(|pair| *pair)
            .find_map(|pair| {
                let AddressPair {
                    allocated,
                    provided,
                } = pair;
                if key == provided {
                    Some(allocated)
                } else {
                    None
                }
            })
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

