// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

use {
    alloc::vec::Vec,
    super::header,
};

#[derive(Debug)]
pub struct Header {
}

impl Header {
    pub fn read(elf: &[u8], header: &header::Header) -> Vec<Self> {
        Vec::new()
    }
}

