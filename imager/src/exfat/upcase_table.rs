use std::collections::HashMap;

#[derive(Debug)]
pub struct UpcaseTable {
    map: HashMap<u16, u16>,
}

impl UpcaseTable {
    pub fn new() -> Self {
        let map: HashMap<u16, u16> = (0x0000..0xffff)
            .filter_map(|n| std::char::from_u32(n))
            .map(|c| (c as u16, c
                .to_uppercase()
                .to_string()
                .as_bytes()
                .to_vec()))
            .filter(|(_, u)| u.len() <= 2)
            .filter_map(|(c, u)| {
                let mut i = u.iter();
                match i.next() {
                    Some(lower_byte) => match i.next() {
                        Some(higher_byte) => Some((c, ((*higher_byte as u16) << 8) + *lower_byte as u16)),
                        None => Some((c, *lower_byte as u16)),
                    },
                    None => None,
                }
            })
            .collect();
        Self {
            map,
        }
    }

    pub fn to_upcase(&self, c: u16) -> u16 {
        match self.map.get(&c) {
            Some(upcase) => *upcase,
            None => c,
        }
    }
}

