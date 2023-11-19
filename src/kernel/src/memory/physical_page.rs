use {
    core::{
        cmp,
        iter::{
            Chain,
            StepBy,
        },
        ops::RangeInclusive,
        ptr,
    },
    crate::{
        serial_print,
        serial_println,
    },
    super::super::uefi::services::boot::memory_allocation,
};

static mut MANAGER: Manager<'static> = Manager::<'static> {
    pages: 0,
    present_bit_map: &mut [],
    search_point: 0,
};

pub struct Manager<'a> {
    pages: usize,
    present_bit_map: &'a mut [u8],
    search_point: usize,
}

impl Manager<'static> {
    pub fn init(
        present_bit_map: &'static mut [u8],
        map: &memory_allocation::MemoryDescriptors,
    ) {
        unsafe {
            MANAGER = Self::new(present_bit_map, map);
            serial_println!("Number of used pages = {:#x}", MANAGER.used_pages());
            serial_println!("Number of unused pages = {:#x}", MANAGER.unused_pages());
        }
    }
}

impl<'a> Manager<'a> {
    pub fn alloc(
        &mut self,
        request: Request,
    ) -> Chunk {
        let pages: usize = request.size;
        let align: usize = request.align;
        let start_page_candidate_first_range_start: usize = ((self.search_point + align - 1) / align) * align;
        let start_page_candidate_first_range_end: usize = ((self.pages - request.size) / align) * align;
        let start_page_candidate_first_range: StepBy<RangeInclusive<usize>> = (start_page_candidate_first_range_start..=start_page_candidate_first_range_end).step_by(align);
        let start_page_candidate_second_range_start: usize = 0;
        let start_page_candidate_second_range_end: usize = (self.search_point / align) * align;
        let start_page_candidate_second_range: StepBy<RangeInclusive<usize>> = (start_page_candidate_second_range_start..=start_page_candidate_second_range_end).step_by(align);
        let mut start_page_candidates: Chain<StepBy<RangeInclusive<usize>>, StepBy<RangeInclusive<usize>>> = start_page_candidate_first_range.chain(start_page_candidate_second_range);
        let start_page: usize = start_page_candidates
            .find(|start_page_candidate| self.pages_are_available(*start_page_candidate, pages))
            .expect("Can't allocate physical pages!");
        self.alloc_pages(start_page, pages);
        self.search_point = start_page + pages;
        Chunk {
            start_page,
            pages,
        }
    }

    pub fn dealloc(
        &mut self,
        chunk: &mut Chunk,
    ) {
        let start_pages: usize = chunk.start_page;
        let pages: usize = chunk.pages;
        self.dealloc_pages(start_pages, pages);
        self.search_point = (0..=self.search_point)
            .rev()
            .chain((self.search_point + 1..self.pages).rev())
            .find(|search_point| *search_point == 0 || (0 < *search_point && self.page_is_available(*search_point) && !self.page_is_available(*search_point - 1)))
            .expect("Can't deallocate physical pages!");
    }

    fn new(
        present_bit_map: &'a mut [u8],
        map: &memory_allocation::MemoryDescriptors,
    ) -> Self {
        let mut pages: usize = 0;
        map
            .clone()
            .for_each(|descriptor| {
                serial_println!("memory descriptor = {:#x?}", descriptor);
                let physical_start: usize = descriptor.physical_start() as usize;
                serial_println!("physical_start = {:#x?}", physical_start);
                let physical_page_start: usize = physical_start / memory_allocation::PAGE_SIZE;
                serial_println!("physical_page_start = {:#x?}", physical_page_start);
                let physical_end: usize = descriptor.physical_end() as usize;
                serial_println!("physical_end = {:#x?}", physical_end);
                let physical_page_end: usize = physical_end / memory_allocation::PAGE_SIZE;
                serial_println!("physical_page_end = {:#x?}", physical_page_end);
                pages = cmp::max(pages, physical_page_end);
                let memory_type: memory_allocation::MemoryType = descriptor.memory_type();
                (physical_page_start..physical_page_end)
                    .for_each(|page| {
                        let bit_map_index: usize = page / 8;
                        let bit_map_offset: usize = page % 8;
                        let byte: &mut u8 = &mut present_bit_map[bit_map_index];
                        let mask: u8 = 0x01u8 << bit_map_offset;
                        match memory_type {
                            memory_allocation::MemoryType::BootServicesCode
                            | memory_allocation::MemoryType::BootServicesData
                            | memory_allocation::MemoryType::ConventionalMemory => *byte &= !mask,
                            _ => *byte |= mask,
                        }
                    });
            });
        let search_point: usize = 0;
        Self{
            pages,
            present_bit_map,
            search_point,
        }
    }

    fn page_is_available(&self, page: usize) -> bool {
        let index: usize = page / 8;
        let offset: usize = page % 8;
        let mask: u8 = 0x01u8 << offset;
        self.present_bit_map[index] & mask == 0
    }

    fn pages_are_available(&self, start_page: usize, pages: usize) -> bool {
        (start_page..start_page + pages)
            .all(|page| self.page_is_available(page))
    }

    fn alloc_page(&mut self, page: usize) {
        let index: usize = page / 8;
        let offset: usize = page % 8;
        let mask: u8 = 0x01u8 << offset;
        self.present_bit_map[index] |= mask;
    }

    fn alloc_pages(&mut self, start_page: usize, pages: usize) {
        (start_page..start_page + pages)
            .for_each(|page| {
                self.alloc_page(page);
            });
    }

    fn dealloc_page(&mut self, page: usize) {
        let index: usize = page / 8;
        let offset: usize = page % 8;
        let mask: u8 = 0x01u8 << offset;
        self.present_bit_map[index] &= !mask;
    }

    fn dealloc_pages(&mut self, start_page: usize, pages: usize) {
        (start_page..start_page + pages)
            .for_each(|page| {
                self.dealloc_page(page);
            });
    }

    fn used_pages(&self) -> usize {
        self.present_bit_map
            .iter()
            .map(|byte| byte.count_ones() as usize)
            .sum()
    }

    fn unused_pages(&self) -> usize {
        self.present_bit_map
            .iter()
            .map(|byte| byte.count_zeros() as usize)
            .sum()
    }
}

#[derive(Debug)]
pub struct Chunk {
    start_page: usize,
    pages: usize,
}

impl Chunk {
    pub fn address(&self) -> usize {
        self.start_page * memory_allocation::PAGE_SIZE
    }

    pub fn size(&self) -> usize {
        self.pages * memory_allocation::PAGE_SIZE
    }

    pub fn get_mut(&self) -> &mut [u8] {
        let slice: *mut [u8] = ptr::slice_from_raw_parts_mut(self.address() as *mut u8, self.size());
        unsafe {
            &mut *slice
        }
    }
}

impl From<Request> for Chunk {
    fn from(request: Request) -> Self {
        unsafe {
            MANAGER.alloc(request)
        }
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            MANAGER.dealloc(self)
        }
    }
}

pub struct Request {
    size: usize,
    align: usize,
}

impl Request {
    pub fn new(size: usize, align: usize) -> Self {
        match align.count_ones() {
            1 => Self {
                size,
                align,
            },
            _ => panic!("Can't create an allocate request!"),
        }
    }
}

