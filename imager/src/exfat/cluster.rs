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

	pub fn append(&mut self, mut bytes: Vec<u8>) -> u32 {
		let cluster = Cluster::new(self, bytes);
		let cluster_number: u32 = cluster.cluster_number;
		self.clusters.push(cluster);
		cluster_number
	}
}

#[derive(Debug)]
struct Cluster {
	cluster_number: u32,
	bytes: Vec<u8>,
	next_cluster: Option<Box<Cluster>>,
}

impl Cluster {
	fn new(clusters: &mut Clusters, mut bytes: Vec<u8>) -> Self {
		let cluster_number: u32 = clusters.next_cluster_number;
		clusters.next_cluster_number += 1;
		let remaining_bytes: Vec<u8> = bytes.split_off(clusters.cluster_size);
		bytes.resize(clusters.cluster_size, 0x00);
		let next_cluster: Option<Box<Cluster>> = if 0 < remaining_bytes.len() {
			Some(Box::new(Self::new(clusters, remaining_bytes)))
		} else {
			None
		};
		Self {
			cluster_number,
			bytes,
			next_cluster,
		}
	}
}

