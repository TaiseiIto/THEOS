extern crate alloc;

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell,
        fmt,
        mem,
        ptr,
    },
    crate::{
        serial_print,
        serial_println,
    },
    super::{
        memory::physical_page,
        uefi::services::boot::memory_allocation,
    },
};

pub struct Allocated<'a> {
    slice: &'a mut [u8],
    layout: Layout,
}

impl<'a> Allocated<'a> {
    pub fn new(size: usize, align: usize) -> Self {
        let layout = Layout::from_size_align(size, Self::align(align)).expect("Can't allocate memory!");
        let slice: *mut [u8] = ptr::slice_from_raw_parts_mut(unsafe {
            ALLOCATOR.alloc(layout)
        }, size);
        let slice: &'a mut [u8] = unsafe {
            &mut *slice
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
        let mut fixed_align: usize = 1;
        while fixed_align < align {
            fixed_align *= 2;
        }
        fixed_align
    }
}

impl Drop for Allocated<'_> {
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

impl fmt::Debug for Allocated<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let address: *const u8 = self.slice.as_ptr();
        let address: usize = address as usize;
        formatter
            .debug_struct("Allocated")
            .field("address", &address)
            .field("size", &self.slice.len())
            .field("layout", &self.layout)
            .finish()
    }
}

#[global_allocator]
static mut ALLOCATOR: Allocator<'static> = Allocator {
    chunk_list: cell::UnsafeCell::new(None),
};

struct Allocator<'a> {
    chunk_list: cell::UnsafeCell<Option<&'a mut ChunkList<'a>>>,
}

unsafe impl GlobalAlloc for Allocator<'_> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match &mut *self.chunk_list.get() {
            Some(chunk_list) => chunk_list.alloc(layout),
            chunk_list @ None => {
                *chunk_list = Some(ChunkList::new());
                self.alloc(layout)
            }
        }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, _: Layout) {
        (&mut *self.chunk_list.get())
            .as_mut()
            .expect("Can't dealloc memory!")
            .dealloc(pointer as usize);
        if let Some(chunk_list) = &mut *self.chunk_list.get() {
            chunk_list.delete_unnecessary_chunk_lists();
        }
    }
}

const CHUNK_LIST_CAPACITY: usize = (memory_allocation::PAGE_SIZE - mem::size_of::<physical_page::Chunk>() - mem::size_of::<Option<&mut ChunkList>>()) / mem::size_of::<Option<Chunk>>();

struct ChunkList<'a> {
    page: physical_page::Chunk,
    chunks: [Option<Chunk>; CHUNK_LIST_CAPACITY],
    next: Option<&'a mut Self>,
}

