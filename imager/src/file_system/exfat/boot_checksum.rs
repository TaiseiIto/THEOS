use {
    std::{
        convert::Into,
        fmt,
        mem,
    },
    super::{
        boot_sector,
        extended_boot_sector,
        oem_parameter,
        reserved_sector,
        super::super::binary::Binary,
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
        let boot_sector: Vec<u8> = boot_sector.into();
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
            .map(|(_, byte)| byte)
            .fold(0u32, |checksum, byte| (checksum << 15) + (checksum >> 1) + (byte as u32));
        Self {
            size,
            checksum,
        }
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        let size: usize = bytes.len();
        let checksum: [u8; mem::size_of::<u32>()] = bytes[0..4]
            .try_into()
            .expect("Can't read boot checksum.");
        let checksum: u32 = unsafe {
            mem::transmute::<[u8; mem::size_of::<u32>()], u32>(checksum)
        };
        Self {
            size,
            checksum,
        }
    }
}

impl Into<Vec<u8>> for &BootChecksum {
    fn into(self) -> Vec<u8> {
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

impl fmt::Display for BootChecksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let checksum = format!("checksum: {:#010x}", self.checksum);
        let size = format!("size: {:#x}", self.size);
        write!(f, "{}\n{}", checksum, size)
    }
}

