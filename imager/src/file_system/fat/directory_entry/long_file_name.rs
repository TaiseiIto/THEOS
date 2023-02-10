#[repr(packed)]
pub struct LongFileName {
    order: u8,
    name0: [u16; 5],
    attribute: u8,
    reserved0: u8,
    checksum: u8,
    name1: [u16; 6],
    reserved1: u16,
    name2: [u16; 2],
}

