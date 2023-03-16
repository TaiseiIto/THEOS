use super::super::CpuidOutRegisters;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx0x00000001 {
    eax: Eax,
}

impl Ecx0x00000001 {
    pub fn new() -> Self {
        let eax: u32 = 7;
        let ecx: u32 = 1;
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
    avx_vnni: bool,
    avx512_bf16: bool,
    fast_rep_movsb: bool,
    fast_rep_stosb: bool,
    fast_rep_cmpsb_rep_scasb: bool,
    hreset: bool,
}

impl Eax {
    const AVX_VNNI_SHIFT: usize = 4;
    const AVX512_BF16_SHIFT: usize = 5;
    const FAST_REP_MOVSB_SHIFT: usize = 10;
    const FAST_REP_STOSB_SHIFT: usize = 11;
    const FAST_REP_CMPSB_REP_SCASB_SHIFT: usize = 12;
    const HRESET_SHIFT: usize = 22;

    const AVX_VNNI_MASK: u32 = (1 << Self::AVX_VNNI_SHIFT) as u32;
    const AVX512_BF16_MASK: u32 = (1 << Self::AVX512_BF16_SHIFT) as u32;
    const FAST_REP_MOVSB_MASK: u32 = (1 << Self::FAST_REP_MOVSB_SHIFT) as u32;
    const FAST_REP_STOSB_MASK: u32 = (1 << Self::FAST_REP_STOSB_SHIFT) as u32;
    const FAST_REP_CMPSB_REP_SCASB_MASK: u32 = (1 << Self::FAST_REP_CMPSB_REP_SCASB_SHIFT) as u32;
    const HRESET_MASK: u32 = (1 << Self::HRESET_SHIFT) as u32;
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let avx_vnni = eax & Self::AVX_VNNI_MASK != 0;
        let avx512_bf16 = eax & Self::AVX512_BF16_MASK != 0;
        let fast_rep_movsb = eax & Self::FAST_REP_MOVSB_MASK != 0;
        let fast_rep_stosb = eax & Self::FAST_REP_STOSB_MASK != 0;
        let fast_rep_cmpsb_rep_scasb = eax & Self::FAST_REP_CMPSB_REP_SCASB_MASK != 0;
        let hreset = eax & Self::HRESET_MASK != 0;
        Self {
            avx_vnni,
            avx512_bf16,
            fast_rep_movsb,
            fast_rep_stosb,
            fast_rep_cmpsb_rep_scasb,
            hreset,
        }
    }
}

