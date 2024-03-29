mod allocation_bitmap;
mod boot_checksum;
mod boot_sector;
mod cluster;
mod directory_entry;
mod extended_boot_sector;
mod fat;
mod node;
mod oem_parameter;
mod reserved_sector;
mod upcase_table;

use {
    std::{
        fmt,
        path::PathBuf,
        rc::Rc,
    },
    super::super::{
        guid,
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
    root_directory: Rc<node::Node>,
}

impl Exfat {
    pub fn allocation_bitmap(&self) -> allocation_bitmap::AllocationBitmap {
        self.root_directory.allocation_bitmap(&self.clusters)
    }

    pub fn new(boot_sector: &PathBuf, source_directory: &PathBuf, has_volume_guid: bool, rand_generator: &mut rand::Generator) -> Self {
        let boot_sector = boot_sector::BootSector::new(boot_sector);
        let mut clusters = cluster::Clusters::new(boot_sector.cluster_size());
        let extended_boot_sectors = [extended_boot_sector::ExtendedBootSector::new(boot_sector.bytes_per_sector()); NUM_OF_EXTENDED_BOOT_SECTORS];
        let upcase_table = upcase_table::UpcaseTable::new();
        let root_directory = node::Node::root_directory(&source_directory, &boot_sector, &mut clusters, &upcase_table, has_volume_guid, rand_generator);
        let oem_parameters = oem_parameter::OemParameters::null(boot_sector.bytes_per_sector());
        let reserved_sector = reserved_sector::ReservedSector::new(boot_sector.bytes_per_sector());
        let fat = fat::Fat::new(&clusters, boot_sector.bytes_per_sector());
        let boot_sector: boot_sector::BootSector = boot_sector.fix(&fat, &root_directory, &clusters);
        let boot_checksum = boot_checksum::BootChecksum::new(&boot_sector, &extended_boot_sectors, &oem_parameters, &reserved_sector, boot_sector.bytes_per_sector());
        Self {
            boot_checksum,
            boot_sector,
            clusters,
            extended_boot_sectors,
            fat,
            oem_parameters,
            reserved_sector,
            root_directory,
        }
    }

    pub fn volume_guid(&self) -> Option<guid::Guid> {
        self.root_directory.volume_guid()
    }

    pub fn volume_label(&self) -> String {
        self.root_directory.volume_label()
    }
}

impl From<&Vec<u8>> for Exfat {
    fn from(bytes: &Vec<u8>) -> Self {
        let boot_sector = boot_sector::BootSector::from(bytes);
        let sector_size: usize = boot_sector.bytes_per_sector();
        let sectors: Vec<Vec<u8>> = bytes
            .chunks(sector_size)
            .map(|sector| sector.to_vec())
            .collect();
        let main_boot_region_sectors: Vec<Vec<u8>> = sectors[0..12].to_vec();
        let backup_boot_region_sectors: Vec<Vec<u8>> = sectors[0..12].to_vec();
        let main_boot_region_equals_backup_boot_region: bool = main_boot_region_sectors
            .iter()
            .zip(backup_boot_region_sectors.iter())
            .map(|(main_boot_region_sector, backup_boot_region_sector)| main_boot_region_sector
                .iter()
                .zip(backup_boot_region_sector.iter())
                .map(|(main_boot_region_byte, backup_boot_region_byte)| main_boot_region_byte == backup_boot_region_byte)
                .fold(true, |sector_equality, byte_equality| sector_equality && byte_equality))
            .fold(true, |region_equality, sector_equality| region_equality && sector_equality);
        assert!(main_boot_region_equals_backup_boot_region);
        let mut sector_offset: usize = 1;
        let extended_boot_sectors: Vec<extended_boot_sector::ExtendedBootSector> = main_boot_region_sectors[sector_offset..sector_offset + NUM_OF_EXTENDED_BOOT_SECTORS]
            .iter()
            .map(|sector| extended_boot_sector::ExtendedBootSector::from(sector))
            .collect();
        let extended_boot_sectors: [extended_boot_sector::ExtendedBootSector; NUM_OF_EXTENDED_BOOT_SECTORS] = extended_boot_sectors
            .try_into()
            .expect("Can't read extended boot sectors.");
        sector_offset += NUM_OF_EXTENDED_BOOT_SECTORS;
        let oem_parameters = oem_parameter::OemParameters::from(&main_boot_region_sectors[sector_offset]);
        sector_offset += 1;
        let reserved_sector = reserved_sector::ReservedSector::from(&main_boot_region_sectors[sector_offset]);
        sector_offset += 1;
        let boot_checksum = boot_checksum::BootChecksum::from(&main_boot_region_sectors[sector_offset]);
        let fat_offset: usize = boot_sector.fat_offset() as usize;
        let fat_length: usize = boot_sector.fat_length() as usize;
        let fat: Vec<Vec<u8>> = sectors[fat_offset..fat_offset + fat_length].to_vec();
        let fat: Vec<u8> = fat
            .into_iter()
            .flatten()
            .collect();
        let cluster_size: usize = boot_sector.cluster_size();
        let cluster_heap_offset: usize = boot_sector.cluster_heap_offset() as usize;
        let cluster_count: u32 = boot_sector.cluster_count();
        let fat = fat::Fat::read(&fat, sector_size, cluster_count);
        let clusters: Vec<Vec<u8>> = sectors[cluster_heap_offset..].to_vec();
        let clusters: Vec<u8> = clusters
            .into_iter()
            .flatten()
            .collect();
        let mut clusters = cluster::Clusters::read(clusters, &fat, cluster_size);
        let first_cluster_of_root_directory: u32 = boot_sector.first_cluster_of_root_directory();
        let root_directory = node::Node::read_root_directory(&clusters, &fat, first_cluster_of_root_directory, cluster_size);
        let allocation_bitmap: allocation_bitmap::AllocationBitmap = root_directory.allocation_bitmap(&clusters);
        clusters.set_used_flags(&allocation_bitmap);
        Self {
            boot_checksum,
            boot_sector,
            clusters,
            extended_boot_sectors,
            fat,
            oem_parameters,
            reserved_sector,
            root_directory,
        }
    }
}

impl Into<Vec<u8>> for &Exfat {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        let mut boot_sector: Vec<u8> = (&self.boot_sector).into();
        let mut extended_boot_sectors: Vec<u8> = self.extended_boot_sectors
            .iter()
            .map(|extended_boot_sector| Into::<Vec<u8>>::into(extended_boot_sector).into_iter())
            .flatten()
            .collect();
        let mut oem_parameters: Vec<u8> = (&self.oem_parameters).into();
        let mut reserved_sector: Vec<u8> = (&self.reserved_sector).into();
        let mut boot_checksum: Vec<u8> = (&self.boot_checksum).into();
        bytes.append(&mut boot_sector);
        bytes.append(&mut extended_boot_sectors);
        bytes.append(&mut oem_parameters);
        bytes.append(&mut reserved_sector);
        bytes.append(&mut boot_checksum);
        bytes.append(&mut bytes.clone());
        let fat: Vec<u8> = (&self.fat).into();
        let num_of_fats: usize = self.boot_sector.num_of_fats();
        let mut fat: Vec<u8> = (0..num_of_fats)
            .map(|_| fat.clone().into_iter())
            .flatten()
            .collect();
        bytes.append(&mut fat);
        let cluster_heap_offset: usize = self.boot_sector.cluster_heap_offset() as usize;
        let cluster_heap_offset: usize = cluster_heap_offset * self.boot_sector.bytes_per_sector();
        bytes.resize(cluster_heap_offset, 0x00);
        let mut clusters: Vec<u8> = (&self.clusters).into();
        bytes.append(&mut clusters);
        bytes
    }
}

