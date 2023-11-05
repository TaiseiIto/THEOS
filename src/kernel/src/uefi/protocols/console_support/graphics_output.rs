// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.9 Graphics Output Protocol

use {
    core::{
        mem,
        slice,
    },
    super::super::super::{
        services::boot::{
            memory_allocation,
            protocol_handler,
        },
        tables::system,
        types::{
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(Debug)]
#[repr(C)]
pub struct GraphicsOutput<'a> {
    query_mode: QueryMode,
    set_mode: SetMode,
    blt: Blt,
    mode: &'a Mode<'a>,
}

impl GraphicsOutput<'_> {
    #[allow(dead_code)]
    pub fn new() -> &'static Self {
        let guid = protocol_handler::Guid::new(
            0x9042a9de,
            0x23dc,
            0x4a38,
            [
                0x96,
                0xfb,
                0x7a,
                0xde,
                0xd0,
                0x80,
                0x51,
                0x6a
            ],
        );
        let graphics_output = void::Void::new();
        let mut graphics_output: &void::Void = &graphics_output;
        system::system()
            .boot_services
            .locate_protocol(
                &guid,
                void::Void::null(),
                &mut graphics_output,
            )
            .expect("Can't get a graphics output protocol!");
        let graphics_output: *const void::Void = &*graphics_output;
        let graphics_output: usize = graphics_output as usize;
        let graphics_output: *const Self = graphics_output as *const Self;
        unsafe {
            &*graphics_output
        }
    }

    pub fn write_pixel(&self, x: u32, y: u32, red: u8, green: u8, blue: u8) {
        self.mode.write_pixel(x, y, red, green, blue);
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct QueryMode(pub extern "efiapi" fn(&GraphicsOutput, u32, &usize, &mut &ModeInformation) -> status::Status);

#[derive(Debug)]
#[repr(C)]
struct ModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
    pixel_information: PixelBitMask,
    pixels_per_scan_line: u32,
}

impl ModeInformation {
    fn write_pixel(&self, frame_buffer: &mut [u32], x: u32, y: u32, red: u8, green: u8, blue: u8) {
        let red: u32 = red as u32;
        let green: u32 = green as u32;
        let blue: u32 = blue as u32;
        let color: u32 = match &self.pixel_format {
            PixelFormat::RedGreenBlueReserved8BitPerColor => red + (green << 8) + (blue << 16),
            PixelFormat::BlueGreenRedReserved8BitPerColor => blue + (green << 8) + (red << 16),
            PixelFormat::PixelBitMask => {
                let red_shift: usize = (0usize..32usize)
                    .filter(|shift| (red >> shift) << shift == red)
                    .max()
                    .expect("Can't write pixel!");
                let green_shift: usize = (0usize..32usize)
                    .filter(|shift| (green >> shift) << shift == green)
                    .max()
                    .expect("Can't write pixel!");
                let blue_shift: usize = (0usize..32usize)
                    .filter(|shift| (blue >> shift) << shift == blue)
                    .max()
                    .expect("Can't write pixel!");
                (red << red_shift) + (green << green_shift) + (blue << blue_shift)
            },
            PixelFormat::PixelBltOnly => 0x00000000, // Unimplemented
        };
        let offset: u32 = x + y * self.pixels_per_scan_line;
        let offset: usize = offset as usize;
        *frame_buffer
            .get_mut(offset)
            .expect("Cant write pixel!") = color;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
enum PixelFormat {
    RedGreenBlueReserved8BitPerColor,
    BlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBltOnly,
}

#[derive(Debug)]
#[repr(C)]
struct PixelBitMask {
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    reserved_mask: u32,
}

#[derive(WrappedFunction)]
#[repr(C)]
struct SetMode(pub extern "efiapi" fn(&GraphicsOutput, u32) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct Blt(pub extern "efiapi" fn(&GraphicsOutput, &BltPixel, BltOperation, usize, usize, usize, usize, usize, usize, usize) -> status::Status);

#[derive(Debug)]
#[repr(C)]
struct BltPixel {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo,
    Max,
}

#[derive(Debug)]
#[repr(C)]
struct Mode<'a> {
    max_mode: u32,
    info: &'a ModeInformation,
    size_of_info: usize,
    frame_buffer_base: memory_allocation::PhysicalAddress,
    frame_buffer_size: usize,
}

impl Mode<'_> {
    fn frame_buffer(&self) -> &mut [u32] {
        let frame_buffer_base: usize = self.frame_buffer_base as usize;
        let frame_buffer_base: *mut u32 = frame_buffer_base as *mut u32;
        let frame_buffer_size: usize = self.frame_buffer_size / (mem::size_of::<u32>() / mem::size_of::<u8>());
        unsafe {
            slice::from_raw_parts_mut(frame_buffer_base, frame_buffer_size)
        }
    }

    fn write_pixel(&self, x: u32, y: u32, red: u8, green: u8, blue: u8) {
        self.info.write_pixel(self.frame_buffer(), x, y, red, green, blue);
    }
}

