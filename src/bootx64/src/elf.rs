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

    pub fn deploy(&self) -> BTreeMap<usize, memory::Pages> {
        self
            .necessary_page_numbers()
            .into_iter()
            .map(|page_number| (page_number, memory::Pages::new(1)))
            .collect()
    }

    fn necessary_page_numbers(&self) -> BTreeSet<usize> {
        self.programs
            .iter()
            .map(|program| program.necessary_page_numbers())
            .fold(BTreeSet::<usize>::new(), |necessary_page_numbers, next_necessary_page_numbers| necessary_page_numbers
                .union(&next_necessary_page_numbers)
                .cloned()
                .collect()
            )
    }
}