impl<'a> ChunkList<'a> {
    fn new() -> &'a mut Self {
        let page_size: usize = 1;
        let page_align: usize = 1;
        let page: physical_page::Chunk = physical_page::Request::new(page_size, page_align).into();
        let chunk_list: &mut [u8] = page.get_mut();
        let chunk_list: &mut Self = unsafe {
            mem::transmute(&mut chunk_list[0])
        };
        chunk_list.page = page;
        chunk_list.chunks
            .iter_mut()
            .for_each(|chunk| {
                *chunk = None;
            });
        chunk_list.next = None;
        chunk_list
    }

    fn alloc(&'a mut self, layout: Layout) -> *mut u8 {
        let (available_chunk, previous_free_chunk, next_free_chunk): (&mut Chunk, &mut Option<Chunk>, &mut Option<Chunk>) = self.get_available_chunk(&layout);
        let available_chunk_address: usize = available_chunk.address;
        let available_chunk_size: usize = available_chunk.size;
        let requested_size: usize = layout.size();
        let requested_align: usize = layout.align();
        let allocated_chunk_address: usize = ((available_chunk_address + requested_align - 1) / requested_align) * requested_align;
        let allocated_chunk_size: usize = requested_size;
        let previous_free_chunk_address: usize = available_chunk_address;
        let previous_free_chunk_size: usize = allocated_chunk_address - previous_free_chunk_address;
        let next_free_chunk_address: usize = allocated_chunk_address + allocated_chunk_size;
        let next_free_chunk_size: usize = available_chunk_size - previous_free_chunk_size - allocated_chunk_size;
        available_chunk.address = allocated_chunk_address;
        available_chunk.size = allocated_chunk_size;
        available_chunk.allocated = true;
        serial_println!("allocated chunk = {:#x?}", available_chunk);
        *previous_free_chunk = if previous_free_chunk_size == 0 {
            None
        } else {
            Some(Chunk {
                pages: None,
                address: previous_free_chunk_address,
                size: previous_free_chunk_size,
                allocated: false,
            })
        };
        *next_free_chunk = if next_free_chunk_size == 0 {
            None
        } else {
            Some(Chunk {
                pages: None,
                address: next_free_chunk_address,
                size: next_free_chunk_size,
                allocated: false,
            })
        };
        allocated_chunk_address as *mut u8
    }

    fn get_available_chunk(&'a mut self, layout: &Layout) -> (&mut Chunk, &mut Option<Chunk>, &mut Option<Chunk>) {
        let (available_chunk, previous_free_chunk, next_free_chunk): (Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) = self.scan_available_chunk(layout, None, None, None);
        let available_chunk: &mut Chunk = available_chunk
            .expect("Can't find an available chunk!")
            .as_mut()
            .expect("Can't find an available chunk!");
        let previous_free_chunk: &mut Option<Chunk> = previous_free_chunk.expect("Can't find a previous free chunk!");
        let next_free_chunk: &mut Option<Chunk> = next_free_chunk.expect("Can't find a next free chunk!");
        (available_chunk, previous_free_chunk, next_free_chunk)
    }

    fn scan_available_chunk(&'a mut self, layout: &Layout, available_chunk: Option<&'a mut Option<Chunk>>, previous_free_chunk: Option<&'a mut Option<Chunk>>, next_free_chunk: Option<&'a mut Option<Chunk>>) -> (Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) {
        let (available_chunk, previous_free_chunk, next_free_chunk): (Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) = self.chunks
            .iter_mut()
            .fold((available_chunk, previous_free_chunk, next_free_chunk), |(available_chunk, previous_free_chunk, next_free_chunk), chunk| match (available_chunk, previous_free_chunk, next_free_chunk, chunk) {
                (None, previous_free_chunk, next_free_chunk, chunk @ None) => (Some(chunk), previous_free_chunk, next_free_chunk),
                (available_chunk @ None, previous_free_chunk, next_free_chunk, chunk @ Some(_)) |
                (available_chunk @ Some(None), previous_free_chunk, next_free_chunk, chunk @ Some(_)) => if match chunk {
                    Some(chunk) => chunk.available_for(layout),
                    None => false,
                } {
                    (Some(chunk), previous_free_chunk, next_free_chunk)
                } else {
                    (available_chunk, previous_free_chunk, next_free_chunk)
                },
                (available_chunk, None, next_free_chunk, chunk @ None) => (available_chunk, Some(chunk), next_free_chunk),
                (available_chunk, previous_free_chunk, None, chunk @ None) => (available_chunk, previous_free_chunk, Some(chunk)),
                (available_chunk, previous_free_chunk, next_free_chunk, _) => (available_chunk, previous_free_chunk, next_free_chunk),
            });
        match &mut self.next {
            Some(next) => next.scan_available_chunk(layout, available_chunk, previous_free_chunk, next_free_chunk),
            next @ None => match (available_chunk, previous_free_chunk, next_free_chunk) {
                (available_chunk @ None, previous_chunk, next_chunk) |
                (available_chunk , previous_chunk @ None, next_chunk) |
                (available_chunk , previous_chunk, next_chunk @ None) => {
                    *next = Some(ChunkList::new());
                    next.as_mut()
                        .expect("Can't scan available chunk!")
                        .scan_available_chunk(layout, available_chunk, previous_chunk, next_chunk)
                },
                (Some(available_chunk @ None), Some(previous_free_chunk), Some(next_free_chunk)) => {
                    *available_chunk = Some(Chunk::new(layout));
                    (Some(available_chunk), Some(previous_free_chunk), Some(next_free_chunk))
                },
                chunks @ (Some(Some(_)), Some(_), Some(_)) => chunks,
            },
        }
    }

    fn dealloc(&'a mut self, address: usize) {
        let (deallocated_chunk, mut previous_chunk, mut next_chunk): (&mut Option<Chunk>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) = self.get_deallocated_chunk(address);
        let mut drop_deallocated_chunk: bool = false;
        match deallocated_chunk {
            Some(deallocated_chunk) => {
                serial_println!("deallocated chunk = {:#x?}", deallocated_chunk);
                deallocated_chunk.allocated = false;
                if let Some(previous_chunk) = &mut previous_chunk {
                    let mut merge_previous_chunk: bool = false;
                    if let Some(previous_chunk) = previous_chunk {
                        if !previous_chunk.allocated {
                            merge_previous_chunk = true;
                            deallocated_chunk.address = previous_chunk.address;
                            deallocated_chunk.size += previous_chunk.size;
                        }
                    }
                    if merge_previous_chunk {
                        serial_println!("Delete previous chunk {:#x?}", previous_chunk);
                        **previous_chunk = None;
                    }
                }
                if let Some(next_chunk) = &mut next_chunk {
                    let mut merge_next_chunk: bool = false;
                    if let Some(next_chunk) = next_chunk {
                        if !next_chunk.allocated {
                            merge_next_chunk = true;
                            deallocated_chunk.size += next_chunk.size;
                            match (&mut deallocated_chunk.pages, &mut next_chunk.pages) {
                                (Some(deallocated_chunk_pages), Some(next_chunk_pages)) => {
                                    deallocated_chunk_pages.merge(next_chunk_pages.copy());
                                },
                                (Some(_), None) => {},
                                (deallocated_chunk_pages @ None, Some(next_chunk_pages)) => {
                                    *deallocated_chunk_pages = Some(next_chunk_pages.copy());
                                },
                                (None, None) => {},
                            }
                            next_chunk.pages = None;
                        }
                    }
                    if merge_next_chunk {
                        serial_println!("Delete next chunk {:#x?}", next_chunk);
                        **next_chunk = None;
                    }
                }
                if deallocated_chunk.can_deallocate_pages() {
                    drop_deallocated_chunk = true;
                }
            },
            None => panic!("Can't deallocate memory!"),
        }
        if drop_deallocated_chunk {
            serial_println!("Delete deallocated chunk {:#x?}", deallocated_chunk);
            *deallocated_chunk = None;
        }
    }

    fn get_deallocated_chunk(&'a mut self, address: usize) -> (&mut Option<Chunk>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) {
        let (deallocated_chunk, previous_chunk, next_chunk): (Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) = self.scan_deallocated_chunk(address, None, None, None);
        let deallocated_chunk: &mut Option<Chunk> = deallocated_chunk.expect("Can't find the deallocated chunk!");
        (deallocated_chunk, previous_chunk, next_chunk)
    }

    fn scan_deallocated_chunk(&'a mut self, address: usize, deallocated_chunk: Option<&'a mut Option<Chunk>>, previous_chunk: Option<&'a mut Option<Chunk>>, next_chunk: Option<&'a mut Option<Chunk>>) -> (Option<&'a mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) {
        let (deallocated_chunk, previous_chunk, next_chunk): (Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) = self.chunks
            .iter_mut()
            .fold((deallocated_chunk, previous_chunk, next_chunk), |(deallocated_chunk, previous_chunk, next_chunk), chunk| match chunk {
                Some(ref inner_chunk) => if inner_chunk.address == address {
                        (Some(chunk), previous_chunk, next_chunk)
                    } else if inner_chunk.address + inner_chunk.size == address {
                        (deallocated_chunk, Some(chunk), next_chunk)
                    } else {
                        match deallocated_chunk {
                            Some(deallocated_chunk) => match deallocated_chunk {
                                Some(ref inner_deallocated_chunk) => if inner_deallocated_chunk.address + inner_deallocated_chunk.size == inner_chunk.address {
                                        (Some(deallocated_chunk), previous_chunk, Some(chunk))
                                    } else {
                                        (Some(deallocated_chunk), previous_chunk, next_chunk)
                                    },
                                None => (Some(deallocated_chunk), previous_chunk, next_chunk),
                            },
                            None => (deallocated_chunk, previous_chunk, next_chunk),
                        }
                    },
                None => (deallocated_chunk, previous_chunk, next_chunk),
            });
        match &mut self.next {
            Some(next) => next.scan_deallocated_chunk(address, deallocated_chunk, previous_chunk, next_chunk),
            None => (deallocated_chunk, previous_chunk, next_chunk),
        }
    }

    fn delete_unnecessary_chunk_lists(&'a mut self) {
        if let Some(ref mut next) = self.next {
            *next = next.delete_unnecessary_chunk_list();
            next.delete_unnecessary_chunk_lists();
        }
    }

    fn delete_unnecessary_chunk_list(&'a mut self) -> &'a mut Self {
        if self.chunks.iter().all(|chunk| chunk.is_none()) {
            match self.next {
                Some(ref mut next) => {
                    self.page = physical_page::Chunk::null();
                    *next
                },
                None => self,
            }
        } else {
            self
        }
    }
}

