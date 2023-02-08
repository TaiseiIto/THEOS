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
        Test,
    },
};

pub const VOLUME_LABEL_MAX_LENGTH: usize = 11;

#[allow(dead_code)]
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct RawVolumeLabel {
    entry_type: u8,
    character_count: u8,
    volume_label: [u16; VOLUME_LABEL_MAX_LENGTH],
    reserved: u64,
}

impl RawVolumeLabel {
    pub fn character_count(&self) -> u8 {
        self.character_count
    }

    pub fn volume_label(&self) -> [u16; VOLUME_LABEL_MAX_LENGTH] {
        self.volume_label
    }
}

impl From<&DirectoryEntry> for RawVolumeLabel {
    fn from(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::VolumeLabel {
                volume_label,
            } => {
                let mut volume_label: Vec<u16> = volume_label
                    .encode_utf16()
                    .collect();
                let character_count = volume_label.len() as u8;
                while volume_label.len() < VOLUME_LABEL_MAX_LENGTH {
                    volume_label.push(0x0000);
                }
                let (volume_label, _): (&[u16], &[u16]) = volume_label.split_at(VOLUME_LABEL_MAX_LENGTH);
                let volume_label: [u16; VOLUME_LABEL_MAX_LENGTH] = volume_label
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
}

impl From<&[u8; DIRECTORY_ENTRY_SIZE]> for RawVolumeLabel {
    fn from(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

impl Into<[u8; DIRECTORY_ENTRY_SIZE]> for &RawVolumeLabel {
    fn into(self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<RawVolumeLabel, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }
}

impl<'a> Test<'a> for RawVolumeLabel {
}

