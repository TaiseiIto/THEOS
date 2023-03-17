use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x80000001 {
    eax: Eax,
}

impl Eax0x80000001 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 1;
        let ecx: u32 = 0;
        if eax <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx: _,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let eax: Eax = eax.into();
            Some(Self {
                eax,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    extended_processor_signature: u32,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let extended_processor_signature: u32 = eax;
        Self {
            extended_processor_signature,
        }
    }
}

