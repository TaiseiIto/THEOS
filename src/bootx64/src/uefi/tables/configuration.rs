// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.6 EFI Configuration Table & Propaties Table

use super::super::{
    services::boot::protocol_handler,
    types::void,
};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Configuration<'a> {
    vendor_guid: protocol_handler::Guid,
    vendor_table: &'a void::Void,
}

