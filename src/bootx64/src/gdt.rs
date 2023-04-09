// References
// Intel 64 an IA-32 Architectures Software Developer's Manual 3.3.4.5 Segment Discriptor

use core::{
    arch::asm,
    mem,
    slice,
};

#[derive(Debug)]
pub struct Register {
    base: u64,
    limit: u16,
}

impl Register {
    pub fn get() -> Self {
        let mut gdtr: u128 = 0;
        let gdtrp: &mut u128 = &mut gdtr;
        let gdtrp: *mut u128 = gdtrp as *mut u128;
        let gdtrp: usize = gdtrp as usize;
        unsafe {
            asm!(
                "sgdt [{}]",
                in(reg) gdtrp,
            );
        }
        let base: u64 = (gdtr >> 16) as u64;
        let limit: u16 = gdtr as u16;
        Self {
            base,
            limit,
        }
    }
}

impl Into<&'static [u64]> for Register {
    fn into(self) -> &'static [u64] {
        let base: *const u64 = self.base as *const u64;
        let limit: usize = self.limit as usize;
        let length: usize = (limit + 1) / mem::size_of::<u64>();
        unsafe {
            slice::from_raw_parts(base, length)
        }
    }
}

