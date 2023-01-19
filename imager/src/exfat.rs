mod allocation_bitmap;
mod boot_sector;
mod cluster;
mod directory_entry;
mod fat;
mod object;
mod upcase_table;

use {
    std::path,
    super::rand,
};

#[derive(Debug)]
pub struct Exfat {
    boot_sector: boot_sector::BootSector,
    clusters: cluster::Clusters,
    fat: fat::Fat,
    object: object::Object,
    upcase_table: upcase_table::UpcaseTable,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf, rand_generator: &mut rand::Generator) -> Self {
        let boot_sector = boot_sector::BootSector::new(boot_sector);
        let mut clusters = cluster::Clusters::new(boot_sector.cluster_size());
        let upcase_table = upcase_table::UpcaseTable::new();
        let object = object::Object::root(source_directory, &boot_sector, &mut clusters, &upcase_table, rand_generator);
        let fat = fat::Fat::new(&clusters);
        Self {
            boot_sector,
            clusters,
            fat,
            object,
            upcase_table,
        }
    }
}

