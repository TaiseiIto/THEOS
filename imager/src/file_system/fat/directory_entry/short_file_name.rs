use {
    std::convert::From,
    super::DirectoryEntry,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct ShortFileName {
    name: [u8; NAME_LENGTH],
    attribute: u8,
    name_flags: u8,
    created_time_centi_second: u8,
    created_time: u32,
    accessed_date: u16,
    first_cluster_high: u16,
    written_time: u32,
    first_cluster_low: u16,
    file_size: u32,
}

const BASENAME_LENGTH: usize = 8;
const EXTENSION_LENGTH: usize = 3;
pub const NAME_LENGTH: usize = BASENAME_LENGTH + EXTENSION_LENGTH;

impl From<&DirectoryEntry> for ShortFileName {
    fn from(directory_entry: &DirectoryEntry) -> Self {
        if let DirectoryEntry::ShortFileName {
            name,
            attribute,
            accessed_time,
            created_time,
            written_time,
            first_cluster,
            size,
            long_file_name,
        } = directory_entry {
            let (name, irreversible, _, _): (String, bool, bool, bool) = name
                .chars()
                .fold((String::new(), false, false, true), |(name, irreversible, dot_flag, head_flag), c| match c {
                    'a' | 'A' |
                    'b' | 'B' |
                    'c' | 'C' |
                    'd' | 'D' |
                    'e' | 'E' |
                    'f' | 'F' |
                    'g' | 'G' |
                    'h' | 'H' |
                    'i' | 'I' |
                    'j' | 'J' |
                    'k' | 'K' |
                    'l' | 'L' |
                    'm' | 'M' |
                    'n' | 'N' |
                    'o' | 'O' |
                    'p' | 'P' |
                    'q' | 'Q' |
                    'r' | 'R' |
                    's' | 'S' |
                    't' | 'T' |
                    'u' | 'U' |
                    'v' | 'V' |
                    'w' | 'W' |
                    'x' | 'X' |
                    'y' | 'Y' |
                    'z' | 'Z' |
                    '0' |
                    '1' |
                    '2' |
                    '3' |
                    '4' |
                    '5' |
                    '6' |
                    '7' |
                    '8' |
                    '9' |
                    '$' |
                    '%' |
                    '\'' |
                    '-' |
                    '_' |
                    '@' |
                    '~' |
                    '`' |
                    '!' |
                    '(' |
                    ')' |
                    '{' |
                    '}' |
                    '^' |
                    '#' |
                    '&' => {
                        let mut name: String = name;
                        if dot_flag && !head_flag {
                            name.push('.');
                        }
                        name.push(c);
                        (name, irreversible, false, false)
                    },
                    ' ' => (name, true, false, false),
                    '.' => {
                        (name, true, true, head_flag)
                    },
                    _ => {
                        let mut name: String = name;
                        name.push('_');
                        (name, true, false, false)
                    },
                });
            let mut name: Vec<String> = name
                .split(".")
                .map(|name| name.to_string())
                .collect();
            let (basename, extension): (String, String) = match name.pop() {
                Some(extension) => {
                    let base_name: String = name
                        .iter()
                        .fold(String::new(), |base_name, name| base_name + name);
                    match base_name.len() {
                        0 => (extension, "".to_string()),
                        _ => (base_name, extension),
                    }
                },
                None => ("".to_string(), "".to_string()),
            };
            let mut basename: Vec<u8> = basename.into_bytes();
            basename.resize(BASENAME_LENGTH, 0x20);
            let mut extension: Vec<u8> = extension.into_bytes();
            extension.resize(EXTENSION_LENGTH, 0x20);
            let name: Vec<u8> = [basename, extension].concat();
            let name: [u8; NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a short file name.");
            let attribute: u8 = attribute.into();
            let name_flags: u8 = 0;
            let created_time_centi_second: u8 = created_time.fat_centi_second();
            let created_time: u32 = created_time.fat_timestamp();
            let accessed_date: u16 = (accessed_time.fat_timestamp() >> 16) as u16;
            let first_cluster_high: u16 = (first_cluster >> 16) as u16;
            let written_time: u32 = written_time.fat_timestamp();
            let first_cluster_low: u16 = *first_cluster as u16;
            let file_size: u32 = *size as u32;
            Self {
                name,
                attribute,
                name_flags,
                created_time_centi_second,
                created_time,
                accessed_date,
                first_cluster_high,
                written_time,
                first_cluster_low,
                file_size,
            }
        } else {
            panic!("Can't generate a short file name.");
        }
    }
}

