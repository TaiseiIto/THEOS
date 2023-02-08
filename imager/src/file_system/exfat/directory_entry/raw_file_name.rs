use {
    std::mem,
    super::{
        DirectoryEntry,
        DIRECTORY_ENTRY_SIZE,
        FILE_NAME_BLOCK_LENGTH,
        Raw,
    }
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct RawFileName {
    entry_type: u8,
    general_flags: u8,
    file_name: [u16; FILE_NAME_BLOCK_LENGTH],
}

impl RawFileName {
    pub fn general_flags(&self) -> u8 {
        self.general_flags
    }

    pub fn file_name(&self) -> [u16; FILE_NAME_BLOCK_LENGTH] {
        self.file_name
    }
}

impl Raw for RawFileName {
    fn new(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::FileName {
                general_flags,
                file_name,
                next_file_name: _,
            } => {
                let general_flags: u8 = general_flags.into();
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

