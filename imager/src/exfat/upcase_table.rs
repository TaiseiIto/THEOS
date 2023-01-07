use std::{
    collections::HashMap,
    mem,
};
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
        let mut map: Vec<(u16, u16)> = self.map
            .iter()
            .map(|(c, u)| (*c as u16, u.as_bytes().to_vec()))
            .filter(|(_, u)| 2 <= u.len())
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
            .filter(|(c, u)| c != u)
            .collect::<Vec<(u16, u16)>>();
        map.sort_by(|(left, _), (right, _)| left.partial_cmp(&right).unwrap());
        let (mut words, last_c): (Vec<u16>, u16) = map
            .iter()
            .fold((vec![], 0), |(words, last_c), (c, u)| {
                let mut words: Vec<u16> = words;
                if last_c + 1 < *c {
                    words.push(0xffff);
                    words.push(c - last_c);
                }
                words.push(*u);
                (words, *c)
            });
        if last_c != 0xffff {
            words.push(0xffff);
            words.push(0 - last_c);
        }
        let mut bytes: Vec<u8> = words
            .iter()
            .map(|w| vec![*w as u8, (*w >> 8) as u8])
            .flatten()
            .collect();
        while bytes.len() % mem::size_of::<super::RawSector>() != 0 {
            bytes.push(0x0000);
        }
        vec![]
    }
}

