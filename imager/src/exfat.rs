use std::{
    fmt,
    path,
};

#[derive(Debug)]
pub struct Exfat {
}

impl Exfat {
    pub fn new(boot_sector: path::PathBuf, source_directory: path::PathBuf) -> Self {
        Self {
        }
    }
}

impl fmt::Display for Exfat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

