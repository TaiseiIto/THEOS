// References
// https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

use super::EI_NIDENT;

#[derive(Debug)]
pub struct Ident {
    mag: [u8; MAG_LENGTH],
    class: Class,
    data: Data,
    version: u8,
    osabi: OsAbi,
    abi_version: u8,
    pad: u8,
}

const MAG: [u8; MAG_LENGTH] = [0x7f, 0x45, 0x4c, 0x46];
const MAG_BEGIN: usize = 0;
const MAG_LENGTH: usize = 4;
const MAG_END: usize = MAG_BEGIN + MAG_LENGTH;
const CLASS_OFFSET: usize = MAG_END;
const DATA_OFFSET: usize = CLASS_OFFSET + 1;
const VERSION_OFFSET: usize = DATA_OFFSET + 1;
const OSABI_OFFSET: usize = VERSION_OFFSET + 1;
const ABI_VERSION_OFFSET: usize = OSABI_OFFSET + 1;
const PAD_OFFSET: usize = ABI_VERSION_OFFSET + 1;

impl Ident {
    pub fn new(ident: [u8; EI_NIDENT]) -> Self {
        let mag: [u8; MAG_LENGTH] = ident[MAG_BEGIN..MAG_END]
            .try_into()
            .expect("Can't read an ELF!");
        let class: Class = ident[CLASS_OFFSET].into();
        let data: Data = ident[DATA_OFFSET].into();
        let version: u8 = ident[VERSION_OFFSET];
        let osabi: OsAbi = ident[OSABI_OFFSET].into();
        let abi_version: u8 = ident[ABI_VERSION_OFFSET];
        let pad: u8 = ident[PAD_OFFSET];
        if let MAG = mag {
            Self {
                mag,
                class,
                data,
                version,
                osabi,
                abi_version,
                pad,
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

#[derive(Debug)]
pub enum OsAbi {
    SystemV,
    Hpux,
    NetBsd,
    Linux,
    GnuHurd,
    Solaris,
    Aix,
    Irix,
    FreeBsd,
    Tru64,
    Modesto,
    OpenBsd,
    OpenVms,
    NonStopKernel,
    Aros,
    FenixOs,
    NuxiCloudAbi,
    StratusTechnologiesOpenVos,
}

impl From<u8> for OsAbi {
    fn from(osabi: u8) -> Self {
        match osabi {
            0x00 => Self::SystemV,
            0x01 => Self::Hpux,
            0x02 => Self::NetBsd,
            0x03 => Self::Linux,
            0x04 => Self::GnuHurd,
            0x06 => Self::Solaris,
            0x07 => Self::Aix,
            0x08 => Self::Irix,
            0x09 => Self::FreeBsd,
            0x0a => Self::Tru64,
            0x0b => Self::Modesto,
            0x0c => Self::OpenBsd,
            0x0d => Self::OpenVms,
            0x0e => Self::NonStopKernel,
            0x0f => Self::Aros,
            0x10 => Self::FenixOs,
            0x11 => Self::NuxiCloudAbi,
            0x12 => Self::StratusTechnologiesOpenVos,
            _ => panic!("Can't read an ELF!"),
        }
    }
}

