use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000009 {
    ia32_platform_dca_cap_msr: u32,
}

impl Eax0x00000009 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 9;
        let ecx: u32 = 0;
        if eax <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx: _,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let ia32_platform_dca_cap_msr: u32 = eax;
            Some(Self {
                ia32_platform_dca_cap_msr,
            })
        } else {
            None
        }
    }
}

