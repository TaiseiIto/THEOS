mod fat12;
mod fat16;
mod fat32;

use {
    std::{
        fs,
        path::PathBuf,
    },
    super::super::file_system_type,
};

#[derive(Debug)]
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
    pub fn new(boot_sector: &PathBuf) -> Self {
        let bytes: Vec<u8> = fs::read(boot_sector).expect("Can't generate a boot sector.");
        let file_system_type = file_system_type::FileSystemType::identify(&bytes);
        match file_system_type {
            file_system_type::FileSystemType::Exfat => panic!("Can't generate a boot sector."),
            file_system_type::FileSystemType::Fat12 => {
                let content = fat12::Fat12::new(boot_sector);
                Self::Fat12 {
                    content,
                }
            },
            file_system_type::FileSystemType::Fat16 => {
                let content = fat16::Fat16::new(boot_sector);
                Self::Fat16 {
                    content,
                }
            },
            file_system_type::FileSystemType::Fat32 => {
                let content = fat32::Fat32::new(boot_sector);
                Self::Fat32 {
                    content,
                }
            },
        }
    }
}

