use std::{
    fs,
    mem,
    path,
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

    pub fn cluster_size(&self) -> usize {
        let bytes_per_sector: usize = 1 << self.bytes_per_sector_shift;
        let sectors_per_cluster: usize = 1 << self.sectors_per_cluster_shift;
        bytes_per_sector * sectors_per_cluster
    }
}

