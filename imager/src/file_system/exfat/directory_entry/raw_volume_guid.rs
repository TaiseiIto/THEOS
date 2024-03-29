use {
    std::mem,
    super::{
        DirectoryEntry,
        DIRECTORY_ENTRY_SIZE,
    }
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct RawVolumeGuid {
    entry_type: u8,
    secondary_count: u8,
    set_checksum: u16,
    general_flags: u16,
    volume_guid: u128,
    reserved: [u8; 0xa],
}

impl RawVolumeGuid {
    pub fn general_flags(&self) -> u16 {
        self.general_flags
    }

    pub fn volume_guid(&self) -> u128 {
        self.volume_guid
    }
}

impl From<&DirectoryEntry> for RawVolumeGuid {
    fn from(directory_entry: &DirectoryEntry) -> Self {
        let entry_type: u8 = directory_entry.entry_type().to_byte();
        match directory_entry {
            DirectoryEntry::VolumeGuid {
                general_flags,
                volume_guid,
            } => {
                let secondary_count: u8 = 0;
                let set_checksum: u16 = 0;
                let general_flags: u16 = Into::<u8>::into(general_flags) as u16;
                let volume_guid: u128 = *volume_guid;
                let reserved: [u8; 0xa] = [0; 0xa];
                let ref raw_volume_guid = Self {
                    entry_type,
                    secondary_count,
                    set_checksum,
                    general_flags,
                    volume_guid,
                    reserved,
                };
                let raw_volume_guid: [u8; DIRECTORY_ENTRY_SIZE] = raw_volume_guid.into();
                let bytes: Vec<u8> = raw_volume_guid.to_vec();
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
                    general_flags,
                    volume_guid,
                    reserved,
                }
            },
            _ => panic!("Can't convert a DirectoryEntry into a RawVolumeGuid."),
        }
    }
}

impl From<&[u8; DIRECTORY_ENTRY_SIZE]> for RawVolumeGuid {
    fn from(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

impl Into<[u8; DIRECTORY_ENTRY_SIZE]> for &RawVolumeGuid {
    fn into(self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<RawVolumeGuid, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }
}

