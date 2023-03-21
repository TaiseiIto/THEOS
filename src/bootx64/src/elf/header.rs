// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod e_ident;
pub mod e_type;

#[derive(Debug)]
pub struct Header {
    e_ident: e_ident::Ident,
    e_type: e_type::Type,
}

const EI_NIDENT: usize = 0x10;
const E_TYPE_BEGIN: usize = EI_NIDENT;
const E_TYPE_LENGTH: usize = 2;
const E_TYPE_END: usize = E_TYPE_BEGIN + E_TYPE_LENGTH;

impl Header {
    pub fn new(header: &[u8]) -> Self {
        let e_ident: [u8; EI_NIDENT] = header[..EI_NIDENT]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_ident: e_ident::Ident = e_ident.into();
        let e_type: [u8; 2] = header[E_TYPE_BEGIN..E_TYPE_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_type = u16::from_le_bytes(e_type);
        let e_type: e_type::Type = e_type.into();
        Self {
            e_ident,
            e_type,
        }
    }
}

