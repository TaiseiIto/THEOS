use {
    std::{
        fs,
        mem,
        path,
    },
    super::{
        cluster,
        fat,
        object,
        super::{
            binary::Binary,
            time,
        },
    },
};

#[allow(dead_code)]
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
    pub fn bytes_per_sector(&self) -> usize {
        1 << self.bytes_per_sector_shift
    }

    pub fn cluster_count(&self) -> u32 {
        self.cluster_count
    }

    pub fn cluster_heap_offset(&self) -> u32 {
        self.cluster_heap_offset
    }

    pub fn cluster_size(&self) -> usize {
        self.bytes_per_sector() * self.sectors_per_cluster()
    }

    pub fn correct(self, fat: &fat::Fat, root_directory: &object::Object, clusters: &cluster::Clusters) -> Self {
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
        let volume_length: u64 = ((cluster_heap_offset as usize) + (cluster_count as usize) * self.sectors_per_cluster()) as u64;
        let first_cluster_of_root_directory: u32 = root_directory.first_cluster();
        let volume_serial_number: u32 = time::Time::current_time().unix_timestamp() as u32;
        let file_system_revision: u16 = self.file_system_revision;
        let volume_flags: u16 = self.volume_flags;
        let bytes_per_sector_shift: u8 = self.bytes_per_sector_shift;
        let sectors_per_cluster_shift: u8 = self.sectors_per_cluster_shift;
        let drive_select: u8 = self.drive_select;
        let percent_in_use: u8 = self.percent_in_use;
        let reserved: [u8; 0x7] = self.reserved;
        let boot_code: [u8; 0x186] = self.boot_code;
        let boot_signature: u16 = self.boot_signature;
        Self {
            jump_boot,
            file_system_name,
            must_be_zero,
            partition_offset,
            volume_length,
            fat_offset,
            fat_length,
            cluster_heap_offset,
            cluster_count,
            first_cluster_of_root_directory,
            volume_serial_number,
            file_system_revision,
            volume_flags,
            bytes_per_sector_shift,
            sectors_per_cluster_shift,
            num_of_fats,
            drive_select,
            percent_in_use,
            reserved,
            boot_code,
            boot_signature,
        }
    }

    pub fn fat_length(&self) -> u32 {
        self.fat_length
    }

    pub fn fat_offset(&self) -> u32 {
        self.fat_offset
    }

    pub fn first_cluster_of_root_directory(&self) -> u32 {
        self.first_cluster_of_root_directory
    }

    pub fn new(boot_sector: path::PathBuf) -> Self {
        let boot_sector: Vec<u8> = fs::read(&boot_sector).expect(&format!("Can't read {}!", boot_sector.display()));
        Self::read(&boot_sector)
    }

    pub fn num_of_fats(&self) -> usize {
        self.num_of_fats as usize
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        const SIZE: usize = mem::size_of::<BootSector>();
        let boot_sector = &bytes[0..SIZE];
        let boot_sector: [u8; SIZE] = boot_sector.try_into().expect("Can't convert boot sector from Vec<u8> to [u8; SIZE]!");
        unsafe {
            mem::transmute::<[u8; SIZE], Self>(boot_sector)
        }
    }

    pub fn sectors_per_cluster(&self) -> usize {
        1 << self.sectors_per_cluster_shift
    }
}

impl Binary for BootSector {
    fn to_bytes(&self) -> Vec<u8> {
        let boot_sector: [u8; mem::size_of::<Self>()] = unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        };
        let mut boot_sector: Vec<u8> = boot_sector.to_vec();
        boot_sector.resize(self.bytes_per_sector(), 0x00);
        boot_sector
    }
}

