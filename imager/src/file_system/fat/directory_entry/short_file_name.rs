#[repr(packed)]
pub struct ShortFileName {
    stem: [u8; STEM_LENGTH],
    extension: [u8; EXTENSION_LENGTH],
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

pub const STEM_LENGTH: usize = 8;
pub const EXTENSION_LENGTH: usize = 3;

