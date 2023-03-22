use super::super::{
    rdmsr,
    super::cpuid,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ia32Efer {
    sce: bool,
    lme: bool,
    lma: bool,
    nxe: bool,
}

impl Ia32Efer {
    const SCE_SHIFT: usize = 0;
    const LME_SHIFT: usize = 8;
    const LMA_SHIFT: usize = 10;
    const NXE_SHIFT: usize = 11;

    const SCE_MASK: u64 = 1 << Self::SCE_SHIFT;
    const LME_MASK: u64 = 1 << Self::LME_SHIFT;
    const LMA_MASK: u64 = 1 << Self::LMA_SHIFT;
    const NXE_MASK: u64 = 1 << Self::NXE_SHIFT;

    pub fn get(cpuid: &Option<cpuid::Cpuid>) -> Option<Self> {
        match cpuid {
            Some(cpuid) => if cpuid.supports_ia32_efer() {
                let ia32_efer: u64 = rdmsr(0xc0000080);
                let sce: bool = ia32_efer & Self::SCE_MASK != 0;
                let lme: bool = ia32_efer & Self::LME_MASK != 0;
                let lma: bool = ia32_efer & Self::LMA_MASK != 0;
                let nxe: bool = ia32_efer & Self::NXE_MASK != 0;
                Some(Self {
                    sce,
                    lme,
                    lma,
                    nxe,
                })
            } else {
                None
            },
            None => None,
        }
    }
}

