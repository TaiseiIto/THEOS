// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.4 Simple File System Protocol

use {
    super::{
        file_protocol,
        super::super::{
            services::boot::protocol_handler,
            tables::system,
            types::{
                status,
                void,
            },
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(Debug)]
#[repr(C)]
pub struct SimpleFileSystem {
    revision: u64,
    open_volume: OpenVolume,
}

impl SimpleFileSystem {
    pub fn new() -> Self {
        let guid: protocol_handler::Guid = protocol_handler::Guid::new(
            0x0964e5b22,
            0x6459,
            0x11d2,
            [
                0x8e,
                0x39,
                0x00,
                0xa0,
                0xc9,
                0x69,
                0x72,
                0x3b
            ],
        );
        let simple_file_system = void::Void::new();
        let mut simple_file_system: &void::Void = &simple_file_system;
        system::system()
            .boot_services
            .handle_protocol(
                system::image(),
                &guid,
                &mut simple_file_system,
            )
            .expect("Can't get a simple file system protocol!");
        let simple_file_system: *const void::Void = &*simple_file_system;
        let simple_file_system: usize = simple_file_system as usize;
        let simple_file_system: *const Self = simple_file_system as *const Self;
        let revision: u64 = unsafe {
            (*simple_file_system).revision
        };
        let open_volume = unsafe {
            (*simple_file_system).open_volume.0
        };
        let open_volume = OpenVolume(open_volume);
        Self {
            revision,
            open_volume,
        }
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct OpenVolume(pub extern "efiapi" fn(&SimpleFileSystem, &mut &file_protocol::FileProtocol) -> status::Status);

