use super::{
    header,
    super::services::runtime::time,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.5 Runtime Services Table
#[derive(Debug)]
#[repr(C)]
pub struct RuntimeServices {
    header: header::Header,
    get_time: time::GetTime,
    set_time: time::SetTime,
}

