use super::header;

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.5 Runtime Services Table
#[derive(Clone, Debug)]
pub struct RuntimeServices {
    header: header::Header,
}

