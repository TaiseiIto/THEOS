use std::{
    fmt,
    fs,
    path,
};

mod boot_checksum_sector;
mod boot_sector;
mod extended_boot_sector;
mod oem_parameter_sector;
mod reserved_sector;
mod upcase_table;

#[derive(Debug)]
pub struct Exfat {
    boot_sector: boot_sector::BootSector,
    extended_boot_sectors: [extended_boot_sector::ExtendedBootSector; 0x8],
    oem_parameter_sector: oem_parameter_sector::OemParameterSector,
    reserved_sector: reserved_sector::ReservedSector,
    boot_checksum_sector: Option<boot_checksum_sector::BootChecksumSector>,
}

impl Exfat {
    pub fn new(boot_sector: &path::Path, src: &path::Path) -> Self {
        let boot_sector = boot_sector::BootSector::new(&boot_sector);
        Self {
            boot_sector,
            extended_boot_sectors: [extended_boot_sector::ExtendedBootSector::new(); 0x8],
            oem_parameter_sector: oem_parameter_sector::OemParameterSector::null_parameters(),
            reserved_sector: reserved_sector::ReservedSector::new(),
            boot_checksum_sector: None,
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

trait Sectors {
    fn to_bytes(&self) -> Vec<RawSector>;
}

trait Packable {
    type Packed;
    fn pack(&self) -> Self::Packed;
}

trait Unpackable {
    type Unpacked;
    fn unpack(&self) -> Self::Unpacked;
}

