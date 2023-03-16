use super::super::CpuidOutRegisters;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx0x00000000 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
    ecx: Ecx,
}

impl Ecx0x00000000 {
    pub fn new() -> Self {
        let eax: u32 = 7;
        let ecx: u32 = 0;
        let CpuidOutRegisters {
            eax,
            ebx,
            edx,
            ecx,
        } = CpuidOutRegisters::cpuid(eax, ecx);
        let eax: Eax = eax.into();
        let ebx: Ebx = ebx.into();
        let edx: Edx = edx.into();
        let ecx: Ecx = ecx.into();
        Self {
            eax,
            ebx,
            edx,
            ecx,
        }
    }

    pub fn supports_5_level_paging(&self) -> bool {
        self.ecx.supports_5_level_paging()
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    avx512_4vnniw: bool,
    avx512_4fmaps: bool,
    fast_short_rep_mov: bool,
    avx512_vp2intersect: bool,
    md_clear: bool,
    serialize: bool,
    hybrid: bool,
    pconfig: bool,
    cet_ibt: bool,
    ibrs_and_ibpb: bool,
    stibp: bool,
    l1d_flush: bool,
    ia32_arch_capabilities_msr: bool,
    ia32_core_capabilities_msr: bool,
    ssbd: bool,
}

impl Edx {
    const AVX512_4VNNIW_SHIFT: usize = 2;
    const AVX512_4FMAPS_SHIFT: usize = 3;
    const FAST_SHORT_REP_MOV_SHIFT: usize = 4;
    const AVX512_VP2INTERSECT_SHIFT: usize = 8;
    const MD_CLEAR_SHIFT: usize = 10;
    const SERIALIZE_SHIFT: usize = 14;
    const HYBRID_SHIFT: usize = 15;
    const PCONFIG_SHIFT: usize = 18;
    const CET_IBT_SHIFT: usize = 20;
    const IBRS_AND_IBPB_SHIFT: usize = 26;
    const STIBP_SHIFT: usize = 27;
    const L1D_FLUSH_SHIFT: usize = 28;
    const IA32_ARCH_CAPABILITIES_MSR_SHIFT: usize = 29;
    const IA32_CORE_CAPABILITIES_MSR_SHIFT: usize = 30;
    const SSBD_SHIFT: usize = 31;

