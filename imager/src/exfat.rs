mod boot_checksum_sector;
mod boot_sector;
mod extended_boot_sector;
mod object;
mod oem_parameter_sector;
mod reserved_sector;
mod upcase_table;

use std::{
    fmt,
    fs,
    path,
};

#[derive(Debug)]
pub struct Exfat {
    boot_sector: boot_sector::BootSector,
    extended_boot_sectors: [extended_boot_sector::ExtendedBootSector; 0x8],
    oem_parameter_sector: oem_parameter_sector::OemParameterSector,
    reserved_sector: reserved_sector::ReservedSector,
    boot_checksum_sector: Option<boot_checksum_sector::BootChecksumSector>,
    upcase_table: upcase_table::UpcaseTable,
    object: object::Object,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, src: path::PathBuf) -> Self {
        let boot_sector = boot_sector::BootSector::new(boot_sector);
        Self {
            boot_sector,
            extended_boot_sectors: [extended_boot_sector::ExtendedBootSector::new(); 0x8],
            oem_parameter_sector: oem_parameter_sector::OemParameterSector::null_parameters(),
            reserved_sector: reserved_sector::ReservedSector::new(),
            boot_checksum_sector: None,
            upcase_table: upcase_table::UpcaseTable::new(),
            object: object::Object::new(src),
        }.checksum()
    }

    fn checksum(self) -> Self {
        let boot_checksum_sector = boot_checksum_sector::BootChecksumSector::new(&self);
        Self {
            boot_sector: self.boot_sector,
            extended_boot_sectors: self.extended_boot_sectors,
            oem_parameter_sector: self.oem_parameter_sector,
            reserved_sector: self.reserved_sector,
            boot_checksum_sector: Some(boot_checksum_sector),
            upcase_table: self.upcase_table,
            object: self.object,
        }
    }

    pub fn dump(self, dst_file: path::PathBuf) {
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
        sectors.append(&mut self.upcase_table.to_sectors());
        sectors.into_iter().map(|sector| sector.to_bytes().to_vec()).flatten().collect()
    }
}

impl fmt::Display for Exfat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let boot_sector = format!("{}", self.boot_sector)
            .replace("boot_sector", "exfat.boot_sector");
        write!(f, "{}\n", boot_sector)?;
        for extended_boot_sector in self.extended_boot_sectors {
            let extended_boot_sector = format!("{}", extended_boot_sector)
                .replace("extended_boot_sector", "exfat.extended_boot_sector");
            write!(f, "{}\n", extended_boot_sector)?;
        }
        let oem_parameter_sector = format!("{}", self.oem_parameter_sector)
            .replace("oem_parameter_sector", "exfat.oem_parameter_sector");
        write!(f, "{}\n", oem_parameter_sector)?;
        let reserved_sector = format!("{}", self.reserved_sector)
            .replace("reserved_sector", "exfat.reserved_sector");
        write!(f, "{}\n", reserved_sector)?;
        if let Some(ref boot_checksum_sector) = self.boot_checksum_sector {
            let boot_checksum_sector = format!("{}", boot_checksum_sector)
                .replace("boot_checksum_sector", "exfat.boot_checksum_sector");
            write!(f, "{}\n", boot_checksum_sector)?;
        }
        write!(f, "{}\n", self.upcase_table)?;
        write!(f, "{}", self.object)
    }
}

type RawSector = [u8; 0x200];

trait Sector {
    fn to_bytes(&self) -> RawSector;
}

impl Sector for RawSector {
    fn to_bytes(&self) -> RawSector {
        *self
    }
}

trait Sectors {
    fn to_sectors(&self) -> Vec<Box<dyn Sector>>;
}

trait Packable {
    type Packed;
    fn pack(&self) -> Self::Packed;
}

trait Unpackable {
    type Unpacked;
    fn unpack(&self) -> Self::Unpacked;
}

