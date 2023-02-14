use std::{
    ffi::OsStr,
    fmt,
    path::PathBuf,
};

#[derive(Debug)]
pub enum DirectoryEntry {
    ShortFileName {
        stem: [u8; STEM_LENGTH],
        extension: [u8; EXTENSION_LENGTH],
    },
}

const STEM_LENGTH: usize = 8;
const EXTENSION_LENGTH: usize = 3;

impl DirectoryEntry {
    pub fn new(path: &PathBuf) -> Self {
        let (stem, stem_irreversibility, _, _): (String, bool, bool, bool) = path
            .file_stem()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
            .to_uppercase()
            .chars()
            .fold((String::new(), false, false, true), |(stem, stem_irreversibility, dot_flag, head_flag), c| match c {
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
                    let stem_irreversibility: bool = stem_irreversibility;
                    let dot_flag: bool = false;
                    let head_flag: bool = false;
                    (stem, stem_irreversibility, dot_flag, head_flag)
                },
                ' ' => {
                    let stem: String = stem;
                    let stem_irreversibility: bool = true;
                    let dot_flag: bool = dot_flag;
                    let head_flag: bool = head_flag;
                    (stem, stem_irreversibility, dot_flag, head_flag)
                },
                '.' => {
                    if head_flag {
                        let stem: String = stem;
                        let stem_irreversibility: bool = true;
                        let dot_flag: bool = true;
                        let head_flag: bool = true;
                        (stem, stem_irreversibility, dot_flag, head_flag)
                    } else {
                        if dot_flag {
                            let stem: String = stem;
                            let stem_irreversibility: bool = true;
                            let dot_flag: bool = true;
                            let head_flag: bool = head_flag;
                            (stem, stem_irreversibility, dot_flag, head_flag)
                        } else {
                            let mut stem: String = stem;
                            stem.push('.');
                            let stem_irreversibility: bool = stem_irreversibility;
                            let dot_flag: bool = true;
                            let head_flag: bool = head_flag;
                            (stem, stem_irreversibility, dot_flag, head_flag)
                        }
                    }
                },
                _ => {
                    let mut stem: String = stem;
                    let stem_irreversibility: bool = true;
                    let dot_flag: bool = false;
                    let head_flag: bool = false;
                    stem.push('_');
                    (stem, stem_irreversibility, dot_flag, head_flag)
                },
            });
        let mut stem: Vec<u8> = stem.into_bytes();
        let stem_irreversibility: bool = stem_irreversibility || STEM_LENGTH < stem.len();
        stem.resize(STEM_LENGTH, ' ' as u8);
        let stem: [u8; STEM_LENGTH] = stem
            .try_into()
            .expect("Can't generate a directory entry.");
        let (extension, extension_irreversibility): (String, bool) = path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
            .to_uppercase()
            .chars()
            .fold((String::new(), false), |(stem, extension_irreversibility), c| match c {
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
                    let extension_irreversibility: bool = extension_irreversibility;
                    (stem, extension_irreversibility)
                },
                ' ' => {
                    let stem: String = stem;
                    let extension_irreversibility: bool = true;
                    (stem, extension_irreversibility)
                },
                '.' => panic!("Can't generate an extension."),
                _ => {
                    let mut stem: String = stem;
                    stem.push('_');
                    let extension_irreversibility: bool = true;
                    (stem, extension_irreversibility)
                },
            });
        let mut extension: Vec<u8> = extension.into_bytes();
        let extension_irreversibility: bool = extension_irreversibility || EXTENSION_LENGTH < extension.len();
        extension.resize(EXTENSION_LENGTH, ' ' as u8);
        let extension: [u8; EXTENSION_LENGTH] = extension
            .try_into()
            .expect("Can't generate a directory entry.");
        Self::ShortFileName {
            stem,
            extension,
        }
    }
}

impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = match self {
            Self::ShortFileName {
                stem,
                extension,
            } => {
                let stem: Vec<u8> = stem.to_vec();
                let extension: Vec<u8> = extension.to_vec();
                let mut name: Vec<u8> = vec![];
                name.extend(stem);
                name.extend(extension);
                let name = String::from_utf8(name).expect("Can't print a directory entry.");
                format!("short file name:{}", name)
            }
        };
        write!(f, "{}", string)
    }
}

