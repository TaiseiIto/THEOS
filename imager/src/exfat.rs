use std::fmt;
use std::fs;
use std::mem;
use std::path;

mod boot_sector;
mod extended_boot_sector;
mod oem_parameter_sector;

#[derive(Debug)]
pub struct Exfat {
    boot_sector: boot_sector::BootSector,
    extended_boot_sectors: [extended_boot_sector::ExtendedBootSector; 0x8],
    oem_parameter_sector: oem_parameter_sector::OemParameterSector,
    reserved_sector: ReservedSector,
    boot_checksum_sector: Option<BootChecksumSector>,
}

impl Exfat {
    pub fn new(boot_sector: &path::Path, src: &path::Path) -> Self {
        let boot_sector = boot_sector::BootSector::new(&boot_sector);
        Self {
            boot_sector,
            extended_boot_sectors: [extended_boot_sector::ExtendedBootSector::new(); 0x8],
            oem_parameter_sector: oem_parameter_sector::OemParameterSector::null_parameters(),
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

    pub fn dump(self, dst_file: &path::Path) {
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

