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
    oem_parameter_sector: OemParameterSector,
    reserved_sector: ReservedSector,
    boot_checksum_sector: Option<BootChecksumSector>,
}

impl Exfat {
    fn new(boot_sector: &path::Path, src: &path::Path) -> Self {
        let boot_sector = BootSector::new(&boot_sector);
        Self {
            boot_sector,
            extended_boot_sectors: [ExtendedBootSector::new(); 0x8],
            oem_parameter_sector: OemParameterSector::null_parameters(),
            reserved_sector: ReservedSector::new(),
            boot_checksum_sector: None,
        }.checksum()
    }

    fn checksum(self) -> Self {
        let boot_checksum_sector = BootChecksumSector::new(&self);
        Self {
            boot_sector: self.boot_sector,
            extended_boot_sectors: self.extended_boot_sectors,
            oem_parameter_sector: self.oem_parameter_sector,
            reserved_sector: self.reserved_sector,
            boot_checksum_sector: Some(boot_checksum_sector),
        }
    }

    fn dump(self, dst_file: &path::Path) {
        let dst_file_name: String = dst_file.display().to_string();
        fs::write(dst_file, self.to_bytes()).expect(&format!("Can't create a new file {}.", dst_file_name));
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut sectors: Vec<Box<dyn Sector>> = vec![];
        for _ in 0..2 {
            sectors.push(Box::new(self.boot_sector));
            for extended_boot_sector in self.extended_boot_sectors {
                sectors.push(Box::new(extended_boot_sector));
            }
            sectors.push(Box::new(self.oem_parameter_sector));
            sectors.push(Box::new(self.reserved_sector));
            if let Some(boot_checksum_sector) = self.boot_checksum_sector {
                sectors.push(Box::new(boot_checksum_sector));
            } else {
                panic!("Can't convert ExFAT into bytes.");
            }
        }
        sectors.into_iter().map(|sector| sector.to_bytes().to_vec()).flatten().collect()
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
        let oem_parameter_sector = format!("{}", self.oem_parameter_sector);
        let oem_parameter_sector = oem_parameter_sector.replace("oem_parameter_sector", "exfat.oem_parameter_sector");
        write!(f, "{}\n", oem_parameter_sector)?;
        let reserved_sector = format!("{}", self.reserved_sector);
        let reserved_sector = reserved_sector.replace("reserved_sector", "exfat.reserved_sector");
        write!(f, "{}\n", reserved_sector)?;
        if let Some(ref boot_checksum_sector) = self.boot_checksum_sector {
            let boot_checksum_sector = format!("{}", boot_checksum_sector);
            let boot_checksum_sector = boot_checksum_sector.replace("boot_checksum_sector", "exfat.boot_checksum_sector");
            write!(f, "{}\n", boot_checksum_sector)?;
        }
        write!(f, "")
    }
}

type RawSector = [u8; 0x200];

trait Sector {
    fn to_bytes(&self) -> RawSector;
}

trait Packable {
    type Packed;
    fn pack(&self) -> Self::Packed;
}

trait Unpackable {
    type Unpacked;
    fn unpack(&self) -> Self::Unpacked;
}

#[derive(Clone, Copy, Debug)]
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
}

impl Packable for ExtendedBootSector {
    type Packed = PackedExtendedBootSector;

    fn pack(&self) -> Self::Packed {
        Self::Packed {
            boot_code: self.boot_code,
            boot_signature: self.boot_signature,
        }
    }
}

impl Sector for ExtendedBootSector {
    fn to_bytes(&self) -> RawSector {
        self.pack().to_bytes()
    }
}

impl fmt::Display for ExtendedBootSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "extended_boot_sector.boot_code = {:x?}\n", self.boot_code)?;
        write!(f, "extended_boot_sector.boot_signature = {:x?}", self.boot_signature)
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct PackedExtendedBootSector {
    boot_code: [u8; 0x1fc],
    boot_signature: u32,
}

impl Unpackable for PackedExtendedBootSector {
    type Unpacked = ExtendedBootSector;

    fn unpack(&self) -> Self::Unpacked {
        Self::Unpacked {
            boot_code: self.boot_code,
            boot_signature: self.boot_signature,
        }
    }
}

impl Sector for PackedExtendedBootSector {
    fn to_bytes(&self) -> RawSector {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}

impl fmt::Display for PackedExtendedBootSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
struct OemParameterSector {
    parameters: [OemParameter; 0xa],
    reserved: [u8; 0x20],
}

impl OemParameterSector {
    fn null_parameters() -> Self {
        Self {
            parameters: [OemParameter::null_parameter(); 0xa],
            reserved: [0; 0x20],
        }
    }
}

impl Packable for OemParameterSector {
    type Packed = PackedOemParameterSector;

