// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

#[allow(dead_code)]
#[derive(Debug)]
pub struct Flags {
    execute: bool,
    write: bool,
    read: bool,
    operating_system_specified: u8,
    processor_specified: u8,
}

impl From<u32> for Flags {
    fn from(p_flags: u32) -> Self {
        let execute: bool = p_flags & 0x00000001 != 0;
        let write: bool = p_flags & 0x00000002 != 0;
        let read: bool = p_flags & 0x00000004 != 0;
        let operating_system_specified: u8 = (p_flags >> 20) as u8;
        let processor_specified: u8 = (p_flags >> 28) as u8;
        Self {
            execute,
            write,
            read,
            operating_system_specified,
            processor_specified,
        }
    }
}

