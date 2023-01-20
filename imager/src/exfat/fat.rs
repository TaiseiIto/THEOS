use {
    std::collections::HashMap,
    super::cluster,
};

#[derive(Debug)]
pub struct Fat {
    cluster_chain: HashMap<u32, Option<u32>>,
    sector_size: usize,
}

impl Fat {
    pub fn new(clusters: &cluster::Clusters, sector_size: usize) -> Self {
        let cluster_chain: HashMap<u32, Option<u32>> = clusters.cluster_chain();
        Self {
            cluster_chain,
            sector_size,
        }
    }

    pub fn sectors_per_fat(&self) -> usize {
        self.to_bytes().len() / self.sector_size
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let max_cluster_number: u32 = *self.cluster_chain
            .keys()
            .max()
            .expect("Can't find max cluster number.");
        let cluster_chain: Vec<u32> = (0..=max_cluster_number)
            .map(|cluster_number| match cluster_number {
                0 => 0xfffffff8,
                1 => 0xffffffff,
                cluster_number => match self.cluster_chain.get(&cluster_number) {
                    Some(next_cluster_number) => match next_cluster_number {
                        Some(next_cluster_number) => *next_cluster_number,
                        None => 0xffffffff,
                    },
                    None => 0xffffffff,
                },
            })
            .collect();
        let mut bytes: Vec<u8> = cluster_chain
            .into_iter()
            .map(|cluster_number| vec![
                cluster_number as u8,
                (cluster_number >> 0x8) as u8,
                (cluster_number >> 0x10) as u8,
                (cluster_number >> 0x18) as u8,
            ])
            .flatten()
            .collect();
        bytes.resize(self.sector_size, 0xff);
        bytes
    }
}

