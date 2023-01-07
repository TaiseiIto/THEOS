use super::*;

#[derive(Clone, Copy, Debug)]
pub struct BootSector {
    jump_boot: [u8; 0x3],
    file_system_name: [char; 0x8],
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
    sector_per_cluster_shift: u8,
    number_of_fats: u8,
    drive_select: u8,
    percent_in_use: u8,
    reserved: [u8; 0x7],
    boot_code: [u8; 0x186],
    boot_signature: u16,
}

impl BootSector {
    pub fn new(file: &path::Path) -> Self {
        let boot_sector: Vec<u8> = fs::read(file).expect(&format!("Failed to open {}", file.display()));
        let boot_sector: [u8; mem::size_of::<PackedBootSector>()] = boot_sector.try_into().expect(&format!("The length of boot sector must be {}.", mem::size_of::<PackedBootSector>()));
        let boot_sector = PackedBootSector::new(boot_sector);
        boot_sector.unpack()
    }
}

impl Sector for BootSector {
    fn to_bytes(&self) -> RawSector {
        self.pack().to_bytes()
    }
}

impl Packable for BootSector {
    type Packed = PackedBootSector;

    fn pack(&self) -> Self::Packed {
        Self::Packed {
            jump_boot: self.jump_boot,
            file_system_name: self.file_system_name.map(|c| c as u8),
            must_be_zero: self.must_be_zero,
            partition_offset: self.partition_offset,
            volume_length: self.volume_length,
            fat_offset: self.fat_offset,
            fat_length: self.fat_length,
            cluster_heap_offset: self.cluster_heap_offset,
            cluster_count: self.cluster_count,
            first_cluster_of_root_directory: self.first_cluster_of_root_directory,
            volume_serial_number: self.volume_serial_number,
            file_system_revision: self.file_system_revision,
            volume_flags: self.volume_flags,
            bytes_per_sector_shift: self.bytes_per_sector_shift,
            sector_per_cluster_shift: self.sector_per_cluster_shift,
            number_of_fats: self.number_of_fats,
            drive_select: self.drive_select,
            percent_in_use: self.percent_in_use,
            reserved: self.reserved,
            boot_code: self.boot_code,
            boot_signature: self.boot_signature,
        }
    }
}

impl fmt::Display for BootSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "boot_sector.jump_boot = {:x?}\n", self.jump_boot)?;
        write!(f, "boot_sector.file_system_name = \"{}\"\n", self.file_system_name.iter().collect::<String>())?;
        write!(f, "boot_sector.must_be_zero = {:x?}\n", self.must_be_zero)?;
        write!(f, "boot_sector.partition_offset = {:#x}\n", self.partition_offset)?;
        write!(f, "boot_sector.volume_length = {:#x}\n", self.volume_length)?;
        write!(f, "boot_sector.fat_offset = {:#x}\n", self.fat_offset)?;
        write!(f, "boot_sector.fat_length = {:#x}\n", self.fat_length)?;
        write!(f, "boot_sector.cluster_heap_offset = {:#x}\n", self.cluster_heap_offset)?;
        write!(f, "boot_sector.cluster_count = {:#x}\n", self.cluster_count)?;
        write!(f, "boot_sector.first_cluster_of_root_directory = {:#x}\n", self.first_cluster_of_root_directory)?;
        write!(f, "boot_sector.volume_serial_number = {:#x}\n", self.volume_serial_number)?;
        write!(f, "boot_sector.file_system_revision = {:#x}\n", self.file_system_revision)?;
        write!(f, "boot_sector.volume_flags = {:#x}\n", self.volume_flags)?;
        write!(f, "boot_sector.bytes_per_sector_shift = {:#x}\n", self.bytes_per_sector_shift)?;
        write!(f, "boot_sector.sector_per_cluster_shift = {:#x}\n", self.sector_per_cluster_shift)?;
        write!(f, "boot_sector.number_of_fats = {:#x}\n", self.number_of_fats)?;
        write!(f, "boot_sector.drive_select = {:#x}\n", self.drive_select)?;
        write!(f, "boot_sector.percent_in_use = {:#x}\n", self.percent_in_use)?;
        write!(f, "boot_sector.reserved = {:x?}\n", self.reserved)?;
        write!(f, "boot_sector.boot_code = {:x?}\n", self.boot_code)?;
        write!(f, "boot_sector.boot_signature = {:#x}", self.boot_signature)
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct PackedBootSector {
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
    sector_per_cluster_shift: u8,
    number_of_fats: u8,
    drive_select: u8,
    percent_in_use: u8,
    reserved: [u8; 0x7],
    boot_code: [u8; 0x186],
    boot_signature: u16,
}

impl PackedBootSector {
    pub fn new(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        unsafe {
            mem::transmute::<[u8; mem::size_of::<Self>()], Self>(bytes)
        }
    }
}

impl Unpackable for PackedBootSector {
    type Unpacked = BootSector;

    fn unpack(&self) -> Self::Unpacked {
        Self::Unpacked {
            jump_boot: self.jump_boot,
            file_system_name: self.file_system_name.map(|byte| char::from(byte)),
            must_be_zero: self.must_be_zero,
            partition_offset: self.partition_offset,
            volume_length: self.volume_length,
            fat_offset: self.fat_offset,
            fat_length: self.fat_length,
            cluster_heap_offset: self.cluster_heap_offset,
            cluster_count: self.cluster_count,
            first_cluster_of_root_directory: self.first_cluster_of_root_directory,
            volume_serial_number: self.volume_serial_number,
            file_system_revision: self.file_system_revision,
            volume_flags: self.volume_flags,
            bytes_per_sector_shift: self.bytes_per_sector_shift,
            sector_per_cluster_shift: self.sector_per_cluster_shift,
            number_of_fats: self.number_of_fats,
            drive_select: self.drive_select,
            percent_in_use: self.percent_in_use,
            reserved: self.reserved,
            boot_code: self.boot_code,
            boot_signature: self.boot_signature,
        }
    }
}

impl Sector for PackedBootSector {
    fn to_bytes(&self) -> RawSector {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}


