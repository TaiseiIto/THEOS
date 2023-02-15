use {
    std::{
        convert::{
            From,
            Into,
        },
        fmt,
        fs,
        mem,
        path::PathBuf,
        str,
    },
    super::{
        cluster,
        fat,
        node,
        super::super::time,
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

    pub fn correct(self, fat: &fat::Fat, root_directory: &node::Node, clusters: &cluster::Clusters) -> Self {
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
        let number_of_clusters: usize = clusters.number_of_clusters();
        let number_of_used_clusters: usize = clusters.number_of_used_clusters();
        let percent_in_use: f64 = 100f64 * number_of_used_clusters as f64 / number_of_clusters as f64;
        let percent_in_use: u8 = percent_in_use.floor() as u8;
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

    pub fn new(boot_sector: &PathBuf) -> Self {
        let boot_sector: Vec<u8> = fs::read(boot_sector).expect(&format!("Can't read {}!", boot_sector.display()));
        Self::from(&boot_sector)
    }

    pub fn num_of_fats(&self) -> usize {
        self.num_of_fats as usize
    }

    pub fn sectors_per_cluster(&self) -> usize {
        1 << self.sectors_per_cluster_shift
    }
}

impl From<&Vec<u8>> for BootSector {
    fn from(bytes: &Vec<u8>) -> Self {
        const SIZE: usize = mem::size_of::<BootSector>();
        let boot_sector = &bytes[0..SIZE];
        let boot_sector: [u8; SIZE] = boot_sector.try_into().expect("Can't convert boot sector from Vec<u8> to [u8; SIZE]!");
        let boot_sector: Self = unsafe {
            mem::transmute::<[u8; SIZE], Self>(boot_sector)
        };
        let validation: bool = boot_sector.must_be_zero
            .iter()
            .map(|byte| *byte == 0)
            .fold(true, |entire_validation, byte_validation| entire_validation && byte_validation);
        if !validation {
            panic!("The boot sector is broken.");
        }
        boot_sector
    }
}

impl Into<Vec<u8>> for &BootSector {
    fn into(self) -> Vec<u8> {
        let boot_sector: [u8; mem::size_of::<BootSector>()] = unsafe {
            mem::transmute::<BootSector, [u8; mem::size_of::<BootSector>()]>(*self)
        };
        let mut boot_sector: Vec<u8> = boot_sector.to_vec();
        boot_sector.resize(self.bytes_per_sector(), 0x00);
        boot_sector
    }
}

impl fmt::Display for BootSector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let jump_boot: String = "jump_boot: ".to_string() + &self.jump_boot
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ");
        let file_system_name: String = format!("file_system_name: \"{}\"", str::from_utf8(&self.file_system_name).expect("Can't print a boot sector."));
        let must_be_zero: String = "must_be_zero: ".to_string() + &self.must_be_zero
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ");
        let partition_offset: u64 = self.partition_offset;
        let partition_offset: String = format!("partition_offset: {:#018x}", partition_offset);
        let volume_length: u64 = self.volume_length;
        let volume_length: String = format!("volume_length: {:#018x}", volume_length);
        let fat_offset: u32 = self.fat_offset;
        let fat_offset: String = format!("fat_offset: {:#010x}", fat_offset);
        let fat_length: u32 = self.fat_length;
        let fat_length: String = format!("fat_length: {:#010x}", fat_length);
        let cluster_heap_offset: u32 = self.cluster_heap_offset;
        let cluster_heap_offset: String = format!("cluster_heap_offset: {:#010x}", cluster_heap_offset);
        let cluster_count: u32 = self.cluster_count;
        let cluster_count: String = format!("cluster_count: {:#010x}", cluster_count);
        let first_cluster_of_root_directory: u32 = self.first_cluster_of_root_directory;
        let first_cluster_of_root_directory: String = format!("first_cluster_of_root_directory: {:#010x}", first_cluster_of_root_directory);
        let volume_serial_number: u32 = self.volume_serial_number;
        let volume_serial_number = time::Time::from_unix_timestamp(volume_serial_number as i128);
        let volume_serial_number: String = format!("volume_serial_number: {}", volume_serial_number);
        let file_system_revision: u16 = self.file_system_revision;
        let file_system_revision: String = format!("file_system_revision: {:#06x}", file_system_revision);
        let volume_flags: u16 = self.volume_flags;
        let volume_flags: String = format!("volume_flags: {:#06x}", volume_flags);
        let bytes_per_sector_shift: u8 = self.bytes_per_sector_shift;
        let bytes_per_sector_shift: String = format!("bytes_per_sector_shift: {:#010x}", bytes_per_sector_shift);
        let sectors_per_cluster_shift: u8 = self.sectors_per_cluster_shift;
        let sectors_per_cluster_shift: String = format!("sectors_per_cluster_shift: {:#010x}", sectors_per_cluster_shift);
        let num_of_fats: u8 = self.num_of_fats;
        let num_of_fats: String = format!("num_of_fats: {:#010x}", num_of_fats);
        let drive_select: u8 = self.drive_select;
        let drive_select: String = format!("drive_select: {:#010x}", drive_select);
        let percent_in_use: u8 = self.percent_in_use;
        let percent_in_use: String = format!("percent_in_use: {:#04x}", percent_in_use);
        let reserved: String = "reserved: ".to_string() + &self.reserved
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ");
        let boot_code: String = "boot_code: ".to_string() + &self.boot_code
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ");
        let boot_signature: u16 = self.boot_signature;
        let boot_signature: String = format!("boot_signature: {:#06x}", boot_signature);
        let boot_sector: Vec<String> = vec![
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
        ];
        let boot_sector: String = boot_sector.join("\n");
        write!(f, "{}", boot_sector)
    }
}

