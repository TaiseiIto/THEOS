use {
    std::collections::HashMap,
    super::{
        cluster,
        directory_entry,
    },
};

#[derive(Debug)]
pub struct AllocationBitmap {
    bitmap: HashMap<u32, bool>,
}

impl AllocationBitmap {
    pub fn allocation_bitmap(clusters: &mut cluster::Clusters, root_directory_entries: &Vec<&directory_entry::DirectoryEntry>, num_of_fats: usize) {
        let num_of_clusters: usize = clusters.len();
        let cluster_size: usize = clusters.cluster_size();
        let bits_per_cluster: usize = 8 * cluster_size;
        let allocation_bitmap_directory_entry: usize = 1;
        let num_of_root_directory_entries: usize = root_directory_entries.len() + allocation_bitmap_directory_entry;
        let size_of_root_directory_entries: usize = directory_entry::DIRECTORY_ENTRY_SIZE * num_of_root_directory_entries;
        let num_of_clusters_of_root_directory: usize = (size_of_root_directory_entries + cluster_size - 1) / cluster_size;
        let num_of_clusters: usize = num_of_clusters + num_of_clusters_of_root_directory;
        let mut num_of_allocation_bitmap_clusters: usize = 0;
        loop {
            let num_of_clusters: usize = num_of_clusters + num_of_fats * num_of_allocation_bitmap_clusters;
            let necessary_allocation_bitmap_clusters: usize = (num_of_clusters + bits_per_cluster - 1) / bits_per_cluster;
            if necessary_allocation_bitmap_clusters <= num_of_allocation_bitmap_clusters {
                break;
            }
            num_of_allocation_bitmap_clusters += 1;
        }
        let num_of_clusters: usize = num_of_clusters + num_of_fats * num_of_allocation_bitmap_clusters;
        let allocation_bitmaps: Vec<Self> = (0..num_of_fats)
            .map(|_| Self::all_clusters_are_used(num_of_clusters))
            .collect();
        let allocation_bitmaps: Vec<Vec<u8>> = allocation_bitmaps
            .into_iter()
            .map(|allocation_bitmap| allocation_bitmap.to_bytes())
            .collect();
        let allocation_bitmap_clusters: Vec<u32> = allocation_bitmaps
            .into_iter()
            .map(|allocation_bitmap| clusters.append(allocation_bitmap, 0xff))
            .collect();
        println!("num_of_clusters = {}", num_of_clusters);
        println!("allocation_bitmap_clusters = {:?}", allocation_bitmap_clusters);
    }

    pub fn all_clusters_are_used(num_of_clusters: usize) -> Self {
        let num_of_clusters: u32 = num_of_clusters as u32;
        let bitmap: HashMap<u32, bool> = (0..num_of_clusters)
            .map(|n| (n + cluster::FIRST_CLUSTER_NUMBER, true))
            .collect();
        Self {
            bitmap,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
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
        for (i, unavailability) in bitmap.iter().enumerate() {
            let byte_offset = i / 8;
            let bit_offset = i % 8;
            bytes[byte_offset] &= (u8::from(*unavailability) << bit_offset) | !(1 << bit_offset);
        }
        bytes
    }
}

