// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.9 Graphics Output Protocol

use {
    super::super::super::{
        services::boot::protocol_handler,
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
pub struct GraphicsOutput {
    query_mode: QueryMode,
}

impl GraphicsOutput {
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
}

#[derive(Debug)]
#[repr(C)]
pub struct ModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
}

#[derive(WrappedFunction)]
#[repr(C)]
struct QueryMode(pub extern "efiapi" fn(&GraphicsOutput, u32, &usize, &mut &ModeInformation) -> status::Status);

