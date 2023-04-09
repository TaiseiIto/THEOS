extern crate alloc;

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell,
        slice,
    },
    crate::{
        uefi_print,
        uefi_println,
    },
    super::uefi::{
        services::boot::memory_allocation,
        tables::system,
        types::void,
    },
};

pub struct Allocated<'a> {
    slice: &'a mut [u8],
    layout: Layout,
}

impl<'a> Allocated<'a> {
    pub fn new(size: usize, align: usize) -> Self {
        let layout = Layout::from_size_align(size, Self::align(align)).expect("Can't allocate memory!");
        let slice: &'a mut [u8] = unsafe {
            slice::from_raw_parts_mut(ALLOCATOR.alloc(layout), size)
        };
        Self {
            slice,
            layout,
        }
    }

    pub fn get_ref(&self) -> &[u8] {
        self.slice.as_ref()
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        self.slice
    }

    fn align(align: usize) -> usize {
        let mut fixed_align = 1;
        while fixed_align < align {
            fixed_align *= 2;
        }
        fixed_align
    }
}

impl<'a> Drop for Allocated<'a> {
    fn drop(&mut self) {
        let layout: Layout = self.layout;
        let slice: &mut [u8] = self.slice;
        let reference: &mut u8 = &mut slice[0];
        let pointer = reference as *mut u8;
        unsafe {
            ALLOCATOR.dealloc(pointer, layout)
        }
    }
}

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator {
    address_map: cell::UnsafeCell::new(INITIAL_ADDRESS_MAP),
};

pub struct Allocator {
    address_map: cell::UnsafeCell<AddressMap>,
}

impl Allocator {
    fn remaining_pairs(&self) -> usize {
        unsafe {
            self.address_map
                .get()
                .as_ref()
                .expect("Can't get a number of remaining pairs!")
                .remaining_pairs()
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let requested_size: usize = layout.size();
        uefi_println!("alloc.requested_size = {:#x}", requested_size);
        let align: usize = layout.align();
        let align: usize = if align <= 8 {
            1
        } else {
            align
        };
        uefi_println!("alloc.align = {:#x}", align);
        let memory_type = memory_allocation::MemoryType::LoaderData;
        let allocated_size: usize = align + requested_size - 1;
        uefi_println!("alloc.allocated_size = {:#x}", allocated_size);
        let allocated = void::Void::new();
        let mut allocated = &allocated;
        system::system()
            .boot_services
            .allocate_pool(
                memory_type,
                allocated_size,
                &mut allocated,
            )
            .expect("Can't allocate memory!");
        let allocated = allocated as *const void::Void;
        let allocated = allocated as usize;
        uefi_println!("alloc.allocated = {:#x}", allocated);
        let provided = ((allocated + align - 1) / align) * align;
        uefi_println!("alloc.provided = {:#x}", provided);
        self.address_map
            .get()
            .as_mut()
            .expect("Can't allocate memory!")
            .insert(allocated, provided);
        uefi_println!("alloc.remaining_pairs = {:#x}", self.remaining_pairs());
        let provided = provided as *mut u8;
        provided
    }

    unsafe fn dealloc(&self, pointer: *mut u8, _: Layout) {
        let provided = pointer as usize;
        uefi_println!("dealloc.provided = {:#x}", provided);
        let allocated: usize = self.address_map
            .get()
            .as_ref()
            .expect("Can't free memory!")
            .find(provided)
            .expect("Can't free memory!");
        uefi_println!("dealloc.allocated = {:#x}", allocated);
        let allocated = allocated as *const void::Void;
        let allocated = &*allocated;
        system::system()
            .boot_services
            .free_pool(allocated)
            .expect("Can't free memory!");
        self.address_map
            .get()
            .as_mut()
            .expect("Can't free memory!")
            .delete(provided);
        uefi_println!("dealloc.remaining_pairs = {:#x}", self.remaining_pairs());
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

    fn delete(&mut self, provided: usize) {
        let key: usize = provided;
        self.pairs
            .iter_mut()
            .for_each(|pair| 
                if let Some(AddressPair {
                    allocated: _,
                    provided,
                }) = pair {
                    if *provided == key {
                        *pair = None;
                    }
                }
            )
    }

    fn remaining_pairs(&self) -> usize {
        self.pairs
            .iter()
            .filter(|pair| pair.is_none())
            .count()
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

