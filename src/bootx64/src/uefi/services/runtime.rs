// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8 Runtime Services

pub mod time;
pub mod variable;
pub mod virtual_memory;

use {
    core::fmt,
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
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.5.1 Reset System
#[repr(C)]
pub struct ResetSystem(extern "efiapi" fn(ResetType, status::Status, usize, &void::Void));

impl fmt::Debug for ResetSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

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
#[repr(C)]
pub struct GetNextHighMonotonicCount(extern "efiapi" fn(&mut u32) -> status::Status);

impl fmt::Debug for GetNextHighMonotonicCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.5.3 Update Capsule
#[repr(C)]
pub struct UpdateCapsule(extern "efiapi" fn(&&CapsuleHeader, usize, memory_allocation::PhysicalAddress) -> status::Status);

impl fmt::Debug for UpdateCapsule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct CapsuleHeader {
    capsule_guid: protocol_handler::Guid,
    header_size: u32,
    flags: u32,
    capsule_image_size: u32,
}

