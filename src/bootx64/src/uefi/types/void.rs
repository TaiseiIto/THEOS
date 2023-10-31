use core::fmt;

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 2.3.1 Data Types
// VOID
#[repr(C)]
pub struct Void {
}

impl Void {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn null<'a>() -> &'a Self {
        0usize.into()
    }

    pub fn move_to_higher_half<'a>(&self, highest_parallel_offset: usize) -> &'a Self {
        let address: usize = self.into();
        (highest_parallel_offset + address).into()
    }
}

impl fmt::Debug for Void {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let void: usize = self.into();
        write!(f, "{:#x}", void)
    }
}

impl From<usize> for &Void {
    fn from(virtual_address: usize) -> Self {
        let virtual_address: *const Void = virtual_address as *const Void;
        unsafe {
            &*virtual_address
        }
    }
}

impl Into<usize> for &Void {
    fn into(self) -> usize {
        let virtual_address = self as *const Void;
        virtual_address as usize
    }
}

