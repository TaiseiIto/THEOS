use super::CpuidOutRegisters;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x80000000 {
    max_eax: u32,
}

impl Eax0x80000000 {
    pub fn new() -> Self {
        let eax: u32 = 0x80000000;
        let ecx: u32 = 0x00000000;
        let CpuidOutRegisters {
            eax,
            ebx: _,
            edx: _,
            ecx: _,
        } = CpuidOutRegisters::cpuid(eax, ecx);
        let max_eax: u32 = eax;
        Self {
            max_eax,
        }
    }

    pub fn max_eax(&self) -> u32 {
        self.max_eax
    }
}

