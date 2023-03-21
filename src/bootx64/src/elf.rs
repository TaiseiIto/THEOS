// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod header;
pub mod program;
pub mod section;

use alloc::vec::Vec;

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
}

