use {
    std::mem,
    super::{
        boot_sector,
        extended_boot_sector,
        oem_parameter,
        reserved_sector,
    },
};

#[derive(Debug)]
pub struct BootChecksum {
    size: usize,
    checksum: u32,
}

impl BootChecksum {
    pub fn new(
        boot_sector: &boot_sector::BootSector,
        extended_boot_sector: &[extended_boot_sector::ExtendedBootSector],
        oem_parameters: &oem_parameter::OemParameters,
        reserved_sector: &reserved_sector::ReservedSector,
        size: usize,
    ) -> Self {
        let boot_sector: Vec<u8> = boot_sector.to_bytes();
        let extended_boot_sector: Vec<u8> = extended_boot_sector
            .iter()
            .map(|extended_boot_sector| extended_boot_sector.to_bytes().into_iter())
            .flatten()
            .collect();
        let oem_parameters: Vec<u8> = oem_parameters.to_bytes();
        let reserved_sector: Vec<u8> = reserved_sector.to_bytes();
        let bytes: Vec<u8> = [boot_sector, extended_boot_sector, oem_parameters, reserved_sector]
            .into_iter()
            .map(|sector| sector.into_iter())
            .flatten()
            .collect();
        let checksum: u32 = bytes
            .into_iter()
            .enumerate()
            .filter(|(i, _)| match i {
                106 | 107 | 112 => false,
                _ => true,
            })
            .map(|(i, byte)| byte)
            .fold(0u32, |checksum, byte| (checksum << 15) + (checksum >> 1) + (byte as u32));
        Self {
            size,
            checksum,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let checksum: [u8; 4] = unsafe {
            mem::transmute::<u32, [u8; 4]>(self.checksum)
        };
        let checksum: Vec<u8> = checksum
            .into_iter()
            .collect();
        let checksum: Vec<u8> = (0..self.size / checksum.len())
            .map(|_| checksum.clone())
            .flatten()
            .collect();
        checksum
    }
}

