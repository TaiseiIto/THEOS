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

const NUM_OF_EXTENDED_BOOT_SECTORS: usize = 0x8;

#[derive(Debug)]
pub struct Exfat {
    boot_checksum: boot_checksum::BootChecksum,
    boot_sector: boot_sector::BootSector,
    clusters: cluster::Clusters,
    extended_boot_sectors: [extended_boot_sector::ExtendedBootSector; NUM_OF_EXTENDED_BOOT_SECTORS],
    fat: fat::Fat,
    oem_parameters: oem_parameter::OemParameters,
    reserved_sector: reserved_sector::ReservedSector,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf, rand_generator: &mut rand::Generator) -> Self {
        let boot_sector = boot_sector::BootSector::new(boot_sector);
        let mut clusters = cluster::Clusters::new(boot_sector.cluster_size());
        let extended_boot_sectors = [extended_boot_sector::ExtendedBootSector::new(boot_sector.bytes_per_sector()); NUM_OF_EXTENDED_BOOT_SECTORS];
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
            oem_parameters,
            reserved_sector,
        }
    }

    pub fn read(bytes: &Vec<u8>) {
        let boot_sector = boot_sector::BootSector::read(bytes);
        println!("boot_sector = {:#x?}", boot_sector);
        let sector_size: usize = boot_sector.bytes_per_sector();
        let sectors: Vec<Vec<u8>> = bytes
            .chunks(sector_size)
            .map(|sector| sector.to_vec())
            .collect();
        let mut sector_offset: usize = 1;
        let extended_boot_sector: Vec<extended_boot_sector::ExtendedBootSector> = sectors[sector_offset..sector_offset + NUM_OF_EXTENDED_BOOT_SECTORS]
            .iter()
            .map(|sector| extended_boot_sector::ExtendedBootSector::read(sector))
            .collect();
        sector_offset += NUM_OF_EXTENDED_BOOT_SECTORS;
        let oem_parameters = oem_parameter::OemParameters::read(&sectors[sector_offset]);
        sector_offset += 1;
        let reserved_sector = reserved_sector::ReservedSector::read(&sectors[sector_offset]);
        sector_offset += 1;
        let boot_checksum = boot_checksum::BootChecksum::read(&sectors[sector_offset]);
        let fat_offset: usize = boot_sector.fat_offset() as usize;
        let fat_length: usize = boot_sector.fat_length() as usize;
        let fat: Vec<Vec<u8>> = sectors[fat_offset..fat_offset + fat_length].to_vec();
        let fat: Vec<u8> = fat
            .into_iter()
            .flatten()
            .collect();
        let fat = fat::Fat::read(&fat, sector_size);
        let cluster_heap_offset: usize = boot_sector.cluster_heap_offset() as usize;
        let clusters: Vec<Vec<u8>> = sectors[cluster_heap_offset..].to_vec();
        let clusters: Vec<u8> = clusters
            .into_iter()
            .flatten()
            .collect();
        let cluster_size: usize = boot_sector.cluster_size();
        let first_cluster_of_root_directory: u32 = boot_sector.first_cluster_of_root_directory();
        let root_directory = object::FileOrDirectory::read_directory(&clusters, &fat, first_cluster_of_root_directory, cluster_size);
        println!("{:#?}", root_directory);
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

