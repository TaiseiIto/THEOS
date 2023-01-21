use super::super::binary::Binary;

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

impl Binary for ReservedSector {
    fn to_bytes(&self) -> Vec<u8> {
        (0..self.size)
            .map(|_| 0)
            .collect()
    }
}

