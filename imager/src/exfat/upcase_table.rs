use std::collections::HashMap;
use super::Sectors;

pub struct UpcaseTable {
    map: HashMap<char, String>,
}

impl UpcaseTable {
    pub fn new() -> Self {
        Self {
            map: (0x0000..0xffff)
                .flat_map(|n| std::char::from_u32(n))
                .map(|c| (c, c.to_uppercase().collect()))
                .collect(),
        }
    }
}

impl Sectors for UpcaseTable {
    fn to_bytes(&self) -> Vec<super::RawSector> {
        let bytes: Vec<(u16, Vec<u8>)> = self.map
            .iter()
            .map(|(c, u)| (*c as u16, u.as_bytes().to_vec()))
            .filter(|(_, u)| 2 < u.len())
            .collect();
        vec![]
    }
}

