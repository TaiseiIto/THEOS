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
    program_headers: Vec<program::Header>,
    section_headers: Vec<section::Header>,
}

impl Elf {
    pub fn new(elf: &[u8]) -> Self {
        let header: header::Header = elf.into();
        let program_headers = program::Header::read(elf, &header);
        let section_headers = section::Header::read(elf, &header);
        Self {
            header,
            program_headers,
            section_headers,
        }
    }
}

