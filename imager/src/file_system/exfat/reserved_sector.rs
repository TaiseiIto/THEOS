use std::{
    convert::{
        From,
        Into,
    },
    fmt,
};

#[derive(Debug)]
pub struct ReservedSector {
    size: usize,
}

impl ReservedSector {
    pub fn new(size: usize) -> Self {
        Self {
            size,
        }
    }
}

impl From<&Vec<u8>> for ReservedSector {
    fn from(bytes: &Vec<u8>) -> Self {
        let size: usize = bytes.len();
        Self {
            size,
        }
    }
}

impl Into<Vec<u8>> for &ReservedSector {
    fn into(self) -> Vec<u8> {
        (0..self.size)
            .map(|_| 0)
            .collect()
    }
}

impl fmt::Display for ReservedSector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size: {:#x}", self.size)
    }
}

