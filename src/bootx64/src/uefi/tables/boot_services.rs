use super::{
    header,
    super::services::boot::event,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.4 Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices {
    header: header::Header,
    raise_tpl: event::RaiseTpl,
}

