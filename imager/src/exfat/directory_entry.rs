use {
    std::{
        ffi,
        mem,
        path,
    },
    super::{
        object,
        super::time,
        upcase_table,
    },
};

const FILE_NAME_BLOCK_LENGTH: usize = 0xf;
const DIRECTORY_ENTRY_SIZE: usize = 0x20;

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
        general_flags: GeneralFlags,
        name_length: u8,
        name_hash: u16,
        first_cluster: u32,
        data_length: usize,
        file_name: Box<Self>,
    },
    FileName {
        general_flags: GeneralFlags,
        file_name: [u16; FILE_NAME_BLOCK_LENGTH],
        next_file_name: Option<Box<Self>>,
    },
}

impl DirectoryEntry {
    pub fn file(path: &path::PathBuf, content: &object::FileOrDirectory, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let file_attributes = FileAttributes::new(path);
        let create_time: time::Time = time::Time::get_changed_time(path);
        let modified_time: time::Time = time::Time::get_modified_time(path);
        let accessed_time: time::Time = time::Time::get_accessed_time(path);
        let file_name: &ffi::OsStr = path.file_name().expect(&format!("Can't extract base name from {}", path.display()));
        let file_name: &str = file_name.to_str().expect("Can't convert OsStr to String.");
        let file_name: String = file_name.to_string();
        let stream_extension: Box<Self> = Box::new(Self::stream_extension(file_name, content, upcase_table));
        Self::File {
            file_attributes,
            create_time,
            modified_time,
            accessed_time,
            stream_extension,
        }
    }

    fn stream_extension(file_name: String, content: &object::FileOrDirectory, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let general_flags = GeneralFlags::stream_extension();
        let file_name: Vec<u16> = file_name
            .chars()
            .map(|c| c as u16)
            .collect();
        let name_length: u8 = file_name.len() as u8;
        let name_hash: u16 = file_name
            .iter()
            .map(|c| [*c as u8, (*c >> 8) as u8])
            .flatten()
            .fold(0, |name_hash, c| (name_hash << 15) + (name_hash >> 1) + (c as u16));
        let (first_cluster, data_length): (u32, usize) = match content {
            object::FileOrDirectory::File {
                first_cluster,
                length,
            } => (*first_cluster, *length),
            _ => (0, 0),
        };
        let file_name: Box<Self> = Box::new(Self::file_name(file_name));
        Self::StreamExtension {
            general_flags,
            name_length,
            name_hash,
            first_cluster,
            data_length,
            file_name,
        }
    }

    fn file_name(mut file_name: Vec<u16>) -> Self {
        let general_flags = GeneralFlags::file_name();
        let remaining_file_name: Option<Vec<u16>> = if FILE_NAME_BLOCK_LENGTH < file_name.len() {
            Some(file_name.split_off(FILE_NAME_BLOCK_LENGTH))
        } else {
            None
        };
        file_name.resize(FILE_NAME_BLOCK_LENGTH, 0x00);
        let file_name: [u16; FILE_NAME_BLOCK_LENGTH] = file_name.try_into().expect("Can't convert Vec<u16> to [u16; FILE_NAME_BLOCK_LENGTH]");
        let next_file_name: Option<Box<Self>> = match remaining_file_name {
            Some(remaining_file_name) => Some(Box::new(Self::file_name(remaining_file_name))),
            None => None,
        };
        Self::FileName {
            general_flags,
            file_name,
            next_file_name,
        }
    }

    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        let entry_type: u8 = self.entry_type().to_byte();
        match self {
            Self::File {
                file_attributes,
                create_time,
                modified_time,
                accessed_time,
                stream_extension,
            } => {
                [0; DIRECTORY_ENTRY_SIZE]
            },
            Self::StreamExtension {
                general_flags,
                name_length,
                name_hash,
                first_cluster,
                data_length,
                file_name,
            } => {
                let general_flags: u8 = general_flags.to_byte();
                let reserved_1: u8 = 0;
                let name_length: u8 = *name_length;
                let name_hash: u16 = *name_hash;
                let reserved_2: u16 = 0;
                let reserved_3: u32 = 0;
                let data_length: u64 = *data_length as u64;
                let valid_data_length: u64 = data_length;
                let first_cluster: u32 = *first_cluster;
                let raw = RawStreamExtension {
                    entry_type,
                    general_flags,
                    reserved_1,
                    name_length,
                    name_hash,
                    reserved_2,
                    valid_data_length,
                    reserved_3,
                    first_cluster,
                    data_length,
                };
                raw.to_bytes()
            },
            Self::FileName {
                general_flags,
                file_name,
                next_file_name,
            } => {
                let general_flags: u8 = general_flags.to_byte();
                let file_name: [u16; FILE_NAME_BLOCK_LENGTH] = *file_name;
                let raw = RawFileName {
                    entry_type,
                    general_flags,
                    file_name,
                };
                raw.to_bytes()
            },
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
                general_flags,
                name_length,
                name_hash,
                first_cluster,
                data_length,
                file_name,
            } => EntryType::stream_extension(),
            Self::FileName {
                general_flags,
                file_name,
                next_file_name,
            } => EntryType::file_name(),
        }
    }
}

trait Raw {
    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE];
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct RawFileName {
    entry_type: u8,
    general_flags: u8,
    file_name: [u16; FILE_NAME_BLOCK_LENGTH],
}

impl Raw for RawFileName {
    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct RawStreamExtension {
    entry_type: u8,
    general_flags: u8,
    reserved_1: u8,
    name_length: u8,
    name_hash: u16,
    reserved_2: u16,
    valid_data_length: u64,
    reserved_3: u32,
    first_cluster: u32,
    data_length: u64,
}

impl Raw for RawStreamExtension {
    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
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

#[derive(Debug)]
struct GeneralFlags {
    allocation_possible: bool,
    no_fat_chain: bool,
}

impl GeneralFlags {
    fn stream_extension() -> Self {
        let allocation_possible = true;
        let no_fat_chain = false;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }

    fn file_name() -> Self {
        let allocation_possible = false;
        let no_fat_chain = false;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }

    fn to_byte(&self) -> u8 {
        let allocation_possible = if self.allocation_possible {
            1
        } else {
            0
        };
        let no_fat_chain = if self.no_fat_chain {
            2
        } else {
            0
        };
        allocation_possible + no_fat_chain
    }
}