    fn pack(&self) -> Self::Packed {
        Self::Packed {
            parameters: self.parameters.map(|parameter| parameter.pack()),
            reserved: self.reserved,
        }
    }
}

impl Sector for OemParameterSector {
    fn to_bytes(&self) -> RawSector {
        self.pack().to_bytes()
    }
}

impl fmt::Display for OemParameterSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, parameter) in self.parameters.iter().enumerate() {
            let parameter = format!("{}", parameter);
            let parameter = parameter.replace("oem_parameter", &format!("opem_parameters.oem_parameter[{}]", i));
            write!(f, "{}\n", parameter)?;
        }
        write!(f, "oem_parameter_sector.reserved = {:x?}", self.reserved)
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct PackedOemParameterSector {
    parameters: [PackedOemParameter; 0xa],
    reserved: [u8; 0x20],
}

impl Unpackable for PackedOemParameterSector {
    type Unpacked = OemParameterSector;

    fn unpack(&self) -> Self::Unpacked {
        Self::Unpacked {
            parameters: self.parameters.map(|parameter| parameter.unpack()),
            reserved: self.reserved,
        }
    }
}

impl Sector for PackedOemParameterSector {
    fn to_bytes(&self) -> RawSector {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}

impl fmt::Display for PackedOemParameterSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
struct OemParameter {
    parameters_guid: [u8; 0x10],
    custom_defined: [u8; 0x20],
}

impl OemParameter {
    fn null_parameter() -> Self {
        Self {
            parameters_guid: [0; 0x10],
            custom_defined: [0; 0x20],
        }
    }
}

impl Packable for OemParameter {
    type Packed = PackedOemParameter;

    fn pack(&self) -> Self::Packed {
        Self::Packed {
            parameters_guid: self.parameters_guid,
            custom_defined: self.custom_defined,
        }
    }
}

impl fmt::Display for OemParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "oem_parameter.parameters_guid = {:x?}\n", self.parameters_guid)?;
        write!(f, "oem_parameter.custom_defined = {:x?}", self.custom_defined)
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct PackedOemParameter {
    parameters_guid: [u8; 0x10],
    custom_defined: [u8; 0x20],
}

impl Unpackable for PackedOemParameter {
    type Unpacked = OemParameter;

    fn unpack(&self) -> Self::Unpacked {
        Self::Unpacked {
            parameters_guid: self.parameters_guid,
            custom_defined: self.custom_defined,
        }
    }
}

impl fmt::Display for PackedOemParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
struct ReservedSector {
    bytes: [u8; mem::size_of::<RawSector>()],
}

impl ReservedSector {
    fn new() -> Self {
        Self {
            bytes: [0; mem::size_of::<RawSector>()],
        }
    }
}

impl Sector for ReservedSector {
    fn to_bytes(&self) -> RawSector {
        self.bytes
    }
}

impl fmt::Display for ReservedSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "reserved_sector.bytes = {:x?}", self.bytes)
    }
}

#[derive(Clone, Copy, Debug)]
struct BootChecksumSector {
    checksum: [u32; mem::size_of::<RawSector>() / mem::size_of::<u32>()],
}

impl BootChecksumSector {
    fn new(exfat: &Exfat) -> Self {
        let mut sectors: Vec<Box<dyn Sector>> = vec![];
        sectors.push(Box::new(exfat.boot_sector));
        for extended_boot_sector in exfat.extended_boot_sectors {
            sectors.push(Box::new(extended_boot_sector));
        }
        sectors.push(Box::new(exfat.oem_parameter_sector));
        sectors.push(Box::new(exfat.reserved_sector));
        let checksum: u32 = sectors
            .into_iter()
            .map(|sector| sector.to_bytes().to_vec())
            .flatten()
            .enumerate()
            .filter(|(i, _)| match i {
                106 | 107 | 112 => false,
                _ => true,
            })
            .map(|(_, byte)| byte)
            .fold(0 as u32, |checksum, byte| match checksum & 1 {
                1 => 0x80000000,
                0 => 0,
                _ => panic!("Can't create checksum sector."),
            } + (checksum >> 1) + (byte as u32));
        Self {
            checksum: [checksum; 0x80],
        }
    }
}

impl Sector for BootChecksumSector {
    fn to_bytes(&self) -> RawSector {
        unsafe {
            mem::transmute::<Self, RawSector>(*self)
        }
    }
}

impl fmt::Display for BootChecksumSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "boot_checksum_sector.checksum = {:x?}", self.checksum)
    }
}

