// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

#[derive(Debug)]
pub enum Type {
    Unknown,
    Relocatable,
    Executable,
    Shared,
    Core,
}

impl From<u16> for Type {
    fn from(e_type: u16) -> Self {
        match e_type {
            0x0000 => Self::Unknown,
            0x0001 => Self::Relocatable,
            0x0002 => Self::Executable,
            0x0003 => Self::Shared,
            0x0004 => Self::Core,
            _ => panic!("Can't read an ELF!"),
        }
    }
}

