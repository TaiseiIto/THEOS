mod attribute;
mod long_file_name;
mod name_flags;
mod short_file_name;

use {
    std::{
        cell::RefCell,
        collections::HashSet,
        ffi::OsStr,
        fmt,
        fs,
        path::PathBuf,
    },
    super::{
        node,
        super::super::time,
    },
};

#[derive(Clone, Debug)]
pub enum DirectoryEntry {
    ShortFileName {
        stem: RefCell<[u8; short_file_name::STEM_LENGTH]>,
        extension: [u8; short_file_name::EXTENSION_LENGTH],
        attribute: attribute::Attribute,
        name_flags: name_flags::NameFlags,
        created_time: time::Time,
        accessed_time: time::Time,
        written_time: time::Time,
        cluster: RefCell<Option<u32>>,
        size: usize,
        long_file_name: Option<Box<Self>>,
    },
    LongFileName {
        name: [u16; long_file_name::LONG_FILE_NAME_LENGTH],
        order: usize,
        next: Option<Box<Self>>,
    },
}

pub const DIRECTORY_ENTRY_SIZE: usize = 32;

impl DirectoryEntry {
    pub fn current_directory_entry(&self) -> Self {
        if let Self::ShortFileName {
            stem: _,
            extension: _,
            attribute,
            name_flags,
            created_time,
            accessed_time,
            written_time,
            cluster,
            size,
            long_file_name: _,
        } = self {
            let mut stem: Vec<u8> = "."
                .as_bytes().to_vec();
            stem.resize(short_file_name::STEM_LENGTH, ' ' as u8);
            let stem: [u8; short_file_name::STEM_LENGTH] = stem
                .try_into()
                .expect("Can't generate a current directory entry.");
            let stem: RefCell<[u8; short_file_name::STEM_LENGTH]> = RefCell::new(stem);
            let mut extension: Vec<u8> = vec![];
            extension.resize(short_file_name::EXTENSION_LENGTH, ' ' as u8);
            let extension: [u8; short_file_name::EXTENSION_LENGTH] = extension
                .try_into()
                .expect("Can't generate a current directory entry.");
            let attribute: attribute::Attribute = *attribute;
            let name_flags: name_flags::NameFlags = *name_flags;
            let created_time: time::Time = *created_time;
            let accessed_time: time::Time = *accessed_time;
            let written_time: time::Time = *written_time;
            let cluster: RefCell<Option<u32>> = cluster.clone();
            let size: usize = *size;
            let long_file_name: Option<Box<Self>> = None;
            Self::ShortFileName {
                stem,
                extension,
                attribute,
                name_flags,
                created_time,
                accessed_time,
                written_time,
                cluster,
                size,
                long_file_name,
            }
        } else {
            panic!("Can't generate a current directory entry.")
        }
    }

    pub fn deduplicate(directory_entries: &Vec<&Self>) {
        let mut duplication: HashSet<[u8; short_file_name::STEM_LENGTH]> = HashSet::new();
        for directory_entry in directory_entries.iter() {
            if let Self::ShortFileName {
                stem,
                extension: _,
                attribute: _,
                name_flags: _,
                created_time: _,
                accessed_time: _,
                written_time: _,
                cluster: _,
                size: _,
                long_file_name: Some(long_file_name),
            } = directory_entry {
                let mut new_stem: [u8; short_file_name::STEM_LENGTH] = stem.borrow().clone();
                eprintln!("duplication = {:?}", duplication);
                while duplication.contains(&new_stem) {
                    let stem = String::from_utf8(new_stem.to_vec()).expect("Can't deduplicate file name stems.");
                    let mut stem: Vec<String> = stem
                        .split('~')
                        .map(|string| string.to_string())
                        .collect();
                    if let Some(suffix) = stem.pop() {
                        let stem: String = stem.join("~");
                        let mut stem: Vec<u8> = stem
                            .as_bytes()
                            .to_vec();
                        let suffix: usize = suffix
                            .parse()
                            .expect("Can't deduplicate file name stems.");
                        let suffix: usize = suffix + 1;
                        let suffix: String = format!("~{}", suffix);
                        let suffix: Vec<u8> = suffix
                            .as_bytes()
                            .to_vec();
                        stem.truncate(short_file_name::STEM_LENGTH - suffix.len());
                        stem.extend(suffix);
                        new_stem = stem
                            .try_into()
                            .expect("Can't deduplicate file name stems.")
                    } else {
                        panic!("Can't deduplicate file name stems.");
                    }
                }
                duplication.insert(new_stem.clone());
                eprintln!("duplication = {:?}", duplication);
                *stem.borrow_mut() = new_stem.clone();
            }
        }
    }

