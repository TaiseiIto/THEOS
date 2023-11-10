// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.3 String Protocol

use super::{
    FontStyle,
    super::super::types::char16,
};

// EFI_FONT_INFO
#[repr(C)]
pub struct FontInfo<'a> {
    font_style: FontStyle,
    font_size: u16,
    font_name: char16::String<'a>,
}

impl<'a> FontInfo<'a> {
    pub fn font_style(&'a self) -> &'a FontStyle {
        &self.font_style
    }

    pub fn font_size(&'a self) -> &'a u16 {
        &self.font_size
    }

    pub fn font_name(&'a self) -> &'a char16::String<'a> {
        &self.font_name
    }
}

