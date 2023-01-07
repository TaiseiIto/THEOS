use std::{
    fmt,
    mem,
};
use super::{
        Sector,
        Packable,
        Unpackable,
    };

#[derive(Clone, Copy, Debug)]
pub struct ExtendedBootSector {
    boot_code: [u8; 0x1fc],
    boot_signature: u32,
}

impl ExtendedBootSector {
    pub fn new() -> Self {
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
    fn to_bytes(&self) -> super::RawSector {
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
pub struct PackedExtendedBootSector {
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
    fn to_bytes(&self) -> super::RawSector {
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

