// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod header;
pub mod program;
pub mod section;

use {
    alloc::{
        collections::{
            btree_map::BTreeMap,
            btree_set::BTreeSet,
        },
        vec::Vec,
    },
    crate::{
        uefi_print,
        uefi_println,
    },
    super::{
        memory,
        uefi::services::boot::memory_allocation,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Elf<'a> {
    header: header::Header,
    programs: Vec<program::Program>,
    sections: Vec<section::Section>,
    deployed: BTreeMap<memory::PageRange, memory::Pages<'a>>,
}

impl Elf<'_> {
    pub fn new(elf: &[u8]) -> Self {
        let header: header::Header = elf.into();
        let programs = program::Program::read(&header, elf);
        let sections = section::Section::read(&header, elf);
        let deployed: BTreeMap<memory::PageRange, memory::Pages> = programs
            .iter()
            .map(|program| program.necessary_page_numbers())
            .fold(
                BTreeSet::<usize>::new(),
                |page_numbers, next_page_numbers|
                    page_numbers
                        .union(&next_page_numbers)
                        .cloned()
                        .collect()
            )
            .into_iter()
            .fold(
                BTreeSet::<memory::PageRange>::new(),
                |mut page_ranges, page_number| {
                    let mut page_range_start: usize = page_number;
                    let mut page_range_end: usize = page_number + 1;
                    if let Some(page_range) = page_ranges
                        .clone()
                        .iter()
                        .find(|page_range| page_range_end == page_range.start()) {
                        page_range_end = page_range.end();
                        page_ranges.remove(page_range);
                    }
                    if let Some(page_range) = page_ranges
                        .clone()
                        .iter()
                        .find(|page_range| page_range.end() == page_range_start) {
                        page_range_start = page_range.start();
                        page_ranges.remove(page_range);
                    }
                    let page_range = memory::PageRange::new(page_range_start..page_range_end);
                    page_ranges.insert(page_range);
                    page_ranges
                }
            )
            .into_iter()
            .map(|page_range| {
                let mut pages = memory::Pages::new(page_range.size());
                let page_range: memory::PageRange = page_range.clone();
                programs
                    .iter()
                    .for_each(|program| {
                        let start_page: usize = program.start_page();
                        let start_offset: usize = program.start_offset();
                        if page_range.contains(start_page) {
                            uefi_println!("start_page = {:#x}", start_page);
                            uefi_println!("page_range.start() = {:#x}", page_range.start());
                            uefi_println!("start_offset = {:#x}", start_offset);
                            pages.write(start_page - page_range.start(), start_offset, program.bytes());
                        }
                    });
                (page_range, pages)
            })
            .collect();
        Self {
            header,
            programs,
            sections,
            deployed,
        }
    }

    pub fn page_map(&self) -> BTreeMap<usize, usize> {
        self.deployed
            .iter()
            .map(|(page_range, pages)| page_range
                .clone()
                .enumerate()
                .map(|(i, page)| (pages.physical_address() as usize + i * memory_allocation::PAGE_SIZE, page * memory_allocation::PAGE_SIZE))
            )
            .flatten()
            .collect()
    }

    pub fn run(&self) {
        self.header.run()
    }
}

