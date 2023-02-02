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
pub struct RawUpcaseTable {
    entry_type: u8,
    reserved_1: [u8; 0x3],
    table_checksum: u32,
    reserved_2: [u8; 0xc],
    first_cluster: u32,
    data_length: u64,
}

impl RawUpcaseTable {
    pub fn data_length(&self) -> u64 {
        self.data_length
    }

    pub fn first_cluster(&self) -> u32 {
        self.first_cluster
    }

    pub fn table_checksum(&self) -> u32 {
        self.table_checksum
    }
}

impl Raw for RawUpcaseTable {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::UpcaseTable {
                table_checksum,
                first_cluster,
                data_length,
                upcase_table: _,
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


