extern crate regex;

mod boot_checksum_sector;
mod boot_sector;
mod directory_entry;
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
    boot_checksum_sector: boot_checksum_sector::BootChecksumSector,
    upcase_table: upcase_table::UpcaseTable,
    object: object::Object,
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, src: path::PathBuf) -> Self {
        let upcase_table = upcase_table::UpcaseTable::new();
        let object = object::Object::new(&src);
        let boot_sector = boot_sector::BootSector::new(boot_sector);
        let extended_boot_sectors = [extended_boot_sector::ExtendedBootSector::new(); 0x8];
        let oem_parameter_sector = oem_parameter_sector::OemParameterSector::null_parameters();
        let reserved_sector = reserved_sector::ReservedSector::new();
        let boot_checksum_sector = boot_checksum_sector::BootChecksumSector::new(
            &boot_sector,
            &extended_boot_sectors,
            &oem_parameter_sector,
            &reserved_sector,
        );
        Self {
            boot_sector,
            extended_boot_sectors,
            oem_parameter_sector,
            reserved_sector,
            boot_checksum_sector,
            upcase_table,
            object,
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
            sectors.push(Box::new(self.boot_checksum_sector));
        }
        sectors.append(&mut self.upcase_table.to_sectors());
        sectors.into_iter().map(|sector| sector.to_bytes().to_vec()).flatten().collect()
    }
}

impl fmt::Display for Exfat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        let boot_sector: String = format!("{}\n", self.boot_sector);
        string += &boot_sector;
        for extended_boot_sector in self.extended_boot_sectors {
            let extended_boot_sector: String = format!("{}\n", extended_boot_sector);
            string += &extended_boot_sector;
        }
        let oem_parameter_sector: String = format!("{}\n", self.oem_parameter_sector);
        string += &oem_parameter_sector;
        let reserved_sector: String = format!("{}\n", self.reserved_sector);
        string += &reserved_sector;
        let boot_checksum_sector: String = format!("{}\n", self.boot_checksum_sector);
        string += &boot_checksum_sector;
        let upcase_table: String = format!("{}\n", self.upcase_table);
        string += &upcase_table;
        let object: String = format!("{}", self.object);
        string += &object;
        let regex = regex::Regex::new("^|\n").expect("Can't create a Regex.");
        let string: String = regex.replace_all(&string, "$0exfat.");
        write!(f, "{}", string)
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

