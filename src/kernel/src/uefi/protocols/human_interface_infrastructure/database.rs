// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.8 Database Protocol

use super::super::super::types::void;

// EFI_HII_HANDLE
pub type Handle<'a> = &'a void::Void;

