use {
    std::path,
    super::super::time,
};

#[derive(Debug)]
pub enum DirectoryEntry {
    File {
        file_attributes: FileAttributes,
        create_time: time::Time,
        modified_time: time::Time,
        accessed_time: time::Time,
        stream_extension: Option<Box<Self>>,
    },
    StreamExtension {
        allocation_possible: bool,
        no_fat_chain: bool,
    },
    FileName {
        allocation_possible: bool,
        no_fat_chain: bool,
    },
}

impl DirectoryEntry {
    pub fn file(path: &path::PathBuf) -> Self {
        let file_attributes = FileAttributes::new(path);
        let create_time: time::Time = time::Time::get_changed_time(path);
        let modified_time: time::Time = time::Time::get_modified_time(path);
        let accessed_time: time::Time = time::Time::get_accessed_time(path);
        let stream_extension: Option<Box<Self>> = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => Some(Box::new(Self::stream_extension(name.to_string()))),
                None => None,
            },
            None => None,
        };
        Self::File {
            file_attributes,
            create_time,
            modified_time,
            accessed_time,
            stream_extension,
        }
    }

    fn stream_extension(name: String) -> Self {
        let allocation_possible = true;
        let no_fat_chain = false;
        Self::StreamExtension {
            allocation_possible,
            no_fat_chain,
        }
    }

    fn file_name(name: String) -> Self {
        let allocation_possible = false;
        let no_fat_chain = false;
        Self::FileName {
            allocation_possible,
            no_fat_chain,
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
            } => EntryType::stream_extension(),
            Self::FileName {
                allocation_possible,
                no_fat_chain,
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

