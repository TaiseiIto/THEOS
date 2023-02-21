mod fat12;
mod fat16;
mod fat32;

use {
    std::{
        fmt,
        fs,
        path::PathBuf,
    },
    super::{
        cluster,
        fat,
        super::file_system_type,
    },
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
    pub fn cluster_size(&self) -> usize {
        match self {
            Self::Fat12 {
                content,
            } => content.cluster_size(),
            Self::Fat16 {
                content,
            } => content.cluster_size(),
            Self::Fat32 {
                content,
            } => content.cluster_size(),
        }
    }

    pub fn fix(self, fat: &fat::Fat, clusters: &cluster::Clusters) -> Self {
        match self {
            Self::Fat12 {
                content,
            } => Self::Fat12 {
                content: content.fix(fat, clusters),
            },
            Self::Fat16 {
                content,
            } => Self::Fat16 {
                content,
            },
            Self::Fat32 {
                content,
            } => Self::Fat32 {
                content,
            },
        }
    }

    pub fn media(&self) -> u8 {
        match self {
            Self::Fat12 {
                content,
            } => content.media(),
            Self::Fat16 {
                content,
            } => content.media(),
            Self::Fat32 {
                content,
            } => content.media(),
        }
    }

    pub fn root_directory_entries(&self) -> Option<usize> {
        match self {
            Self::Fat12 {
                content,
            } => content.root_directory_entries(),
            Self::Fat16 {
                content,
            } => content.root_directory_entries(),
            Self::Fat32 {
                content,
            } => content.root_directory_entries(),
        }
    }

    pub fn sector_size(&self) -> usize {
        match self {
            Self::Fat12 {
                content,
            } => content.sector_size(),
            Self::Fat16 {
                content,
            } => content.sector_size(),
            Self::Fat32 {
                content,
            } => content.sector_size(),
        }
    }

    pub fn select(boot_sectors: Vec<Self>, clusters: &cluster::Clusters) -> Self {
        const FAT16MIN: usize = 4086;
        const FAT32MIN: usize = 65526;
        let number_of_clusters: usize = clusters.number_of_clusters();
        boot_sectors
            .into_iter()
            .filter(|boot_sector| match boot_sector {
                Self::Fat12 {
                    content: _,
                } => number_of_clusters < FAT16MIN,
                Self::Fat16 {
                    content: _,
                } => FAT16MIN <= number_of_clusters && number_of_clusters < FAT32MIN,
                Self::Fat32 {
                    content: _,
                } => FAT32MIN <= number_of_clusters,
            })
            .next()
            .expect("Can't find boot sector.")
    }

    pub fn volume_label(&self) -> String {
        let volume_label: [u8; 0xb] = match self {
            Self::Fat12 {
                content,
            } => content.volume_label(),
            Self::Fat16 {
                content,
            } => content.volume_label(),
            Self::Fat32 {
                content,
            } => content.volume_label(),
        };
        let volume_label: Vec<u8> = volume_label.to_vec();
        String::from_utf8(volume_label).expect("Can't get a volume_label.")
    }
}

impl From<&PathBuf> for BootSector {
    fn from(boot_sector: &PathBuf) -> Self {
        let boot_sector: &Vec<u8> = &fs::read(boot_sector).expect("Can't generate a boot sector.");
        boot_sector.into()
    }
}

impl From<&Vec<u8>> for BootSector {
    fn from(bytes: &Vec<u8>) -> Self {
        let file_system_type = file_system_type::FileSystemType::identify(bytes);
        match file_system_type {
            file_system_type::FileSystemType::Exfat => panic!("Can't generate a boot sector."),
            file_system_type::FileSystemType::Fat12 => {
                let content: fat12::Fat12 = bytes.into();
                Self::Fat12 {
                    content,
                }
            },
            file_system_type::FileSystemType::Fat16 => {
                let content: fat16::Fat16 = bytes.into();
                Self::Fat16 {
                    content,
                }
            },
            file_system_type::FileSystemType::Fat32 => {
                let content: fat32::Fat32 = bytes.into();
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

