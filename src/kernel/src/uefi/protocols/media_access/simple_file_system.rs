// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.4 Simple File System Protocol

extern crate alloc;

use {
    alloc::vec::Vec,
    core::str,
    super::{
        file_protocol,
        super::{
            efi_loaded_image::efi_loaded_image,
            super::{
                services::boot::protocol_handler,
                tables::system,
                types::{
                    status,
                    void,
                },
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
    #[allow(dead_code)]
    pub fn new<'a>() -> &'a Self {
        let guid = protocol_handler::Guid::new(
            0x964e5b22,
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
            .open_protocol(
                efi_loaded_image::EfiLoadedImage::new().device_handle(),
                &guid,
                &mut simple_file_system,
                system::image(),
                void::Void::null(),
                protocol_handler::OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            )
            .expect("Can't get a simple file system protocol!");
        let simple_file_system: *const void::Void = &*simple_file_system;
        let simple_file_system: usize = simple_file_system as usize;
        let simple_file_system: *const Self = simple_file_system as *const Self;
        unsafe {
            &*simple_file_system
        }
    }
    
    pub fn open_volume<'a>(&self) -> &'a file_protocol::FileProtocol {
        let volume = void::Void::new();
        let volume: &void::Void = &volume;
        let volume: *const void::Void = &*volume;
        let volume: usize = volume as usize;
        let volume: *const file_protocol::FileProtocol = volume as *const file_protocol::FileProtocol;
        let mut volume: &file_protocol::FileProtocol = unsafe {
            &*volume
        };
        match self.open_volume.0(self, &mut volume) {
            status::SUCCESS => volume,
            _ => panic!("Can't open the volume!"),
        }
    }

    #[allow(dead_code)]
    pub fn read_file(&self, path: &str) -> Vec<u8> {
        let mut path: str::Split<char> = path.split('/');
        path.next().expect("Can't read a file!");
        let name: &str = path.next().expect("Can't read a file!");
        path
            .fold(
                file_protocol::Node::root_child(self, name),
                |node, name| node.child(name)
            )
            .read_file()
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct OpenVolume(pub extern "efiapi" fn(&SimpleFileSystem, &mut &file_protocol::FileProtocol) -> status::Status);

