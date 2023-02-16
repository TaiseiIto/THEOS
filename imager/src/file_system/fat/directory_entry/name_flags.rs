use std::path::PathBuf;

#[derive(Debug)]
pub struct NameFlags {
    lowercase_stem: bool,
    lowercase_extension: bool,
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
            .expect("Can't generate name flags.")
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

