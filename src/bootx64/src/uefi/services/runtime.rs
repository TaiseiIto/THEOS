// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8 Runtime Services

pub mod time;
pub mod variable;
pub mod virtual_memory;

use {
    super::{
        boot::{
            memory_allocation,
            protocol_handler,
        },
        super::types::{
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.5.1 Reset System
#[derive(WrappedFunction)]
#[repr(C)]
pub struct ResetSystem(pub extern "efiapi" fn(ResetType, status::Status, usize, &void::Void));

#[allow(dead_code)]
#[repr(C)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific,
}

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.5.2 Get Next High Monotonic Count
#[derive(WrappedFunction)]
#[repr(C)]
pub struct GetNextHighMonotonicCount(pub extern "efiapi" fn(&mut u32) -> status::Status);

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.5.3 Update Capsule
#[derive(WrappedFunction)]
#[repr(C)]
pub struct UpdateCapsule(pub extern "efiapi" fn(&&CapsuleHeader, usize, memory_allocation::PhysicalAddress) -> status::Status);

#[repr(C)]
pub struct CapsuleHeader {
    capsule_guid: protocol_handler::Guid,
    header_size: u32,
    flags: u32,
    capsule_image_size: u32,
}

#[derive(WrappedFunction)]
#[repr(C)]
pub struct QueryCapsuleCapabilities(pub extern "efiapi" fn(&&CapsuleHeader, usize, &mut u64, &mut ResetType) -> status::Status);

