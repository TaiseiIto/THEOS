use {
    std::{
        ffi,
        mem,
        path,
    },
    super::{
        cluster,
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
    UpcaseTable {
        table_checksum: u32,
        first_cluster: u32,
        data_length: usize,
    },
    VolumeLabel {
        volume_label: String,
    },
}

impl DirectoryEntry {
    pub fn file(path: &path::PathBuf, first_cluster: u32, data_length: usize, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let file_attributes = FileAttributes::new(path);
        let create_time: time::Time = time::Time::get_changed_time(path);
        let modified_time: time::Time = time::Time::get_modified_time(path);
        let accessed_time: time::Time = time::Time::get_accessed_time(path);
        let file_name: &ffi::OsStr = path.file_name().expect(&format!("Can't extract base name from {}", path.display()));
        let file_name: &str = file_name.to_str().expect("Can't convert OsStr to String.");
        let file_name: String = file_name.to_string();
        let stream_extension: Box<Self> = Box::new(Self::stream_extension(file_name, first_cluster, data_length, upcase_table));
        Self::File {
            file_attributes,
            create_time,
            modified_time,
            accessed_time,
            stream_extension,
        }
    }

    fn stream_extension(file_name: String, first_cluster: u32, data_length: usize, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let general_flags = GeneralFlags::stream_extension();
        let file_name: Vec<u16> = file_name
            .chars()
            .map(|c| c as u16)
            .collect();
        let name_length: u8 = file_name.len() as u8;
        let name_hash: u16 = file_name
            .iter()
            .map(|c| upcase_table.to_upcase(*c))
            .map(|c| [c as u8, (c >> 8) as u8])
            .flatten()
            .fold(0, |name_hash, c| (name_hash << 15) + (name_hash >> 1) + (c as u16));
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

    pub fn upcase_table(upcase_table: &upcase_table::UpcaseTable, clusters: &mut cluster::Clusters) -> Self {
        let bytes: Vec<u8> = upcase_table.to_bytes();
        let data_length: usize = bytes.len();
        let first_cluster: u32 = clusters.append(bytes);
        let table_checksum: u32 = upcase_table.table_checksum();
        Self::UpcaseTable {
            table_checksum,
            first_cluster,
            data_length,
        }
    }

    pub fn volume_label(volume_label: &str) -> Self {
        let volume_label: String = volume_label.to_string();
        Self::VolumeLabel {
            volume_label,
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
            } => RawFile::new(self).to_bytes(),
            Self::StreamExtension {
                general_flags,
                name_length,
                name_hash,
                first_cluster,
                data_length,
                file_name,
            } => RawStreamExtension::new(self).to_bytes(),
            Self::FileName {
                general_flags,
                file_name,
                next_file_name,
            } => RawFileName::new(self).to_bytes(),
            Self::UpcaseTable {
                table_checksum,
                first_cluster,
                data_length,
            } => RawUpcaseTable::new(self).to_bytes(),
            Self::VolumeLabel {
                volume_label,
            } => RawVolumeLabel::new(self).to_bytes(),
        }
    }

    pub fn entry_set_to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.to_bytes().to_vec();
        let mut tail_bytes: Vec<u8> = match self {
            Self::File {
                file_attributes,
                create_time,
                modified_time,
                accessed_time,
                stream_extension,
            } => stream_extension.entry_set_to_bytes(),
            Self::StreamExtension {
                general_flags,
                name_length,
                name_hash,
                first_cluster,
                data_length,
                file_name,
            } => file_name.entry_set_to_bytes(),
            Self::FileName {
                general_flags,
                file_name,
                next_file_name,
            } => match next_file_name {
                Some(next_file_name) => next_file_name.entry_set_to_bytes(),
                None => vec![],
            },
            Self::UpcaseTable {
                table_checksum,
                first_cluster,
                data_length,
            } => vec![],
            Self::VolumeLabel {
                volume_label,
            } => vec![],
        };
        bytes.append(&mut tail_bytes);
        bytes
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
            Self::UpcaseTable {
                table_checksum,
                first_cluster,
                data_length,
            } => EntryType::upcase_table(),
            Self::VolumeLabel {
                volume_label,
            } => EntryType::volume_label(),
        }
    }

    fn directory_entry_set_length(&self) -> usize {
        match self {
            Self::File {
                file_attributes,
                create_time,
                modified_time,
                accessed_time,
                stream_extension,
            } => 1 + stream_extension.directory_entry_set_length(),
            Self::StreamExtension {
                general_flags,
                name_length,
                name_hash,
                first_cluster,
                data_length,
                file_name,
            } => 1 + file_name.directory_entry_set_length(),
            Self::FileName {
                general_flags,
                file_name,
                next_file_name,
            } => 1 + match next_file_name {
                Some(next_file_name) => next_file_name.directory_entry_set_length(),
                None => 0,
            },
            Self::UpcaseTable {
                table_checksum,
                first_cluster,
                data_length,
            } => 1,
            Self::VolumeLabel {
                volume_label,
            } => 1,
        }
    }
}

