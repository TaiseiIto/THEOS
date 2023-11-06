// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34 HII (Human Interface Infrastructure) Protocols

pub mod font;
pub mod font_ex;
pub mod string;
pub mod database;

// EFI_HII_FONT_STYLE
pub type FontStyle = u32;

// EFI_STRING_ID
pub type StringId = u16;

