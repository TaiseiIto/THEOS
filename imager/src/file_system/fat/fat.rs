use {
    std::{
        collections::HashMap,
        fmt,
    },
    super::{
        boot_sector,
        cluster,
    },
};

#[derive(Debug)]
pub struct Fat {
    bit: Bit,
    cluster_chain: HashMap<u32, Option<u32>>,
    media: u8,
    sector_size: usize,
}

impl Fat {
    pub fn new(clusters: &cluster::Clusters, boot_sector: &boot_sector::BootSector) -> Self {
        let bit: Bit = boot_sector.into();
        let cluster_chain: HashMap<u32, Option<u32>> = clusters.cluster_chain();
        let media: u8 = boot_sector.media();
        let sector_size: usize = boot_sector.sector_size();
        Self {
            bit,
            cluster_chain,
            media,
            sector_size,
        }
    }

    pub fn read(bytes: &Vec<u8>, boot_sector: &boot_sector::BootSector) -> Self {
        let bit: Bit = boot_sector.into();
        let media: u8 = boot_sector.media();
        let sector_size: usize = boot_sector.sector_size();
        let cluster_chain: Vec<u32> = match bit {
            Bit::Fat12 => bytes
                .chunks(3)
                .map(|clusters| {
                    let mut clusters: Vec<u8> = clusters.to_vec();
                    clusters.resize(3, 0x00);
                    vec![
                        (((clusters[0] as u32)) + ((clusters[1] as u32) << 8)) & 0x00000fff,
                        (((clusters[1] as u32) >> 4) + ((clusters[2] as u32) << 4)) & 0x00000fff,
                    ]})
                .collect::<Vec<Vec<u32>>>()
                .concat(),
            Bit::Fat16 => bytes
                .chunks(2)
                .map(|cluster| cluster[0] as u32 + ((cluster[1] as u32) << 8))
                .collect(),
            Bit::Fat32 => bytes
                .chunks(4)
                .map(|cluster| cluster
                    .iter()
                    .rev()
                    .fold(0x00000000u32, |cluster, byte| (cluster << 8) + (*byte as u32)))
                .collect(),
        };
        let cluster_chain: HashMap<u32, Option<u32>> = cluster_chain
            .into_iter()
            .enumerate()
            .filter_map(|(cluster, next_cluster)| {
                let cluster: u32 = cluster as u32;
                let min: u32 = 2;
                let max: u32 = match bit {
                    Bit::Fat12 => 0x00000ff6,
                    Bit::Fat16 => 0x0000fff6,
                    Bit::Fat32 => 0x0ffffff6,
                };
                let min_end_of_chain: u32 = match bit {
                    Bit::Fat12 => 0x00000ff8,
                    Bit::Fat16 => 0x0000fff8,
                    Bit::Fat32 => 0x0ffffff8,
                };
                let max_end_of_chain: u32 = match bit {
                    Bit::Fat12 => 0x00000fff,
                    Bit::Fat16 => 0x0000ffff,
                    Bit::Fat32 => 0x0fffffff,
                };
                if min <= cluster && cluster <= max {
                    if min <= next_cluster && next_cluster <= max {
                        Some((cluster, Some(next_cluster)))
                    } else if min_end_of_chain <= next_cluster && next_cluster <= max_end_of_chain {
                        Some((cluster, None))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        Self {
            bit,
            cluster_chain,
            media,
            sector_size,
        }
    }

    pub fn to_chains(&self) -> HashMap<u32, Vec<u32>> {
        self.cluster_chain
            .iter()
            .fold(HashMap::<u32, Vec<u32>>::new(), |mut chains, (cluster, next)| {
                if let Some(next) = next {
                    let mut cluster: u32 = *cluster;
                    let mut chain: Vec<u32> = vec![cluster, *next];
                    let last_to_first: HashMap<u32, u32> = chains
                        .iter()
                        .filter_map(|(first, clusters)| {
                            match clusters.last() {
                                Some(last) => Some((*last, *first)),
                                None => None,
                            }
                        })
                        .collect();
                    if let Some(first) = last_to_first.get(&cluster) {
                        cluster = *first;
                        chain = chains.get(&cluster).expect("Can't get cluster chains.").clone();
                        chain.push(*next);
                    }
                    if let Some(continuation) = chains.get(next) {
                        chain.append(&mut continuation[1..].to_vec());
                        chains.remove(next);
                    }
                    chains.insert(cluster, chain);
                } else {
                    chains.insert(*cluster, vec![*cluster]);
                }
                chains
            })
    }
}

impl fmt::Display for Fat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let media: String = format!("media: {:#0x}", self.media);
        let bit: &Bit = &self.bit;
        let bit: usize = bit.into();
        let digits: usize = bit / 4;
        let mut chains: Vec<Vec<u32>> = self
            .to_chains()
            .into_values()
            .collect();
        chains.sort_by_key(|chain| chain[0]);
        let chains: String = chains
            .into_iter()
            .map(|chain| {
                let chain: String = chain
                    .into_iter()
                    .map(|cluster| format!("{:0digits$x}", cluster))
                    .collect::<Vec<String>>()
                    .join(",");
                format!("cluster_chain: [{}]", chain)
            })
            .collect::<Vec<String>>()
            .join("\n");
        let fat: Vec<String> = vec![
            media,
            chains,
        ];
        let fat: String = fat
            .into_iter()
            .map(|element| element
                .lines()
                .map(|line| format!("fat.{}", line))
                .collect::<Vec<String>>()
                .join("\n"))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", fat)
    }
}

impl Into<Vec<u8>> for &Fat {
    fn into(self) -> Vec<u8> {
        let fat0: u32 = 0xffffff00 + self.media as u32;
        let fat1: u32 = 0xffffffff;
        let max_cluster_number: u32 = *self.cluster_chain
            .keys()
            .max()
            .expect("Can't find max cluster number.");
        let mut cluster_chain: Vec<u32> = (0..=max_cluster_number)
            .map(|cluster_number| match cluster_number {
                0 => fat0,
                1 => fat1,
                cluster_number => match self.cluster_chain.get(&cluster_number) {
                    Some(next_cluster_number) => match next_cluster_number {
                        Some(next_cluster_number) => *next_cluster_number,
                        None => 0xffffffff,
                    },
                    None => 0xffffffff,
                },
            })
            .collect();
        if cluster_chain.len() % 2 == 1 {
            cluster_chain.push(0);
        }
        let mut bytes: Vec<u8> = match self.bit {
            Bit::Fat12 => cluster_chain
                .into_iter()
                .map(|cluster_number| cluster_number & 0xfff)
                .collect::<Vec<u32>>()
                .chunks(2)
                .map(|cluster_numbers| (cluster_numbers[0] + (cluster_numbers[1] << 12))
                    .to_le_bytes()
                    .to_vec()[..3]
                    .to_vec())
                .collect::<Vec<Vec<u8>>>()
                .concat(),
            Bit::Fat16 => cluster_chain
                .into_iter()
                .map(|cluster_number| (cluster_number as u16)
                    .to_le_bytes()
                    .to_vec())
                .collect::<Vec<Vec<u8>>>()
                .concat(),
            Bit::Fat32 => cluster_chain
                .into_iter()
                .map(|cluster_number| cluster_number
                    .to_le_bytes()
                    .to_vec())
                .collect::<Vec<Vec<u8>>>()
                .concat(),
        };
        let size: usize = ((bytes.len() + self.sector_size - 1) / self.sector_size) * self.sector_size;
        bytes.resize(size, 0x00);
        bytes
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
                content: _,
            } => Self::Fat12,
            boot_sector::BootSector::Fat16 {
                content: _,
            } => Self::Fat16,
            boot_sector::BootSector::Fat32 {
                content: _,
            } => Self::Fat32,
        }
    }
}

impl Into<usize> for &Bit {
    fn into(self) -> usize {
        match self {
            Bit::Fat12 => 12,
            Bit::Fat16 => 16,
            Bit::Fat32 => 32,
        }
    }
}