trait Raw {
    fn new(directory_entry: &DirectoryEntry) -> Self;
    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE];
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct RawFile {
    entry_type: u8,
    secondary_count: u8,
    set_checksum: u16,
    file_attributes: u16,
    reserved_1: u16,
    create_timestamp: u32,
    last_modified_timestamp: u32,
    last_accessed_timestamp: u32,
    create_10ms_increment: u8,
    last_modified_10ms_increment: u8,
    create_utc_offset: u8,
    last_modified_utc_offset: u8,
    last_accessed_utc_offset: u8,
    reserved_2: [u8; 7],
}

impl Raw for RawFile {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::File {
                file_attributes,
                create_time,
                modified_time,
                accessed_time,
                stream_extension,
            } => {
                let secondary_count: u8 = stream_extension.directory_entry_set_length() as u8;
                let set_checksum: u16 = 0;
                let file_attributes: u16 = file_attributes.to_word();
                let reserved_1: u16 = 0;
                let create_timestamp: u32 = create_time.get_timestamp();
                let last_modified_timestamp: u32 = modified_time.get_timestamp();
                let last_accessed_timestamp: u32 = accessed_time.get_timestamp();
                let create_10ms_increment: u8 = create_time.get_10ms_increment();
                let last_modified_10ms_increment: u8 = modified_time.get_10ms_increment();
                let create_utc_offset: u8 = create_time.get_utc_offset();
                let last_modified_utc_offset: u8 = modified_time.get_utc_offset();
                let last_accessed_utc_offset: u8 = accessed_time.get_utc_offset();
                let reserved_2: [u8; 7] = [0; 7];
                let raw_file = Self {
                    entry_type,
                    secondary_count,
                    set_checksum,
                    file_attributes,
                    reserved_1,
                    create_timestamp,
                    last_modified_timestamp,
                    last_accessed_timestamp,
                    create_10ms_increment,
                    last_modified_10ms_increment,
                    create_utc_offset,
                    last_modified_utc_offset,
                    last_accessed_utc_offset,
                    reserved_2,
                };
                let mut bytes: Vec<u8> = raw_file.to_bytes().to_vec();
                let mut tail_bytes: Vec<u8> = stream_extension.entry_set_to_bytes();
                bytes.append(&mut tail_bytes);
                let set_checksum: u16 = bytes
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != 2 && *i != 3)
                    .map(|(_, byte)| byte)
                    .fold(0 as u16, |checksum, byte| (checksum << 15) + (checksum >> 1) + *byte as u16);
                Self {
                    entry_type,
                    secondary_count,
                    set_checksum,
                    file_attributes,
                    reserved_1,
                    create_timestamp,
                    last_modified_timestamp,
                    last_accessed_timestamp,
                    create_10ms_increment,
                    last_modified_10ms_increment,
                    create_utc_offset,
                    last_modified_utc_offset,
                    last_accessed_utc_offset,
                    reserved_2,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawFile."),
        }
    }

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
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::StreamExtension {
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
                Self {
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
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawStreamExtension."),
        }
    }

    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct RawFileName {
    entry_type: u8,
    general_flags: u8,
    file_name: [u16; FILE_NAME_BLOCK_LENGTH],
}

