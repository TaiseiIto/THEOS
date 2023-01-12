use {
    std::{
        ffi,
        path,
    },
    super::super::time,
};

#[derive(Debug)]
pub enum DirectoryEntry {
    File {
        file_attributes: FileAttributes,
        create_time: time::Time,
        modified_time: time::Time,
        accessed_time: time::Time,
        stream_extension: Box<Self>,
    },
    StreamExtension {
        allocation_possible: bool,
        no_fat_chain: bool,
        file_name: Box<Self>,
    },
    FileName {
        allocation_possible: bool,
        no_fat_chain: bool,
        file_name: Vec<u16>,
        next_file_name: Option<Box<Self>>,
    },
}

impl DirectoryEntry {
    pub fn file(path: &path::PathBuf) -> Self {
        let file_attributes = FileAttributes::new(path);
        let create_time: time::Time = time::Time::get_changed_time(path);
        let modified_time: time::Time = time::Time::get_modified_time(path);
        let accessed_time: time::Time = time::Time::get_accessed_time(path);
        let file_name: &ffi::OsStr = path.file_name().expect(&format!("Can't extract base name from {}", path.display()));
        let file_name: &str = file_name.to_str().expect("Can't convert OsStr to String.");
        let file_name: String = file_name.to_string();
        let stream_extension: Box<Self> = Box::new(Self::stream_extension(file_name));
        Self::File {
            file_attributes,
            create_time,
            modified_time,
            accessed_time,
            stream_extension,
        }
    }

    fn stream_extension(file_name: String) -> Self {
        let allocation_possible = true;
        let no_fat_chain = false;
        let file_name: Vec<u16> = file_name
            .chars()
            .map(|c| c as u16)
            .collect();
        let file_name: Box<Self> = Box::new(Self::file_name(file_name));
        Self::StreamExtension {
            allocation_possible,
            no_fat_chain,
            file_name,
        }
    }

    fn file_name(mut file_name: Vec<u16>) -> Self {
        const FILE_NAME_BLOCK_SIZE: usize = 0xf;
        let allocation_possible = false;
        let no_fat_chain = false;
        let remaining_file_name: Option<Vec<u16>> = if FILE_NAME_BLOCK_SIZE < file_name.len() {
            Some(file_name.split_off(FILE_NAME_BLOCK_SIZE))
        } else {
            None
        };
        file_name.resize(FILE_NAME_BLOCK_SIZE, 0x00);
        let next_file_name: Option<Box<Self>> = match remaining_file_name {
            Some(remaining_file_name) => Some(Box::new(Self::file_name(remaining_file_name))),
            None => None,
        };
        Self::FileName {
            allocation_possible,
            no_fat_chain,
            file_name,
            next_file_name,
        }
    }

    fn entry_type(&self) -> EntryType {
        match self {
            Self::File {
                file_attributes,
                create_time,
                modified_time,
                accessed_time,
                stream_extension,
            } => EntryType::file(),
            Self::StreamExtension {
                allocation_possible,
                no_fat_chain,
                file_name,
            } => EntryType::stream_extension(),
            Self::FileName {
                allocation_possible,
                no_fat_chain,
                file_name,
                next_file_name,
            } => EntryType::file_name(),
        }
    }
}

#[derive(Debug)]
struct EntryType {
    type_code: TypeCode,
    type_importance: bool,
    type_category: bool,
    in_use: bool,
}

impl EntryType {
    fn file() -> Self {
        let type_code = TypeCode::File;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn stream_extension() -> Self {
        let type_code = TypeCode::StreamExtension;
        let type_importance = false;
        let type_category = true;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn file_name() -> Self {
        let type_code = TypeCode::FileName;
        let type_importance = false;
        let type_category = true;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    fn to_byte(&self) -> u8 {
        let type_code: u8 = self.type_code.to_byte();
        let type_importance: u8 = if self.type_importance {
            1 << 5
        } else {
            0
        };
        let type_category: u8 = if self.type_category {
            1 << 6
        } else {
            0
        };
        let in_use: u8 = if self.in_use {
            1 << 7
        } else {
            0
        };
        type_code + type_importance + type_category + in_use
    }
}

#[derive(Debug)]
enum TypeCode {
    File,
    StreamExtension,
    FileName,
}

impl TypeCode {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::File => 0x05,
            Self::StreamExtension => 0x00,
            Self::FileName => 0x01,
        }
    }
}

#[derive(Debug)]
struct FileAttributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    directory: bool,
    archive: bool,
}

impl FileAttributes {
    fn new(path: &path::PathBuf) -> Self {
        let read_only = true;
        let hidden = false;
        let system = true;
        let directory = path.is_dir();
        let archive = false;
        Self {
            read_only,
            hidden,
            system,
            directory,
            archive,
        }
    }
}

