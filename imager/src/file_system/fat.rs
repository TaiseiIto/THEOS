mod fat12;
mod fat16;

use {
    std::{
        fmt,
        path::PathBuf,
    },
    super::super::binary::Binary,
};

#[derive(Debug)]
pub struct Fat {
    fat12_boot_sector: Option<fat12::boot_sector::BootSector>,
}

impl Fat {
    pub fn new(fat12_boot_sector: PathBuf) -> Self {
        let fat12_boot_sector: Option<fat12::boot_sector::BootSector> = Some(fat12::boot_sector::BootSector::new(fat12_boot_sector));
        Self {
            fat12_boot_sector,
        }
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        let fat12_boot_sector: Option<fat12::boot_sector::BootSector> = Some(fat12::boot_sector::BootSector::read(bytes));
        Self {
            fat12_boot_sector,
        }
    }
}

impl Binary for Fat {
    fn to_bytes(&self) -> Vec<u8> {
        match self.fat12_boot_sector {
            Some(fat12_boot_sector) => fat12_boot_sector.to_bytes(),
            None => vec![],
        }
    }
}

impl fmt::Display for Fat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fat12_boot_sector: String = match self.fat12_boot_sector {
            Some(fat12_boot_sector) => format!("{}", fat12_boot_sector),
            None => "".to_string(),
        };
        write!(f, "{}", fat12_boot_sector)
    }
}

