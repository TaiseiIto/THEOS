// Reference
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 2.3.1 Data Types
#[derive(Debug)]
pub struct Char16 {
    character: *const u16,
}

