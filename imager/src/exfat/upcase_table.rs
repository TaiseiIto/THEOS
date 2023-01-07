use std::collections::HashMap;
use super::Sectors;

pub struct UpcaseTable {
    map: HashMap<char, String>,
}

impl UpcaseTable {
    pub fn new() -> Self {
        Self {
            map: (0x0000..0xffff)
                .filter_map(|n| std::char::from_u32(n))
                .map(|c| (c, c.to_uppercase().collect()))
                .collect(),
        }
    }
}

impl Sectors for UpcaseTable {
    fn to_bytes(&self) -> Vec<super::RawSector> {
        let bytes: Vec<(u16, u16)> = self.map
            .iter()
            .map(|(c, u)| (*c as u16, u.as_bytes().to_vec()))
            .filter(|(_, u)| 2 < u.len())
            .filter_map(|(c, u)| {
                let mut i = u.iter();
                match i.next() {
                    Some(lower_byte) => match i.next() {
                        Some(upper_byte) => Some((c, ((*upper_byte as u16) << 8) + *lower_byte as u16)),
                        None => Some((c, *lower_byte as u16)),
                    },
                    None => None,
                }
            })
            .collect();
        vec![]
    }
}

