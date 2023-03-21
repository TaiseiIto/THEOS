// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf

pub mod header;

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

