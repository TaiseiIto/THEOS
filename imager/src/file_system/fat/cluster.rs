use {
    std::collections::{
        HashMap,
        VecDeque,
    },
    super::super::super::binary::Binary,
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

    pub fn append_available_cluster(&mut self) {
        let cluster = Cluster::available_cluster(self);
        self.clusters.push(cluster);
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

    pub fn cluster_chain_bytes(&self, first_cluster_number: u32) -> Vec<u8> {
        match self.clusters
            .iter()
            .find(|cluster| cluster.cluster_number == first_cluster_number) {
            Some(first_cluster) => first_cluster.cluster_chain_bytes(),
            None => vec![],
        }
    }

    pub fn fix_size(&mut self, size: usize) {
        while self.number_of_clusters() * self.cluster_size < size {
            self.append_available_cluster();
        }
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

    pub fn number_of_used_clusters(&self) -> usize {
        self.clusters
            .iter()
            .map(|cluster| cluster.number_of_used_clusters())
            .sum()
    }

    pub fn used_flags(&self) -> HashMap<u32, bool> {
        self.clusters
            .iter()
            .filter_map(|cluster| match cluster.used {
                Some(used) => Some((cluster.cluster_number, used)),
                None => None,
            })
            .collect()
    }

    fn cluster_bytes(&self, cluster_number: u32) -> Option<Vec<u8>> {
        self.clusters
            .iter()
            .find_map(|cluster| cluster.bytes(cluster_number))
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
            .map(|cluster_number| match self.cluster_bytes(cluster_number) {
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
    next_cluster: Option<Box<Self>>,
    used: Option<bool>
}

impl Cluster {
    fn available_cluster(clusters: &mut Clusters) -> Self {
        let blank: u8 = 0x00;
        let cluster: Vec<u8> = (0..clusters.cluster_size)
            .map(|_| blank)
            .collect();
        let mut cluster: Self = Self::new(clusters, &cluster, blank).expect("Can't create an available cluster.");
        cluster.used = Some(false);
        cluster
    }

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

    fn cluster_chain_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.bytes.clone();
        let mut tail: Vec<u8> = match self.next_cluster {
            Some(ref next_cluster) => next_cluster.cluster_chain_bytes(),
            None => vec![],
        };
        bytes.append(&mut tail);
        bytes
    }

    fn bytes(&self, cluster_number: u32) -> Option<Vec<u8>> {
        if cluster_number == self.cluster_number {
            Some(self.bytes.clone())
        } else {
            match &self.next_cluster {
                Some(next_cluster) => next_cluster.bytes(cluster_number),
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
        let used: Option<bool> = Some(true);
        Some(Self {
            cluster_number,
            bytes,
            next_cluster,
            used,
        })
    }

    fn number_of_clusters(&self) -> usize {
        match &self.next_cluster {
            Some(next_cluster) => 1 + next_cluster.number_of_clusters(),
            None => 1,
        }
    }

    fn number_of_used_clusters(&self) -> usize {
        let used_cluster: usize = match &self.used {
            Some(true) => 1,
            _ => 0,
        };
        let following_used_cluster: usize = match &self.next_cluster {
            Some(next_cluster) => next_cluster.number_of_used_clusters(),
            None => 0,
        };
        used_cluster + following_used_cluster
    }

    fn read(mut clusters: VecDeque<(u32, Vec<u8>)>) -> Self {
        let (cluster_number, bytes): (u32, Vec<u8>) = clusters.remove(0).expect("Can't read a cluster.");
        let next_cluster: Option<Box<Self>> = match clusters.len() {
            0 => None,
            _ => Some(Box::new(Self::read(clusters))),
        };
        let used: Option<bool> = None;
        Self {
            cluster_number,
            bytes,
            next_cluster,
            used
        }
    }
}

