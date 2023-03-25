pub mod paging;

use {
    core::slice,
    super::uefi::{
        services::boot::memory_allocation,
        tables::system,
    },
};

#[derive(Debug)]
pub struct Pages<'a> {
    bytes: &'a [u8],
    pages: usize,
    physical_address: memory_allocation::PhysicalAddress,
}

impl Pages<'_> {
    pub fn new(pages: usize) -> Self {
        let mut physical_address: memory_allocation::PhysicalAddress = 0;
        system::system()
            .boot_services
            .allocate_pages(
                memory_allocation::AllocateType::AllocateAnyPages,
                memory_allocation::MemoryType::LoaderData,
                pages,
                &mut physical_address,
            )
            .expect("Can't allocate pages!");
        // Assume identity mapping.
        let virtual_address: *const u8 = physical_address as *const u8;
        let length: usize = pages * memory_allocation::PAGE_SIZE;
        let bytes: &[u8] = unsafe {
            slice::from_raw_parts(virtual_address, length)
        };
        Self {
            bytes,
            pages,
            physical_address,
        }
    }
}

impl Drop for Pages<'_> {
    fn drop(&mut self) {
        system::system()
            .boot_services
            .free_pages(
                self.physical_address,
                self.pages,
            )
            .expect("Can't free pages!");
    }
}

