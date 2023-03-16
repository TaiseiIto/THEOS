use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000007 {
    eax: Eax,
    ebx: Ebx,
}

impl Eax0x00000007 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 7;
        if eax <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(eax);
            let eax: Eax = eax.into();
            let ebx: Ebx = ebx.into();
            Some(Self {
                eax,
                ebx,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    maximum_input_value: u32,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let maximum_input_value: u32 = eax;
        Self {
            maximum_input_value,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    fsgsbase: bool,
    ia32_tsc_adjust_msr: bool,
    sgx: bool,
    bmi1: bool,
    hle: bool,
    avx2: bool,
    fdp_excptn_only: bool,
    smep: bool,
    bmi2: bool,
    supports_enhanced_rep_movsb_stosb: bool,
    invpcid: bool,
    rtm: bool,
    rdt_m: bool,
    deprecates_fpu: bool,
    mpx: bool,
    rdt_a: bool,
    avx512f: bool,
    avx512dq: bool,
    rdseed: bool,
    adx: bool,
    smap: bool,
    avx512_ifma: bool,
    clflushopt: bool,
    clwb: bool,
    intel_processor_trace: bool,
    avx512pf: bool,
    avx512er: bool,
    avx512cd: bool,
    sha: bool,
    avx512bw: bool,
    avx512vl: bool,
}

impl Ebx {
    const FSGSBASE_SHIFT: usize = 0;
    const IA32_TSC_ADJUST_MSR_SHIFT: usize = 1;
    const SGX_SHIFT: usize = 2;
    const BMI1_SHIFT: usize = 3;
    const HLE_SHIFT: usize = 4;
    const AVX2_SHIFT: usize = 5;
    const FDP_EXCPTN_ONLY_SHIFT: usize = 6;
    const SMEP_SHIFT: usize = 7;
    const BMI2_SHIFT: usize = 8;
    const SUPPORTS_ENHANCED_REP_MOVSB_STOSB_SHIFT: usize = 9;
    const INVPCID_SHIFT: usize = 10;
    const RTM_SHIFT: usize = 11;
    const RDT_M_SHIFT: usize = 12;
    const DEPRECATES_FPU_SHIFT: usize = 13;
    const MPX_SHIFT: usize = 14;
    const RDT_A_SHIFT: usize = 15;
    const AVX512F_SHIFT: usize = 16;
    const AVX512DQ_SHIFT: usize = 17;
    const RDSEED_SHIFT: usize = 18;
    const ADX_SHIFT: usize = 19;
    const SMAP_SHIFT: usize = 20;
    const AVX512_IFMA_SHIFT: usize = 21;
    const CLFLUSHOPT_SHIFT: usize = 23;
    const CLWB_SHIFT: usize = 24;
    const INTEL_PROCESSOR_TRACE_SHIFT: usize = 25;
    const AVX512PF_SHIFT: usize = 26;
    const AVX512ER_SHIFT: usize = 27;
    const AVX512CD_SHIFT: usize = 28;
    const SHA_SHIFT: usize = 29;
    const AVX512BW_SHIFT: usize = 30;
    const AVX512VL_SHIFT: usize = 31;

    const FSGSBASE_MASK: u32 = (1 << Self::FSGSBASE_SHIFT) as u32;
    const IA32_TSC_ADJUST_MSR_MASK: u32 = (1 << Self::IA32_TSC_ADJUST_MSR_SHIFT) as u32;
    const SGX_MASK: u32 = (1 << Self::SGX_SHIFT) as u32;
    const BMI1_MASK: u32 = (1 << Self::BMI1_SHIFT) as u32;
    const HLE_MASK: u32 = (1 << Self::HLE_SHIFT) as u32;
    const AVX2_MASK: u32 = (1 << Self::AVX2_SHIFT) as u32;
    const FDP_EXCPTN_ONLY_MASK: u32 = (1 << Self::FDP_EXCPTN_ONLY_SHIFT) as u32;
    const SMEP_MASK: u32 = (1 << Self::SMEP_SHIFT) as u32;
    const BMI2_MASK: u32 = (1 << Self::BMI2_SHIFT) as u32;
    const SUPPORTS_ENHANCED_REP_MOVSB_STOSB_MASK: u32 = (1 << Self::SUPPORTS_ENHANCED_REP_MOVSB_STOSB_SHIFT) as u32;
    const INVPCID_MASK: u32 = (1 << Self::INVPCID_SHIFT) as u32;
    const RTM_MASK: u32 = (1 << Self::RTM_SHIFT) as u32;
    const RDT_M_MASK: u32 = (1 << Self::RDT_M_SHIFT) as u32;
    const DEPRECATES_FPU_MASK: u32 = (1 << Self::DEPRECATES_FPU_SHIFT) as u32;
    const MPX_MASK: u32 = (1 << Self::MPX_SHIFT) as u32;
    const RDT_A_MASK: u32 = (1 << Self::RDT_A_SHIFT) as u32;
    const AVX512F_MASK: u32 = (1 << Self::AVX512F_SHIFT) as u32;
    const AVX512DQ_MASK: u32 = (1 << Self::AVX512DQ_SHIFT) as u32;
    const RDSEED_MASK: u32 = (1 << Self::RDSEED_SHIFT) as u32;
    const ADX_MASK: u32 = (1 << Self::ADX_SHIFT) as u32;
    const SMAP_MASK: u32 = (1 << Self::SMAP_SHIFT) as u32;
    const AVX512_IFMA_MASK: u32 = (1 << Self::AVX512_IFMA_SHIFT) as u32;
    const CLFLUSHOPT_MASK: u32 = (1 << Self::CLFLUSHOPT_SHIFT) as u32;
    const CLWB_MASK: u32 = (1 << Self::CLWB_SHIFT) as u32;
    const INTEL_PROCESSOR_TRACE_MASK: u32 = (1 << Self::INTEL_PROCESSOR_TRACE_SHIFT) as u32;
    const AVX512PF_MASK: u32 = (1 << Self::AVX512PF_SHIFT) as u32;
    const AVX512ER_MASK: u32 = (1 << Self::AVX512ER_SHIFT) as u32;
    const AVX512CD_MASK: u32 = (1 << Self::AVX512CD_SHIFT) as u32;
    const SHA_MASK: u32 = (1 << Self::SHA_SHIFT) as u32;
    const AVX512BW_MASK: u32 = (1 << Self::AVX512BW_SHIFT) as u32;
    const AVX512VL_MASK: u32 = (1 << Self::AVX512VL_SHIFT) as u32;
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let fsgsbase: bool = ebx & Self::FSGSBASE_MASK != 0;
        let ia32_tsc_adjust_msr: bool = ebx & Self::IA32_TSC_ADJUST_MSR_MASK != 0;
        let sgx: bool = ebx & Self::SGX_MASK != 0;
        let bmi1: bool = ebx & Self::BMI1_MASK != 0;
        let hle: bool = ebx & Self::HLE_MASK != 0;
        let avx2: bool = ebx & Self::AVX2_MASK != 0;
        let fdp_excptn_only: bool = ebx & Self::FDP_EXCPTN_ONLY_MASK != 0;
        let smep: bool = ebx & Self::SMEP_MASK != 0;
        let bmi2: bool = ebx & Self::BMI2_MASK != 0;
        let supports_enhanced_rep_movsb_stosb: bool = ebx & Self::SUPPORTS_ENHANCED_REP_MOVSB_STOSB_MASK != 0;
        let invpcid: bool = ebx & Self::INVPCID_MASK != 0;
        let rtm: bool = ebx & Self::RTM_MASK != 0;
        let rdt_m: bool = ebx & Self::RDT_M_MASK != 0;
        let deprecates_fpu: bool = ebx & Self::DEPRECATES_FPU_MASK != 0;
        let mpx: bool = ebx & Self::MPX_MASK != 0;
        let rdt_a: bool = ebx & Self::RDT_A_MASK != 0;
        let avx512f: bool = ebx & Self::AVX512F_MASK != 0;
        let avx512dq: bool = ebx & Self::AVX512DQ_MASK != 0;
        let rdseed: bool = ebx & Self::RDSEED_MASK != 0;
        let adx: bool = ebx & Self::ADX_MASK != 0;
        let smap: bool = ebx & Self::SMAP_MASK != 0;
        let avx512_ifma: bool = ebx & Self::AVX512_IFMA_MASK != 0;
        let clflushopt: bool = ebx & Self::CLFLUSHOPT_MASK != 0;
        let clwb: bool = ebx & Self::CLWB_MASK != 0;
        let intel_processor_trace: bool = ebx & Self::INTEL_PROCESSOR_TRACE_MASK != 0;
        let avx512pf: bool = ebx & Self::AVX512PF_MASK != 0;
        let avx512er: bool = ebx & Self::AVX512ER_MASK != 0;
        let avx512cd: bool = ebx & Self::AVX512CD_MASK != 0;
        let sha: bool = ebx & Self::SHA_MASK != 0;
        let avx512bw: bool = ebx & Self::AVX512BW_MASK != 0;
        let avx512vl: bool = ebx & Self::AVX512VL_MASK != 0;
        Self {
            fsgsbase,
            ia32_tsc_adjust_msr,
            sgx,
            bmi1,
            hle,
            avx2,
            fdp_excptn_only,
            smep,
            bmi2,
            supports_enhanced_rep_movsb_stosb,
            invpcid,
            rtm,
            rdt_m,
            deprecates_fpu,
            mpx,
            rdt_a,
            avx512f,
            avx512dq,
            rdseed,
            adx,
            smap,
            avx512_ifma,
            clflushopt,
            clwb,
            intel_processor_trace,
            avx512pf,
            avx512er,
            avx512cd,
            sha,
            avx512bw,
            avx512vl,
        }
    }
}