impl Raw for RawFileName {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::FileName {
                general_flags,
                file_name,
                next_file_name,
            } => {
                let general_flags: u8 = general_flags.to_byte();
                let file_name: [u16; FILE_NAME_BLOCK_LENGTH] = *file_name;
                Self {
                    entry_type,
                    general_flags,
                    file_name,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawFileName."),
        }
    }

    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct RawUpcaseTable {
    entry_type: u8,
    reserved_1: [u8; 0x3],
    table_checksum: u32,
    reserved_2: [u8; 0xc],
    first_cluster: u32,
    data_length: u64,
}

impl Raw for RawUpcaseTable {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::UpcaseTable {
                table_checksum,
                first_cluster,
                data_length,
            } => {
                let reserved_1: [u8; 0x3] = [0x0; 0x3];
                let table_checksum: u32 = *table_checksum;
                let reserved_2: [u8; 0xc] = [0x0; 0xc];
                let first_cluster: u32 = *first_cluster;
                let data_length: u64 = *data_length as u64;
                Self {
                    entry_type,
                    reserved_1,
                    table_checksum,
                    reserved_2,
                    first_cluster,
                    data_length,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawUpcaseTable."),
        }
    }

    fn to_bytes(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}

const VOLUME_LABEL_MAX_LENGTH: usize = 11;

#[repr(packed)]
#[derive(Copy, Clone)]
struct RawVolumeLabel {
    entry_type: u8,
    character_count: u8,
    volume_label: [u16; VOLUME_LABEL_MAX_LENGTH],
    reserved: u64,
}

impl Raw for RawVolumeLabel {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::VolumeLabel {
                volume_label,
            } => {
                let mut volume_label: Vec<u16> = volume_label
                    .chars()
                    .map(|c| c.to_string().into_bytes())
                    .filter(|c| c.len() <= 2)
                    .map(|c| {
                        let mut i = c.iter();
                        match i.next() {
                            Some(lower_byte) => match i.next() {
                                Some(higher_byte) => ((*higher_byte as u16) << 8) + *lower_byte as u16,
                                None => *lower_byte as u16,
                            },
                            None => 0x0000,
                        }
                    })
                    .collect();
                let character_count = volume_label.len() as u8;
                while volume_label.len() < VOLUME_LABEL_MAX_LENGTH {
                    volume_label.push(0x0000);
                }
                let volume_label: [u16; VOLUME_LABEL_MAX_LENGTH] = volume_label
                    .chunks(VOLUME_LABEL_MAX_LENGTH)
                    .next()
                    .expect("Can't convert volume label into [u16; VOLUME_LABEL_MAX_LENGTH].")
                    .try_into()
                    .expect("Can't convert volume label into [u16; VOLUME_LABEL_MAX_LENGTH].");
                let reserved: u64 = 0;
                Self {
                    entry_type,
                    character_count,
                    volume_label,
                    reserved,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawVolumeLabel."),
        }
    }

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

    fn upcase_table() -> Self {
        let type_code = TypeCode::UpcaseTable;
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

    fn volume_label() -> Self {
        let type_code = TypeCode::VolumeLabel;
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
    UpcaseTable,
    VolumeLabel,
}

impl TypeCode {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::File => 0x05,
            Self::StreamExtension => 0x00,
            Self::FileName => 0x01,
            Self::UpcaseTable => 0x02,
            Self::VolumeLabel => 0x03,
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

    fn to_word(&self) -> u16 {
        let read_only: u16 = match self.read_only {
            true => 1,
            false => 0,
        };
        let hidden: u16 = match self.hidden {
            true => 1 << 1,
            false => 0,
        };
        let system: u16 = match self.system {
            true => 1 << 2,
            false => 0,
        };
        let directory: u16 = match self.system {
            true => 1 << 4,
            false => 0,
        };
        let archive: u16 = match self.archive {
            true => 1 << 5,
            false => 0,
        };
        read_only + hidden + system + directory + archive
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

