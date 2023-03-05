use core::fmt;

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 2.3.1 Data Types
pub type Handle<'a> = &'a Void;

pub struct Void {
}

impl fmt::Debug for Void {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let void = self as *const Void;
        let void = void as usize;
        write!(f, "{:#x}", void)
    }
}

