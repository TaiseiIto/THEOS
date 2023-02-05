mod exfat;
mod fat;
mod file_system_type;

use {
    std::{
        collections::HashMap,
        fmt,
        fs,
        path::PathBuf,
    },
    super::{
        binary::Binary,
        rand,
    },
};

#[derive(Debug)]
pub enum FileSystem {
    Exfat {
        content: exfat::Exfat,
    },
    Fat {
        content: fat::Fat,
    },
}

impl FileSystem {
    pub fn new(boot_sector_candidates: Vec<PathBuf>, source_directory: PathBuf, rand_generator: &mut rand::Generator) -> Self {
        let boot_sector_candidates: HashMap<file_system_type::FileSystemType, PathBuf> = boot_sector_candidates
            .into_iter()
            .map(|boot_sector| {
                let binary: Vec<u8> = fs::read(&boot_sector).expect("Can't generate a file system.");
                let file_system = file_system_type::FileSystemType::identify(&binary);
                (file_system, boot_sector)
            })
            .collect();
        let exfat_boot_sector: Option<&PathBuf> = boot_sector_candidates.get(&file_system_type::FileSystemType::Exfat);
        let fat12_boot_sector: Option<&PathBuf> = boot_sector_candidates.get(&file_system_type::FileSystemType::Fat12);
        let fat16_boot_sector: Option<&PathBuf> = boot_sector_candidates.get(&file_system_type::FileSystemType::Fat16);
        let fat32_boot_sector: Option<&PathBuf> = boot_sector_candidates.get(&file_system_type::FileSystemType::Fat32);
        match (
            exfat_boot_sector,
            fat12_boot_sector,
            fat16_boot_sector,
            fat32_boot_sector,
        ) {
            (
                Some(exfat_boot_sector),
                None,
                None,
                None,
            ) => {
                let content = exfat::Exfat::new(exfat_boot_sector, &source_directory, rand_generator);
                Self::Exfat {
                    content,
                }
            },
            (
                None,
                Some(fat12_boot_sector),
                Some(fat16_boot_sector),
                Some(fat32_boot_sector),
            ) => {
                let content = fat::Fat::new(fat12_boot_sector);
                Self::Fat {
                    content,
                }
            },
            _ => panic!("Can't generate a file system."),
        }
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        let file_system = file_system_type::FileSystemType::identify(bytes);
        match file_system {
            file_system_type::FileSystemType::Exfat => {
                let content = exfat::Exfat::read(bytes);
                Self::Exfat {
                    content,
                }
            },
            file_system_type::FileSystemType::Fat12 |
            file_system_type::FileSystemType::Fat16 |
            file_system_type::FileSystemType::Fat32 => {
                let content = fat::Fat::read(bytes);
                Self::Fat {
                    content,
                }
            },
        }
    }
}

impl Binary for FileSystem {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Exfat {
                content,
            } => content.to_bytes(),
            Self::Fat {
                content,
            } => content.to_bytes(),
        }
    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exfat {
                content,
            } => write!(f, "{}", content),
            Self::Fat {
                content,
            } => write!(f, "{}", content),
        }
    }
}

