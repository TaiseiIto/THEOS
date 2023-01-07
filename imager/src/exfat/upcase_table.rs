use std::collections::HashMap;

pub struct UpcaseTable {
    map: HashMap<char, char>,
}

impl UpcaseTable {
    pub fn new() -> Self {
        let chars: Vec<(u32, Vec<u8>)> = (0x0000..0xffff)
            .flat_map(|n| std::char::from_u32(n))
            .map(|c| (c, c.to_uppercase().collect::<String>()))
            .map(|(c, u)| (c as u32, u.as_bytes().to_vec()))
            .collect();
        Self {
            map: HashMap::new(),
        }
    }
}

