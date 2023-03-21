// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod header;
pub mod program;

use alloc::vec::Vec;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Elf {
    header: header::Header,
    program_headers: Vec<program::Header>,
}

impl Elf {
    pub fn new(elf: &[u8]) -> Self {
        let header: header::Header = elf.into();
        let program_headers = program::Header::read(elf, &header);
        Self {
            header,
            program_headers,
        }
    }
}

