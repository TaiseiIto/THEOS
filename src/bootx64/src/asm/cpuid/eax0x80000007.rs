use super::{
    CpuidOutRegisters,
    eax0x80000000::Eax0x80000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x80000007 {
    edx: Edx,
}

impl Eax0x80000007 {
    pub fn new(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000007;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x80000000.max_eax() {
            let CpuidOutRegisters {
                eax: _,
                ebx: _,
                edx,
                ecx: _,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let edx: Edx = edx.into();
            Some(Self {
                edx,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    invariant_tsc: bool,
}

impl Edx {
    const INVARIANT_TSC_SHIFT: usize = 8;
    const INVARIANT_TSC_MASK: u32 = (1 << Self::INVARIANT_TSC_SHIFT) as u32;
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let invariant_tsc: bool = edx & Self::INVARIANT_TSC_MASK != 0;
        Self {
            invariant_tsc,
        }
    }
}

