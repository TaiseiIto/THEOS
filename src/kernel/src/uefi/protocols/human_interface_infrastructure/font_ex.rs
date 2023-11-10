// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 34.2 EFI HII Font Ex Protoxol

use {
    core::{
        fmt,
        slice,
    },
    super::{
        string,
        super::console_support::graphics_output,
    },
};

// EFI_FONT_DISPLAY_INFO
#[repr(C)]
pub struct FontDisplayInfo<'a> {
    foreground_color: graphics_output::BltPixel,
    background_color: graphics_output::BltPixel,
    font_info_mask: FontInfoMask,
    font_info: string::FontInfo<'a>,
}

impl<'a> FontDisplayInfo<'a> {
    pub fn null() -> &'a Self {
        let null: usize = 0;
        let null: *const Self = null as *const Self;
        unsafe {
            &*null
        }
    }

    #[allow(dead_code)]
    pub fn foreground_color(&self) -> &graphics_output::BltPixel {
        &self.foreground_color
    }
}

impl fmt::Debug for FontDisplayInfo<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut formatter = formatter
            .debug_struct("FontDisplayInfo");
        let formatter = formatter
            .field("foreground_color", &self.foreground_color)
            .field("background_color", &self.background_color)
            .field("font_info_mask", &self.font_info_mask);
        let formatter = if self.font_info_mask.sys_font() {
            formatter
        } else {
            formatter.field("font_info.font_name", self.font_info.font_name())
        };
        let formatter = if self.font_info_mask.sys_size() {
            formatter
        } else {
            formatter.field("font_info.font_size", self.font_info.font_size())
        };
        let formatter = if self.font_info_mask.sys_style() {
            formatter
        } else {
            formatter.field("font_info.font_style", self.font_info.font_style())
        };
        formatter.finish()
    }
}

// EFI_FONT_INFO_MASK
#[derive(Debug)]
pub struct FontInfoMask(u32);

impl FontInfoMask {
    const SYS_FONT: u32 = 0x00000001;
    const SYS_SIZE: u32 = 0x00000002;
    const SYS_STYLE: u32 = 0x00000004;

    pub fn sys_font(&self) -> bool {
        self.0 & Self::SYS_FONT != 0
    }

    pub fn sys_size(&self) -> bool {
        self.0 & Self::SYS_SIZE != 0
    }

    pub fn sys_style(&self) -> bool {
        self.0 & Self::SYS_STYLE != 0
    }
}

// EFI_IMAGE_OUTPUT
#[derive(Debug)]
#[repr(C)]
pub struct ImageOutput<'a> {
    width: u16,
    height: u16,
    bitmap_or_screen: ImageOutputUnion<'a>,
}

impl ImageOutput<'_> {
    #[allow(dead_code)]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[allow(dead_code)]
    pub fn height(&self) -> u16 {
        self.height
    }

    #[allow(dead_code)]
    pub fn bitmap(&self) -> &[graphics_output::BltPixel] {
        let bitmap: &graphics_output::BltPixel = unsafe {
            self.bitmap_or_screen.bitmap
        };
        let size: u16 = self.width * self.height;
        let size: usize = size as usize;
        unsafe {
            slice::from_raw_parts(bitmap, size)
        }
    }
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

