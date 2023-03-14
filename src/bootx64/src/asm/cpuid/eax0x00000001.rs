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
}

impl Eax0x00000001 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        if 1 <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx,
                ecx: _,
            } = CpuidOutRegisters::cpuid(1);
            let eax: Eax = eax.into();
            let ebx: Ebx = ebx.into();
            let edx: Edx = edx.into();
            Some(Self {
                eax,
                ebx,
                edx,
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

