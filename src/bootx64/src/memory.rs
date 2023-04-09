pub mod paging;

use {
    core::{
        cmp::Ordering,
        ops::Range,
        ptr,
        slice,
    },
    super::uefi::{
        services::boot::memory_allocation,
        tables::system,
    },
};

#[derive(Debug)]
pub struct Pages<'a> {
    bytes: &'a mut [u8],
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
        let virtual_address: *mut u8 = physical_address as *mut u8;
        let length: usize = pages * memory_allocation::PAGE_SIZE;
        unsafe {
            ptr::write_bytes(virtual_address, 0x00, length);
        }
        let bytes: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(virtual_address, length)
        };
        Self {
            bytes,
            pages,
            physical_address,
        }
    }

    pub fn bytes(&mut self) -> &mut [u8] {
        self.bytes
    }

    pub fn physical_address(&self) -> memory_allocation::PhysicalAddress {
        self.physical_address
    }

    pub fn write(&mut self, page: usize, offset: usize, bytes: &[u8]) {
        let start: usize = page * memory_allocation::PAGE_SIZE + offset;
        let end: usize = start + bytes.len();
        self.bytes[start..end].copy_from_slice(bytes);
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageRange(Range<usize>);

impl PageRange {
    pub fn new(range: Range<usize>) -> Self {
        Self(range)
    }

    pub fn contains(&self, page: usize) -> bool {
        self.0.contains(&page)
    }

    pub fn start(&self) -> usize {
        self.0.start
    }

    pub fn end(&self) -> usize {
        self.0.end
    }

    pub fn size(&self) -> usize {
        self.end() - self.start()
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

impl Iterator for PageRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

