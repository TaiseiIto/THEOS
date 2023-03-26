pub mod paging;

use {
    alloc::collections::btree_set::BTreeSet,
    core::{
        cmp::Ordering,
        ops::Range,
        slice,
    },
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

#[derive(Debug, Eq, PartialEq)]
pub struct PageRange(Range<usize>);

impl PageRange {
    pub fn new(range: Range<usize>) -> Self {
        Self(range)
    }

    pub fn start(&self) -> usize {
        self.0.start
    }

    pub fn end(&self) -> usize {
        self.0.end
    }
}

impl PartialOrd for PageRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PageRange {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.start.cmp(&other.0.start) {
            Ordering::Equal => self.0.end.cmp(&other.0.end),
            ordering => ordering,
        }
    }
}

