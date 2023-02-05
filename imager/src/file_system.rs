mod exfat;

use {
    std::{
        fmt,
        fs,
        path::PathBuf,
        str,
    },
    super::{
        binary,
        rand,
    },
};

#[derive(Debug)]
pub enum FileSystem {
    Exfat {
        content: exfat::Exfat,
    },
    Fat,
}

impl FileSystem {
    pub fn new(boot_sector_candidates: Vec<PathBuf>, source_directory: PathBuf, rand_generator: &mut rand::Generator) -> Self {
        let file_system: Vec<FileSystemType> = boot_sector_candidates
            .iter()
            .map(|boot_sector| FileSystemType::identify(&fs::read(&boot_sector).expect("Can't generate a file system.")))
            .collect();
        match file_system[0] {
            FileSystemType::Exfat => {
                let boot_sector: PathBuf = boot_sector_candidates
                    .into_iter()
                    .next()
                    .expect("Can't generate a file system.");
                let content = exfat::Exfat::new(boot_sector, source_directory, rand_generator);
                Self::Exfat {
                    content,
                }
            },
            FileSystemType::Fat12 |
            FileSystemType::Fat16 |
            FileSystemType::Fat32 => Self::Fat,
        }
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        let file_system = FileSystemType::identify(bytes);
        match file_system {
            FileSystemType::Exfat => {
                let content = exfat::Exfat::read(bytes);
                Self::Exfat {
                    content,
                }
            },
            FileSystemType::Fat12 |
            FileSystemType::Fat16 |
            FileSystemType::Fat32 => Self::Fat,
        }
    }
}

impl binary::Binary for FileSystem {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Exfat {
                content,
            } => content.to_bytes(),
            Self::Fat => vec![],
        }
    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exfat {
                content,
            } => write!(f, "{}", content),
            Self::Fat => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub enum FileSystemType {
    Exfat,
    Fat12,
    Fat16,
    Fat32,
}

impl FileSystemType {
    fn identify(bytes: &Vec<u8>) -> Self {
        let file_system: &str = str::from_utf8(&bytes[3..11]).expect("Can't identify file sistem.");
        match file_system {
            "EXFAT   " => Self::Exfat,
            _ => {
                let file_system: &str = str::from_utf8(&bytes[54..62]).expect("Can't identify file system.");
                match file_system {
                    "FAT12   " => Self::Fat12,
                    "FAT16   " => Self::Fat16,
                    _ => {
                        let file_system: &str = str::from_utf8(&bytes[82..90]).expect("Can't identify file system.");
                        match file_system {
                            "FAT32   " => Self::Fat32,
                            _ => panic!("Can't identify file system."),
                        }
                    },
                }
            },
        }
    }
}

