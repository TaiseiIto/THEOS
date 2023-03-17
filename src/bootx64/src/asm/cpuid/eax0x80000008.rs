use super::{
    CpuidOutRegisters,
    eax0x80000000::Eax0x80000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x80000008 {
    eax: Eax,
}

impl Eax0x80000008 {
    pub fn new(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000008;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x80000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx: _,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let eax: Eax = eax.into();
            Some(Self {
                eax,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    physical_address_bits: u8,
    linear_address_bits: u8,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let [
            physical_address_bits,
            linear_address_bits,
            _,
            _,
        ] = eax.to_le_bytes();
        Self {
            physical_address_bits,
            linear_address_bits,
        }
    }
}

