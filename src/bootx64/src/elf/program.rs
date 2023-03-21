// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod p_flags;
pub mod p_type;

use {
    alloc::vec::Vec,
    core::mem,
    super::header,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Program {
    header: Header,
    bytes: Vec<u8>
}

impl Program {
    pub fn new(header: Header, elf: &[u8]) -> Self {
        let begin: usize = header.p_offset;
        let size: usize = header.p_filesz;
        let end: usize = begin + size;
        let bytes: Vec<u8> = elf[begin..end].to_vec();
        Self {
            header,
            bytes,
        }
    }

    pub fn read(header: &header::Header, elf: &[u8]) -> Vec<Self> {
        let headers: Vec<Header> = Header::read(header, elf);
        headers
            .into_iter()
            .map(|header| Self::new(header, elf))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    p_type: p_type::Type,
    p_flags: p_flags::Flags,
    p_offset: usize,
    p_vaddr: usize,
    p_paddr: usize,
    p_filesz: usize,
    p_memsz: usize,
    p_align: usize,
}

const P_TYPE_BEGIN: usize = 0;
const P_TYPE_LENGTH: usize = mem::size_of::<u32>();
const P_TYPE_END: usize = P_TYPE_BEGIN + P_TYPE_LENGTH;
const P_FLAGS_BEGIN: usize = P_TYPE_END;
const P_FLAGS_LENGTH: usize = mem::size_of::<u32>();
const P_FLAGS_END: usize = P_FLAGS_BEGIN + P_FLAGS_LENGTH;
const P_OFFSET_BEGIN: usize = P_FLAGS_END;
const P_OFFSET_LENGTH: usize = mem::size_of::<usize>();
const P_OFFSET_END: usize = P_OFFSET_BEGIN + P_OFFSET_LENGTH;
const P_VADDR_BEGIN: usize = P_OFFSET_END;
const P_VADDR_LENGTH: usize = mem::size_of::<usize>();
const P_VADDR_END: usize = P_VADDR_BEGIN + P_VADDR_LENGTH;
const P_PADDR_BEGIN: usize = P_VADDR_END;
const P_PADDR_LENGTH: usize = mem::size_of::<usize>();
const P_PADDR_END: usize = P_PADDR_BEGIN + P_PADDR_LENGTH;
const P_FILESZ_BEGIN: usize = P_PADDR_END;
const P_FILESZ_LENGTH: usize = mem::size_of::<usize>();
const P_FILESZ_END: usize = P_FILESZ_BEGIN + P_FILESZ_LENGTH;
const P_MEMSZ_BEGIN: usize = P_FILESZ_END;
const P_MEMSZ_LENGTH: usize = mem::size_of::<usize>();
const P_MEMSZ_END: usize = P_MEMSZ_BEGIN + P_MEMSZ_LENGTH;
const P_ALIGN_BEGIN: usize = P_MEMSZ_END;
const P_ALIGN_LENGTH: usize = mem::size_of::<usize>();
const P_ALIGN_END: usize = P_ALIGN_BEGIN + P_ALIGN_LENGTH;

impl Header {
    pub fn read(header: &header::Header, elf: &[u8]) -> Vec<Self> {
        let header_size: usize = header.e_phentsize();
        let header_number: usize = header.e_phnum();
        let headers_begin: usize = header.e_phoff();
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
        let p_type: [u8; P_TYPE_LENGTH] = header[P_TYPE_BEGIN..P_TYPE_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_type = u32::from_le_bytes(p_type);
        let p_type: p_type::Type = p_type.into();
        let p_flags: [u8; P_FLAGS_LENGTH] = header[P_FLAGS_BEGIN..P_FLAGS_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_flags = u32::from_le_bytes(p_flags);
        let p_flags: p_flags::Flags = p_flags.into();
        let p_offset: [u8; P_OFFSET_LENGTH] = header[P_OFFSET_BEGIN..P_OFFSET_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_offset = usize::from_le_bytes(p_offset);
        let p_vaddr: [u8; P_VADDR_LENGTH] = header[P_VADDR_BEGIN..P_VADDR_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_vaddr = usize::from_le_bytes(p_vaddr);
        let p_paddr: [u8; P_PADDR_LENGTH] = header[P_PADDR_BEGIN..P_PADDR_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_paddr = usize::from_le_bytes(p_paddr);
        let p_filesz: [u8; P_FILESZ_LENGTH] = header[P_FILESZ_BEGIN..P_FILESZ_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_filesz = usize::from_le_bytes(p_filesz);
        let p_memsz: [u8; P_MEMSZ_LENGTH] = header[P_MEMSZ_BEGIN..P_MEMSZ_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_memsz = usize::from_le_bytes(p_memsz);
        let p_align: [u8; P_ALIGN_LENGTH] = header[P_ALIGN_BEGIN..P_ALIGN_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_align = usize::from_le_bytes(p_align);
        Self {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        }
    }
}

