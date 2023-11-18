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
        serial_println!("Allocator.alloc");
        let chunk_list: &mut Option<&mut ChunkList> = &mut *self.chunk_list.get();
        serial_println!("Got a chunk list");
        match chunk_list {
            Some(chunk_list) => chunk_list.alloc(layout),
            None => {
                serial_println!("Create a new chunk list");
                *chunk_list = Some(ChunkList::new());
                serial_println!("Created a new chunk list");
                self.alloc(layout)
            }
        }
    }

    unsafe fn dealloc(&self, _pointer: *mut u8, _: Layout) {
        panic!("The global allocator is unimplemented!");
    }
}

const CHUNK_LIST_CAPACITY: usize = (memory_allocation::PAGE_SIZE - mem::size_of::<physical_page::Chunk>() - 2 * mem::size_of::<Option<usize>>()) / mem::size_of::<Option<Chunk>>();

struct ChunkList<'a> {
    page: physical_page::Chunk,
    chunks: [Option<Chunk<'a>>; CHUNK_LIST_CAPACITY],
    previous: Option<&'a mut Self>,
    next: Option<&'a mut Self>,
}

impl<'a> ChunkList<'a> {
    fn new() -> &'a mut Self {
        serial_println!("ChunkList.new");
        let page_size: usize = 1;
        serial_println!("page_size = {:#x?}", page_size);
        let page_align: usize = 1;
        serial_println!("page_align = {:#x?}", page_align);
        let page: physical_page::Chunk = physical_page::Request::new(page_size, page_align).into();
        serial_println!("page = {:#x?}", page);
        let chunk_list: &mut [u8] = page.get_mut();
        serial_println!("got chunk list slice");
        let chunk_list: &mut Self = unsafe {
            mem::transmute(&mut chunk_list[0])
        };
        chunk_list.page = page;
        chunk_list.chunks
            .iter_mut()
            .for_each(|chunk| {
                *chunk = None;
            });
        chunk_list.previous = None;
        chunk_list.next = None;
        panic!("Unimplemented!");
    }

    fn alloc(&'a mut self, layout: Layout) -> *mut u8 {
        serial_println!("ChunkList.alloc");
        let (available_chunk, previous_free_chunk, next_free_chunk): (Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) = self.get_available_chunk(&layout, None, None, None);
        serial_println!("available_chunk = {:#x?}", available_chunk);
        serial_println!("previous_free_chunk = {:#x?}", previous_free_chunk);
        serial_println!("next_free_chunk = {:#x?}", next_free_chunk);
        panic!("The global allocator is unimplemented!")
    }

    fn get_available_chunk(&'a mut self, layout: &Layout, available_chunk: Option<&'a mut Option<Chunk<'a>>>, previous_free_chunk: Option<&'a mut Option<Chunk<'a>>>, next_free_chunk: Option<&'a mut Option<Chunk<'a>>>) -> (Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>, Option<&mut Option<Chunk>>) {
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
        match self.next.as_mut() {
            Some(next) => next.get_available_chunk(layout, available_chunk, previous_free_chunk, next_free_chunk),
            None => (available_chunk, previous_free_chunk, next_free_chunk),
        }
    }
}

pub struct Chunk<'a> {
    slice: &'a mut [u8],
    allocated: bool,
    previous: Option<&'a mut Self>,
    next: Option<&'a mut Self>,
}

impl<'a> Chunk<'a> {
    fn address(&self) -> usize {
        self.slice.as_ptr() as usize
    }

    fn size(&self) -> usize {
        self.slice.len()
    }

    fn available_for(&self, layout: &Layout) -> bool {
        let my_begin: usize = self.address();
        let my_end: usize = my_begin + self.size();
        let requested_size: usize = layout.size();
        let requested_align: usize = layout.align();
        let requested_begin: usize = ((my_begin + requested_align - 1) / requested_align) * requested_align;
        let requested_end: usize = requested_begin + requested_size;
        !self.allocated && requested_end <= my_end
    }
}

impl fmt::Debug for Chunk<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let address: *const u8 = self.slice.as_ptr();
        let address: usize = address as usize;
        formatter
            .debug_struct("Chunk")
            .field("address", &address)
            .field("size", &self.slice.len())
            .field("allocated", &self.allocated)
            .field("next", &self.next)
            .finish()
    }
}

