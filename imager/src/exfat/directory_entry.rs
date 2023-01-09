use std::fmt;

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

