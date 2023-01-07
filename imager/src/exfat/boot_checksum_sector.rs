use std::{
    fmt,
    mem,
};
use super::Sector;

#[derive(Clone, Copy, Debug)]
pub struct BootChecksumSector {
    checksum: [u32; mem::size_of::<super::RawSector>() / mem::size_of::<u32>()],
}

impl BootChecksumSector {
    pub fn new(exfat: &super::Exfat) -> Self {
        let mut sectors: Vec<Box<dyn Sector>> = vec![];
        sectors.push(Box::new(exfat.boot_sector));
        for extended_boot_sector in exfat.extended_boot_sectors {
            sectors.push(Box::new(extended_boot_sector));
        }
        sectors.push(Box::new(exfat.oem_parameter_sector));
        sectors.push(Box::new(exfat.reserved_sector));
        let checksum: u32 = sectors
            .into_iter()
            .map(|sector| sector.to_bytes().to_vec())
            .flatten()
            .enumerate()
            .filter(|(i, _)| match i {
                106 | 107 | 112 => false,
                _ => true,
            })
            .map(|(_, byte)| byte)
            .fold(0 as u32, |checksum, byte| match checksum & 1 {
                1 => 0x80000000,
                0 => 0,
                _ => panic!("Can't create checksum sector."),
            } + (checksum >> 1) + (byte as u32));
        Self {
            checksum: [checksum; 0x80],
        }
    }
}

impl Sector for BootChecksumSector {
    fn to_bytes(&self) -> super::RawSector {
        unsafe {
            mem::transmute::<Self, super::RawSector>(*self)
        }
    }
}

impl fmt::Display for BootChecksumSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "boot_checksum_sector.checksum = {:x?}", self.checksum)
    }
}

