mod fat12;
mod fat16;
mod fat32;

use {
    std::{
        convert::Into,
        fmt,
        fs,
        path::PathBuf,
    },
    super::super::file_system_type,
};

#[derive(Clone, Copy, Debug)]
pub enum BootSector {
    Fat12 {
        content: fat12::Fat12,
    },
    Fat16 {
        content: fat16::Fat16,
    },
    Fat32 {
        content: fat32::Fat32,
    },
}

impl BootSector {
    pub fn get_cluster_size(&self) -> usize {
        match self {
            Self::Fat12 {
                content,
            } => content.get_cluster_size(),
            Self::Fat16 {
                content,
            } => content.get_cluster_size(),
            Self::Fat32 {
                content,
            } => content.get_cluster_size(),
        }
    }

    pub fn new(boot_sector: &PathBuf) -> Self {
        let boot_sector: Vec<u8> = fs::read(boot_sector).expect("Can't generate a boot sector.");
        Self::read(&boot_sector)
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        let file_system_type = file_system_type::FileSystemType::identify(bytes);
        match file_system_type {
            file_system_type::FileSystemType::Exfat => panic!("Can't generate a boot sector."),
            file_system_type::FileSystemType::Fat12 => {
                let content = fat12::Fat12::read(bytes);
                Self::Fat12 {
                    content,
                }
            },
            file_system_type::FileSystemType::Fat16 => {
                let content = fat16::Fat16::read(bytes);
                Self::Fat16 {
                    content,
                }
            },
            file_system_type::FileSystemType::Fat32 => {
                let content = fat32::Fat32::read(bytes);
                Self::Fat32 {
                    content,
                }
            },
        }
    }
}

impl Into<Vec<u8>> for &BootSector {
    fn into(self) -> Vec<u8> {
        match self {
            BootSector::Fat12 {
                content,
            } => content.into(),
            BootSector::Fat16 {
                content,
            } => content.into(),
            BootSector::Fat32 {
                content,
            } => content.into(),
        }
    }
}

impl fmt::Display for BootSector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fat12 {
                content,
            } => write!(f, "FAT12 boot sector\n{}", content),
            Self::Fat16 {
                content,
            } => write!(f, "FAT16 boot sector\n{}", content),
            Self::Fat32 {
                content,
            } => write!(f, "FAT32 boot sector\n{}", content),
        }
    }
}