    pub fn parent_directory_entry(&self) -> Self {
        if let Self::ShortFileName {
            stem: _,
            extension: _,
            attribute,
            name_flags,
            created_time,
            accessed_time,
            written_time,
            cluster,
            size,
            long_file_name: _,
        } = self {
            let mut stem: Vec<u8> = ".."
                .as_bytes().to_vec();
            stem.resize(short_file_name::STEM_LENGTH, ' ' as u8);
            let stem: [u8; short_file_name::STEM_LENGTH] = stem
                .try_into()
                .expect("Can't generate a current directory entry.");
            let stem: RefCell<[u8; short_file_name::STEM_LENGTH]> = RefCell::new(stem);
            let mut extension: Vec<u8> = vec![];
            extension.resize(short_file_name::EXTENSION_LENGTH, ' ' as u8);
            let extension: [u8; short_file_name::EXTENSION_LENGTH] = extension
                .try_into()
                .expect("Can't generate a current directory entry.");
            let attribute: attribute::Attribute = *attribute;
            let name_flags: name_flags::NameFlags = *name_flags;
            let created_time: time::Time = *created_time;
            let accessed_time: time::Time = *accessed_time;
            let written_time: time::Time = *written_time;
            let cluster: RefCell<Option<u32>> = cluster.clone();
            let size: usize = *size;
            let long_file_name: Option<Box<Self>> = None;
            Self::ShortFileName {
                stem,
                extension,
                attribute,
                name_flags,
                created_time,
                accessed_time,
                written_time,
                cluster,
                size,
                long_file_name,
            }
        } else {
            panic!("Can't generate a current directory entry.")
        }
    }

    pub fn parent_root_directory_entry() -> Self {
        let stem: [u8; short_file_name::STEM_LENGTH] = [0; short_file_name::STEM_LENGTH];
        let stem: RefCell<[u8; short_file_name::STEM_LENGTH]> = RefCell::new(stem);
        let extension: [u8; short_file_name::EXTENSION_LENGTH] = [0; short_file_name::EXTENSION_LENGTH];
        let attribute = attribute::Attribute::root();
        let name_flags = name_flags::NameFlags::root();
        let created_time = time::Time::from_fat_timestamp(0, 0, 0);
        let accessed_time = time::Time::from_fat_timestamp(0, 0, 0);
        let written_time = time::Time::from_fat_timestamp(0, 0, 0);
        let cluster: u32 = 0;
        let cluster: Option<u32> = Some(cluster);
        let cluster: RefCell<Option<u32>> = RefCell::new(cluster);
        let size: usize = 0;
        let long_file_name: Option<Box<Self>> = None;
        let root_directory_entry = Self::ShortFileName {
            stem,
            extension,
            attribute,
            name_flags,
            created_time,
            accessed_time,
            written_time,
            cluster,
            size,
            long_file_name,
        };
        root_directory_entry.parent_directory_entry()
    }

    pub fn set_cluster(&self, cluster_number: u32) {
        if let Self::ShortFileName {
            stem,
            extension,
            attribute,
            name_flags,
            created_time,
            accessed_time,
            written_time,
            cluster,
            size,
            long_file_name,
        } = self {
            *cluster.borrow_mut() = Some(cluster_number);
        } else {
            panic!("Can't set cluster.");
        }
    }

