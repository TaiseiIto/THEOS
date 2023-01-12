mod boot_sector;
mod cluster;
mod directory_entry;
mod object;
mod upcase_table;

use std::path;

#[derive(Debug)]
pub struct Exfat {
    boot_sector: boot_sector::BootSector,
    clusters: cluster::Clusters,
    object: object::Object,
    upcase_table: upcase_table::UpcaseTable,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf) -> Self {
        let boot_sector = boot_sector::BootSector::new(boot_sector);
        let mut clusters = cluster::Clusters::new(boot_sector.cluster_size());
        let upcase_table = upcase_table::UpcaseTable::new();
        let object = object::Object::new(source_directory, &mut clusters, &upcase_table);
        Self {
            boot_sector,
            clusters,
            object,
            upcase_table,
        }
    }
}

