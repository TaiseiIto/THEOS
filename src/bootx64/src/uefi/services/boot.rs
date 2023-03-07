// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7 Boot Services

use {
    core::fmt,
    super::super::types::{
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
pub struct InstallConfigurationTable(extern "efiapi" fn(&protocol_handler::Guid, &void::Void) -> status::Status);

impl fmt::Debug for InstallConfigurationTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

