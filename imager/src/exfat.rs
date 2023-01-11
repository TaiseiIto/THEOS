mod boot_sector;
mod cluster;
mod object;

use std::path;

#[derive(Debug)]
pub struct Exfat {
	boot_sector: boot_sector::BootSector,
    clusters: cluster::Clusters,
    object: object::Object,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf) -> Self {
		let boot_sector = boot_sector::BootSector::new(boot_sector);
        let clusters = cluster::Clusters::new(boot_sector.cluster_size());
        let object = object::Object::new(source_directory);
        Self {
			boot_sector,
			clusters,
            object,
        }
    }
}

