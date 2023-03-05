use super::{
    char16,
    table_header,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.3 System Table
#[derive(Debug)]
#[repr(C)]
pub struct SystemTable {
    header: table_header::TableHeader,
    firmware_vendor: char16::String,
    firmware_revision: u32,
}

