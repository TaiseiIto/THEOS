// References
// Intel 64 an IA-32 Architectures Software Developer's Manual, Volume 3, Chapter 2.5 Control Registers

use core::arch::asm;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr2 {
    page_fault_linear_address: u64,
}

impl Cr2 {
    pub fn get() -> Self {
        let mut cr2: u64;
        unsafe {
            asm!(
                "mov rax, cr2",
                out("rax") cr2,
            );
        }
        let page_fault_linear_address: u64 = cr2;
        Self {
            page_fault_linear_address,
        }
    }
}

