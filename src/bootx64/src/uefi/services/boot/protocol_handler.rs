// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.3 Protocol Hander Services

use {
    core::fmt,
    super::super::super::types::{
        handle,
        status,
        void,
    },
};

#[repr(C)]
pub struct InstallProtocolInterface(extern "efiapi" fn(&mut handle::Handle<'_>, &Guid, InterfaceType, &void::Void) -> status::Status);

impl fmt::Debug for InstallProtocolInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

#[allow(dead_code)]
#[repr(C)]
pub enum InterfaceType {
    NativeInterface,
}

