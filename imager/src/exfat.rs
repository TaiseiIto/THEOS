mod allocation_bitmap;
mod boot_checksum;
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
    binary::Binary,
    std::path,
    super::{
        binary,
        rand,
    },
};

#[derive(Debug)]
pub struct Exfat {
    boot_checksum: boot_checksum::BootChecksum,
    boot_sector: boot_sector::BootSector,
    clusters: cluster::Clusters,
    extended_boot_sectors: [extended_boot_sector::ExtendedBootSector; 0x8],
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
        let extended_boot_sectors = [extended_boot_sector::ExtendedBootSector::new(boot_sector.bytes_per_sector()); 0x8];
        let upcase_table = upcase_table::UpcaseTable::new();
        let object = object::Object::root(source_directory, &boot_sector, &mut clusters, &upcase_table, rand_generator);
        let oem_parameters = oem_parameter::OemParameters::null(boot_sector.bytes_per_sector());
        let reserved_sector = reserved_sector::ReservedSector::new(boot_sector.bytes_per_sector());
        let fat = fat::Fat::new(&clusters, boot_sector.bytes_per_sector());
        let boot_sector: boot_sector::BootSector = boot_sector.correct(&fat, &object, &clusters);
        let boot_checksum = boot_checksum::BootChecksum::new(&boot_sector, &extended_boot_sectors, &oem_parameters, &reserved_sector, boot_sector.bytes_per_sector());
        Self {
            boot_checksum,
            boot_sector,
            clusters,
            extended_boot_sectors,
            fat,
            object,
            oem_parameters,
            reserved_sector,
            upcase_table,
        }
    }
}

impl Binary for Exfat {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        let mut boot_sector: Vec<u8> = self.boot_sector.to_bytes();
        let mut extended_boot_sectors: Vec<u8> = self.extended_boot_sectors
            .iter()
            .map(|extended_boot_sector| extended_boot_sector.to_bytes().into_iter())
            .flatten()
            .collect();
        let mut oem_parameters: Vec<u8> = self.oem_parameters.to_bytes();
        let mut reserved_sector: Vec<u8> = self.reserved_sector.to_bytes();
        let mut boot_checksum: Vec<u8> = self.boot_checksum.to_bytes();
        bytes.append(&mut boot_sector);
        bytes.append(&mut extended_boot_sectors);
        bytes.append(&mut oem_parameters);
        bytes.append(&mut reserved_sector);
        bytes.append(&mut boot_checksum);
        bytes.append(&mut bytes.clone());
        let fat: Vec<u8> = self.fat.to_bytes();
        let num_of_fats: usize = self.boot_sector.num_of_fats();
        let mut fat: Vec<u8> = (0..num_of_fats)
            .map(|_| fat.clone().into_iter())
            .flatten()
            .collect();
        bytes.append(&mut fat);
        let cluster_heap_offset: usize = self.boot_sector.cluster_heap_offset() as usize;
        let cluster_heap_offset: usize = cluster_heap_offset * self.boot_sector.bytes_per_sector();
        bytes.resize(cluster_heap_offset, 0x00);
        let mut clusters: Vec<u8> = self.clusters.to_bytes();
        bytes.append(&mut clusters);
        bytes
    }
}

