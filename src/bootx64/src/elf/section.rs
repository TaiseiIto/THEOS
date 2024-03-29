// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod sh_flags;
pub mod sh_type;

use {
    alloc::{
        string::String,
        vec::Vec,
    },
    core::mem,
    super::header,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Section {
    name: String,
    header: Header,
    bytes: Vec<u8>,
}

impl Section {
    pub fn new(header: Header, elf: &[u8], shstrtab: &[u8]) -> Self {
        let begin: usize = header.sh_offset;
        let size: usize = header.sh_size;
        let end: usize = begin + size;
        let bytes: Vec<u8> = elf[begin..end].to_vec();
        let name_begin: usize = header.sh_name as usize;
        let name: Vec<u8> = shstrtab[name_begin..]
            .split(|byte| *byte == 0x00)
            .next()
            .expect("Can't read an ELF!")
            .to_vec();
        let name = String::from_utf8(name).expect("Can't read an ELF!");
        Self {
            name,
            header,
            bytes,
        }
    }

    pub fn read(header: &header::Header, elf: &[u8]) -> Vec<Self> {
        let headers: Vec<Header> = Header::read(header, elf);
        let shstrndx: usize = header.e_shstrndx();
        let shstrtab: &Header = &headers[shstrndx];
        let shstrtab_begin: usize = shstrtab.sh_offset;
        let shstrtab_size: usize = shstrtab.sh_size;
        let shstrtab_end: usize = shstrtab_begin + shstrtab_size;
        let shstrtab: &[u8] = &elf[shstrtab_begin..shstrtab_end];
        headers
            .into_iter()
            .map(|header| Self::new(header, elf, shstrtab))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    sh_name: u32,
    sh_type: sh_type::Type,
    sh_flags: sh_flags::Flags,
    sh_addr: usize,
    sh_offset: usize,
    sh_size: usize,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: usize,
    sh_entsize: usize,
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
const SH_LINK_BEGIN: usize = SH_SIZE_END;
const SH_LINK_LENGTH: usize = mem::size_of::<u32>();
const SH_LINK_END: usize = SH_LINK_BEGIN + SH_LINK_LENGTH;
const SH_INFO_BEGIN: usize = SH_LINK_END;
const SH_INFO_LENGTH: usize = mem::size_of::<u32>();
const SH_INFO_END: usize = SH_INFO_BEGIN + SH_INFO_LENGTH;
const SH_ADDRALIGN_BEGIN: usize = SH_INFO_END;
const SH_ADDRALIGN_LENGTH: usize = mem::size_of::<usize>();
const SH_ADDRALIGN_END: usize = SH_ADDRALIGN_BEGIN + SH_ADDRALIGN_LENGTH;
const SH_ENTSIZE_BEGIN: usize = SH_ADDRALIGN_END;
const SH_ENTSIZE_LENGTH: usize = mem::size_of::<usize>();
const SH_ENTSIZE_END: usize = SH_ENTSIZE_BEGIN + SH_ENTSIZE_LENGTH;

impl Header {
    pub fn read(header: &header::Header, elf: &[u8]) -> Vec<Self> {
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
        let sh_link: [u8; SH_LINK_LENGTH] = header[SH_LINK_BEGIN..SH_LINK_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_link = u32::from_le_bytes(sh_link);
        let sh_info: [u8; SH_INFO_LENGTH] = header[SH_INFO_BEGIN..SH_INFO_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_info = u32::from_le_bytes(sh_info);
        let sh_addralign: [u8; SH_ADDRALIGN_LENGTH] = header[SH_ADDRALIGN_BEGIN..SH_ADDRALIGN_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_addralign = usize::from_le_bytes(sh_addralign);
        let sh_entsize: [u8; SH_ENTSIZE_LENGTH] = header[SH_ENTSIZE_BEGIN..SH_ENTSIZE_END]
            .try_into()
            .expect("Can't read an ELF!");
        let sh_entsize = usize::from_le_bytes(sh_entsize);
        Self {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        }
    }
}

