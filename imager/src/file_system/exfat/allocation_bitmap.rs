use {
    std::{
        collections::HashMap,
        fmt,
    },
    super::cluster,
};

#[derive(Debug)]
pub struct AllocationBitmap {
    bitmap: HashMap<u32, bool>,
}

impl AllocationBitmap {
    pub fn map(&self) -> &HashMap<u32, bool> {
        &self.bitmap
    }

    pub fn new(clusters: &cluster::Clusters) -> Self {
        let bitmap: HashMap<u32, bool> = clusters.used_flags();
        Self {
            bitmap,
        }
    }

    pub fn read(bytes: Vec<u8>, num_of_clusters: usize) -> Self {
        let bitmap: Vec<bool> = bytes
            .into_iter()
            .map(|byte| (0..8).map(move |bit_offset| byte & (1 << bit_offset) != 0))
            .flatten()
            .collect();
        let bitmap: Vec<bool> = bitmap[0..num_of_clusters].to_vec();
        let bitmap: HashMap<u32, bool> = bitmap
            .into_iter()
            .enumerate()
            .map(|(i, bit)| (i as u32 + cluster::FIRST_CLUSTER_NUMBER, bit))
            .collect();
        Self {
            bitmap,
        }
    }
}

impl Into<Vec<u8>> for &AllocationBitmap {
    fn into(self) -> Vec<u8> {
        let max_cluster_number: u32 = self.bitmap
            .iter()
            .map(|(cluster_number, _)| *cluster_number)
            .max()
            .expect("Can't extract max cluster number.");
        let mut bitmap: Vec<bool> = (0..=max_cluster_number - cluster::FIRST_CLUSTER_NUMBER)
            .map(|_| false)
            .collect();
        self.bitmap
            .iter()
            .for_each( |(cluster_number, unavailability)| bitmap[(*cluster_number - cluster::FIRST_CLUSTER_NUMBER) as usize] = *unavailability);
        let mut bytes: Vec<u8> = (0..(bitmap.len() + 7) / 8)
            .map(|_| 0xff)
            .collect();
        bitmap
            .into_iter()
            .enumerate()
            .for_each(|(i, unavailability)| {
                let byte_offset = i / 8;
                let bit_offset = i % 8;
                bytes[byte_offset] &= (u8::from(unavailability) << bit_offset) | !(1 << bit_offset);
            });
        bytes
    }
}

impl fmt::Display for AllocationBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bitmap: Vec<(u32, bool)> = self.bitmap
            .iter()
            .map(|(cluster, used)| (*cluster, *used))
            .collect();
        bitmap.sort_by(|(left_cluster, _), (right_cluster, _)| left_cluster.partial_cmp(right_cluster).unwrap());
        let bitmap: String = bitmap
            .into_iter()
            .map(|(cluster, used)| format!("cluster[{:#010x}]: {}", cluster, if used {
                "in use"
            } else {
                "available"
            }))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", bitmap)
    }
}

