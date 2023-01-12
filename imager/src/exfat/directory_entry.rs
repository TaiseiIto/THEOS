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
        stream_extension: Box<DirectoryEntry>,
    },
    StreamExtension,
}

impl DirectoryEntry {
    pub fn file(path: &path::PathBuf) -> Self {
        let file_attributes = FileAttributes::new(path);
        let create_time: time::Time = time::Time::get_changed_time(path);
        let modified_time: time::Time = time::Time::get_modified_time(path);
        let accessed_time: time::Time = time::Time::get_accessed_time(path);
        let stream_extension = Box::new(Self::stream_extension());
        Self::File {
            file_attributes,
            create_time,
            modified_time,
            accessed_time,
            stream_extension,
        }
    }

    fn stream_extension() -> Self {
        Self::StreamExtension
    }

    fn entry_type(&self) -> EntryType {
        EntryType::file()
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
}

#[derive(Debug)]
enum TypeCode {
    File,
    StreamExtension,
}

impl TypeCode {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::File => 0x05,
            Self::StreamExtension => 0x00,
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

