// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.5 File Protocol

#[derive(Debug)]
#[repr(C)]
pub struct FileProtocol {
    revision: u64,
}

