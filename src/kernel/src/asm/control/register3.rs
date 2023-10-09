// References
// Intel 64 an IA-32 Architectures Software Developer's Manual, Volume 3, Chapter 2.5 Control Registers

use core::arch::asm;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr3(u64);

impl Cr3 {
    pub fn get() -> Self {
        let mut cr3: u64;
        unsafe {
            asm!(
                "mov rax, cr3",
                out("rax") cr3,
            );
        }
        Self(cr3)
    }
}

impl Into<u64> for &Cr3 {
    fn into(self) -> u64 {
        self.0
    }
}

