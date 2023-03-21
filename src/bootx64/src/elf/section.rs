// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

use {
    alloc::vec::Vec,
    super::header,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
}

impl Header {
    pub fn read(elf: &[u8], header: &header::Header) -> Vec<Self> {
        let header_size: usize = header.e_shentsize();
        let header_number: usize = header.e_shnum();
        let headers_begin: usize = header.e_shoff();
        let headers_size: usize = header_number * header_size;
        let headers_end: usize = headers_begin + headers_size;
        elf[headers_begin..headers_end]
            .chunks(header_size)
            .map(|header| header.into())
            .collect()
    }
}

impl From<&[u8]> for Header {
    fn from(header: &[u8]) -> Self {
        Self {
        }
    }
}

