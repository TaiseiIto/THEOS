use std::{
    char,
    collections::HashMap,
    fmt,
};

#[derive(Clone, Debug)]
pub struct UpcaseTable {
    map: HashMap<u16, u16>,
}

impl UpcaseTable {
    pub fn new() -> Self {
        let map: HashMap<u16, u16> = (0x0000..=0xffff)
            .filter_map(|lower| {
                let upper: [u16; 1] = [lower];
                let upper: String = char::decode_utf16(upper.iter().cloned())
                    .filter_map(|upper| upper.ok())
                    .collect();
                let upper: String = upper.to_uppercase();
                let upper: Vec<u16> = upper.encode_utf16().collect();
                if upper.len() == 1 {
                    match upper.get(0) {
                        Some(upper) => Some((lower, *upper)),
                        None => None,
                    }
                } else {
                    None
                }
            })
            .collect();
        Self {
            map,
        }
    }

    pub fn table_checksum(&self) -> u32 {
        Into::<Vec<u8>>::into(self)
            .into_iter()
            .fold(0 as u32, |checksum, byte| (checksum << 15) + (checksum >> 1) + byte as u32)
    }

    pub fn capitalize_char(&self, c: u16) -> u16 {
        match self.map.get(&c) {
            Some(upcase) => *upcase,
            None => c,
        }
    }

    pub fn capitalize_str(&self, string: &str) -> String {
        let string: Vec<u16> = string
            .encode_utf16()
            .map(|c| self.capitalize_char(c))
            .collect();
        char::decode_utf16(string)
            .filter_map(|c| c.ok())
            .collect()
    }
}

impl From<&Vec<u8>> for UpcaseTable {
    fn from(map: &Vec<u8>) -> Self {
        let map: HashMap<u16, u16> = map
            .chunks(2)
            .map(|pair| match pair {
                [lower_byte, upper_byte] => ((*upper_byte as u16) << 8) + (*lower_byte as u16),
                _ => panic!("Can't read an upcase table."),
            })
            .fold((vec![], 0u16, false), |(mut map, next_c, compressed), next_word| if compressed {
                let mut uncompressed: Vec<(u16, u16)> = (next_c..next_c + next_word)
                    .map(|c| (c, c))
                    .collect();
                map.append(&mut uncompressed);
                let next_c: u16 = next_c + next_word;
                let compressed: bool = false;
                (map, next_c, compressed)
            } else {
                match next_word {
                    0xffff => {
                        let compressed = true;
                        (map, next_c, compressed)
                    },
                    next_word => {
                        map.push((next_c, next_word));
                        let next_c: u16 = next_c + 1;
                        let compressed = false;
                        (map, next_c, compressed)
                    },
                }
            })
            .0
            .into_iter()
            .collect();
        Self {
            map
        }
    }
}

impl Into<Vec<u8>> for &UpcaseTable {
    fn into(self) -> Vec<u8> {
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

impl fmt::Display for UpcaseTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map: Vec<(u16, u16)> = self.map
            .iter()
            .map(|(lower, upper)| (*lower, *upper))
            .collect();
        map.sort_by(|(left, _), (right, _)| left.partial_cmp(right).expect("Can't print an upcase table."));
        let map: String = map
            .iter()
            .filter_map(|(lower, upper)| {
                let lower: [u16; 1] = [*lower];
                let lower: String = char::decode_utf16(lower.iter().cloned())
                    .filter_map(|lower| lower.ok())
                    .collect();
                let upper: [u16; 1] = [*upper];
                let upper: String = char::decode_utf16(upper.iter().cloned())
                    .filter_map(|upper| upper.ok())
                    .collect();
                if 0 < lower.len() && 0 < upper.len() {
                    Some(format!("map[\"{}\"]=\"{}\"\n", lower, upper))
                } else {
                    None
                }
            })
            .fold(String::new(), |map, line| map + &line);
        write!(f, "{}", map)
    }
}

