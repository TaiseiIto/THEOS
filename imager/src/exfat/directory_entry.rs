extern crate regex;

use std::fmt;

#[derive(Debug)]
pub struct DirectoryEntry {
    entry_type: EntryType,
    inner: DirectoryEntryEnum,
}

impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let regex = regex::Regex::new("^|\n").expect("Can't create a Regex.");
        let entry_type: String = format!("{}", self.entry_type);
        let entry_type: String = regex.replace_all(&entry_type, "$0directory_entry.");
        write!(f, "{}\n", entry_type)?;
        let inner: String = format!("{}", self.inner);
        let inner: String = regex.replace_all(&inner, "$0directory_entry.");
        write!(f, "{}", inner)
    }
}

#[derive(Debug)]
pub struct EntryType {
    type_code: TypeCode,
    type_importance: bool,
    type_category: bool,
    in_use: bool,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "entry_type.type_code = {}\n", self.type_code)?;
        write!(f, "entry_type.type_importance = {}\n", self.type_importance)?;
        write!(f, "entry_type.type_category = {}\n", self.type_category)?;
        write!(f, "entry_type.in_use = {}", self.in_use)
    }
}

#[derive(Debug)]
pub enum TypeCode {
    AllocationBitmap,
    UpcaseTable,
    VolumeLabel,
    FileDirectory,
    StreamExtension,
    FileName,
}

impl fmt::Display for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AllocationBitmap => write!(f, "AllocationBitmap"),
            Self::UpcaseTable => write!(f, "UpcaseTable"),
            Self::VolumeLabel => write!(f, "VolumeLabel"),
            Self::FileDirectory => write!(f, "FileDirectory"),
            Self::StreamExtension => write!(f, "StreamExtension"),
            Self::FileName => write!(f, "FileName"),
        }
    }
}

#[derive(Debug)]
pub enum DirectoryEntryEnum {
    AllocationBitmap,
    UpcaseTable,
    VolumeLabel,
    FileDirectory,
    StreamExtension,
    FileName,
}

impl fmt::Display for DirectoryEntryEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AllocationBitmap => write!(f, "AllocationBitmap"),
            Self::UpcaseTable => write!(f, "UpcaseTable"),
            Self::VolumeLabel => write!(f, "VolumeLabel"),
            Self::FileDirectory => write!(f, "FileDirectory"),
            Self::StreamExtension => write!(f, "StreamExtension"),
            Self::FileName => write!(f, "FileName"),
        }
    }
}

