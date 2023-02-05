pub mod exfat;

use std::{
    convert,
    str,
};

#[derive(Debug)]
pub enum FileSystem {
    ExFat,
    Fat,
}

impl str::FromStr for FileSystem {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}

impl convert::TryFrom<&str> for FileSystem {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s: String = s.to_lowercase();
        match &*s {
            "exfat" => Ok(Self::ExFat),
            "fat" => Ok(Self::Fat),
            _ => Err("Can't interpret file system."),
        }
    }
}

