use {
    std::{
        collections::HashMap,
        mem,
    },
    super::{
        cluster,
        super::binary::Binary,
    },
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

    pub fn read(bytes: &Vec<u8>, sector_size: usize) -> Self {
        let cluster_chain: Vec<u32> = bytes
            .chunks(mem::size_of::<u32>())
            .map(|cluster| {
                let cluster: [u8; mem::size_of::<u32>()] = cluster
                    .try_into()
                    .expect("Can't read FAT.");
                unsafe {
                    mem::transmute::<[u8; mem::size_of::<u32>()], u32>(cluster)
                }
            })
            .collect();
        let cluster_chain: HashMap<u32, Option<u32>> = (0..cluster_chain.len())
            .map(|cluster_number| {
                let next_cluster_number: u32 = cluster_chain[cluster_number];
                let next_cluster_number: Option<u32> = if 2 <= next_cluster_number && next_cluster_number < 0xfffffff8 {
                    Some(next_cluster_number)
                } else {
                    None
                };
                let cluster_number: u32 = cluster_number as u32;
                (cluster_number, next_cluster_number)
            })
            .collect();
        Self {
            cluster_chain,
            sector_size,
        }
    }

    pub fn sectors_per_fat(&self) -> usize {
        self.to_bytes().len() / self.sector_size
    }
}

impl Binary for Fat {
    fn to_bytes(&self) -> Vec<u8> {
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
        let size: usize = (bytes.len() + self.sector_size - 1) / self.sector_size * self.sector_size;
        bytes.resize(size, 0xff);
        bytes
    }
}

