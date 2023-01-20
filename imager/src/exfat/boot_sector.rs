use {
    std::{
        fs,
        mem,
        path,
    },
    super::{
        cluster,
        fat,
    },
};

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct BootSector {
    jump_boot: [u8; 0x3],
    file_system_name: [u8; 0x8],
    must_be_zero: [u8; 0x35],
    partition_offset: u64,
    volume_length: u64,
    fat_offset: u32,
    fat_length: u32,
    cluster_heap_offset: u32,
    cluster_count: u32,
    first_cluster_of_root_directory: u32,
    volume_serial_number: u32,
    file_system_revision: u16,
    volume_flags: u16,
    bytes_per_sector_shift: u8,
    sectors_per_cluster_shift: u8,
    num_of_fats: u8,
    drive_select: u8,
    percent_in_use: u8,
    reserved: [u8; 0x7],
    boot_code: [u8; 0x186],
    boot_signature: u16,
}

impl BootSector {
    pub fn new(boot_sector: path::PathBuf) -> Self {
        const SIZE: usize = mem::size_of::<BootSector>();
        type Bytes = [u8; SIZE];
        let boot_sector: Vec<u8> = fs::read(&boot_sector).expect(&format!("Can't read {}!", boot_sector.display()));
        let boot_sector: Bytes = boot_sector.try_into().expect("Can't convert boot sector from Vec<u8> to Bytes!");
        unsafe {
            mem::transmute::<Bytes, Self>(boot_sector)
        }
    }

    pub fn correct(self, fat: &fat::Fat, clusters: &cluster::Clusters) {
        let jump_boot: [u8; 0x3] = self.jump_boot;
        let file_system_name: [u8; 0x8] = self.file_system_name;
        let must_be_zero: [u8; 0x35] = self.must_be_zero;
        let partition_offset: u64 = self.partition_offset;
        // let volume_length = ;
        let fat_offset: u32 = self.fat_offset;
        let fat_length: u32 = fat.sectors_per_fat() as u32;
        let num_of_fats: u8 = self.num_of_fats;
        let cluster_heap_offset: u32 = (((fat_offset as usize) + (fat_length as usize) * (num_of_fats as usize) + self.sectors_per_cluster() - 1) / self.sectors_per_cluster() * self.sectors_per_cluster()) as u32;
        let cluster_count: u32 = clusters.number_of_clusters() as u32;
    }

    pub fn cluster_size(&self) -> usize {
        self.bytes_per_sector() * self.sectors_per_cluster()
    }

    pub fn bytes_per_sector(&self) -> usize {
        1 << self.bytes_per_sector_shift
    }

    pub fn sectors_per_cluster(&self) -> usize {
        1 << self.sectors_per_cluster_shift
    }

    pub fn num_of_fats(&self) -> usize {
        self.num_of_fats as usize
    }
}

