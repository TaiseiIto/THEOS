// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod header;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Elf {
    header: header::Header,
}

impl Elf {
    pub fn new(elf: &[u8]) -> Self {
        let header = header::Header::new(elf);
        Self {
            header,
        }
    }
}

