use {
    std::collections::HashMap,
    super::super::binary::Binary,
};

#[derive(Debug)]
pub struct UpcaseTable {
    map: HashMap<u16, u16>,
}

impl UpcaseTable {
    pub fn new() -> Self {
        let map: HashMap<u16, u16> = (0x0000..=0xffff)
            .filter_map(|n| std::char::from_u32(n))
            .map(|c| (c as u16, c
                .to_uppercase()
                .to_string()
                .as_bytes()
                .to_vec()))
            .filter(|(_, u)| u.len() <= 2)
            .filter_map(|(c, u)| {
                let mut i = u.into_iter();
                match i.next() {
                    Some(lower_byte) => match i.next() {
                        Some(higher_byte) => Some((c, ((higher_byte as u16) << 8) + lower_byte as u16)),
                        None => Some((c, lower_byte as u16)),
                    },
                    None => None,
                }
            })
            .collect();
        Self {
            map,
        }
    }

    pub fn table_checksum(&self) -> u32 {
        self
            .to_bytes()
            .into_iter()
            .fold(0 as u32, |checksum, byte| (checksum << 15) + (checksum >> 1) + byte as u32)
    }

    pub fn to_upcase(&self, c: u16) -> u16 {
        match self.map.get(&c) {
            Some(upcase) => *upcase,
            None => c,
        }
    }
}

impl Binary for UpcaseTable {
    fn to_bytes(&self) -> Vec<u8> {
        let mut map: Vec<(u16, u16)> = self.map
            .iter()
            .filter(|(c, u)| c != u)
            .map(|(c, u)| (*c, *u))
            .collect();
        map.sort_by(|(left, _), (right, _)| left.partial_cmp(&right).expect("Can't convert upcase table into bytes!"));
        let (mut words, last_c): (Vec<u16>, u16) = map
            .into_iter()
            .fold((vec![], 0), |(words, last_c), (c, u)| {
                let mut words: Vec<u16> = words;
                if last_c + 1 < c {
                    words.push(0xffff);
                    words.push(c - last_c);
                }
                words.push(u);
                (words, c)
            });
        if last_c != 0xffff {
            words.push(0xffff);
            words.push(0 - last_c);
        }
        words
            .into_iter()
            .map(|w| vec![w as u8, (w >> 8) as u8])
            .flatten()
            .collect()
    }
}

