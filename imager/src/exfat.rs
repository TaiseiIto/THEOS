mod allocation_bitmap;
mod boot_sector;
mod cluster;
mod directory_entry;
mod extended_boot_sector;
mod fat;
mod object;
mod oem_parameter;
mod reserved_sector;
mod upcase_table;

use {
    std::path,
    super::rand,
};

#[derive(Debug)]
pub struct Exfat {
    boot_sector: boot_sector::BootSector,
    clusters: cluster::Clusters,
    extended_boot_sector: [extended_boot_sector::ExtendedBootSector; 0x8],
    fat: fat::Fat,
    object: object::Object,
    oem_parameters: oem_parameter::OemParameters,
    reserved_sector: reserved_sector::ReservedSector,
    upcase_table: upcase_table::UpcaseTable,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf, rand_generator: &mut rand::Generator) -> Self {
        let boot_sector = boot_sector::BootSector::new(boot_sector);
        let mut clusters = cluster::Clusters::new(boot_sector.cluster_size());
        let extended_boot_sector = [extended_boot_sector::ExtendedBootSector::new(boot_sector.bytes_per_sector()); 0x8];
        let upcase_table = upcase_table::UpcaseTable::new();
        let object = object::Object::root(source_directory, &boot_sector, &mut clusters, &upcase_table, rand_generator);
        let oem_parameters = oem_parameter::OemParameters::null(boot_sector.bytes_per_sector());
        let reserved_sector = reserved_sector::ReservedSector::new(boot_sector.bytes_per_sector());
        let fat = fat::Fat::new(&clusters, boot_sector.bytes_per_sector());
        let boot_sector: boot_sector::BootSector = boot_sector.correct(&fat, &object, &clusters);
        Self {
            boot_sector,
            clusters,
            extended_boot_sector,
            fat,
            object,
            oem_parameters,
            reserved_sector,
            upcase_table,
        }
    }
}

