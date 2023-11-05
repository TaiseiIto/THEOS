// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.9 Graphics Output Protocol

use {
    crate::{
        serial_print,
        serial_println,
    },
    core::slice,
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
        serial_println!("write_pixel({:#x?}, {:#x?}, {:#x?}, {:#x?}, {:#x?})", x, y, red, green, blue);
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct QueryMode(pub extern "efiapi" fn(&GraphicsOutput, u32, &usize, &mut &ModeInformation) -> status::Status);

#[derive(Debug)]
#[repr(C)]
pub struct ModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
    pixel_information: PixelBitMask,
    pixels_per_scan_line: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum PixelFormat {
    RedGreenBlueReserved8BitPerColor,
    BlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBltOnly,
    PixelFormatMax,
}

#[derive(Debug)]
#[repr(C)]
pub struct PixelBitMask {
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
pub struct BltPixel {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo,
    Max,
}

#[derive(Debug)]
#[repr(C)]
pub struct Mode<'a> {
    max_mode: u32,
    info: &'a ModeInformation,
    size_of_info: usize,
    frame_buffer_base: memory_allocation::PhysicalAddress,
    frame_buffer_size: usize,
}

