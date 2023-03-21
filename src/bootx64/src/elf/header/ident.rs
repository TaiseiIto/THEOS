use super::EI_NIDENT;

#[derive(Debug)]
pub struct Ident {
    mag: [u8; MAG_LENGTH],
    class: Class,
    data: Data,
}

const MAG: [u8; MAG_LENGTH] = [0x7f, 0x45, 0x4c, 0x46];
const MAG_BEGIN: usize = 0;
const MAG_LENGTH: usize = 4;
const MAG_END: usize = MAG_BEGIN + MAG_LENGTH;
const CLASS_OFFSET: usize = MAG_END;
const DATA_OFFSET: usize = MAG_END + 1;

impl Ident {
    pub fn new(ident: [u8; EI_NIDENT]) -> Self {
        let mag: [u8; MAG_LENGTH] = ident[MAG_BEGIN..MAG_END]
            .try_into()
            .expect("Can't read an ELF!");
        let class: Class = ident[CLASS_OFFSET].into();
        let data: Data = ident[DATA_OFFSET].into();
        if let MAG = mag {
            Self {
                mag,
                class,
                data,
            }
        } else {
            panic!("Can't read an ELF!");
        }
    }
}

#[derive(Debug)]
pub enum Class {
    ClassNone,
    Class32,
    Class64,
}

impl From<u8> for Class {
    fn from(class: u8) -> Self {
        match class {
            0 => Self::ClassNone,
            1 => Self::Class32,
            2 => Self::Class64,
            _ => panic!("Can't read an ELF!"),
        }
    }
}

#[derive(Debug)]
pub enum Data {
    DataNone,
    Data2LSB, // Little endian
    Data2MSB, // Big endian
}

impl From<u8> for Data {
    fn from(data: u8) -> Self {
        match data {
            0 => Self::DataNone,
            1 => Self::Data2LSB,
            2 => Self::Data2MSB,
            _ => panic!("Can't read an ELF!"),
        }
    }
}

