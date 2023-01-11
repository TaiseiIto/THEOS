mod boot_sector;

use std::path;

#[derive(Debug)]
pub struct Exfat {
	boot_sector: boot_sector::BootSector,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf) -> Self {
		let boot_sector = boot_sector::BootSector::new(boot_sector);
        Self {
			boot_sector,
        }
    }
}

