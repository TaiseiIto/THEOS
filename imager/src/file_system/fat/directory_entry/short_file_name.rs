use {
    std::convert::From,
    super::DirectoryEntry,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct ShortFileName {
    name: [u8; NAME_LENGTH],
    attribute: u8,
    name_flags: u8,
    created_time_centi_second: u8,
    created_time: u32,
    accessed_date: u16,
    first_cluster_high: u16,
    written_time: u32,
    first_cluster_low: u16,
    file_size: u32,
}

pub const NAME_LENGTH: usize = 11;

impl From<&DirectoryEntry> for ShortFileName {
    fn from(directory_entry: &DirectoryEntry) -> Self {
        if let DirectoryEntry::ShortFileName {
            name,
            attribute,
            accessed_time,
            created_time,
            written_time,
            first_cluster,
            size,
            long_file_name,
        } = directory_entry {
            let irreversible: bool = false;
            let name: String = name.to_uppercase();
            let mut name: Vec<u8> = name.into_bytes();
            name.resize(NAME_LENGTH, 0x20);
            let name: [u8; NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a short file name.");
            let attribute: u8 = attribute.into();
            let name_flags: u8 = 0;
            let created_time_centi_second: u8 = created_time.fat_centi_second();
            let created_time: u32 = created_time.fat_timestamp();
            let accessed_date: u16 = (accessed_time.fat_timestamp() >> 16) as u16;
            let first_cluster_high: u16 = (first_cluster >> 16) as u16;
            let written_time: u32 = written_time.fat_timestamp();
            let first_cluster_low: u16 = *first_cluster as u16;
            let file_size: u32 = *size as u32;
            Self {
                name,
                attribute,
                name_flags,
                created_time_centi_second,
                created_time,
                accessed_date,
                first_cluster_high,
                written_time,
                first_cluster_low,
                file_size,
            }
        } else {
            panic!("Can't generate a short file name.");
        }
    }
}

