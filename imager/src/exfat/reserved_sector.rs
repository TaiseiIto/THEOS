use {
    super::super::binary::Binary,
    std::fmt,
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

    pub fn read(bytes: &Vec<u8>) -> Self {
        let size: usize = bytes.len();
        Self {
            size,
        }
    }
}

impl Binary for ReservedSector {
    fn to_bytes(&self) -> Vec<u8> {
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

