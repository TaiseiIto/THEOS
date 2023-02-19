use {
    std::collections::HashMap,
    super::{
        boot_sector,
        cluster,
    },
};

#[derive(Debug)]
pub struct Fat {
    cluster_chain: HashMap<u32, Option<u32>>,
    sector_size: usize,
    bit: Bit,
}

impl Fat {
    pub fn new(clusters: &cluster::Clusters, boot_sector: &boot_sector::BootSector) -> Self {
        let sector_size: usize = boot_sector.sector_size();
        let cluster_chain: HashMap<u32, Option<u32>> = clusters.cluster_chain();
        let bit: Bit = boot_sector.into();
        Self {
            cluster_chain,
            sector_size,
            bit,
        }
    }
}

#[derive(Debug)]
enum Bit {
    Fat12,
    Fat16,
    Fat32,
}

impl From<&boot_sector::BootSector> for Bit {
    fn from(boot_sector: &boot_sector::BootSector) -> Self {
        match boot_sector {
            boot_sector::BootSector::Fat12 {
                content,
            } => Self::Fat12,
            boot_sector::BootSector::Fat16 {
                content,
            } => Self::Fat16,
            boot_sector::BootSector::Fat32 {
                content,
            } => Self::Fat32,
        }
    }
}

