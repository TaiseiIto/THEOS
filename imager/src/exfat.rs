mod boot_sector;
mod object;

use std::path;

#[derive(Debug)]
pub struct Exfat {
	boot_sector: boot_sector::BootSector,
    object: object::Object,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf) -> Self {
		let boot_sector = boot_sector::BootSector::new(boot_sector);
        let object = object::Object::new(source_directory);
        Self {
			boot_sector,
            object,
        }
    }
}

