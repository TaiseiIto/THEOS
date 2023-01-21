use {
    std::collections::HashMap,
    super::{
        cluster,
        super::binary::Binary,
    },
};

#[derive(Debug)]
pub struct AllocationBitmap {
    bitmap: HashMap<u32, bool>,
}

impl AllocationBitmap {
    pub fn all_clusters_are_used(num_of_clusters: usize) -> Self {
        let num_of_clusters: u32 = num_of_clusters as u32;
        let bitmap: HashMap<u32, bool> = (0..num_of_clusters)
            .map(|n| (n + cluster::FIRST_CLUSTER_NUMBER, true))
            .collect();
        Self {
            bitmap,
        }
    }
}

impl Binary for AllocationBitmap {
    fn to_bytes(&self) -> Vec<u8> {
        let max_cluster_number: u32 = self.bitmap
            .iter()
            .map(|(cluster_number, _)| *cluster_number)
            .max()
            .expect("Can't extract max cluster number.");
        let mut bitmap: Vec<bool> = (0..=max_cluster_number - cluster::FIRST_CLUSTER_NUMBER)
            .map(|_| false)
            .collect();
        for (cluster_number, unavailability) in &self.bitmap {
            bitmap[(*cluster_number - cluster::FIRST_CLUSTER_NUMBER) as usize] = *unavailability;
        }
        let mut bytes: Vec<u8> = (0..(bitmap.len() + 7) / 8)
            .map(|_| 0xff)
            .collect();
        for (i, unavailability) in bitmap.into_iter().enumerate() {
            let byte_offset = i / 8;
            let bit_offset = i % 8;
            bytes[byte_offset] &= (u8::from(unavailability) << bit_offset) | !(1 << bit_offset);
        }
        bytes
    }
}

