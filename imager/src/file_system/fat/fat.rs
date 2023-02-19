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

impl Into<usize> for &Bit {
    fn into(self) -> usize {
        match self {
            Bit::Fat12 => 12,
            Bit::Fat16 => 16,
            Bit::Fat32 => 32,
        }
    }
}

