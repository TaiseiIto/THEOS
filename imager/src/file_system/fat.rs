mod boot_sector;
mod cluster;
mod directory_entry;
mod node;

use {
    std::{
        convert::Into,
        fmt,
        fs,
        path::PathBuf,
    },
    super::{
        super::binary::Binary,
    },
};

#[derive(Debug)]
pub struct Fat {
    boot_sector: boot_sector::BootSector,
}

impl Fat {
    pub fn new(boot_sector_candidates: Vec<PathBuf>, root: &PathBuf) -> Self {
        let boot_sector_candidates: Vec<boot_sector::BootSector> = boot_sector_candidates
            .into_iter()
            .map(|boot_sector_path| {
                let boot_sector_binary: Vec<u8> = fs::read(&boot_sector_path).expect("Can't generate a FAT file system.");
                boot_sector::BootSector::read(&boot_sector_binary)
            })
            .collect();
        let cluster_size: usize = boot_sector_candidates
            .iter()
            .map(|boot_sector_candidate| boot_sector_candidate.get_cluster_size())
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
        let mut clusters = cluster::Clusters::new(cluster_size);
        eprintln!("cluster_size: {:#x}", cluster_size);
        let root = node::FileOrDirectory::new(root, &mut clusters);
        eprintln!("{}", root);
        let boot_sector: boot_sector::BootSector = boot_sector_candidates[0];
        Self {
            boot_sector,
        }
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        let boot_sector = boot_sector::BootSector::read(bytes);
        Self {
            boot_sector,
        }
    }
}

impl Into<Vec<u8>> for &Fat {
    fn into(self) -> Vec<u8> {
        (&self.boot_sector).into()
    }
}

impl fmt::Display for Fat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.boot_sector)
    }
}