    const AVX512_4VNNIW_MASK: u32 = (1 << Self::AVX512_4VNNIW_SHIFT) as u32;
    const AVX512_4FMAPS_MASK: u32 = (1 << Self::AVX512_4FMAPS_SHIFT) as u32;
    const FAST_SHORT_REP_MOV_MASK: u32 = (1 << Self::FAST_SHORT_REP_MOV_SHIFT) as u32;
    const AVX512_VP2INTERSECT_MASK: u32 = (1 << Self::AVX512_VP2INTERSECT_SHIFT) as u32;
    const MD_CLEAR_MASK: u32 = (1 << Self::MD_CLEAR_SHIFT) as u32;
    const SERIALIZE_MASK: u32 = (1 << Self::SERIALIZE_SHIFT) as u32;
    const HYBRID_MASK: u32 = (1 << Self::HYBRID_SHIFT) as u32;
    const PCONFIG_MASK: u32 = (1 << Self::PCONFIG_SHIFT) as u32;
    const CET_IBT_MASK: u32 = (1 << Self::CET_IBT_SHIFT) as u32;
    const IBRS_AND_IBPB_MASK: u32 = (1 << Self::IBRS_AND_IBPB_SHIFT) as u32;
    const STIBP_MASK: u32 = (1 << Self::STIBP_SHIFT) as u32;
    const L1D_FLUSH_MASK: u32 = (1 << Self::L1D_FLUSH_SHIFT) as u32;
    const IA32_ARCH_CAPABILITIES_MSR_MASK: u32 = (1 << Self::IA32_ARCH_CAPABILITIES_MSR_SHIFT) as u32;
    const IA32_CORE_CAPABILITIES_MSR_MASK: u32 = (1 << Self::IA32_CORE_CAPABILITIES_MSR_SHIFT) as u32;
    const SSBD_MASK: u32 = (1 << Self::SSBD_SHIFT) as u32;
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let avx512_4vnniw = edx & Self::AVX512_4VNNIW_MASK != 0;
        let avx512_4fmaps = edx & Self::AVX512_4FMAPS_MASK != 0;
        let fast_short_rep_mov = edx & Self::FAST_SHORT_REP_MOV_MASK != 0;
        let avx512_vp2intersect = edx & Self::AVX512_VP2INTERSECT_MASK != 0;
        let md_clear = edx & Self::MD_CLEAR_MASK != 0;
        let serialize = edx & Self::SERIALIZE_MASK != 0;
        let hybrid = edx & Self::HYBRID_MASK != 0;
        let pconfig = edx & Self::PCONFIG_MASK != 0;
        let cet_ibt = edx & Self::CET_IBT_MASK != 0;
        let ibrs_and_ibpb = edx & Self::IBRS_AND_IBPB_MASK != 0;
        let stibp = edx & Self::STIBP_MASK != 0;
        let l1d_flush = edx & Self::L1D_FLUSH_MASK != 0;
        let ia32_arch_capabilities_msr = edx & Self::IA32_ARCH_CAPABILITIES_MSR_MASK != 0;
        let ia32_core_capabilities_msr = edx & Self::IA32_CORE_CAPABILITIES_MSR_MASK != 0;
        let ssbd = edx & Self::SSBD_MASK != 0;
        Self {
            avx512_4vnniw,
            avx512_4fmaps,
            fast_short_rep_mov,
            avx512_vp2intersect,
            md_clear,
            serialize,
            hybrid,
            pconfig,
            cet_ibt,
            ibrs_and_ibpb,
            stibp,
            l1d_flush,
            ia32_arch_capabilities_msr,
            ia32_core_capabilities_msr,
            ssbd,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx {
    prefetchwt1: bool,
    avx512_vbmi: bool,
    umip: bool,
    pku: bool,
    ospke: bool,
    waitpkg: bool,
    avx512_vbmi2: bool,
    cet_ss: bool,
    gfni: bool,
    vaes: bool,
    vpclmulqdq: bool,
    avx512_vnni: bool,
    avx512_bitalg: bool,
    tme_en: bool,
    avx512_vpopcntdq: bool,
    la57: bool,
    mawau: u8,
    rdpid_and_ia32_tsc_aux: bool,
    kl: bool,
    cldemote: bool,
    movdiri: bool,
    movdir64b: bool,
    sgx_lc: bool,
    pks: bool,
}

impl Ecx {
    const PREFETCHWT1_SHIFT: usize = 0;
    const AVX512_VBMI_SHIFT: usize = 1;
    const UMIP_SHIFT: usize = 2;
    const PKU_SHIFT: usize = 3;
    const OSPKE_SHIFT: usize = 4;
    const WAITPKG_SHIFT: usize = 5;
    const AVX512_VBMI2_SHIFT: usize = 6;
    const CET_SS_SHIFT: usize = 7;
    const GFNI_SHIFT: usize = 8;
    const VAES_SHIFT: usize = 9;
    const VPCLMULQDQ_SHIFT: usize = 10;
    const AVX512_VNNI_SHIFT: usize = 11;
    const AVX512_BITALG_SHIFT: usize = 12;
    const TME_EN_SHIFT: usize = 13;
    const AVX512_VPOPCNTDQ_SHIFT: usize = 14;
    const LA57_SHIFT: usize = 16;
    const MAWAU_SHIFT: usize = 17;
    const RDPID_AND_IA32_TSC_AUX_SHIFT: usize = 22;
    const KL_SHIFT: usize = 23;
    const CLDEMOTE_SHIFT: usize = 25;
    const MOVDIRI_SHIFT: usize = 27;
    const MOVDIR64B_SHIFT: usize = 28;
    const SGX_LC_SHIFT: usize = 30;
    const PKS_SHIFT: usize = 31;

    const MAWAU_SHIFT_END: usize = 21;
    const MAWAU_LENGTH: usize = Self::MAWAU_SHIFT_END - Self::MAWAU_SHIFT + 1;

    const PREFETCHWT1_MASK: u32 = (1 << Self::PREFETCHWT1_SHIFT) as u32;
    const AVX512_VBMI_MASK: u32 = (1 << Self::AVX512_VBMI_SHIFT) as u32;
    const UMIP_MASK: u32 = (1 << Self::UMIP_SHIFT) as u32;
    const PKU_MASK: u32 = (1 << Self::PKU_SHIFT) as u32;
    const OSPKE_MASK: u32 = (1 << Self::OSPKE_SHIFT) as u32;
    const WAITPKG_MASK: u32 = (1 << Self::WAITPKG_SHIFT) as u32;
    const AVX512_VBMI2_MASK: u32 = (1 << Self::AVX512_VBMI2_SHIFT) as u32;
    const CET_SS_MASK: u32 = (1 << Self::CET_SS_SHIFT) as u32;
    const GFNI_MASK: u32 = (1 << Self::GFNI_SHIFT) as u32;
    const VAES_MASK: u32 = (1 << Self::VAES_SHIFT) as u32;
    const VPCLMULQDQ_MASK: u32 = (1 << Self::VPCLMULQDQ_SHIFT) as u32;
    const AVX512_VNNI_MASK: u32 = (1 << Self::AVX512_VNNI_SHIFT) as u32;
    const AVX512_BITALG_MASK: u32 = (1 << Self::AVX512_BITALG_SHIFT) as u32;
    const TME_EN_MASK: u32 = (1 << Self::TME_EN_SHIFT) as u32;
    const AVX512_VPOPCNTDQ_MASK: u32 = (1 << Self::AVX512_VPOPCNTDQ_SHIFT) as u32;
    const LA57_MASK: u32 = (1 << Self::LA57_SHIFT) as u32;
    const MAWAU_MASK: u32 = (((1 << Self::MAWAU_LENGTH) - 1) << Self::MAWAU_SHIFT) as u32;
    const RDPID_AND_IA32_TSC_AUX_MASK: u32 = (1 << Self::RDPID_AND_IA32_TSC_AUX_SHIFT) as u32;
    const KL_MASK: u32 = (1 << Self::KL_SHIFT) as u32;
    const CLDEMOTE_MASK: u32 = (1 << Self::CLDEMOTE_SHIFT) as u32;
    const MOVDIRI_MASK: u32 = (1 << Self::MOVDIRI_SHIFT) as u32;
    const MOVDIR64B_MASK: u32 = (1 << Self::MOVDIR64B_SHIFT) as u32;
    const SGX_LC_MASK: u32 = (1 << Self::SGX_LC_SHIFT) as u32;
    const PKS_MASK: u32 = (1 << Self::PKS_SHIFT) as u32;

    pub fn supports_5_level_paging(&self) -> bool {
        self.la57
    }
}

impl From<u32> for Ecx {
    fn from(ecx: u32) -> Self {
        let prefetchwt1: bool = ecx & Self::PREFETCHWT1_MASK != 0;
        let avx512_vbmi: bool = ecx & Self::AVX512_VBMI_MASK != 0;
        let umip: bool = ecx & Self::UMIP_MASK != 0;
        let pku: bool = ecx & Self::PKU_MASK != 0;
        let ospke: bool = ecx & Self::OSPKE_MASK != 0;
        let waitpkg: bool = ecx & Self::WAITPKG_MASK != 0;
        let avx512_vbmi2: bool = ecx & Self::AVX512_VBMI2_MASK != 0;
        let cet_ss: bool = ecx & Self::CET_SS_MASK != 0;
        let gfni: bool = ecx & Self::GFNI_MASK != 0;
        let vaes: bool = ecx & Self::VAES_MASK != 0;
        let vpclmulqdq: bool = ecx & Self::VPCLMULQDQ_MASK != 0;
        let avx512_vnni: bool = ecx & Self::AVX512_VNNI_MASK != 0;
        let avx512_bitalg: bool = ecx & Self::AVX512_BITALG_MASK != 0;
        let tme_en: bool = ecx & Self::TME_EN_MASK != 0;
        let avx512_vpopcntdq: bool = ecx & Self::AVX512_VPOPCNTDQ_MASK != 0;
        let la57: bool = ecx & Self::LA57_MASK != 0;
        let mawau: u8 = ((ecx & Self::MAWAU_MASK) << Self::MAWAU_SHIFT) as u8;
        let rdpid_and_ia32_tsc_aux: bool = ecx & Self::RDPID_AND_IA32_TSC_AUX_MASK != 0;
        let kl: bool = ecx & Self::KL_MASK != 0;
        let cldemote: bool = ecx & Self::CLDEMOTE_MASK != 0;
        let movdiri: bool = ecx & Self::MOVDIRI_MASK != 0;
        let movdir64b: bool = ecx & Self::MOVDIR64B_MASK != 0;
        let sgx_lc: bool = ecx & Self::SGX_LC_MASK != 0;
        let pks: bool = ecx & Self::PKS_MASK != 0;
        Self {
            prefetchwt1,
            avx512_vbmi,
            umip,
            pku,
            ospke,
            waitpkg,
            avx512_vbmi2,
            cet_ss,
            gfni,
            vaes,
            vpclmulqdq,
            avx512_vnni,
            avx512_bitalg,
            tme_en,
            avx512_vpopcntdq,
            la57,
            mawau,
            rdpid_and_ia32_tsc_aux,
            kl,
            cldemote,
            movdiri,
            movdir64b,
            sgx_lc,
            pks,
        }
    }
}

