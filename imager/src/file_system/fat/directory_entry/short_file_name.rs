use super::attribute;

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct ShortFileName {
    name: [u8; 11],
    attribute: u8,
    name_flags: u8,
    created_time_centi_second: u8,
    created_time: u16,
    created_date: u16,
    accessed_date: u16,
    first_cluster_high: u16,
    written_time: u16,
    written_date: u16,
    first_cluster_low: u16,
    file_size: u32,
}

