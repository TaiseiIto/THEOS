use super::super::CpuidOutRegisters;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx0x00000000 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
}

impl Ecx0x00000000 {
    pub fn new() -> Self {
        let eax: u32 = 0x0000000d;
        let ecx: u32 = 0x00000000;
        let CpuidOutRegisters {
            eax,
            ebx,
            edx,
            ecx: _,
        } = CpuidOutRegisters::cpuid(eax, ecx);
        let eax: Eax = eax.into();
        let ebx: Ebx = ebx.into();
        let edx: Edx = edx.into();
        Self {
            eax,
            ebx,
            edx,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    x87_state: bool,
    sse_state: bool,
    avx_state: bool,
    mpx_state: u8,
    avx_512_state: u8,
    ia32_xss_0: bool,
    pkru_state: bool,
    ia32_xss_1: bool,
    ia32_xss_2: bool,
}

impl Eax {
    const X87_STATE_SHIFT: usize = 0;
    const SSE_STATE_SHIFT: usize = 1;
    const AVX_STATE_SHIFT: usize = 2;
    const MPX_STATE_SHIFT: usize = 3;
    const AVX_512_STATE_SHIFT: usize = 5;
    const IA32_XSS_0_SHIFT: usize = 8;
    const PKRU_STATE_SHIFT: usize = 9;
    const IA32_XSS_1_SHIFT: usize = 13;
    const IA32_XSS_2_SHIFT: usize = 16;

    const X87_STATE_SHIFT_END: usize = 0;
    const SSE_STATE_SHIFT_END: usize = 1;
    const AVX_STATE_SHIFT_END: usize = 2;
    const MPX_STATE_SHIFT_END: usize = 4;
    const AVX_512_STATE_SHIFT_END: usize = 7;
    const IA32_XSS_0_SHIFT_END: usize = 8;
    const PKRU_STATE_SHIFT_END: usize = 9;
    const IA32_XSS_1_SHIFT_END: usize = 13;
    const IA32_XSS_2_SHIFT_END: usize = 16;

    const X87_STATE_LENGTH: usize = Self::X87_STATE_SHIFT_END - Self::X87_STATE_SHIFT + 1;
    const SSE_STATE_LENGTH: usize = Self::SSE_STATE_SHIFT_END - Self::SSE_STATE_SHIFT + 1;
    const AVX_STATE_LENGTH: usize = Self::AVX_STATE_SHIFT_END - Self::AVX_STATE_SHIFT + 1;
    const MPX_STATE_LENGTH: usize = Self::MPX_STATE_SHIFT_END - Self::MPX_STATE_SHIFT + 1;
    const AVX_512_STATE_LENGTH: usize = Self::AVX_512_STATE_SHIFT_END - Self::AVX_512_STATE_SHIFT + 1;
    const IA32_XSS_0_LENGTH: usize = Self::IA32_XSS_0_SHIFT_END - Self::IA32_XSS_0_SHIFT + 1;
    const PKRU_STATE_LENGTH: usize = Self::PKRU_STATE_SHIFT_END - Self::PKRU_STATE_SHIFT + 1;
    const IA32_XSS_1_LENGTH: usize = Self::IA32_XSS_1_SHIFT_END - Self::IA32_XSS_1_SHIFT + 1;
    const IA32_XSS_2_LENGTH: usize = Self::IA32_XSS_2_SHIFT_END - Self::IA32_XSS_2_SHIFT + 1;

    const X87_STATE_MASK: u32 = (((1 << Self::X87_STATE_LENGTH) - 1) << Self::X87_STATE_SHIFT) as u32;
    const SSE_STATE_MASK: u32 = (((1 << Self::SSE_STATE_LENGTH) - 1) << Self::SSE_STATE_SHIFT) as u32;
    const AVX_STATE_MASK: u32 = (((1 << Self::AVX_STATE_LENGTH) - 1) << Self::AVX_STATE_SHIFT) as u32;
    const MPX_STATE_MASK: u32 = (((1 << Self::MPX_STATE_LENGTH) - 1) << Self::MPX_STATE_SHIFT) as u32;
    const AVX_512_STATE_MASK: u32 = (((1 << Self::AVX_512_STATE_LENGTH) - 1) << Self::AVX_512_STATE_SHIFT) as u32;
    const IA32_XSS_0_MASK: u32 = (((1 << Self::IA32_XSS_0_LENGTH) - 1) << Self::IA32_XSS_0_SHIFT) as u32;
    const PKRU_STATE_MASK: u32 = (((1 << Self::PKRU_STATE_LENGTH) - 1) << Self::PKRU_STATE_SHIFT) as u32;
    const IA32_XSS_1_MASK: u32 = (((1 << Self::IA32_XSS_1_LENGTH) - 1) << Self::IA32_XSS_1_SHIFT) as u32;
    const IA32_XSS_2_MASK: u32 = (((1 << Self::IA32_XSS_2_LENGTH) - 1) << Self::IA32_XSS_2_SHIFT) as u32;
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let x87_state: bool = eax & Self::X87_STATE_MASK != 0;
        let sse_state: bool = eax & Self::SSE_STATE_MASK != 0;
        let avx_state: bool = eax & Self::AVX_STATE_MASK != 0;
        let mpx_state: u8 = ((eax & Self::MPX_STATE_MASK) >> Self::MPX_STATE_SHIFT) as u8;
        let avx_512_state: u8 = ((eax & Self::AVX_512_STATE_MASK) >> Self::AVX_512_STATE_SHIFT) as u8;
        let ia32_xss_0: bool = eax & Self::IA32_XSS_0_MASK != 0;
        let pkru_state: bool = eax & Self::PKRU_STATE_MASK != 0;
        let ia32_xss_1: bool = eax & Self::IA32_XSS_1_MASK != 0;
        let ia32_xss_2: bool = eax & Self::IA32_XSS_2_MASK != 0;
        Self {
            x87_state,
            sse_state,
            avx_state,
            mpx_state,
            avx_512_state,
            ia32_xss_0,
            pkru_state,
            ia32_xss_1,
            ia32_xss_2,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    maximum_size_required_by_enabled_features_in_xcr0: u32,
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let maximum_size_required_by_enabled_features_in_xcr0: u32 = ebx;
        Self {
            maximum_size_required_by_enabled_features_in_xcr0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    maximum_size_of_the_xsave_xrstor_safe_area: u32,
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let maximum_size_of_the_xsave_xrstor_safe_area: u32 = edx;
        Self {
            maximum_size_of_the_xsave_xrstor_safe_area,
        }
    }
}

