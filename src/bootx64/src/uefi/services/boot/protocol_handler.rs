// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.3 Protocol Hander Services

use {
    super::super::super::{
        protocols::device_path,
        types::{
            event,
            handle,
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(WrappedFunction)]
#[repr(C)]
pub struct InstallProtocolInterface(pub extern "efiapi" fn(&mut handle::Handle<'_>, &Guid, InterfaceType, &void::Void) -> status::Status);

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    pub fn new(
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    ) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

#[allow(dead_code)]
#[repr(C)]
pub enum InterfaceType {
    NativeInterface,
}

#[derive(WrappedFunction)]
#[repr(C)]
pub struct UninstallProtocolInterface(pub extern "efiapi" fn(handle::Handle<'_>, &Guid, &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct ReinstallProtocolInterface(pub extern "efiapi" fn(handle::Handle<'_>, &Guid, &void::Void, &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct RegisterProtocolNotify(pub extern "efiapi" fn(&Guid, event::Event<'_>, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateHandle(pub extern "efiapi" fn(LocateSearchType, &Guid, &void::Void, &mut usize, &mut handle::Handle<'_>) -> status::Status);

#[allow(dead_code)]
#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

#[derive(WrappedFunction)]
#[repr(C)]
pub struct HandleProtocol(pub extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateDevicePath(pub extern "efiapi" fn(&Guid, &mut &device_path::DevicePathProtocol, &mut handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct OpenProtocol(pub extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &void::Void, handle::Handle<'_>, handle::Handle<'_>, u32) -> status::Status);

pub const OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CloseProtocol(pub extern "efiapi" fn(handle::Handle<'_>, &Guid, handle::Handle<'_>, handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct OpenProtocolInformation(pub extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &OpenProtocolInformationEntry<'_>, usize) -> status::Status);

#[repr(C)]
pub struct OpenProtocolInformationEntry<'a> {
    agent_handle: handle::Handle<'a>,
    controller_handle: handle::Handle<'a>,
    attributes: u32,
    open_count: u32,
}

#[derive(WrappedFunction)]
#[repr(C)]
pub struct ConnectController(pub extern "efiapi" fn(handle::Handle<'_>, &handle::Handle<'_>, &device_path::DevicePathProtocol, bool) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct DisconnectController(pub extern "efiapi" fn(handle::Handle<'_>, handle::Handle<'_>, handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct ProtocolsPerHandle(pub extern "efiapi" fn(handle::Handle<'_>, &mut &&Guid, &mut usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateHandleBuffer(pub extern "efiapi" fn(LocateSearchType, &Guid, &void::Void, &mut usize, &mut &handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateProtocol(pub extern "efiapi" fn(&Guid, &void::Void, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct InstallMultipleProtocolInterfaces(pub extern "efiapi" fn(handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct UninstallMultipleProtocolInterfaces(pub extern "efiapi" fn(handle::Handle<'_>) -> status::Status);

