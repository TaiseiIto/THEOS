mod attribute;

use {
    std::{
        ffi::OsStr,
        fmt,
        path::PathBuf,
    },
    super::{
        node,
        super::super::time,
    },
};

#[derive(Debug)]
pub enum DirectoryEntry {
    ShortFileName {
        stem: [u8; STEM_LENGTH],
        extension: [u8; EXTENSION_LENGTH],
        attribute: attribute::Attribute,
        accessed_time: time::Time,
        created_time: time::Time,
        written_time: time::Time,
        long_file_name: Option<Box<Self>>,
    },
    LongFileName {
        name: [u16; LONG_FILE_NAME_LENGTH],
        order: usize,
        next: Option<Box<Self>>,
    },
}

const STEM_LENGTH: usize = 8;
const EXTENSION_LENGTH: usize = 3;
const LONG_FILE_NAME_LENGTH: usize = 13;

impl DirectoryEntry {
    fn long_file_name(name: Vec<u16>, order: usize) -> Self {
        let (name, next): ([u16; LONG_FILE_NAME_LENGTH], Option<Box<Self>>) = if LONG_FILE_NAME_LENGTH <= name.len() {
            let (name, next): (&[u16], &[u16]) = name.split_at(LONG_FILE_NAME_LENGTH);
            let name: [u16; LONG_FILE_NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a long file name directory entry.");
            let next: Vec<u16> = next.to_vec();
            let next: Option<Box<Self>> = Some(Box::new(Self::long_file_name(next, order + 1)));
            (name, next)
        } else {
            let mut name: Vec<u16> = name;
            if name.len() < LONG_FILE_NAME_LENGTH {
                name.push(0x0000);
            }
            name.resize(LONG_FILE_NAME_LENGTH, 0xffff);
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
}

impl From<&PathBuf> for DirectoryEntry {
    fn from(path: &PathBuf) -> Self {
        let (stem, stem_is_irreversible, _, _): (String, bool, bool, bool) = path
            .file_stem()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
            .to_uppercase()
            .chars()
            .fold((String::new(), false, false, true), |(stem, stem_is_irreversible, dot_flag, head_flag), c| match c {
                'A' |
                'B' |
                'C' |
                'D' |
                'E' |
                'F' |
                'G' |
                'H' |
                'I' |
                'J' |
                'K' |
                'L' |
                'M' |
                'N' |
                'O' |
                'P' |
                'Q' |
                'R' |
                'S' |
                'T' |
                'U' |
                'V' |
                'W' |
                'X' |
                'Y' |
                'Z' |
                '$' |
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
                    let mut stem: String = stem;
                    stem.push(c);
                    let stem_is_irreversible: bool = stem_is_irreversible;
                    let dot_flag: bool = false;
                    let head_flag: bool = false;
                    (stem, stem_is_irreversible, dot_flag, head_flag)
                },
                ' ' => {
                    let stem: String = stem;
                    let stem_is_irreversible: bool = true;
                    let dot_flag: bool = dot_flag;
                    let head_flag: bool = head_flag;
                    (stem, stem_is_irreversible, dot_flag, head_flag)
                },
                '.' => {
                    if head_flag {
                        let stem: String = stem;
                        let stem_is_irreversible: bool = true;
                        let dot_flag: bool = true;
                        let head_flag: bool = true;
                        (stem, stem_is_irreversible, dot_flag, head_flag)
                    } else {
                        if dot_flag {
                            let stem: String = stem;
                            let stem_is_irreversible: bool = true;
                            let dot_flag: bool = true;
                            let head_flag: bool = head_flag;
                            (stem, stem_is_irreversible, dot_flag, head_flag)
                        } else {
                            let mut stem: String = stem;
                            stem.push('.');
                            let stem_is_irreversible: bool = stem_is_irreversible;
                            let dot_flag: bool = true;
                            let head_flag: bool = head_flag;
                            (stem, stem_is_irreversible, dot_flag, head_flag)
                        }
                    }
                },
                _ => {
                    let mut stem: String = stem;
                    let stem_is_irreversible: bool = true;
                    let dot_flag: bool = false;
                    let head_flag: bool = false;
                    stem.push('_');
                    (stem, stem_is_irreversible, dot_flag, head_flag)
                },
            });
        let mut stem: Vec<u8> = stem.into_bytes();
        let stem_is_irreversible: bool = stem_is_irreversible || STEM_LENGTH < stem.len();
        stem.resize(STEM_LENGTH, ' ' as u8);
        let stem: [u8; STEM_LENGTH] = stem
            .try_into()
            .expect("Can't generate a directory entry.");
        let (extension, extension_is_irreversible): (String, bool) = path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
            .to_uppercase()
            .chars()
            .fold((String::new(), false), |(stem, extension_is_irreversible), c| match c {
                'A' |
                'B' |
                'C' |
                'D' |
                'E' |
                'F' |
                'G' |
                'H' |
                'I' |
                'J' |
                'K' |
                'L' |
                'M' |
                'N' |
                'O' |
                'P' |
                'Q' |
                'R' |
                'S' |
                'T' |
                'U' |
                'V' |
                'W' |
                'X' |
                'Y' |
                'Z' |
                '$' |
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
                    let mut stem: String = stem;
                    stem.push(c);
                    let extension_is_irreversible: bool = extension_is_irreversible;
                    (stem, extension_is_irreversible)
                },
                ' ' => {
                    let stem: String = stem;
                    let extension_is_irreversible: bool = true;
                    (stem, extension_is_irreversible)
                },
                '.' => panic!("Can't generate an extension."),
                _ => {
                    let mut stem: String = stem;
                    stem.push('_');
                    let extension_is_irreversible: bool = true;
                    (stem, extension_is_irreversible)
                },
            });
        let mut extension: Vec<u8> = extension.into_bytes();
        let extension_is_irreversible: bool = extension_is_irreversible || EXTENSION_LENGTH < extension.len();
        extension.resize(EXTENSION_LENGTH, ' ' as u8);
        let extension: [u8; EXTENSION_LENGTH] = extension
            .try_into()
            .expect("Can't generate a directory entry.");
        let attribute: attribute::Attribute = path.into();
        let accessed_time = time::Time::last_accessed_time(path);
        let created_time = time::Time::last_changed_time(path);
        let written_time = time::Time::last_modified_time(path);
        let long_file_name: Option<Box<Self>> = if stem_is_irreversible || extension_is_irreversible {
            let long_file_name: Vec<u16> = path
                .file_name()
                .expect("Can't generate a directory entry.")
                .to_str()
                .expect("Can't generate a directory entry.")
                .encode_utf16()
                .collect();
            let order: usize = 1;
            Some(Box::new(Self::long_file_name(long_file_name, order)))
        } else {
            None
        };
        Self::ShortFileName {
            stem,
            extension,
            attribute,
            accessed_time,
            created_time,
            written_time,
            long_file_name,
        }
    }
}

impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = match self {
            Self::ShortFileName {
                stem,
                extension,
                attribute,
                accessed_time,
                created_time,
                written_time,
                long_file_name,
            } => {
                let stem: Vec<u8> = stem.to_vec();
                let extension: Vec<u8> = extension.to_vec();
                let mut name: Vec<u8> = vec![];
                name.extend(stem);
                name.extend(extension);
                let name = String::from_utf8(name).expect("Can't print a directory entry.");
                let name: String = format!("short file name: {}", name);
                let created_time: String = format!("created time: {}", created_time);
                let written_time: String = format!("written time: {}", written_time);
                let accessed_time: String = format!("accessed time: {}", accessed_time);
                let attribute: String = format!("{}", attribute);
                let long_file_name: String = match long_file_name {
                    Some(long_file_name) => format!("{}", long_file_name.as_ref()),
                    None => String::new(),
                };
                let elements: Vec<String> = vec![
                    name,
                    created_time,
                    written_time,
                    accessed_time,
                    attribute,
                    long_file_name,
                ];
                elements
                    .into_iter()
                    .fold(String::new(), |string, element| string + &element + "\n")
            },
            Self::LongFileName {
                name,
                order,
                next,
            } => {
                let name = String::from_utf16(&name[..]).expect("Can't print a directory entry.");
                let name: String = format!("long file name: {}", name);
                let order: String = format!("order: {}", order);
                let next: String = match next {
                    Some(next) => format!("{}", next.as_ref()),
                    None => String::new(),
                };
                let elements: Vec<String> = vec![
                    name,
                    order,
                    next,
                ];
                elements
                    .into_iter()
                    .fold(String::new(), |string, element| string + &element + "\n")
            }
        };
        write!(f, "{}", string)
    }
}

