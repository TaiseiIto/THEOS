use {
    std::{
        char,
        collections::HashMap,
        fmt,
        mem,
        str,
    },
    super::super::binary::Binary,
};

#[derive(Debug)]
pub struct UpcaseTable {
    map: HashMap<u16, u16>,
}

impl UpcaseTable {
    pub fn new() -> Self {
        let map: HashMap<u16, u16> = (0x0000..=0xffff)
            .filter_map(|n| char::from_u32(n))
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

    pub fn read(map: Vec<u8>) -> Self {
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

impl fmt::Display for UpcaseTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let map: String = self.map
            .iter()
            .filter_map(|(lower, upper)| {
                let lower: [u8; 2] = unsafe {
                    mem::transmute::<u16, [u8; 2]>(*lower)
                };
                let lower: &[u8] = match lower[1] {
                    0x00 => &lower[..1],
                    _ => &lower[..],
                };
                let lower: Result<&str, str::Utf8Error> = str::from_utf8(lower);
                let upper: [u8; 2] = unsafe {
                    mem::transmute::<u16, [u8; 2]>(*upper)
                };
                let upper: &[u8] = match upper[1] {
                    0x00 => &upper[..1],
                    _ => &upper[..],
                };
                let upper: Result<&str, str::Utf8Error> = str::from_utf8(upper);
                match (lower, upper) {
                    (Ok(lower), Ok(upper)) => Some(format!("map[\"{}\"]=\"{}\"\n", lower, upper)),
                    _ => None,
                }
            })
            .fold(String::new(), |map, line| map + &line);
        write!(f, "{}", map)
    }
}

