use super::{
    CpuidOutRegisters,
    eax0x80000000::Eax0x80000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x80000006 {
    ecx: Ecx,
}

impl Eax0x80000006 {
    pub fn new(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000006;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x80000000.max_eax() {
            let CpuidOutRegisters {
                eax: _,
                ebx: _,
                edx: _,
                ecx,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let ecx: Ecx = ecx.into();
            Some(Self {
                ecx,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx {
    cache_line_size: u8,
    l2_associativity_field: u8,
    cache_size: u16,
}

impl Ecx {
    const CACHE_LINE_SIZE_SHIFT: usize = 0;
    const L2_ASSOCIATIVITY_FIELD_SHIFT: usize = 12;
    const CACHE_SIZE_SHIFT: usize = 16;

    const CACHE_LINE_SIZE_SHIFT_END: usize = 7;
    const L2_ASSOCIATIVITY_FIELD_SHIFT_END: usize = 15;
    const CACHE_SIZE_SHIFT_END: usize = 31;

    const CACHE_LINE_SIZE_LENGTH: usize = Self::CACHE_LINE_SIZE_SHIFT_END - Self::CACHE_LINE_SIZE_SHIFT + 1;
    const L2_ASSOCIATIVITY_FIELD_LENGTH: usize = Self::L2_ASSOCIATIVITY_FIELD_SHIFT_END - Self::L2_ASSOCIATIVITY_FIELD_SHIFT + 1;
    const CACHE_SIZE_LENGTH: usize = Self::CACHE_SIZE_SHIFT_END - Self::CACHE_SIZE_SHIFT + 1;

    const CACHE_LINE_SIZE_MASK: u32 = (((1 << Self::CACHE_LINE_SIZE_LENGTH) - 1) << Self::CACHE_LINE_SIZE_SHIFT) as u32;
    const L2_ASSOCIATIVITY_FIELD_MASK: u32 = (((1 << Self::L2_ASSOCIATIVITY_FIELD_LENGTH) - 1) << Self::L2_ASSOCIATIVITY_FIELD_SHIFT) as u32;
    const CACHE_SIZE_MASK: u32 = (((1 << Self::CACHE_SIZE_LENGTH) - 1) << Self::CACHE_SIZE_SHIFT) as u32;
}

impl From<u32> for Ecx {
    fn from(ecx: u32) -> Self {
        let cache_line_size = ((ecx & Self::CACHE_LINE_SIZE_MASK) >> Self::CACHE_LINE_SIZE_SHIFT) as u8;
        let l2_associativity_field = ((ecx & Self::L2_ASSOCIATIVITY_FIELD_MASK) >> Self::L2_ASSOCIATIVITY_FIELD_SHIFT) as u8;
        let cache_size = ((ecx & Self::CACHE_SIZE_MASK) >> Self::CACHE_SIZE_SHIFT) as u16;
        Self {
            cache_line_size,
            l2_associativity_field,
            cache_size,
        }
    }
}

