// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.4 Virtual Memory

use {
    core::fmt,
    super::super::{
        boot::memory_allocation,
        super::types::{
            status,
            void,
        },
    },
};

#[repr(C)]
pub struct SetVirtualAddressMap(pub extern "efiapi" fn(usize, usize, u32, &memory_allocation::MemoryDescriptor) -> status::Status);

impl fmt::Debug for SetVirtualAddressMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct ConvertPointer(pub extern "efiapi" fn(usize, &void::Void) -> status::Status);

impl fmt::Debug for ConvertPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

