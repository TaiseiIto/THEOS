// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.1 Font Protocol

use {
    alloc::collections::btree_map,
    super::{
        database,
        font_ex,
        StringId,
        super::{
            console_support::graphics_output,
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

    pub fn get_glyph(&self, font: &font_ex::FontDisplayInfo<'_>, character: char) -> btree_map::BTreeMap<Coordinates, bool> {
        let character: u32 = character as u32;
        let character: char16::Char16 = character
            .try_into()
            .expect("Can't get a glyph");
        let blt: usize = 0;
        let blt: *const font_ex::ImageOutput = blt as *const font_ex::ImageOutput;
        let mut blt: &font_ex::ImageOutput = unsafe {
            &*blt
        };
        let baseline: usize = 0;
        let baseline: *mut usize = baseline as *mut usize;
        let baseline: &mut usize = unsafe {
            &mut *baseline
        };
        self.get_glyph.0(
            self,
            character,
            font,
            &mut blt,
            baseline,
        );
        let width: u16 = blt.width();
        let height: u16 = blt.height();
        let bitmap: &[graphics_output::BltPixel] = blt.bitmap();
        (0..width)
            .flat_map(|x| (0..height)
                .map(move |y| (Coordinates::new(x, y), bitmap.get((x + y * width) as usize) == Some(font.foreground_color()))))
            .collect()
    }

    pub fn iter<'a>(&'a self) -> FontIterator<'a> {
        let protocol: &Self = self;
        let handle: FontHandle<'a> = void::Void::null();
        FontIterator {
            protocol,
            handle,
        }
    }
}

#[derive(Debug)]
pub struct FontIterator<'a> {
    protocol: &'a Font,
    handle: FontHandle<'a>,
}

impl<'a> Iterator for FontIterator<'a> {
    type Item = &'a font_ex::FontDisplayInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let input_font_display_info: &'a font_ex::FontDisplayInfo = font_ex::FontDisplayInfo::null();
        let mut output_font_display_info: &'a font_ex::FontDisplayInfo = font_ex::FontDisplayInfo::null();
        self.protocol.get_font_info.0(
            self.protocol,
            &mut self.handle,
            input_font_display_info,
            &mut output_font_display_info,
            char16::String::null(),
        );
        let output_checker: *const font_ex::FontDisplayInfo = output_font_display_info as *const font_ex::FontDisplayInfo;
        let output_checker: usize = output_checker as usize;
        match output_checker {
            0 => None,
            _ => Some(output_font_display_info),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates {
    x: u16,
    y: u16,
}

impl Coordinates {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
        }
    }
}

