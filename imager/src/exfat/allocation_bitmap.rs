use {
    std::{
        collections::HashMap,
        fmt,
    },
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
            .map(|(i, bit)| (i as u32, bit))
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

impl fmt::Display for AllocationBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bitmap: String = self.bitmap
            .iter()
            .map(|(cluster, used)| format!("cluster[{:#08x}]: {}\n", cluster, if *used {
                "used"
            } else {
                "available"
            }))
            .fold(String::new(), |bitmap, line| bitmap + &line);
        write!(f, "{}", bitmap)
    }
}

