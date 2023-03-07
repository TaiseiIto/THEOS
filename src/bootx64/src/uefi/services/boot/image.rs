// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.4 Image Services

use {
    core::fmt,
    super::super::super::{
        protocols::device_path,
        types::{
            handle,
            status,
            void,
        },
    },
};

#[repr(C)]
pub struct LoadImage(extern "efiapi" fn(bool, handle::Handle<'_>, &device_path::DevicePathProtocol, &void::Void, usize, &mut handle::Handle<'_>) -> status::Status);

impl fmt::Debug for LoadImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

