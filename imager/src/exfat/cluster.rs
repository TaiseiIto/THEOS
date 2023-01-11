#[derive(Debug)]
pub struct Clusters {
	cluster_size: usize,
	clusters: Vec<Cluster>,
	next_cluster_number: u32,
}

impl Clusters {
	pub fn new(cluster_size: usize) -> Self {
		let clusters: Vec<Cluster> = vec![];
		let next_cluster_number: u32 = 2;
		Self {
			cluster_size,
			clusters,
			next_cluster_number,
		}
	}
}

#[derive(Debug)]
struct Cluster {
	cluster_number: u32,
	bytes: Vec<u8>,
	next_cluster: Option<Box<Cluster>>,
}

