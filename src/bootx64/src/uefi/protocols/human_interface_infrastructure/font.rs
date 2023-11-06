// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.1 Font Protocol

use {
    super::{
        database,
        font_ex,
        StringId,
        super::{
            super::{
                services::boot::protocol_handler,
                tables::system,
                types::{
                    char8,
                    char16,
                    status,
                    void,
                },
            },
        },
    },
    wrapped_function::WrappedFunction,
};

// EFI_HII_FONT_PROTOCOL
#[derive(Debug)]
#[repr(C)]
pub struct Font {
    string_to_image: StringToImage,
    string_id_to_image: StringIdToImage,
    get_glyph: GetGlyph,
    get_font_info: GetFontInfo,
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

// EFI_HII_STRING_TO_IMAGE
#[derive(WrappedFunction)]
#[repr(C)]
struct StringToImage(pub extern "efiapi" fn(&Font, OutFlags, char16::String, &font_ex::FontDisplayInfo, &mut &font_ex::ImageOutput, usize, usize, &mut &RowInfo, &mut usize, &mut usize) -> status::Status);

// EFI_HII_STRING_ID_TO_IMAGE
#[derive(WrappedFunction)]
#[repr(C)]
struct StringIdToImage(pub extern "efiapi" fn(&Font, OutFlags, database::Handle, StringId, &char8::Char8, &font_ex::FontDisplayInfo, &mut &font_ex::ImageOutput, usize, usize, &mut &RowInfo, &mut usize, &mut usize) -> status::Status);

// EFI_HII_GET_GLYPH
#[derive(WrappedFunction)]
#[repr(C)]
struct GetGlyph(pub extern "efiapi" fn(&Font, char16::Char16, &font_ex::FontDisplayInfo, &mut &font_ex::ImageOutput, &mut usize) -> status::Status);

// EFI_HII_GET_FONT_INFO
#[derive(WrappedFunction)]
#[repr(C)]
struct GetFontInfo(pub extern "efiapi" fn(&Font, &mut FontHandle<'_>, &font_ex::FontDisplayInfo, &mut &font_ex::FontDisplayInfo, char16::String) -> status::Status);

// EFI_FONT_HANDLE
pub type FontHandle<'a> = &'a void::Void;

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

