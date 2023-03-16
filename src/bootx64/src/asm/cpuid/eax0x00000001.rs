use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000001 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
    ecx: Ecx,
}

impl Eax0x00000001 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 1;
        let ecx: u32 = 0;
        if eax <= eax0x00000000.max_eax() {
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
            Some(Self {
                eax,
                ebx,
                edx,
                ecx,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    stepping_id: u8,
    model: u8,
    family_id: u8,
    processor_type: u8,
    extended_model_id: u8,
    extended_family_id: u8,
}

impl Eax {
    const STEPPING_ID_SHIFT: usize = 0;
    const MODEL_SHIFT: usize = 4;
    const FAMILY_ID_SHIFT: usize = 8;
    const PROCESSOR_TYPE_SHIFT: usize = 12;
    const EXTENDED_MODEL_ID_SHIFT: usize = 16;
    const EXTENDED_FAMILY_ID_SHIFT: usize = 20;

    const STEPPING_ID_SHIFT_END: usize = 3;
    const MODEL_SHIFT_END: usize = 7;
    const FAMILY_ID_SHIFT_END: usize = 11;
    const PROCESSOR_TYPE_SHIFT_END: usize = 13;
    const EXTENDED_MODEL_ID_SHIFT_END: usize = 19;
    const EXTENDED_FAMILY_ID_SHIFT_END: usize = 27;

    const STEPPING_ID_LENGTH: usize = Self::STEPPING_ID_SHIFT_END - Self::STEPPING_ID_SHIFT + 1;
    const MODEL_LENGTH: usize = Self::MODEL_SHIFT_END - Self::MODEL_SHIFT + 1;
    const FAMILY_ID_LENGTH: usize = Self::FAMILY_ID_SHIFT_END - Self::FAMILY_ID_SHIFT + 1;
    const PROCESSOR_TYPE_LENGTH: usize = Self::PROCESSOR_TYPE_SHIFT_END - Self::PROCESSOR_TYPE_SHIFT + 1;
    const EXTENDED_MODEL_ID_LENGTH: usize = Self::EXTENDED_MODEL_ID_SHIFT_END - Self::EXTENDED_MODEL_ID_SHIFT + 1;
    const EXTENDED_FAMILY_ID_LENGTH: usize = Self::EXTENDED_FAMILY_ID_SHIFT_END - Self::EXTENDED_FAMILY_ID_SHIFT + 1;

    const STEPPING_ID_MASK: u32 = (((1 << Self::STEPPING_ID_LENGTH) - 1) << Self::STEPPING_ID_SHIFT) as u32;
    const MODEL_MASK: u32 = (((1 << Self::MODEL_LENGTH) - 1) << Self::MODEL_SHIFT) as u32;
    const FAMILY_ID_MASK: u32 = (((1 << Self::FAMILY_ID_LENGTH) - 1) << Self::FAMILY_ID_SHIFT) as u32;
    const PROCESSOR_TYPE_MASK: u32 = (((1 << Self::PROCESSOR_TYPE_LENGTH) - 1) << Self::PROCESSOR_TYPE_SHIFT) as u32;
    const EXTENDED_MODEL_ID_MASK: u32 = (((1 << Self::EXTENDED_MODEL_ID_LENGTH) - 1) << Self::EXTENDED_MODEL_ID_SHIFT) as u32;
    const EXTENDED_FAMILY_ID_MASK: u32 = (((1 << Self::EXTENDED_FAMILY_ID_LENGTH) - 1) << Self::EXTENDED_FAMILY_ID_SHIFT) as u32;
}
    
impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let stepping_id = ((eax & Self::STEPPING_ID_MASK) >> Self::STEPPING_ID_SHIFT) as u8;
        let model = ((eax & Self::MODEL_MASK) >> Self::MODEL_SHIFT) as u8;
        let family_id = ((eax & Self::FAMILY_ID_MASK) >> Self::FAMILY_ID_SHIFT) as u8;
        let processor_type = ((eax & Self::PROCESSOR_TYPE_MASK) >> Self::PROCESSOR_TYPE_SHIFT) as u8;
        let extended_model_id = ((eax & Self::EXTENDED_MODEL_ID_MASK) >> Self::EXTENDED_MODEL_ID_SHIFT) as u8;
        let extended_family_id = ((eax & Self::EXTENDED_FAMILY_ID_MASK) >> Self::EXTENDED_FAMILY_ID_SHIFT) as u8;
        Self {
            stepping_id,
            model,
            family_id,
            processor_type,
            extended_model_id,
            extended_family_id,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    brand_index: u8,
    clflush_line_size: u8,
    max_num_of_ids: u8,
    initial_apic_id: u8,
}

impl Ebx {
    const BRAND_INDEX_SHIFT: usize = 0;
    const CLFLUSH_LINE_SIZE_SHIFT: usize = 8;
    const MAX_NUM_OF_IDS_SHIFT: usize = 16;
    const INITIAL_APIC_ID_SHIFT: usize = 24;

    const BRAND_INDEX_SHIFT_END: usize = 7;
    const CLFLUSH_LINE_SIZE_SHIFT_END: usize = 15;
    const MAX_NUM_OF_IDS_SHIFT_END: usize = 23;
    const INITIAL_APIC_ID_SHIFT_END: usize = 31;

    const BRAND_INDEX_LENGTH: usize = Self::BRAND_INDEX_SHIFT_END - Self::BRAND_INDEX_SHIFT + 1;
    const CLFLUSH_LINE_SIZE_LENGTH: usize = Self::CLFLUSH_LINE_SIZE_SHIFT_END - Self::CLFLUSH_LINE_SIZE_SHIFT + 1;
    const MAX_NUM_OF_IDS_LENGTH: usize = Self::MAX_NUM_OF_IDS_SHIFT_END - Self::MAX_NUM_OF_IDS_SHIFT + 1;
    const INITIAL_APIC_ID_LENGTH: usize = Self::INITIAL_APIC_ID_SHIFT_END - Self::INITIAL_APIC_ID_SHIFT + 1;

    const BRAND_INDEX_MASK: u32 = (((1 << Self::BRAND_INDEX_LENGTH) - 1) << Self::BRAND_INDEX_SHIFT) as u32;
    const CLFLUSH_LINE_SIZE_MASK: u32 = (((1 << Self::CLFLUSH_LINE_SIZE_LENGTH) - 1) << Self::CLFLUSH_LINE_SIZE_SHIFT) as u32;
    const MAX_NUM_OF_IDS_MASK: u32 = (((1 << Self::MAX_NUM_OF_IDS_LENGTH) - 1) << Self::MAX_NUM_OF_IDS_SHIFT) as u32;
    const INITIAL_APIC_ID_MASK: u32 = (((1 << Self::INITIAL_APIC_ID_LENGTH) - 1) << Self::INITIAL_APIC_ID_SHIFT) as u32;
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let brand_index = ((ebx & Self::BRAND_INDEX_MASK) >> Self::BRAND_INDEX_SHIFT) as u8;
        let clflush_line_size = ((ebx & Self::CLFLUSH_LINE_SIZE_MASK) >> Self::CLFLUSH_LINE_SIZE_SHIFT) as u8;
        let max_num_of_ids = ((ebx & Self::MAX_NUM_OF_IDS_MASK) >> Self::MAX_NUM_OF_IDS_SHIFT) as u8;
        let initial_apic_id = ((ebx & Self::INITIAL_APIC_ID_MASK) >> Self::INITIAL_APIC_ID_SHIFT) as u8;
        Self {
            brand_index,
            clflush_line_size,
            max_num_of_ids,
            initial_apic_id,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    fpu: bool,
    vme: bool,
    de: bool,
    pse: bool,
    tsc: bool,
    msr: bool,
    pae: bool,
    mce: bool,
    cx8: bool,
    apic: bool,
    sep: bool,
    mtrr: bool,
    pge: bool,
    mca: bool,
    cmov: bool,
    pat: bool,
    pse36: bool,
    psn: bool,
    clfsh: bool,
    ds: bool,
    acpi: bool,
    mmx: bool,
    fxsr: bool,
    sse: bool,
    sse2: bool,
    ss: bool,
    htt: bool,
    tm: bool,
    pbe: bool,
}

impl Edx {
    const FPU_SHIFT: usize = 0;
    const VME_SHIFT: usize = 1;
    const DE_SHIFT: usize = 2;
    const PSE_SHIFT: usize = 3;
    const TSC_SHIFT: usize = 4;
    const MSR_SHIFT: usize = 5;
    const PAE_SHIFT: usize = 6;
    const MCE_SHIFT: usize = 7;
    const CX8_SHIFT: usize = 8;
    const APIC_SHIFT: usize = 9;
    const SEP_SHIFT: usize = 11;
    const MTRR_SHIFT: usize = 12;
    const PGE_SHIFT: usize = 13;
    const MCA_SHIFT: usize = 14;
    const CMOV_SHIFT: usize = 15;
    const PAT_SHIFT: usize = 16;
    const PSE36_SHIFT: usize = 17;
    const PSN_SHIFT: usize = 18;
    const CLFSH_SHIFT: usize = 19;
    const DS_SHIFT: usize = 21;
    const ACPI_SHIFT: usize = 22;
    const MMX_SHIFT: usize = 23;
    const FXSR_SHIFT: usize = 24;
    const SSE_SHIFT: usize = 25;
    const SSE2_SHIFT: usize = 26;
    const SS_SHIFT: usize = 27;
    const HTT_SHIFT: usize = 28;
    const TM_SHIFT: usize = 29;
    const PBE_SHIFT: usize = 31;


    const FPU_MASK: u32 = (1 << Self::FPU_SHIFT) as u32;
    const VME_MASK: u32 = (1 << Self::VME_SHIFT) as u32;
    const DE_MASK: u32 = (1 << Self::DE_SHIFT) as u32;
    const PSE_MASK: u32 = (1 << Self::PSE_SHIFT) as u32;
    const TSC_MASK: u32 = (1 << Self::TSC_SHIFT) as u32;
    const MSR_MASK: u32 = (1 << Self::MSR_SHIFT) as u32;
    const PAE_MASK: u32 = (1 << Self::PAE_SHIFT) as u32;
    const MCE_MASK: u32 = (1 << Self::MCE_SHIFT) as u32;
    const CX8_MASK: u32 = (1 << Self::CX8_SHIFT) as u32;
    const APIC_MASK: u32 = (1 << Self::APIC_SHIFT) as u32;
    const SEP_MASK: u32 = (1 << Self::SEP_SHIFT) as u32;
    const MTRR_MASK: u32 = (1 << Self::MTRR_SHIFT) as u32;
    const PGE_MASK: u32 = (1 << Self::PGE_SHIFT) as u32;
    const MCA_MASK: u32 = (1 << Self::MCA_SHIFT) as u32;
    const CMOV_MASK: u32 = (1 << Self::CMOV_SHIFT) as u32;
    const PAT_MASK: u32 = (1 << Self::PAT_SHIFT) as u32;
    const PSE36_MASK: u32 = (1 << Self::PSE36_SHIFT) as u32;
    const PSN_MASK: u32 = (1 << Self::PSN_SHIFT) as u32;
    const CLFSH_MASK: u32 = (1 << Self::CLFSH_SHIFT) as u32;
    const DS_MASK: u32 = (1 << Self::DS_SHIFT) as u32;
    const ACPI_MASK: u32 = (1 << Self::ACPI_SHIFT) as u32;
    const MMX_MASK: u32 = (1 << Self::MMX_SHIFT) as u32;
    const FXSR_MASK: u32 = (1 << Self::FXSR_SHIFT) as u32;
    const SSE_MASK: u32 = (1 << Self::SSE_SHIFT) as u32;
    const SSE2_MASK: u32 = (1 << Self::SSE2_SHIFT) as u32;
    const SS_MASK: u32 = (1 << Self::SS_SHIFT) as u32;
    const HTT_MASK: u32 = (1 << Self::HTT_SHIFT) as u32;
    const TM_MASK: u32 = (1 << Self::TM_SHIFT) as u32;
    const PBE_MASK: u32 = (1 << Self::PBE_SHIFT) as u32;
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let fpu: bool = edx & Self::FPU_MASK != 0;
        let vme: bool = edx & Self::VME_MASK != 0;
        let de: bool = edx & Self::DE_MASK != 0;
        let pse: bool = edx & Self::PSE_MASK != 0;
        let tsc: bool = edx & Self::TSC_MASK != 0;
        let msr: bool = edx & Self::MSR_MASK != 0;
        let pae: bool = edx & Self::PAE_MASK != 0;
        let mce: bool = edx & Self::MCE_MASK != 0;
        let cx8: bool = edx & Self::CX8_MASK != 0;
        let apic: bool = edx & Self::APIC_MASK != 0;
        let sep: bool = edx & Self::SEP_MASK != 0;
        let mtrr: bool = edx & Self::MTRR_MASK != 0;
        let pge: bool = edx & Self::PGE_MASK != 0;
        let mca: bool = edx & Self::MCA_MASK != 0;
        let cmov: bool = edx & Self::CMOV_MASK != 0;
        let pat: bool = edx & Self::PAT_MASK != 0;
        let pse36: bool = edx & Self::PSE36_MASK != 0;
        let psn: bool = edx & Self::PSN_MASK != 0;
        let clfsh: bool = edx & Self::CLFSH_MASK != 0;
        let ds: bool = edx & Self::DS_MASK != 0;
        let acpi: bool = edx & Self::ACPI_MASK != 0;
        let mmx: bool = edx & Self::MMX_MASK != 0;
        let fxsr: bool = edx & Self::FXSR_MASK != 0;
        let sse: bool = edx & Self::SSE_MASK != 0;
        let sse2: bool = edx & Self::SSE2_MASK != 0;
        let ss: bool = edx & Self::SS_MASK != 0;
        let htt: bool = edx & Self::HTT_MASK != 0;
        let tm: bool = edx & Self::TM_MASK != 0;
        let pbe: bool = edx & Self::PBE_MASK != 0;
        Self {
            fpu,
            vme,
            de,
            pse,
            tsc,
            msr,
            pae,
            mce,
            cx8,
            apic,
            sep,
            mtrr,
            pge,
            mca,
            cmov,
            pat,
            pse36,
            psn,
            clfsh,
            ds,
            acpi,
            mmx,
            fxsr,
            sse,
            sse2,
            ss,
            htt,
            tm,
            pbe,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx {
    sse3: bool,
    pclmulqdq: bool,
    dtes64: bool,
    monitor: bool,
    ds_cpl: bool,
    vmx: bool,
    smx: bool,
    eist: bool,
    tm2: bool,
    ssse3: bool,
    cnxt_id: bool,
    sdbg: bool,
    fma: bool,
    cmpxchg16b: bool,
    xtpr_update_control: bool,
    pdcm: bool,
    pcid: bool,
    dca: bool,
    sse4_1: bool,
    sse4_2: bool,
    x2apic: bool,
    movbe: bool,
    popcnt: bool,
    tsc_deadline: bool,
    aesni: bool,
    xsave: bool,
    osxsave: bool,
    avx: bool,
    f16c: bool,
    rdrand: bool,
}

impl Ecx {
    const SSE3_SHIFT: usize = 0;
    const PCLMULQDQ_SHIFT: usize = 1;
    const DTES64_SHIFT: usize = 2;
    const MONITOR_SHIFT: usize = 3;
    const DS_CPL_SHIFT: usize = 4;
    const VMX_SHIFT: usize = 5;
    const SMX_SHIFT: usize = 6;
    const EIST_SHIFT: usize = 7;
    const TM2_SHIFT: usize = 8;
    const SSSE3_SHIFT: usize = 9;
    const CNXT_ID_SHIFT: usize = 10;
    const SDBG_SHIFT: usize = 11;
    const FMA_SHIFT: usize = 12;
    const CMPXCHG16B_SHIFT: usize = 13;
    const XTPR_UPDATE_CONTROL_SHIFT: usize = 14;
    const PDCM_SHIFT: usize = 15;
    const PCID_SHIFT: usize = 17;
    const DCA_SHIFT: usize = 18;
    const SSE4_1_SHIFT: usize = 19;
    const SSE4_2_SHIFT: usize = 20;
    const X2APIC_SHIFT: usize = 21;
    const MOVBE_SHIFT: usize = 22;
    const POPCNT_SHIFT: usize = 23;
    const TSC_DEADLINE_SHIFT: usize = 24;
    const AESNI_SHIFT: usize = 25;
    const XSAVE_SHIFT: usize = 26;
    const OSXSAVE_SHIFT: usize = 27;
    const AVX_SHIFT: usize = 28;
    const F16C_SHIFT: usize = 29;
    const RDRAND_SHIFT: usize = 30;

    const SSE3_MASK: u32 = (1 << Self::SSE3_SHIFT) as u32;
    const PCLMULQDQ_MASK: u32 = (1 << Self::PCLMULQDQ_SHIFT) as u32;
    const DTES64_MASK: u32 = (1 << Self::DTES64_SHIFT) as u32;
    const MONITOR_MASK: u32 = (1 << Self::MONITOR_SHIFT) as u32;
    const DS_CPL_MASK: u32 = (1 << Self::DS_CPL_SHIFT) as u32;
    const VMX_MASK: u32 = (1 << Self::VMX_SHIFT) as u32;
    const SMX_MASK: u32 = (1 << Self::SMX_SHIFT) as u32;
    const EIST_MASK: u32 = (1 << Self::EIST_SHIFT) as u32;
    const TM2_MASK: u32 = (1 << Self::TM2_SHIFT) as u32;
    const SSSE3_MASK: u32 = (1 << Self::SSSE3_SHIFT) as u32;
    const CNXT_ID_MASK: u32 = (1 << Self::CNXT_ID_SHIFT) as u32;
    const SDBG_MASK: u32 = (1 << Self::SDBG_SHIFT) as u32;
    const FMA_MASK: u32 = (1 << Self::FMA_SHIFT) as u32;
    const CMPXCHG16B_MASK: u32 = (1 << Self::CMPXCHG16B_SHIFT) as u32;
    const XTPR_UPDATE_CONTROL_MASK: u32 = (1 << Self::XTPR_UPDATE_CONTROL_SHIFT) as u32;
    const PDCM_MASK: u32 = (1 << Self::PDCM_SHIFT) as u32;
    const PCID_MASK: u32 = (1 << Self::PCID_SHIFT) as u32;
    const DCA_MASK: u32 = (1 << Self::DCA_SHIFT) as u32;
    const SSE4_1_MASK: u32 = (1 << Self::SSE4_1_SHIFT) as u32;
    const SSE4_2_MASK: u32 = (1 << Self::SSE4_2_SHIFT) as u32;
    const X2APIC_MASK: u32 = (1 << Self::X2APIC_SHIFT) as u32;
    const MOVBE_MASK: u32 = (1 << Self::MOVBE_SHIFT) as u32;
    const POPCNT_MASK: u32 = (1 << Self::POPCNT_SHIFT) as u32;
    const TSC_DEADLINE_MASK: u32 = (1 << Self::TSC_DEADLINE_SHIFT) as u32;
    const AESNI_MASK: u32 = (1 << Self::AESNI_SHIFT) as u32;
    const XSAVE_MASK: u32 = (1 << Self::XSAVE_SHIFT) as u32;
    const OSXSAVE_MASK: u32 = (1 << Self::OSXSAVE_SHIFT) as u32;
    const AVX_MASK: u32 = (1 << Self::AVX_SHIFT) as u32;
    const F16C_MASK: u32 = (1 << Self::F16C_SHIFT) as u32;
    const RDRAND_MASK: u32 = (1 << Self::RDRAND_SHIFT) as u32;
}

impl From<u32> for Ecx {
    fn from(ecx: u32) -> Self {
        let sse3: bool = ecx & Self::SSE3_MASK != 0;
        let pclmulqdq: bool = ecx & Self::PCLMULQDQ_MASK != 0;
        let dtes64: bool = ecx & Self::DTES64_MASK != 0;
        let monitor: bool = ecx & Self::MONITOR_MASK != 0;
        let ds_cpl: bool = ecx & Self::DS_CPL_MASK != 0;
        let vmx: bool = ecx & Self::VMX_MASK != 0;
        let smx: bool = ecx & Self::SMX_MASK != 0;
        let eist: bool = ecx & Self::EIST_MASK != 0;
        let tm2: bool = ecx & Self::TM2_MASK != 0;
        let ssse3: bool = ecx & Self::SSSE3_MASK != 0;
        let cnxt_id: bool = ecx & Self::CNXT_ID_MASK != 0;
        let sdbg: bool = ecx & Self::SDBG_MASK != 0;
        let fma: bool = ecx & Self::FMA_MASK != 0;
        let cmpxchg16b: bool = ecx & Self::CMPXCHG16B_MASK != 0;
        let xtpr_update_control: bool = ecx & Self::XTPR_UPDATE_CONTROL_MASK != 0;
        let pdcm: bool = ecx & Self::PDCM_MASK != 0;
        let pcid: bool = ecx & Self::PCID_MASK != 0;
        let dca: bool = ecx & Self::DCA_MASK != 0;
        let sse4_1: bool = ecx & Self::SSE4_1_MASK != 0;
        let sse4_2: bool = ecx & Self::SSE4_2_MASK != 0;
        let x2apic: bool = ecx & Self::X2APIC_MASK != 0;
        let movbe: bool = ecx & Self::MOVBE_MASK != 0;
        let popcnt: bool = ecx & Self::POPCNT_MASK != 0;
        let tsc_deadline: bool = ecx & Self::TSC_DEADLINE_MASK != 0;
        let aesni: bool = ecx & Self::AESNI_MASK != 0;
        let xsave: bool = ecx & Self::XSAVE_MASK != 0;
        let osxsave: bool = ecx & Self::OSXSAVE_MASK != 0;
        let avx: bool = ecx & Self::AVX_MASK != 0;
        let f16c: bool = ecx & Self::F16C_MASK != 0;
        let rdrand: bool = ecx & Self::RDRAND_MASK != 0;
        Self {
            sse3,
            pclmulqdq,
            dtes64,
            monitor,
            ds_cpl,
            vmx,
            smx,
            eist,
            tm2,
            ssse3,
            cnxt_id,
            sdbg,
            fma,
            cmpxchg16b,
            xtpr_update_control,
            pdcm,
            pcid,
            dca,
            sse4_1,
            sse4_2,
            x2apic,
            movbe,
            popcnt,
            tsc_deadline,
            aesni,
            xsave,
            osxsave,
            avx,
            f16c,
            rdrand,
        }
    }
}

