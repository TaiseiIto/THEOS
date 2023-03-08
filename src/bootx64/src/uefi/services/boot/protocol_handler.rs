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
pub struct InstallProtocolInterface(extern "efiapi" fn(&mut handle::Handle<'_>, &Guid, InterfaceType, &void::Void) -> status::Status);

#[derive(Clone, Debug)]
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

#[derive(WrappedFunction)]
#[repr(C)]
pub struct UninstallProtocolInterface(extern "efiapi" fn(handle::Handle<'_>, &Guid, &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct ReinstallProtocolInterface(extern "efiapi" fn(handle::Handle<'_>, &Guid, &void::Void, &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct RegisterProtocolNotify(extern "efiapi" fn(&Guid, event::Event<'_>, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateHandle(extern "efiapi" fn(LocateSearchType, &Guid, &void::Void, &mut usize, &mut handle::Handle<'_>) -> status::Status);

#[allow(dead_code)]
#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

#[derive(WrappedFunction)]
#[repr(C)]
pub struct HandleProtocol(extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateDevicePath(extern "efiapi" fn(&Guid, &mut &device_path::DevicePathProtocol, &mut handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct OpenProtocol(extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &void::Void, handle::Handle<'_>, handle::Handle<'_>, u32) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CloseProtocol(extern "efiapi" fn(handle::Handle<'_>, &Guid, handle::Handle<'_>, handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct OpenProtocolInformation(extern "efiapi" fn(handle::Handle<'_>, &Guid, &mut &OpenProtocolInformationEntry<'_>, usize) -> status::Status);

#[repr(C)]
pub struct OpenProtocolInformationEntry<'a> {
    agent_handle: handle::Handle<'a>,
    controller_handle: handle::Handle<'a>,
    attributes: u32,
    open_count: u32,
}

#[derive(WrappedFunction)]
#[repr(C)]
pub struct ConnectController(extern "efiapi" fn(handle::Handle<'_>, &handle::Handle<'_>, &device_path::DevicePathProtocol, bool) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct DisconnectController(extern "efiapi" fn(handle::Handle<'_>, handle::Handle<'_>, handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct ProtocolsPerHandle(extern "efiapi" fn(handle::Handle<'_>, &mut &&Guid, &mut usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateHandleBuffer(extern "efiapi" fn(LocateSearchType, &Guid, &void::Void, &mut usize, &mut &handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LocateProtocol(extern "efiapi" fn(&Guid, &void::Void, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct InstallMultipleProtocolInterfaces(extern "efiapi" fn(handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct UninstallMultipleProtocolInterfaces(extern "efiapi" fn(handle::Handle<'_>) -> status::Status);

