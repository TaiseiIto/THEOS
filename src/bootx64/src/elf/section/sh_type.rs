// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

#[derive(Debug)]
pub enum Type {
    Null,
    Progbits,
    Symtab,
    Strtab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    InitArray,
    FiniArray,
    PreinitArray,
    Group,
    SymtabShndx,
    Num,
    OperatingSystemSpecific(u32),
}

impl From<u32> for Type {
    fn from(sh_type: u32) -> Self {
        match sh_type {
            0x00000000 => Self::Null,
            0x00000001 => Self::Progbits,
            0x00000002 => Self::Symtab,
            0x00000003 => Self::Strtab,
            0x00000004 => Self::Rela,
            0x00000005 => Self::Hash,
            0x00000006 => Self::Dynamic,
            0x00000007 => Self::Note,
            0x00000008 => Self::Nobits,
            0x00000009 => Self::Rel,
            0x0000000a => Self::Shlib,
            0x0000000b => Self::Dynsym,
            0x0000000e => Self::InitArray,
            0x0000000f => Self::FiniArray,
            0x00000010 => Self::PreinitArray,
            0x00000011 => Self::Group,
            0x00000012 => Self::SymtabShndx,
            0x00000013 => Self::Num,
            sh_type => {
                if 0x60000000 <= sh_type {
                    Self::OperatingSystemSpecific(sh_type)
                } else {
                    panic!("Can't read an ELF!")
                }
            },
        }
    }
}

