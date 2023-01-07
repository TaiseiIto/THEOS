use std::{
    fmt,
    mem,
};
use super::Sector;

#[derive(Clone, Copy, Debug)]
pub struct ReservedSector {
    bytes: [u8; mem::size_of::<super::RawSector>()],
}

impl ReservedSector {
    pub fn new() -> Self {
        Self {
            bytes: [0; mem::size_of::<super::RawSector>()],
        }
    }
}

impl Sector for ReservedSector {
    fn to_bytes(&self) -> super::RawSector {
        self.bytes
    }
}

impl fmt::Display for ReservedSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "reserved_sector.bytes = {:x?}", self.bytes)
    }
}

