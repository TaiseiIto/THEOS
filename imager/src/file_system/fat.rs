mod boot_sector;
mod cluster;
mod directory_entry;
mod fat;
mod node;

use std::{
    fmt,
    fs,
    path::PathBuf,
};

#[derive(Debug)]
pub struct Fat {
    boot_sector: boot_sector::BootSector,
    fat: fat::Fat,
    clusters: cluster::Clusters,
    root_directory: node::Content,
    volume_label: String,
}

impl Fat {
    pub fn new(boot_sector_candidates: Vec<PathBuf>, root: &PathBuf) -> Self {
        let boot_sector_candidates: Vec<boot_sector::BootSector> = boot_sector_candidates
            .into_iter()
            .map(|boot_sector| {
                let boot_sector: &Vec<u8> = &fs::read(&boot_sector).expect("Can't generate a FAT file system.");
                boot_sector.into()
            })
            .collect();
        let cluster_size: usize = boot_sector_candidates
            .iter()
            .map(|boot_sector_candidate| boot_sector_candidate.cluster_size())
            .fold((None, true), |(cluster_size, unanimous), next_cluster_size| match cluster_size {
                Some(cluster_size) => if unanimous && cluster_size == next_cluster_size {
                    (Some(cluster_size), unanimous)
                } else {
                    (None, false)
                },
                None => (Some(next_cluster_size), unanimous),
            })
            .0
            .expect("Boot sector candidates are not unanimous about cluster size.");
        let volume_label: String = boot_sector_candidates
            .iter()
            .map(|boot_sector_candidate| boot_sector_candidate.volume_label())
            .fold((None, true), |(volume_label, unanimous), next_volume_label| match volume_label {
                Some(volume_label) => if unanimous && volume_label == next_volume_label {
                    (Some(volume_label), unanimous)
                } else {
                    (None, false)
                }
                None => (Some(next_volume_label), unanimous),
            })
            .0
            .expect("Boot sector candidates are not unanimous about volume label.");
        let (root_directory, clusters): (node::Content, cluster::Clusters) = node::Content::root(&root, cluster_size);
        let boot_sector = boot_sector::BootSector::select(boot_sector_candidates, &clusters);
        let fat = fat::Fat::new(&clusters, &boot_sector);
        let boot_sector = boot_sector.fix(&fat, &clusters);
        Self {
            boot_sector,
            fat,
            clusters,
            root_directory,
            volume_label,
        }
    }
}

impl fmt::Display for Fat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let boot_sector: String = format!("{}", self.boot_sector)
            .lines()
            .map(|line| format!("boot_sector.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let fat: String = format!("{}", self.fat);
        let root_directory: String = format!("{}", self.root_directory);
        let fat: Vec<String> = vec![
            boot_sector,
            fat,
            root_directory,
        ];
        let fat: String = fat.join("\n");
        write!(f, "{}", fat)
    }
}

impl From<&Vec<u8>> for Fat {
    fn from(bytes: &Vec<u8>) -> Self {
        let boot_sector: Vec<u8> = bytes[..0x200].to_vec();
        let boot_sector: boot_sector::BootSector = (&boot_sector).into();
        let fat_offset: usize = boot_sector.reserved_sectors() * boot_sector.sector_size();
        let fat_size: usize = boot_sector.sectors_per_fat() * boot_sector.sector_size();
        let fat: Vec<u8> = bytes[fat_offset..fat_offset + fat_size].to_vec();
        let fat = fat::Fat::read(&fat, &boot_sector);
        let root_directory_offset: usize = fat_offset + boot_sector.fats() * fat_size;
        let root_directory_size: usize = boot_sector.root_directory_entries().expect("Can't read FAT.") * directory_entry::DIRECTORY_ENTRY_SIZE;
        let root_directory: Vec<u8> = bytes[root_directory_offset..root_directory_offset + root_directory_size].to_vec();
        let clusters_offset: usize = root_directory_offset + root_directory_size;
        let cluster_size: usize = boot_sector.cluster_size();
        let clusters: Vec<u8> = bytes[clusters_offset..].to_vec();
        let clusters = cluster::Clusters::read(clusters, &fat, cluster_size);
        let (root_directory, volume_label): (node::Content, String) = node::Content::read_root(&root_directory, &clusters);
        Self {
            boot_sector,
            fat,
            clusters,
            root_directory,
            volume_label,
        }
    }
}

impl Into<Vec<u8>> for &Fat {
    fn into(self) -> Vec<u8> {
        let mut boot_sector: Vec<u8> = (&self.boot_sector).into();
        let sector_size: usize = self.boot_sector.sector_size();
        let reserved_sectors: usize = self.boot_sector.reserved_sectors();
        let reserved_size: usize = reserved_sectors * sector_size;
        boot_sector.resize(reserved_size, 0x00);
        let fat: Vec<u8> = (&self.fat).into();
        let fat: Vec<u8> = fat.repeat(self.boot_sector.fats());
        let root_directory_entries: usize = self.boot_sector
            .root_directory_entries()
            .expect("Can't convert a FAT file system into bytes.");
        let root_directory: Vec<u8> = self.root_directory.root_into_bytes(&self.volume_label, root_directory_entries);
        let clusters: Vec<u8> = (&self.clusters).into();
        vec![
            boot_sector,
            fat,
            root_directory,
            clusters,
        ].concat().to_vec()
    }
}

