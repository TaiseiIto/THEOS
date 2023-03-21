// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

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

