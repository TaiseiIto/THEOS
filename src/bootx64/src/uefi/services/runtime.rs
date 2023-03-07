// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8 Runtime Services

pub mod time;
pub mod variable;
pub mod virtual_memory;

use {
    core::fmt,
    super::super::types::status,
};

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

