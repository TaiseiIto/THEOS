// References
// Intel 64 an IA-32 Architectures Software Developer's Manual 3.3.4.5 Segment Discriptor

use core::arch::asm;

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
        let mut gdtrp: usize = gdtrp as usize;
        unsafe {
            asm!(
                "sgdt [{}]",
                inout(reg) gdtrp,
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
