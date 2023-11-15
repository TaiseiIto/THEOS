extern crate alloc;

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell,
        mem,
        slice,
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
        let mut fixed_align: usize = 1;
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
static mut ALLOCATOR: Allocator<'static> = Allocator {
    chunk_list: cell::UnsafeCell::new(None),
};

pub struct Allocator<'a> {
    chunk_list: cell::UnsafeCell<Option<&'a mut ChunkList<'a>>>,
}

unsafe impl GlobalAlloc for Allocator<'_> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let chunk_list: &mut Option<&mut ChunkList> = &mut *self.chunk_list.get();
        match chunk_list {
            Some(chunk_list) => {
            },
            None => {
                *chunk_list = Some(ChunkList::new());
            },
        }
        panic!("The global allocator is unimplemented!");
    }

    unsafe fn dealloc(&self, _pointer: *mut u8, _: Layout) {
        panic!("The global allocator is unimplemented!");
    }
}

const CHUNK_LIST_CAPACITY: usize = (memory_allocation::PAGE_SIZE - mem::size_of::<physical_page::Chunk>() - 2 * mem::size_of::<Option<usize>>()) / mem::size_of::<Option<Chunk>>();

pub struct ChunkList<'a> {
    page: physical_page::Chunk,
    chunks: [Option<Chunk<'a>>; CHUNK_LIST_CAPACITY],
    previous: Option<&'a mut Self>,
    next: Option<&'a mut Self>,
}

impl<'a> ChunkList<'a> {
    pub fn new() -> &'a mut Self {
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
        chunk_list.previous = None;
        chunk_list.next = None;
        panic!("Unimplemented!");
    }
}

pub struct Chunk<'a> {
    slice: &'a mut [u8],
    allocated: bool,
    previous: Option<&'a mut Self>,
    next: Option<&'a mut Self>,
}

