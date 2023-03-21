pub mod ident;

const EI_NIDENT: usize = 16;

#[derive(Debug)]
pub struct Header {
    e_ident: ident::Ident,
}

impl Header {
    pub fn new(header: &[u8]) -> Self {
        let e_ident: [u8; EI_NIDENT] = header[..EI_NIDENT]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_ident = ident::Ident::new(e_ident);
        Self {
            e_ident,
        }
    }
}

