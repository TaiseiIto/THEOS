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
pub struct RawAllocationBitmap {
    entry_type: u8,
    bitmap_flags: u8,
    reserved: [u8; 0x12],
    first_cluster: u32,
    data_length: u64,
}

impl RawAllocationBitmap {
    pub fn bitmap_flags(&self) -> u8 {
        self.bitmap_flags
    }

    pub fn data_length(&self) -> u64 {
        self.data_length
    }

    pub fn first_cluster(&self) -> u32 {
        self.first_cluster
    }
}

impl From<&DirectoryEntry> for RawAllocationBitmap {
    fn from(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::AllocationBitmap {
                bitmap_identifier,
                first_cluster,
                data_length,
            } => {
                let bitmap_flags: u8 = match bitmap_identifier {
                    true => 0x01,
                    false => 0x00,
                };
                let reserved: [u8; 0x12] = [0; 0x12];
                let first_cluster: u32 = *first_cluster;
                let data_length: u64 = *data_length as u64;
                Self {
                    entry_type,
                    bitmap_flags,
                    reserved,
                    first_cluster,
                    data_length,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawAllocationBitmap."),
        }
    }
}

impl From<&[u8; DIRECTORY_ENTRY_SIZE]> for RawAllocationBitmap {
    fn from(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

impl Into<[u8; DIRECTORY_ENTRY_SIZE]> for &RawAllocationBitmap {
    fn into(self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<RawAllocationBitmap, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }
}

