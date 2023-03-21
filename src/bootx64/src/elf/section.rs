// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod sh_flags;
pub mod sh_type;

use {
    alloc::vec::Vec,
    core::mem,
    super::header,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    sh_name: u32,
    sh_type: sh_type::Type,
    sh_flags: sh_flags::Flags,
    sh_addr: usize,
    sh_offset: usize,
    sh_size: usize,
}

const SH_NAME_BEGIN: usize = 0;
const SH_NAME_LENGTH: usize = mem::size_of::<u32>();
const SH_NAME_END: usize = SH_NAME_BEGIN + SH_NAME_LENGTH;
const SH_TYPE_BEGIN: usize = SH_NAME_END;
const SH_TYPE_LENGTH: usize = mem::size_of::<u32>();
const SH_TYPE_END: usize = SH_TYPE_BEGIN + SH_TYPE_LENGTH;
const SH_FLAGS_BEGIN: usize = SH_TYPE_END;
const SH_FLAGS_LENGTH: usize = mem::size_of::<usize>();
const SH_FLAGS_END: usize = SH_FLAGS_BEGIN + SH_FLAGS_LENGTH;
const SH_ADDR_BEGIN: usize = SH_FLAGS_END;
const SH_ADDR_LENGTH: usize = mem::size_of::<usize>();
const SH_ADDR_END: usize = SH_ADDR_BEGIN + SH_ADDR_LENGTH;
const SH_OFFSET_BEGIN: usize = SH_ADDR_END;
const SH_OFFSET_LENGTH: usize = mem::size_of::<usize>();
const SH_OFFSET_END: usize = SH_OFFSET_BEGIN + SH_OFFSET_LENGTH;
const SH_SIZE_BEGIN: usize = SH_OFFSET_END;
const SH_SIZE_LENGTH: usize = mem::size_of::<usize>();
const SH_SIZE_END: usize = SH_SIZE_BEGIN + SH_SIZE_LENGTH;

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
        let sh_type: [u8; SH_TYPE_LENGTH] = header[SH_TYPE_BEGIN..SH_TYPE_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_type = u32::from_le_bytes(sh_type);
        let sh_type: sh_type::Type = sh_type.into();
        let sh_flags: [u8; SH_FLAGS_LENGTH] = header[SH_FLAGS_BEGIN..SH_FLAGS_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_flags = usize::from_le_bytes(sh_flags);
        let sh_flags: sh_flags::Flags = sh_flags.into();
        let sh_addr: [u8; SH_ADDR_LENGTH] = header[SH_ADDR_BEGIN..SH_ADDR_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_addr = usize::from_le_bytes(sh_addr);
        let sh_offset: [u8; SH_OFFSET_LENGTH] = header[SH_OFFSET_BEGIN..SH_OFFSET_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_offset = usize::from_le_bytes(sh_offset);
        let sh_size: [u8; SH_SIZE_LENGTH] = header[SH_SIZE_BEGIN..SH_SIZE_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_size = usize::from_le_bytes(sh_size);
        Self {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
        }
    }
}