impl fmt::Display for Exfat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let boot_sector: String = format!("{}", self.boot_sector)
            .lines()
            .map(|line| format!("boot_sector.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let extended_boot_sectors: String = self.extended_boot_sectors
            .iter()
            .enumerate()
            .map(|(i, extended_boot_sector)| format!("{}", extended_boot_sector)
                .lines()
                .map(|line| format!("extended_boot_sector[{}].{}", i, line))
                .collect::<Vec<String>>()
                .join("\n"))
            .collect::<Vec<String>>()
            .join("\n");
        let oem_parameters: String = format!("{}", self.oem_parameters)
            .lines()
            .map(|line| format!("oem_parameters.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let reserved_sector: String = format!("reserved_sector.{}", self.reserved_sector);
        let boot_checksum: String = format!("{}", self.boot_checksum)
            .lines()
            .map(|line| format!("boot_checksum.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let allocation_bitmap: String = format!("{}", self.allocation_bitmap())
            .lines()
            .map(|line| format!("allocation_bitmap.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let volume_guid: String = match self.volume_guid() {
            Some(volume_guid) => format!("{}", volume_guid),
            None => "None".to_string(),
        };
        let volume_guid: String = volume_guid
            .lines()
            .map(|line| format!("volume_guid.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let volume_label: String = format!("volume_label: \"{}\"", self.volume_label());
        let mut cluster_used_flags: Vec<(u32, bool)> = self.clusters
            .used_flags()
            .into_iter()
            .collect();
        cluster_used_flags.sort_by(|(left, _), (right, _)| left.partial_cmp(right).expect("Can't print an exFAT."));
        let cluster_used_flags: String = cluster_used_flags
            .into_iter()
            .map(|(cluster_number, used)| format!("cluster[{}] is {}.", cluster_number, if used {
                "in use"
            } else {
                "available"
            }))
            .collect::<Vec<String>>()
            .join("\n");
        let root_directory: String = format!("{}", self.root_directory);
        let exfat: Vec<String> = vec![
            boot_sector,
            extended_boot_sectors,
            oem_parameters,
            reserved_sector,
            boot_checksum,
            allocation_bitmap,
            volume_guid,
            volume_label,
            cluster_used_flags,
            root_directory,
        ];
        let exfat: String = exfat.join("\n");
        write!(f, "{}", exfat)
    }
}

