use std::env;
use std::fs;
use std::mem;
use std::path;

fn main() {
    let args = Args::new(env::args());
    let exfat = Exfat::new(args);
}

#[derive(Debug)]
struct Args {
    boot_sector: String,
    src: String,
    dst: String,
}

impl Args {
    fn new(mut args: env::Args) -> Self {
        let usage: String = String::from("Usage: $ ./imager /path/to/boot/sector /path/to/source/directory /path/to/destination");
        let _my_path: String = args.next().expect(&format!("{}\n{}\n", "Program path is not specified.", usage));
        let boot_sector: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
        let src: String = args.next().expect(&format!("{}\n{}\n", "Source directory is not specified.", usage));
        let dst: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
        Self {
            boot_sector,
            src,
            dst,
        }
    }
}

#[derive(Debug)]
struct Exfat {
    boot_sector: BootSector,
}

impl Exfat {
    fn new(args: Args) -> Self {
        let boot_sector = path::Path::new(&args.boot_sector);
        let src = path::Path::new(&args.src);
        let dst = path::Path::new(&args.dst);
        println!("src = {}", src.display());
        println!("dst = {}", dst.display());
        let boot_sector = BootSector::new(&boot_sector);
        println!("boot_sector.jump_boot = {:x?}", boot_sector.jump_boot);
        println!("boot_sector.file_system_name = \"{}\"", boot_sector.file_system_name.iter().collect::<String>());
        println!("boot_sector.must_be_zero = {:x?}", boot_sector.must_be_zero);
        println!("boot_sector.partition_offset = {:#x}", boot_sector.partition_offset);
        println!("boot_sector.volume_length = {:#x}", boot_sector.volume_length);
        println!("boot_sector.fat_offset = {:#x}", boot_sector.fat_offset);
        println!("boot_sector.fat_length = {:#x}", boot_sector.fat_length);
        println!("boot_sector.cluster_heap_offset = {:#x}", boot_sector.cluster_heap_offset);
        println!("boot_sector.cluster_count = {:#x}", boot_sector.cluster_count);
        println!("boot_sector.first_cluster_of_root_directory = {:#x}", boot_sector.first_cluster_of_root_directory);
        println!("boot_sector.volume_serial_number = {:#x}", boot_sector.volume_serial_number);
        println!("boot_sector.file_system_revision = {:#x}", boot_sector.file_system_revision);
        println!("boot_sector.volume_flags = {:#x}", boot_sector.volume_flags);
        println!("boot_sector.bytes_per_sector_shift = {:#x}", boot_sector.bytes_per_sector_shift);
        println!("boot_sector.sector_per_cluster_shift = {:#x}", boot_sector.sector_per_cluster_shift);
        println!("boot_sector.number_of_fats = {:#x}", boot_sector.number_of_fats);
        println!("boot_sector.drive_select = {:#x}", boot_sector.drive_select);
        println!("boot_sector.percent_in_use = {:#x}", boot_sector.percent_in_use);
        println!("boot_sector.reserved = {:x?}", boot_sector.reserved);
        println!("boot_sector.boot_code = {:x?}", boot_sector.boot_code);
        println!("boot_sector.boot_signature = {:#x}", boot_sector.boot_signature);
        Self {
            boot_sector,
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.boot_sector.into_bytes()
    }
}

#[derive(Debug)]
struct BootSector {
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
    fn new(file: &path::Path) -> Self {
        let boot_sector: Vec<u8> = fs::read(file).expect(&format!("Failed to open {}", file.display()));
        let boot_sector: [u8; mem::size_of::<PackedBootSector>()] = boot_sector.try_into().expect(&format!("The length of boot sector must be {}.", mem::size_of::<PackedBootSector>()));
        let boot_sector = PackedBootSector::new(boot_sector);
        boot_sector.unpack()
    }

    fn into_bytes(self) -> Vec<u8> {
        self.pack().into_bytes().to_vec()
    }

    fn pack(self) -> PackedBootSector {
        PackedBootSector {
            jump_boot: self.jump_boot,
            file_system_name: self.file_system_name
                .iter()
                .map(|c| *c as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .expect("Can't interpret FileSystemName as [u8; 0x8]"),
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

#[repr(packed)]
struct PackedBootSector {
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
    fn new(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        unsafe {
            mem::transmute::<[u8; mem::size_of::<Self>()], Self>(bytes)
        }
    }

    fn into_bytes(self) -> [u8; mem::size_of::<Self>()] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(self)
        }
    }

    fn unpack(self) -> BootSector {
        BootSector {
            jump_boot: self.jump_boot,
            file_system_name: self.file_system_name
                .iter()
                .map(|byte| char::from(*byte))
                .collect::<Vec<char>>()
                .try_into()
                .expect("Can't interpret FileSystemName as [char; 0x8]"),
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

