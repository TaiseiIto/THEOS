use super::super::{
    rdmsr,
    wrmsr,
    super::cpuid,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ia32Efer {
    value: u64,
    sce: bool,
    lme: bool,
    lma: bool,
    nxe: bool,
}

impl Ia32Efer {
    const ADDRESS: u32 = 0xc0000080;

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
                let value: u64 = rdmsr(Self::ADDRESS);
                let sce: bool = value & Self::SCE_MASK != 0;
                let lme: bool = value & Self::LME_MASK != 0;
                let lma: bool = value & Self::LMA_MASK != 0;
                let nxe: bool = value & Self::NXE_MASK != 0;
                Some(Self {
                    value,
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

    pub fn lme(&self) -> bool {
        self.lme
    }

    pub fn nxe(&self) -> bool {
        self.nxe
    }

    pub fn set_nxe(&mut self) {
        self.nxe = true;
        self.value |= Self::NXE_MASK;
        wrmsr(Self::ADDRESS, self.value);
    }
}