    pub fn volume_label(volume_label: String) -> Self {
        let volume_label: String = volume_label
            .to_uppercase()
            .chars()
            .filter(|c| match c {
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
                '&' |
                ' ' => true,
                _ => false,
            })
            .collect();
        let mut volume_label: Vec<u8> = volume_label
            .as_bytes()
            .to_vec();
        volume_label.resize(short_file_name::BASENAME_LENGTH, ' ' as u8);
        let volume_label: [u8; short_file_name::BASENAME_LENGTH] = volume_label
            .try_into()
            .expect("Can't generate a volume label.");
        let stem: [u8; short_file_name::STEM_LENGTH] = volume_label[..short_file_name::STEM_LENGTH]
            .try_into()
            .expect("Can't generate a volume label.");
        let stem: RefCell<[u8; short_file_name::STEM_LENGTH]> = RefCell::new(stem);
        let extension: [u8; short_file_name::EXTENSION_LENGTH] = volume_label[short_file_name::STEM_LENGTH..]
            .try_into()
            .expect("Can't generate a volume label.");
        let attribute = attribute::Attribute::volume_label();
        let name_flags = name_flags::NameFlags::volume_label();
        let current_time = time::Time::current_time();
        let created_time = current_time;
        let accessed_time = current_time;
        let written_time = current_time;
        let cluster: RefCell<Option<u32>> = RefCell::new(Some(0));
        let size: usize = 0;
        let long_file_name: Option<Box<Self>> = None;
        Self::ShortFileName {
            stem,
            extension,
            attribute,
            name_flags,
            created_time,
            accessed_time,
            written_time,
            cluster,
            size,
            long_file_name,
        }
    }

