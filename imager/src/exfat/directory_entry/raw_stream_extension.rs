use {
    std::mem,
    super::{
        DirectoryEntry,
        DIRECTORY_ENTRY_SIZE,
        Raw,
    }
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct RawStreamExtension {
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

impl RawStreamExtension {
    pub fn general_flags(&self) -> u8 {
        self.general_flags
    }

    pub fn name_length(&self) -> u8 {
        self.name_length
    }

    pub fn name_hash(&self) -> u16 {
        self.name_hash
    }

    pub fn first_cluster(&self) -> u32 {
        self.first_cluster
    }

    pub fn data_length(&self) -> u64 {
        self.data_length
    }
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
                file_name: _,
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

    fn raw(&self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<Self, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }

    fn read(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

