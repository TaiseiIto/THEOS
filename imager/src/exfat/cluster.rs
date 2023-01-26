use {
    std::collections::{
        HashMap,
        VecDeque,
    },
    super::{
        fat,
        super::binary::Binary,
    },
};

pub const FIRST_CLUSTER_NUMBER: u32 = 2;

#[derive(Debug)]
pub struct Clusters {
    cluster_size: usize,
    clusters: Vec<Cluster>,
    next_cluster_number: u32,
}

impl Clusters {
    pub fn append(&mut self, bytes: &Vec<u8>, blank: u8) -> u32 {
        let cluster = match Cluster::new(self, bytes, blank) {
            Some(cluster) => cluster,
            None => return 0,
        };
        let cluster_number: u32 = cluster.cluster_number;
        self.clusters.push(cluster);
        cluster_number
    }

    pub fn cluster_chain(&self) -> HashMap<u32, Option<u32>> {
        self.clusters
            .iter()
            .map(|cluster| cluster.cluster_chain())
            .fold(HashMap::<u32, Option<u32>>::new(), |cluster_chains, cluster_chain| {
                let mut cluster_chains: HashMap<u32, Option<u32>> = cluster_chains;
                cluster_chains.extend(cluster_chain.iter());
                cluster_chains
            })
    }

    pub fn cluster_size(&self) -> usize {
        self.cluster_size
    }

    pub fn len(&self) -> usize {
        self.clusters.len()
    }

    pub fn new(cluster_size: usize) -> Self {
        let clusters: Vec<Cluster> = vec![];
        let next_cluster_number: u32 = FIRST_CLUSTER_NUMBER;
        Self {
            cluster_size,
            clusters,
            next_cluster_number,
        }
    }

    pub fn number_of_clusters(&self) -> usize {
        self.clusters
            .iter()
            .map(|cluster| cluster.number_of_clusters())
            .sum()
    }

    pub fn read(bytes: Vec<u8>, fat: &fat::Fat, cluster_size: usize) {
        let clusters: HashMap<u32, Vec<u8>> = bytes
            .chunks(cluster_size)
            .enumerate()
            .map(|(cluster, bytes)| ((cluster as u32) + FIRST_CLUSTER_NUMBER, bytes.to_vec()))
            .collect();
        let fat: HashMap<u32, Vec<u32>> = fat.to_chains();
        let clusters: Vec<Vec<(u32, Vec<u8>)>> = fat
            .into_iter()
            .map(|(_, cluster_number_chain)| cluster_number_chain
                    .into_iter()
                    .map(|cluster_number| (cluster_number, clusters
                        .get(&cluster_number)
                        .expect("Can't read clusters.")
                        .clone()
                    ))
                    .collect()
            )
            .collect();
        let clusters: Vec<Cluster> = clusters
            .into_iter()
            .map(|clusters| Cluster::read(VecDeque::from(clusters)))
            .collect();
    }

    fn get_cluster(&self, cluster_number: u32) -> Option<Vec<u8>> {
        self.clusters
            .iter()
            .filter_map(|cluster| cluster.get_cluster(cluster_number))
            .next()
    }

    fn max_cluster_number(&self) -> u32 {
        self.clusters
            .iter()
            .map(|cluster| cluster.max_cluster_number())
            .max()
            .expect("Can't get max cluster number.")
    }
}

impl Binary for Clusters {
    fn to_bytes(&self) -> Vec<u8> {
        (FIRST_CLUSTER_NUMBER..=self.max_cluster_number())
            .map(|cluster_number| match self.get_cluster(cluster_number) {
                Some(bytes) => bytes,
                None => (0..self.cluster_size)
                    .map(|_| 0u8)
                    .collect::<Vec<u8>>(),
            })
            .flatten()
            .collect()
    }
}

#[derive(Debug)]
struct Cluster {
    cluster_number: u32,
    bytes: Vec<u8>,
    next_cluster: Option<Box<Cluster>>,
}

impl Cluster {
    fn cluster_chain(&self) -> HashMap<u32, Option<u32>> {
        match &self.next_cluster {
            Some(next_cluster) => {
                let mut cluster_chain: HashMap<u32, Option<u32>> = next_cluster.cluster_chain();
                cluster_chain.insert(self.cluster_number, Some(next_cluster.cluster_number));
                cluster_chain
            },
            None => HashMap::from([(self.cluster_number, None)]),
        }
    }

    fn get_cluster(&self, cluster_number: u32) -> Option<Vec<u8>> {
        if cluster_number == self.cluster_number {
            Some(self.bytes.clone())
        } else {
            match &self.next_cluster {
                Some(next_cluster) => next_cluster.get_cluster(cluster_number),
                None => None,
            }
        }
    }

    fn max_cluster_number(&self) -> u32 {
        match &self.next_cluster {
            Some(next_cluster) => {
                let successor_max_cluster_number: u32 = next_cluster.max_cluster_number();
                if successor_max_cluster_number < self.cluster_number {
                    self.cluster_number
                } else {
                    successor_max_cluster_number
                }
            },
            None => self.cluster_number,
        }
    }

    fn new(clusters: &mut Clusters, bytes: &Vec<u8>, blank: u8) -> Option<Self> {
        let mut bytes: Vec<u8> = bytes.clone();
        if bytes.len() == 0 {
            return None;
        }
        let cluster_number: u32 = clusters.next_cluster_number;
        clusters.next_cluster_number += 1;
        let remaining_bytes: Vec<u8> = if clusters.cluster_size < bytes.len() {
            bytes.split_off(clusters.cluster_size)
        } else {
            vec![]
        };
        bytes.resize(clusters.cluster_size, blank);
        let next_cluster: Option<Box<Cluster>> = match Self::new(clusters, &remaining_bytes, blank) {
            Some(next_cluster) => Some(Box::new(next_cluster)),
            None => None,
        };
        Some(Self {
            cluster_number,
            bytes,
            next_cluster,
        })
    }

    fn number_of_clusters(&self) -> usize {
        match &self.next_cluster {
            Some(next_cluster) => 1 + next_cluster.number_of_clusters(),
            None => 1,
        }
    }

    fn read(mut clusters: VecDeque<(u32, Vec<u8>)>) -> Self {
        let (cluster_number, bytes): (u32, Vec<u8>) = clusters.remove(0).expect("Can't read a cluster.");
        let next_cluster: Option<Box<Self>> = match clusters.len() {
            0 => None,
            _ => Some(Box::new(Self::read(clusters))),
        };
        Self {
            cluster_number,
            bytes,
            next_cluster,
        }
    }
}

