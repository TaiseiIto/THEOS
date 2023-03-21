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
    p_type: Type,
    p_flags: u32,
    p_offset: usize,
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

impl Header {
    pub fn read(elf: &[u8], header: &header::Header) -> Vec<Self> {
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
        let p_type: Type = p_type.into();
        let p_flags: [u8; P_FLAGS_LENGTH] = header[P_FLAGS_BEGIN..P_FLAGS_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_flags = u32::from_le_bytes(p_flags);
        let p_offset: [u8; P_OFFSET_LENGTH] = header[P_OFFSET_BEGIN..P_OFFSET_END]
            .try_into()
            .expect("Can't read an ELF!");
        let p_offset = usize::from_le_bytes(p_offset);
        Self {
            p_type,
            p_flags,
            p_offset,
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
    OperatingSystemSpecific(u32),
    ProcessorSpecific(u32),
}

impl From<u32> for Type {
    fn from(p_type: u32) -> Self {
        match p_type {
            0x00000000 => Self::Null,
            0x00000001 => Self::Load,
            0x00000002 => Self::Dynamic,
            0x00000003 => Self::Interp,
            0x00000004 => Self::Note,
            0x00000005 => Self::Shlib,
            0x00000006 => Self::Phdr,
            0x00000007 => Self::Tls,
            p_type => {
                if 0x60000000 <= p_type && p_type < 0x70000000 {
                    Self::OperatingSystemSpecific(p_type)
                } else if 0x70000000 <= p_type && p_type < 0x80000000 {
                    Self::ProcessorSpecific(p_type)
                } else {
                    panic!("Can't read an ELF!")
                }
            },
        }
    }
}

