use super::super::{
    rdmsr,
    super::cpuid,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ia32Efer {
    sce: bool,
}

impl Ia32Efer {
    const SCE_SHIFT: usize = 0;

    const SCE_MASK: u64 = 1 << Self::SCE_SHIFT;

    pub fn get(cpuid: &Option<cpuid::Cpuid>) -> Option<Self> {
        match cpuid {
            Some(cpuid) => if cpuid.supports_ia32_efer() {
                let ia32_efer: u64 = rdmsr(0xc0000080);
                let sce: bool = ia32_efer & Self::SCE_MASK != 0;
                Some(Self {
                    sce,
                })
            } else {
                None
            },
            None => None,
        }
    }
}

