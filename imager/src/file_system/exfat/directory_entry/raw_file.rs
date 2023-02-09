use {
    std::{
        convert::{
            From,
            Into,
        },
        mem,
    },
    super::{
        DirectoryEntry,
        DIRECTORY_ENTRY_SIZE,
    }
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct RawFile {
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
    create_utc_offset: i8,
    last_modified_utc_offset: i8,
    last_accessed_utc_offset: i8,
    reserved_2: [u8; 7],
}

impl RawFile {
    pub fn create_timestamp(&self) -> u32 {
        self.create_timestamp
    }

    pub fn create_10ms_increment(&self) -> u8 {
        self.create_10ms_increment
    }

    pub fn create_utc_offset(&self) -> i8{
        self.create_utc_offset
    }

    pub fn last_modified_timestamp(&self) -> u32 {
        self.last_modified_timestamp
    }

    pub fn last_modified_10ms_increment(&self) -> u8 {
        self.last_modified_10ms_increment
    }

    pub fn last_modified_utc_offset(&self) -> i8 {
        self.last_modified_utc_offset
    }

    pub fn last_accessed_timestamp(&self) -> u32 {
        self.last_accessed_timestamp
    }

    pub fn last_accessed_utc_offset(&self) -> i8 {
        self.last_accessed_utc_offset
    }
}

impl From<&DirectoryEntry> for RawFile {
    fn from(directory_entry: &DirectoryEntry) -> Self {
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
                let file_attributes: u16 = file_attributes.into();
                let reserved_1: u16 = 0;
                let create_timestamp: u32 = create_time.fat_timestamp();
                let last_modified_timestamp: u32 = modified_time.fat_timestamp();
                let last_accessed_timestamp: u32 = accessed_time.fat_timestamp();
                let create_10ms_increment: u8 = create_time.get_10ms_increment();
                let last_modified_10ms_increment: u8 = modified_time.get_10ms_increment();
                let create_utc_offset: i8 = create_time.utc_offset();
                let last_modified_utc_offset: i8 = modified_time.utc_offset();
                let last_accessed_utc_offset: i8 = accessed_time.utc_offset();
                let reserved_2: [u8; 7] = [0; 7];
                let ref raw_file = Self {
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
                let raw_file: [u8; DIRECTORY_ENTRY_SIZE] = raw_file.into();
                let mut bytes: Vec<u8> = raw_file.to_vec();
                let mut tail_bytes: Vec<u8> = stream_extension.as_ref().into();
                bytes.append(&mut tail_bytes);
                let set_checksum: u16 = bytes
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| *i != 2 && *i != 3)
                    .map(|(_, byte)| byte)
                    .fold(0u16, |checksum, byte| (checksum << 15) + (checksum >> 1) + byte as u16);
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
}

impl From<&[u8; DIRECTORY_ENTRY_SIZE]> for RawFile {
    fn from(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

impl Into<[u8; DIRECTORY_ENTRY_SIZE]> for &RawFile {
    fn into(self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<RawFile, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }
}

