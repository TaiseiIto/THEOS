// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

use {
    alloc::vec::Vec,
    core::mem,
    super::header,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    sh_name: u32,
}

const SH_NAME_BEGIN: usize = 0;
const SH_NAME_LENGTH: usize = mem::size_of::<u32>();
const SH_NAME_END: usize = SH_NAME_BEGIN + SH_NAME_LENGTH;

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
        let sh_name: [u8; SH_NAME_LENGTH] = header[SH_NAME_BEGIN..SH_NAME_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_name = u32::from_le_bytes(sh_name);
        Self {
            sh_name,
        }
    }
}

