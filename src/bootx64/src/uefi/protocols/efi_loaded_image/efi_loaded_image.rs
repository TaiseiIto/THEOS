// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 9.1 EFI Loaded Image Protocol

use super::super::super::{
    services::boot::protocol_handler,
    tables::system,
    types::{
        handle,
        void,
    },
};

#[derive(Debug)]
#[repr(C)]
pub struct EfiLoadedImage<'a> {
    revision: u32,
    parent_handle: handle::Handle<'a>,
    system_table: &'a system::System<'a>,
    device_handle: handle::Handle<'a>,
}

impl EfiLoadedImage<'_> {
    pub fn new<'a>() -> &'a Self {
        let guid = protocol_handler::Guid::new(
            0x5b1b31a1,
            0x9562,
            0x11d2,
            [
                0x8e,
                0x3f,
                0x00,
                0xa0,
                0xc9,
                0x69,
                0x72,
                0x3b
            ],
        );
        let efi_loaded_image = void::Void::new();
        let mut efi_loaded_image: &void::Void = &efi_loaded_image;
        system::system()
            .boot_services
            .open_protocol(
                system::image(),
                &guid,
                &mut efi_loaded_image,
                system::image(),
                void::Void::null(),
                protocol_handler::OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            )
            .expect("Can't get an EFI loaded image protocol!");
        let efi_loaded_image: *const void::Void = &*efi_loaded_image;
        let efi_loaded_image: usize = efi_loaded_image as usize;
        let efi_loaded_image: *const Self = efi_loaded_image as *const Self;
        unsafe {
            &*efi_loaded_image
        }
    }
}

