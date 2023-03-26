// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod header;
pub mod program;
pub mod section;

use {
    alloc::{
        collections::btree_set::BTreeSet,
        vec::Vec,
    },
    super::memory,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Elf {
    header: header::Header,
    programs: Vec<program::Program>,
    sections: Vec<section::Section>,
}

impl Elf {
    pub fn new(elf: &[u8]) -> Self {
        let header: header::Header = elf.into();
        let programs = program::Program::read(&header, elf);
        let sections = section::Section::read(&header, elf);
        Self {
            header,
            programs,
            sections,
        }
    }

    pub fn necessary_page_numbers(&self) -> BTreeSet<memory::PageRange> {
        self.programs
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
                    page_ranges.insert(memory::PageRange::new(page_number..page_number + 1));
                    page_ranges
                }
            )
    }
}

