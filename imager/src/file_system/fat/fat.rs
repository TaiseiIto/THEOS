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
}

impl Fat {
    pub fn new(clusters: &cluster::Clusters, boot_sector: &boot_sector::BootSector) -> Self {
        let sector_size: usize = boot_sector.sector_size();
        let cluster_chain: HashMap<u32, Option<u32>> = clusters.cluster_chain();
        Self {
            cluster_chain,
            sector_size,
        }
    }
}

