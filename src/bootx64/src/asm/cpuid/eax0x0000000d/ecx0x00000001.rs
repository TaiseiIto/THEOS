use super::super::CpuidOutRegisters;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx0x00000001 {
    eax: Eax,
}

impl Ecx0x00000001 {
    pub fn new() -> Self {
        let eax: u32 = 0x0000000d;
        let ecx: u32 = 0x00000001;
        let CpuidOutRegisters {
            eax,
            ebx: _,
            edx: _,
            ecx: _,
        } = CpuidOutRegisters::cpuid(eax, ecx);
        let eax: Eax = eax.into();
        Self {
            eax,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    xsaveopt: bool,
    xsavec: bool,
    xgetbv: bool,
    xsaves_xrstors_and_ia32_xss: bool,
}

impl Eax {
    const XSAVEOPT_SHIFT: usize = 0;
    const XSAVEC_SHIFT: usize = 1;
    const XGETBV_SHIFT: usize = 2;
    const XSAVES_XRSTORS_AND_IA32_XSS_SHIFT: usize = 3;

    const XSAVEOPT_MASK: u32 = (1 << Self::XSAVEOPT_SHIFT) as u32;
    const XSAVEC_MASK: u32 = (1 << Self::XSAVEC_SHIFT) as u32;
    const XGETBV_MASK: u32 = (1 << Self::XGETBV_SHIFT) as u32;
    const XSAVES_XRSTORS_AND_IA32_XSS_MASK: u32 = (1 << Self::XSAVES_XRSTORS_AND_IA32_XSS_SHIFT) as u32;
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Eax {
        let xsaveopt: bool = eax & Self::XSAVEOPT_MASK != 0;
        let xsavec: bool = eax & Self::XSAVEC_MASK != 0;
        let xgetbv: bool = eax & Self::XGETBV_MASK != 0;
        let xsaves_xrstors_and_ia32_xss: bool = eax & Self::XSAVES_XRSTORS_AND_IA32_XSS_MASK != 0;
        Self {
            xsaveopt,
            xsavec,
            xgetbv,
            xsaves_xrstors_and_ia32_xss,
        }
    }
}

