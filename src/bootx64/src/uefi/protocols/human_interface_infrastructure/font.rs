// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.1 Font Protocol

use {
    core::fmt,
    super::super::{
        super::{
            services::boot::{
                protocol_handler,
            },
            tables::system,
            types::{
                char16,
                status,
                void,
            },
        },
        console_support::graphics_output,
    },
    wrapped_function::WrappedFunction,
};

// EFI_HII_FONT_PROTOCOL
#[derive(Debug)]
#[repr(C)]
pub struct Font {
    string_to_image: StringToImage,
}

impl Font {
    pub fn new() -> &'static Self {
        let guid = protocol_handler::Guid::new(
            0xe9ca4775,
            0x8657,
            0x47fc,
            [
                0x97,
                0xe7,
                0x7e,
                0xd6,
                0x5a,
                0x08,
                0x43,
                0x24,
            ],
        );
        let font = void::Void::new();
        let mut font: &void::Void = &font;
        system::system()
            .boot_services
            .locate_protocol(
                &guid,
                void::Void::null(),
                &mut font,
            )
            .expect("Can't get a font protocol!");
        let font: *const void::Void = &*font;
        let font: usize = font as usize;
        let font: *const Self = font as *const Self;
        unsafe {
            &*font
        }
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct StringToImage(pub extern "efiapi" fn(&Font, OutFlags, char16::String, &FontDisplayInfo, &mut &ImageOutput, usize, usize, &mut &RowInfo, &mut usize, &mut usize) -> status::Status);

// EFI_FONT_DISPLAY_INFO
#[derive(Debug)]
#[repr(C)]
pub struct FontDisplayInfo<'a> {
    forground_color: graphics_output::BltPixel,
    background_color: graphics_output::BltPixel,
    font_info_mask: FontInfoMask,
    font_info: FontInfo<'a>,
}

// EFI_FONT_INFO
#[derive(Debug)]
#[repr(C)]
pub struct FontInfo<'a> {
    font_style: FontStyle,
    font_size: u16,
    font_name: char16::String<'a>,
}

// EFI_FONT_INFO_MASK
pub type FontInfoMask = u32;

// EFI_HII_FONT_STYLE
pub type FontStyle = u32;

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

// EFI_HII_OUT_FLAGS
pub type OutFlags = u32;

// EFI_HII_ROW_INFO
#[derive(Debug)]
#[repr(C)]
pub struct RowInfo {
    start_index: usize,
    end_index: usize,
    line_height: usize,
    line_width: usize,
    base_line_offset: usize,
}

