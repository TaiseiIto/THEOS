// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.3 Protocol Hander Services

use {
    core::fmt,
    super::super::super::{
        protocols::device_path,
        types::{
            event,
            handle,
            status,
            void,
        },
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
pub struct RegisterProtocolNotify(extern "efiapi" fn(&Guid, event::Event<'_>, &mut &void::Void) -> status::Status);

impl fmt::Debug for RegisterProtocolNotify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct LocateHandle(extern "efiapi" fn(LocateSearchType, &Guid, &void::Void, &mut usize, &mut handle::Handle<'_>) -> status::Status);

impl fmt::Debug for LocateHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[allow(dead_code)]
#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

#[repr(C)]
pub struct HandleProtocol(extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &void::Void) -> status::Status);

impl fmt::Debug for HandleProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct LocateDevicePath(extern "efiapi" fn(&Guid, &mut &device_path::DevicePathProtocol, &mut handle::Handle<'_>) -> status::Status);

impl fmt::Debug for LocateDevicePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct OpenProtocol(extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &void::Void, handle::Handle<'_>, handle::Handle<'_>, u32) -> status::Status);

impl fmt::Debug for OpenProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct CloseProtocol(extern "efiapi" fn(handle::Handle<'_>, &Guid, handle::Handle<'_>, handle::Handle<'_>) -> status::Status);

impl fmt::Debug for CloseProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct OpenProtocolInformation(extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &OpenProtocolInformationEntry<'_>, usize) -> status::Status);

impl fmt::Debug for OpenProtocolInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct OpenProtocolInformationEntry<'a> {
    agent_handle: handle::Handle<'a>,
    controller_handle: handle::Handle<'a>,
    attributes: u32,
    open_count: u32,
}

#[repr(C)]
pub struct ConnectController(extern "efiapi" fn(handle::Handle<'_>, &handle::Handle<'_>, &device_path::DevicePathProtocol, bool) -> status::Status);

impl fmt::Debug for ConnectController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct DisconnectController(extern "efiapi" fn(handle::Handle<'_>, handle::Handle<'_>, handle::Handle<'_>) -> status::Status);

impl fmt::Debug for DisconnectController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct ProtocolsPerHandle(extern "efiapi" fn(handle::Handle<'_>, &mut &&Guid, &mut usize) -> status::Status);

impl fmt::Debug for ProtocolsPerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

