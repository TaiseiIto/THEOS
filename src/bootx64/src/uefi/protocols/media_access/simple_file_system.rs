// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.4 Simple File System Protocol

use {
    super::{
        file_protocol,
        super::super::types::status,
    },
    wrapped_function::WrappedFunction,
};

#[derive(Debug)]
#[repr(C)]
pub struct SimpleFileSystem {
    revision: u64,
    open_volume: OpenVolume,
}

#[derive(WrappedFunction)]
#[repr(C)]
struct OpenVolume(pub extern "efiapi" fn(&SimpleFileSystem, &mut &file_protocol::FileProtocol) -> status::Status);

