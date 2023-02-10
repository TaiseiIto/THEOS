mod attribute;
mod short_file_name;

use {
    std::convert::From,
    super::{
        node,
        super::super::time,
    },
};

#[derive(Debug)]
pub enum DirectoryEntry {
    ShortFileName {
        name: [u8; NAME_LENGTH],
        attribute: attribute::Attribute,
        accessed_time: time::Time,
        created_time: time::Time,
        written_time: time::Time,
        first_cluster: u32,
        size: usize,
        long_file_name: Option<Box<Self>>,
    },
    LongFileName {
        name: [u16; LONG_FILE_NAME_LENGTH],
        order: usize,
        next: Option<Box<Self>>,
    },
}

const BASENAME_LENGTH: usize = 8;
const EXTENSION_LENGTH: usize = 3;
pub const NAME_LENGTH: usize = BASENAME_LENGTH + EXTENSION_LENGTH;
const LONG_FILE_NAME_LENGTH: usize = 13;

impl DirectoryEntry {
    fn long_file_name(mut name: Vec<u16>, order: usize) -> Self {
        let (name, next): ([u16; LONG_FILE_NAME_LENGTH], Option<Box<Self>>) = if LONG_FILE_NAME_LENGTH <= name.len() {
            let (name, next): (&[u16], &[u16]) = name.split_at(LONG_FILE_NAME_LENGTH);
            let name: [u16; LONG_FILE_NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a long file name directory entry.");
            let next: Vec<u16> = next.to_vec();
            let next: Option<Box<Self>> = Some(Box::new(Self::long_file_name(next, order + 1)));
            (name, next)
        } else {
            name.resize(LONG_FILE_NAME_LENGTH, 0x0000);
            let name: [u16; LONG_FILE_NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a long file name directory entry.");
            let next: Option<Box<Self>> = None;
            (name, next)
        };
        Self::LongFileName {
            name,
            order,
            next,
        }
    }

    fn short_file_name(name: String) -> ([u8; NAME_LENGTH], bool) {
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
        let mut extension: Vec<u8> = extension.into_bytes();
        let irreversible: bool = irreversible || BASENAME_LENGTH < basename.len() || EXTENSION_LENGTH < extension.len();
        if irreversible {
            basename.resize(BASENAME_LENGTH - 2, 0x20);
            basename.push('~' as u8);
            basename.push('1' as u8);
        } else {
            basename.resize(BASENAME_LENGTH, 0x20);
        }
        extension.resize(EXTENSION_LENGTH, 0x20);
        let name: Vec<u8> = [basename, extension].concat();
        let name: [u8; NAME_LENGTH] = name
            .try_into()
            .expect("Can't generate a short file name.");
        (name, irreversible)
    }
}

impl From<&node::Node> for DirectoryEntry {
    fn from(node: &node::Node) -> Self {
        let name: String = node.name();
        let long_file_name: Vec<u16> = name.encode_utf16().collect();
        let (name, irreversible): ([u8; NAME_LENGTH], bool) = Self::short_file_name(name);
        let long_file_name: Option<Box<Self>> = if irreversible {
            Some(Box::new(Self::long_file_name(long_file_name, 1)))
        } else {
            None
        };
        let attribute = attribute::Attribute::from(node);
        let accessed_time: time::Time = node.last_accessed_time();
        let created_time: time::Time = node.last_changed_time();
        let written_time: time::Time = node.last_modified_time();
        let first_cluster: u32 = node.first_cluster();
        let size: usize = if node.is_directory() {
            0
        } else {
            node.size()
        };
        Self::ShortFileName {
            name,
            attribute,
            accessed_time,
            created_time,
            written_time,
            first_cluster,
            size,
            long_file_name,
        }
    }
}

