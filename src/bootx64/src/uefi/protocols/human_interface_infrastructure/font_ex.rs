// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.2 EFI HII Font Ex Protoxol

use {
    core::fmt,
    super::{
        string,
        super::console_support::graphics_output,
    },
};

// EFI_FONT_DISPLAY_INFO
#[derive(Debug)]
#[repr(C)]
pub struct FontDisplayInfo<'a> {
    forground_color: graphics_output::BltPixel,
    background_color: graphics_output::BltPixel,
    font_info_mask: FontInfoMask,
    font_info: string::FontInfo<'a>,
}

// EFI_FONT_INFO_MASK
pub type FontInfoMask = u32;

// EFI_IMAGE_OUTPUT
#[derive(Debug)]
#[repr(C)]
pub struct ImageOutput<'a> {
    width: u16,
    height: u16,
    bitmap_or_screen: ImageOutputUnion<'a>,
}

#[allow(dead_code)]
pub union ImageOutputUnion<'a> {
    bitmap: &'a graphics_output::BltPixel,
    screen: &'a graphics_output::GraphicsOutput<'a>,
}

impl fmt::Debug for ImageOutputUnion<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(formatter, "ImageOutputUnion")
    }
}

