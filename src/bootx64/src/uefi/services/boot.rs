// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7 Boot Services

use {
    super::super::types::{
        char16,
        status,
        void,
    },
    wrapped_function::WrappedFunction,
};

pub mod event;
pub mod image;
pub mod memory_allocation;
pub mod protocol_handler;

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.5 Miscellaneous Boot Services

#[derive(WrappedFunction)]
#[repr(C)]
pub struct SetWatchdogTimer(pub extern "efiapi" fn(usize, u64, usize, char16::String) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct Stall(pub extern "efiapi" fn(usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CopyMem(pub extern "efiapi" fn(&void::Void, &void::Void, usize));

#[derive(WrappedFunction)]
#[repr(C)]
pub struct SetMem(pub extern "efiapi" fn(&void::Void, usize, u8));

#[derive(WrappedFunction)]
#[repr(C)]
pub struct GetNextMonotonicCount(pub extern "efiapi" fn(&mut u64) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct InstallConfigurationTable(pub extern "efiapi" fn(&protocol_handler::Guid, &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CalculateCrc32(pub extern "efiapi" fn(&void::Void, usize, &mut u32) -> status::Status);

