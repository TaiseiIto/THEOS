use std::{
    ffi::OsStr,
    fmt,
    path::PathBuf,
};

#[derive(Clone, Copy, Debug)]
pub struct NameFlags {
    lowercase_stem: bool,
    lowercase_extension: bool,
}

impl NameFlags {
    pub fn root() -> Self {
        Self {
            lowercase_stem: false,
            lowercase_extension: false,
        }
    }

    pub fn volume_label() -> Self {
        Self {
            lowercase_stem: false,
            lowercase_extension: false,
        }
    }
}

impl From<&PathBuf> for NameFlags {
    fn from(path: &PathBuf) -> Self {
        let stem: String = path
            .file_stem()
            .expect("Can't generate name flags.")
            .to_str()
            .expect("Can't generate name flags.")
            .to_string();
        let lowercase_stem: bool = stem == stem.to_lowercase();
        let extension: String = path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .expect("Can't generate name flags.")
            .to_string();
        let lowercase_extension: bool = extension == extension.to_lowercase();
        Self {
            lowercase_stem,
            lowercase_extension,
        }
    }
}

impl Into<u8> for &NameFlags {
    fn into(self) -> u8 {
        let lowercase_stem: u8 = if self.lowercase_stem {
            0x08
        } else {
            0x00
        };
        let lowercase_extension: u8 = if self.lowercase_extension {
            0x10
        } else {
            0x00
        };
        lowercase_stem + lowercase_extension
    }
}

impl fmt::Display for NameFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_stem: String = format!("lowercase_stem: {}", self.lowercase_stem);
        let lowercase_extension: String = format!("lowercase_extension: {}", self.lowercase_extension);
        let elements: Vec<String> = vec![
            lowercase_stem,
            lowercase_extension,
        ];
        let string: String = elements.join("\n");
        write!(f, "{}", string)
    }
}

