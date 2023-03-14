use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000001 {
    eax: Eax,
    ebx: Ebx,
}

impl Eax0x00000001 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        if 1 <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(1);
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

