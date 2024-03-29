use {
    std::mem,
    super::{
        attribute,
        DirectoryEntry,
        DIRECTORY_ENTRY_SIZE,
        name_flags,
        super::super::super::time,
    },
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct ShortFileName {
    stem: [u8; STEM_LENGTH],
    extension: [u8; EXTENSION_LENGTH],
    attribute: u8,
    name_flags: u8,
    created_time_centi_second: u8,
    created_time: u32,
    accessed_date: u16,
    cluster_high: u16,
    written_time: u32,
    cluster_low: u16,
    size: u32,
}

pub const STEM_LENGTH: usize = 8;
pub const EXTENSION_LENGTH: usize = 3;
pub const BASENAME_LENGTH: usize = STEM_LENGTH + EXTENSION_LENGTH;

impl ShortFileName {
    pub fn accessed_time(&self) -> time::Time {
        let accessed_time: u32 = (self.accessed_date as u32) << 16;
        let t_10ms_increment: u8 = 0;
        let utc_offset: i8 = 0;
        time::Time::from_fat_timestamp(accessed_time, t_10ms_increment, utc_offset)
    }

    pub fn attribute(&self) -> attribute::Attribute {
        self.attribute.into()
    }

    pub fn cluster(&self) -> u32 {
        let cluster_high: u32 = self.cluster_high as u32;
        let cluster_low: u32 = self.cluster_low as u32;
        cluster_low + (cluster_high << 16)
    }

    pub fn created_time(&self) -> time::Time {
        let utc_offset: i8 = 0;
        time::Time::from_fat_timestamp(self.created_time, self.created_time_centi_second, utc_offset)
    }

    pub fn extension(&self) -> [u8; EXTENSION_LENGTH] {
        self.extension
    }

    pub fn name_flags(&self) -> name_flags::NameFlags {
        self.name_flags.into()
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }

    pub fn stem(&self) -> [u8; STEM_LENGTH] {
        self.stem
    }

    pub fn written_time(&self) -> time::Time {
        let t_10ms_increment: u8 = 0;
        let utc_offset: i8 = 0;
        time::Time::from_fat_timestamp(self.written_time, t_10ms_increment, utc_offset)
    }
}

impl From<&DirectoryEntry> for ShortFileName {
    fn from(directory_entry: &DirectoryEntry) -> Self {
        if let DirectoryEntry::ShortFileName {
                stem,
                extension,
                attribute,
                name_flags,
                created_time,
                accessed_time,
                written_time,
                cluster,
                size,
                long_file_name: _,
                checksum: _,
        } = directory_entry {
            let stem: [u8; STEM_LENGTH] = *stem.borrow();
            let extension: [u8; EXTENSION_LENGTH] = *extension;
            let attribute: u8 = attribute.into();
            let name_flags: u8 = name_flags.into();
            let created_time_centi_second: u8 = created_time.fat_centi_second();
            let created_time: u32 = created_time.fat_timestamp();
            let accessed_date: u16 = (accessed_time.fat_timestamp() >> 16) as u16;
            let cluster: u32 = match *cluster.borrow() {
                Some(cluster) => cluster,
                None => 0,
            };
            let cluster_high: u16 = (cluster >> 16) as u16;
            let written_time: u32 = written_time.fat_timestamp();
            let cluster_low: u16 = cluster as u16;
            let size: u32 = *size as u32;
            Self {
                stem,
                extension,
                attribute,
                name_flags,
                created_time_centi_second,
                created_time,
                accessed_date,
                cluster_high,
                written_time,
                cluster_low,
                size,
            }
        } else {
            panic!("Can't generate a short file name.")
        } 
    }
}

impl From<&[u8; DIRECTORY_ENTRY_SIZE]> for ShortFileName {
    fn from(bytes: &[u8; DIRECTORY_ENTRY_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; DIRECTORY_ENTRY_SIZE], Self>(*bytes)
        }
    }
}

impl Into<[u8; DIRECTORY_ENTRY_SIZE]> for &ShortFileName {
    fn into(self) -> [u8; DIRECTORY_ENTRY_SIZE] {
        unsafe {
            mem::transmute::<ShortFileName, [u8; DIRECTORY_ENTRY_SIZE]>(*self)
        }
    }
}

impl Into<Vec<u8>> for &ShortFileName {
    fn into(self) -> Vec<u8> {
        let bytes: [u8; DIRECTORY_ENTRY_SIZE] = self.into();
        bytes.to_vec()
    }
}

