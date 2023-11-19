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

    unsafe fn dealloc(&self, _pointer: *mut u8, _: Layout) {
        panic!("The global allocator is unimplemented!");
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
            mut next @ None => match (available_chunk, previous_free_chunk, next_free_chunk) {
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
}

impl fmt::Debug for Chunk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Chunk")
            .field("address", &self.address)
            .field("size", &self.size)
            .field("allocated", &self.allocated)
            .finish()
    }
}