    fn long_file_name(name: Vec<u16>, order: usize) -> Self {
        let (name, next): ([u16; long_file_name::LONG_FILE_NAME_LENGTH], Option<Box<Self>>) = if long_file_name::LONG_FILE_NAME_LENGTH <= name.len() {
            let (name, next): (&[u16], &[u16]) = name.split_at(long_file_name::LONG_FILE_NAME_LENGTH);
            let name: [u16; long_file_name::LONG_FILE_NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a long file name directory entry.");
            let next: Vec<u16> = next.to_vec();
            let next: Option<Box<Self>> = Some(Box::new(Self::long_file_name(next, order + 1)));
            (name, next)
        } else {
            let mut name: Vec<u16> = name;
            if name.len() < long_file_name::LONG_FILE_NAME_LENGTH {
                name.push(0x0000);
            }
            name.resize(long_file_name::LONG_FILE_NAME_LENGTH, 0xffff);
            let name: [u16; long_file_name::LONG_FILE_NAME_LENGTH] = name
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

impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = match self {
            Self::ShortFileName {
                stem,
                extension,
                attribute,
                name_flags,
                created_time,
                accessed_time,
                written_time,
                cluster,
                size,
                long_file_name,
            } => {
                let stem: Vec<u8> = stem.borrow().to_vec();
                let extension: Vec<u8> = extension.to_vec();
                let mut name: Vec<u8> = vec![];
                name.extend(stem);
                name.extend(extension);
                let name = String::from_utf8(name).expect("Can't print a directory entry.");
                let name: String = format!("short file name: {}", name);
                let attribute: String = format!("{}", attribute)
                    .lines()
                    .map(|line| format!("attribute.{}", line))
                    .collect::<Vec<String>>()
                    .join("\n");
                let name_flags: String = format!("{}", name_flags);
                let created_time: String = format!("created time: {}", created_time);
                let accessed_time: String = format!("accessed time: {}", accessed_time);
                let written_time: String = format!("written time: {}", written_time);
                let cluster: String = format!("cluster: {:?}", cluster);
                let size: String = format!("size: {}", size);
                let long_file_name: String = match long_file_name {
                    Some(long_file_name) => format!("{}", long_file_name.as_ref()),
                    None => String::new(),
                };
                let elements: Vec<String> = vec![
                    name,
                    attribute,
                    name_flags,
                    created_time,
                    accessed_time,
                    written_time,
                    cluster,
                    size,
                    long_file_name,
                ];
                elements
                    .into_iter()
                    .filter(|element| 0 < element.len())
                    .collect::<Vec<String>>()
                    .join("\n")
            },
            Self::LongFileName {
                name,
                order,
                next,
            } => {
                let (name, _): (Vec<u16>, bool) = name
                    .iter()
                    .fold((vec![], true), |(name, continuity), c| if continuity {
                        match c {
                            0x0000 => {
                                (name, false)
                            },
                            _ => {
                                let mut name: Vec<u16> = name;
                                name.push(*c);
                                (name, true)
                            },
                        }
                    } else {
                        (name, continuity)
                    });
                let name = String::from_utf16(&name).expect("Can't print a directory entry.");
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
                    .filter(|element| 0 < element.len())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        };
        write!(f, "{}", string)
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
        let stem_is_irreversible: bool = stem_is_irreversible || short_file_name::STEM_LENGTH < stem.len();
        stem.resize(short_file_name::STEM_LENGTH, ' ' as u8);
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
        let extension_is_irreversible: bool = extension_is_irreversible || short_file_name::EXTENSION_LENGTH < extension.len();
        extension.resize(short_file_name::EXTENSION_LENGTH, ' ' as u8);
        let attribute: attribute::Attribute = path.into();
        let name_flags: name_flags::NameFlags = path.into();
        let created_time = time::Time::last_changed_time(path);
        let accessed_time = time::Time::last_accessed_time(path);
        let written_time = time::Time::last_modified_time(path);
        let cluster: RefCell<Option<u32>> = RefCell::new(None);
        let size: usize = if path.is_file() {
            fs::metadata(path)
                .expect("Can't generate a directory entry.")
                .len() as usize
        } else if path.is_dir() {
            0
        } else {
            panic!("Can't generate a directory entry.");
        };
        let irreversible: bool = stem_is_irreversible || extension_is_irreversible;
        let long_file_name: Option<Box<Self>> = if irreversible {
            let suffix: Vec<u8> = "~1".as_bytes().to_vec();
            stem.truncate(stem.len() - suffix.len());
            stem.extend(suffix);
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
        let stem: [u8; short_file_name::STEM_LENGTH] = stem
            .try_into()
            .expect("Can't generate a directory entry.");
        let stem: RefCell<[u8; short_file_name::STEM_LENGTH]> = RefCell::new(stem);
        let extension: [u8; short_file_name::EXTENSION_LENGTH] = extension
            .try_into()
            .expect("Can't generate a directory entry.");
        Self::ShortFileName {
            stem,
            extension,
            attribute,
            name_flags,
            created_time,
            accessed_time,
            written_time,
            cluster,
            size,
            long_file_name,
        }
    }
}

impl Into<Vec<u8>> for &DirectoryEntry {
    fn into(self) -> Vec<u8> {
        match self {
            DirectoryEntry::ShortFileName {
                stem,
                extension,
                attribute,
                name_flags,
                created_time,
                accessed_time,
                written_time,
                cluster,
                size,
                long_file_name,
            } => {
                let long_file_name: Vec<u8> = match long_file_name {
                    Some(long_file_name) => {
                        let long_file_name: &long_file_name::LongFileName = &long_file_name.as_ref().into();
                        long_file_name.into()
                    },
                    None => vec![],
                };
                let short_file_name: &short_file_name::ShortFileName = &self.into();
                let short_file_name: Vec<u8> = short_file_name.into();
                let mut bytes: Vec<u8> = vec![];
                bytes.extend(long_file_name);
                bytes.extend(short_file_name);
                bytes
            },
            DirectoryEntry::LongFileName {
                name,
                order,
                next,
            } => {
                let next: Vec<u8> = match next {
                    Some(next) => {
                        let next: &long_file_name::LongFileName = &next.as_ref().into();
                        next.into()
                    },
                    None => vec![],
                };
                let long_file_name: &long_file_name::LongFileName = &self.into();
                let long_file_name: Vec<u8> = long_file_name.into();
                let mut bytes: Vec<u8> = vec![];
                bytes.extend(next);
                bytes.extend(long_file_name);
                bytes
            },
        }
    }
}

