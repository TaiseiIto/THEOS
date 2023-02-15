use {
    std::{
        fmt,
        fs,
        path::PathBuf,
    },
    super::super::node,
};

#[derive(Debug)]
pub struct Attribute {
    read_only: bool,
    hidden: bool,
    system: bool,
    volume_id: bool,
    directory: bool,
    archive: bool,
    long_file_name: bool,
}

impl Attribute {
    pub fn long_file_name() -> Self {
        Self {
            read_only: false,
            hidden: false,
            system: false,
            volume_id: false,
            directory: false,
            archive: false,
            long_file_name: true,
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let read_only: String = format!("read_only: {}", self.read_only);
        let hidden: String = format!("hidden: {}", self.hidden);
        let system: String = format!("system: {}", self.system);
        let volume_id: String = format!("volume_id: {}", self.volume_id);
        let directory: String = format!("directory: {}", self.directory);
        let archive: String = format!("archive: {}", self.archive);
        let long_file_name: String = format!("long_file_name: {}", self.long_file_name);
        let elements: Vec<String> = vec![
            read_only,
            hidden,
            system,
            volume_id,
            directory,
            archive,
            long_file_name,
        ];
        let string: String = elements
            .into_iter()
            .fold(String::new(), |string, element| string + &element + "\n");
        write!(f, "{}", string)
    }
}

impl From<&PathBuf> for Attribute {
    fn from(path: &PathBuf) -> Self {
        if let Ok(metadata) = fs::metadata(path) {
            let read_only: bool = metadata
                .permissions()
                .readonly();
            let hidden: bool = path
                .file_name()
                .expect("Can't generate an attribute.")
                .to_str()
                .expect("Can't generate an attribute.")
                .starts_with(".");
            let system: bool = true;
            let volume_id: bool = false;
            let directory: bool = path.is_dir();
            let archive: bool = false;
            let long_file_name: bool = false;
            Self {
                read_only,
                hidden,
                system,
                volume_id,
                directory,
                archive,
                long_file_name,
            }
        } else {
            panic!("Can't generate an attribute.")
        }
    }
}

const READ_ONLY: u8 = 0x01;
const HIDDEN: u8 = 0x02;
const SYSTEM: u8 = 0x04;
const VOLUME_ID: u8 = 0x08;
const DIRECTORY: u8 = 0x10;
const ARCHIVE: u8 = 0x20;
const LONG_FILE_NAME: u8 = 0xf;

impl From<u8> for Attribute {
    fn from(byte: u8) -> Self {
        let read_only: bool = byte & READ_ONLY != 0;
        let hidden: bool = byte & HIDDEN != 0;
        let system: bool = byte & SYSTEM != 0;
        let volume_id: bool = byte & VOLUME_ID != 0;
        let directory: bool = byte & DIRECTORY != 0;
        let archive: bool = byte & ARCHIVE != 0;
        let long_file_name: bool = byte == LONG_FILE_NAME;
        if long_file_name {
            let read_only: bool = false;
            let hidden: bool = false;
            let system: bool = false;
            let volume_id: bool = false;
            let directory: bool = false;
            let archive: bool = false;
            Self {
                read_only,
                hidden,
                system,
                volume_id,
                directory,
                archive,
                long_file_name,
            }
        } else {
            Self {
                read_only,
                hidden,
                system,
                volume_id,
                directory,
                archive,
                long_file_name,
            }
        }
    }
}

impl Into<u8> for &Attribute {
    fn into(self) -> u8 {
        let read_only: u8 = if self.read_only {
            READ_ONLY
        } else {
            0
        };
        let hidden: u8 = if self.hidden {
           HIDDEN
        } else {
            0
        };
        let system: u8 = if self.system {
            SYSTEM
        } else {
            0
        };
        let volume_id: u8 = if self.volume_id {
            VOLUME_ID
        } else {
            0
        };
        let directory: u8 = if self.directory {
            DIRECTORY
        } else {
            0
        };
        let archive: u8 = if self.archive {
            ARCHIVE
        } else {
            0
        };
        if self.long_file_name {
            LONG_FILE_NAME
        } else {
            read_only + hidden + system + volume_id + directory + archive
        }
    }
}