pub struct Chunk {
    pages: Option<physical_page::Chunk>,
    address: usize,
    size: usize,
    allocated: bool,
}

impl Chunk {
    fn new(layout: &Layout) -> Self {
        let size = layout.size();
        let align = layout.align();
        let num_pages = (size + memory_allocation::PAGE_SIZE - 1) / memory_allocation::PAGE_SIZE;
        let page_align = (align + memory_allocation::PAGE_SIZE - 1) / memory_allocation::PAGE_SIZE;
        let pages: physical_page::Chunk = physical_page::Request::new(num_pages, page_align).into();
        let address: usize = pages.address();
        let size: usize = pages.size();
        let pages: Option<physical_page::Chunk> = Some(pages);
        let allocated: bool = false;
        Self {
            pages,
            address,
            size,
            allocated,
        }
    }

    fn available_for(&self, layout: &Layout) -> bool {
        let my_begin: usize = self.address;
        let my_end: usize = my_begin + self.size;
        let requested_size: usize = layout.size();
        let requested_align: usize = layout.align();
        let requested_begin: usize = ((my_begin + requested_align - 1) / requested_align) * requested_align;
        let requested_end: usize = requested_begin + requested_size;
        !self.allocated && requested_end <= my_end
    }

    fn can_deallocate_pages(&self) -> bool {
        match &self.pages {
            Some(pages) => {
                let my_address: usize = self.address;
                let my_size: usize = self.size;
                let pages_address: usize = pages.address();
                let pages_size: usize = pages.size();
                my_address == pages_address && my_size == pages_size
            },
            None => false,
        }
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Chunk")
            .field("pages", &self.pages)
            .field("address", &self.address)
            .field("size", &self.size)
            .field("allocated", &self.allocated)
            .finish()
    }
}

