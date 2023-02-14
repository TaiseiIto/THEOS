use std::{
    ffi::OsStr,
    path::PathBuf,
};

#[derive(Debug)]
pub enum DirectoryEntry {
    ShortFileName {
        stem: String,
        extension: String,
    },
}

impl DirectoryEntry {
    pub fn new(path: &PathBuf) -> Self {
        let stem: String = path
            .file_stem()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
            .to_string();
        let extension: String = path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
            .to_string();
        Self::ShortFileName {
            stem,
            extension,
        }
    }
}

