use core::fmt;

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.1 Event, Timer and Task Priority Services

#[repr(C)]
pub struct RaiseTpl(extern "efiapi" fn(Tpl) -> Tpl);

impl fmt::Debug for RaiseTpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct RestoreTpl(extern "efiapi" fn(Tpl));

impl fmt::Debug for RestoreTpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

pub type Tpl = usize;
