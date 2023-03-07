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

#[repr(C)]
pub struct UninstallProtocolInterface(extern "efiapi" fn(handle::Handle<'_>, &Guid, &void::Void) -> status::Status);

impl fmt::Debug for UninstallProtocolInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct ReinstallProtocolInterface(extern "efiapi" fn(handle::Handle<'_>, &Guid, &void::Void, &void::Void) -> status::Status);

impl fmt::Debug for ReinstallProtocolInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct HandleProtocol(extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &void::Void) -> status::Status);

impl fmt::Debug for HandleProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

