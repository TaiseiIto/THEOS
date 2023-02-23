use {
    std::mem,
    super::{
        attribute,
        DirectoryEntry,
        DIRECTORY_ENTRY_SIZE,
    },
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct LongFileName {
    order: u8,
    name0: [u16; NAME0_LENGTH],
    attribute: u8,
    reserved0: u8,
    checksum: u8,
    name1: [u16; NAME1_LENGTH],
    reserved1: u16,
    name2: [u16; NAME2_LENGTH],
}

const NAME0_LENGTH: usize = 5;
const NAME1_LENGTH: usize = 6;
const NAME2_LENGTH: usize = 2;
pub const LONG_FILE_NAME_LENGTH: usize = NAME0_LENGTH + NAME1_LENGTH + NAME2_LENGTH;

impl LongFileName {
    pub fn name(&self) -> [u16; LONG_FILE_NAME_LENGTH] {
        let name0: [u16; NAME0_LENGTH] = self.name0;
        let name1: [u16; NAME1_LENGTH] = self.name1;
        let name2: [u16; NAME2_LENGTH] = self.name2;
        [
            &name0[..],
            &name1[..],
            &name2[..],
        ].concat().try_into().expect("Can't get long file name.")
    }

    pub fn order(&self) -> usize {
        self.order as usize
    }
}

impl From<&DirectoryEntry> for LongFileName {
    fn from(directory_entry: &DirectoryEntry) -> Self {
        match directory_entry {
            DirectoryEntry::LongFileName {
                name,
                order,
                next,
            } => {
                let order: u8 = *order as u8;
                let (name0, name): (&[u16], &[u16]) = name.split_at(NAME0_LENGTH);
                let (name1, name2): (&[u16], &[u16]) = name.split_at(NAME1_LENGTH);
                let name0: [u16; NAME0_LENGTH] = name0
                    .try_into()
                    .expect("Can't generate a long file name directory entry.");
                let name1: [u16; NAME1_LENGTH] = name1
                    .try_into()
                    .expect("Can't generate a long file name directory entry.");
                let name2: [u16; NAME2_LENGTH] = name2
                    .try_into()
                    .expect("Can't generate a long file name directory entry.");
                let attribute: u8 = (&attribute::Attribute::long_file_name()).into();
                let reserved0: u8 = 0;
                let checksum: u8 = 0;
                let reserved1: u16 = 0;
                Self {
                    order,
                    name0,
                    attribute,
                    reserved0,
                    checksum,
                    name1,
                    reserved1,
                    name2,
                }
            }
            _ => panic!("Can't generate a long file name directory entry."),
        }
    }
}

impl From<&[u8; DIRECTORY_ENTRY_SIZE]> for LongFileName {
    fn from(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

impl Into<[u8; DIRECTORY_ENTRY_SIZE]> for &LongFileName {
    fn into(self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<LongFileName, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }
}

impl Into<Vec<u8>> for &LongFileName {
    fn into(self) -> Vec<u8> {
        let bytes: [u8; DIRECTORY_ENTRY_SIZE] = self.into();
        bytes.to_vec()
    }
}

