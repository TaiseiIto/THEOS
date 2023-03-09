// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.4 Image Services

use {
    super::super::super::{
        protocols::device_path,
        types::{
            char16,
            handle,
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(WrappedFunction)]
#[repr(C)]
pub struct LoadImage(pub extern "efiapi" fn(bool, handle::Handle<'_>, &device_path::DevicePathProtocol, &void::Void, usize, &mut handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct StartImage(pub extern "efiapi" fn(handle::Handle<'_>, &mut usize, &mut char16::String) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct UnloadImage(pub extern "efiapi" fn(handle::Handle<'_>) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct Exit(pub extern "efiapi" fn(handle::Handle<'_>, status::Status, usize, char16::String) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct ExitBootServices(pub extern "efiapi" fn(handle::Handle<'_>, usize) -> status::Status);

