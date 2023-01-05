use std::env;
use std::fmt;
use std::fs;
use std::mem;
use std::path;

fn main() {
    let args = Args::new(env::args());
    println!("{}", args);
    let exfat = Exfat::new(
        path::Path::new(&args.boot_sector),
        path::Path::new(&args.src),
    );
    println!("{}", exfat);
    exfat.dump(path::Path::new(&args.dst));
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

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arg.boot_sector = {}\n", self.boot_sector)?;
        write!(f, "arg.src = {}\n", self.src)?;
        write!(f, "arg.dst = {}", self.dst)
    }
}

#[derive(Debug)]
struct Exfat {
    boot_sector: BootSector,
    extended_boot_sectors: [ExtendedBootSector; 0x8],
    oem_parameters: OemParameters,
}

impl Exfat {
    fn new(boot_sector: &path::Path, src: &path::Path) -> Self {
        let boot_sector = BootSector::new(&boot_sector);
        Self {
            boot_sector,
            extended_boot_sectors: [ExtendedBootSector::new(); 0x8],
            oem_parameters: OemParameters::null_parameters(),
        }
    }

    fn dump(self, dst_file: &path::Path) {
        let dst_file_name: String = dst_file.display().to_string();
        fs::write(dst_file, self.into_bytes()).expect(&format!("Can't create a new file {}.", dst_file_name));
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.boot_sector.into_bytes();
        for extended_boot_sector in self.extended_boot_sectors {
            bytes.append(&mut extended_boot_sector.into_bytes());
        }
        bytes.append(&mut self.oem_parameters.into_bytes());
        bytes
    }
}

impl fmt::Display for Exfat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let boot_sector = format!("{}", self.boot_sector);
        let boot_sector = boot_sector.replace("boot_sector", "exfat.boot_sector");
        write!(f, "{}\n", boot_sector)?;
        for extended_boot_sector in self.extended_boot_sectors {
            let extended_boot_sector = format!("{}", extended_boot_sector);
            let extended_boot_sector = extended_boot_sector.replace("extended_boot_sector", "exfat.extended_boot_sector");
            write!(f, "{}\n", extended_boot_sector)?;
        }
        let oem_parameters = format!("{}", self.oem_parameters);
        let oem_parameters = oem_parameters.replace("oem_parameters", "exfat.oem_parameters");
        write!(f, "{}", oem_parameters)
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

#[derive(Clone, Copy, Debug)]
struct ExtendedBootSector {
    boot_code: [u8; 0x1fc],
    boot_signature: u32,
}

impl ExtendedBootSector {
    fn new() -> Self {
        Self {
            boot_code: [0; 0x1fc],
            boot_signature: 0xaa550000,
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.pack().into_bytes().to_vec()
    }

    fn pack(self) -> PackedExtendedBootSector {
        PackedExtendedBootSector {
            boot_code: self.boot_code,
            boot_signature: self.boot_signature,
        }
    }
}

impl fmt::Display for ExtendedBootSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "extended_boot_sector.boot_code = {:x?}\n", self.boot_code)?;
        write!(f, "extended_boot_sector.boot_signature = {:x?}", self.boot_signature)
    }
}

#[repr(packed)]
struct PackedExtendedBootSector {
    boot_code: [u8; 0x1fc],
    boot_signature: u32,
}

impl PackedExtendedBootSector {
    fn into_bytes(self) -> [u8; mem::size_of::<Self>()] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(self)
        }
    }
}

#[derive(Debug)]
struct OemParameters {
    parameters: [OemParameter; 0xa],
    reserved: [u8; 0x20],
}

impl OemParameters {
    fn null_parameters() -> Self {
        Self {
            parameters: [OemParameter::null_parameter(); 0xa],
            reserved: [0; 0x20],
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.pack().into_bytes().to_vec()
    }

    fn pack(self) -> PackedOemParameters {
        PackedOemParameters {
            parameters: self.parameters.map(|parameter| parameter.pack()),
            reserved: self.reserved,
        }
    }
}

impl fmt::Display for OemParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, parameter) in self.parameters.iter().enumerate() {
            let parameter = format!("{}", parameter);
            let parameter = parameter.replace("oem_parameter", &format!("opem_parameters.oem_parameter[{}]", i));
            write!(f, "{}\n", parameter)?;
        }
        write!(f, "oem_parameters.reserved = {:x?}", self.reserved)
    }
}

#[repr(packed)]
struct PackedOemParameters {
    parameters: [PackedOemParameter; 0xa],
    reserved: [u8; 0x20],
}

impl PackedOemParameters {
    fn into_bytes(self) -> [u8; mem::size_of::<Self>()] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(self)
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct OemParameter {
    parameters_guid: u16,
    custom_defined: u32,
}

impl OemParameter {
    fn null_parameter() -> Self {
        Self {
            parameters_guid: 0x1234,
            custom_defined: 0x12345678,
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.pack().into_bytes().to_vec()
    }

    fn pack(self) -> PackedOemParameter {
        PackedOemParameter {
            parameters_guid: self.parameters_guid,
            custom_defined: self.custom_defined,
        }
    }
}

impl fmt::Display for OemParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "oem_parameter.parameters_guid = {:#x}\n", self.parameters_guid)?;
        write!(f, "oem_parameter.custom_defined = {:#x}", self.custom_defined)
    }
}

#[repr(packed)]
struct PackedOemParameter {
    parameters_guid: u16,
    custom_defined: u32,
}

impl PackedOemParameter {
    fn into_bytes(self) -> [u8; mem::size_of::<Self>()] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(self)
        }
    }
}

