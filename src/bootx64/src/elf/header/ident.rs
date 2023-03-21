use super::EI_NIDENT;

#[derive(Debug)]
pub struct Ident {
    mag: [u8; MAG_LENGTH],
}

const MAG: [u8; MAG_LENGTH] = [0x7f, 0x45, 0x4c, 0x46];
const MAG_LENGTH: usize = 4;

impl Ident {
    pub fn new(ident: [u8; EI_NIDENT]) -> Self {
        let mag: [u8; MAG_LENGTH] = ident[..MAG_LENGTH]
            .try_into()
            .expect("Can't read an ELF e_ident!");
        if let MAG = mag {
            Self {
                mag,
            }
        } else {
            panic!("Can't read an ELF e_ident!");
        }
    }
}
