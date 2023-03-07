// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7 Boot Services

use {
    core::fmt,
    super::super::types::{
        char16,
        status,
        void,
    },
};

pub mod event;
pub mod image;
pub mod memory_allocation;
pub mod protocol_handler;

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.5 Miscellaneous Boot Services

#[repr(C)]
pub struct SetWatchdogTimer(extern "efiapi" fn(usize, u64, usize, char16::String) -> status::Status);

impl fmt::Debug for SetWatchdogTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct Stall(extern "efiapi" fn(usize) -> status::Status);

impl fmt::Debug for Stall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct GetNextMonotonicCount(extern "efiapi" fn(&mut u64) -> status::Status);

impl fmt::Debug for GetNextMonotonicCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct InstallConfigurationTable(extern "efiapi" fn(&protocol_handler::Guid, &void::Void) -> status::Status);

impl fmt::Debug for InstallConfigurationTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct CalculateCrc32(extern "efiapi" fn(&void::Void, usize, &mut u32) -> status::Status);

impl fmt::Debug for CalculateCrc32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

