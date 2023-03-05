use super::{
    char16::Char16,
    table_header::TableHeader,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.3 System Table
#[derive(Debug)]
#[repr(C)]
pub struct SystemTable {
    header: TableHeader,
    firmware_vendor: Char16,
}

