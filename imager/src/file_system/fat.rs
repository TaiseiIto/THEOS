mod boot_sector;

use {
    std::{
        fmt,
        fs,
        path::PathBuf,
    },
    super::{
        node,
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
        let root = node::FileOrDirectory::new(root);
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

impl Binary for Fat {
    fn to_bytes(&self) -> Vec<u8> {
        self.boot_sector.to_bytes()
    }
}

impl fmt::Display for Fat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.boot_sector)
    }
}

