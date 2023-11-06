// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.3 String Protocol

use super::{
    FontStyle,
    super::super::types::char16,
};

// EFI_FONT_INFO
#[derive(Debug)]
#[repr(C)]
pub struct FontInfo<'a> {
    font_style: FontStyle,
    font_size: u16,
    font_name: char16::String<'a>,
}

