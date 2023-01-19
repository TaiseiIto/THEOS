use {
    std::collections::HashMap,
    super::cluster,
};

#[derive(Debug)]
pub struct Fat {
    cluster_chain: HashMap<u32, Option<u32>>,
}

impl Fat {
    pub fn new(clusters: &cluster::Clusters) -> Self {
        let cluster_chain: HashMap<u32, Option<u32>> = clusters.cluster_chain();
        Self {
            cluster_chain,
        }
    }
}

