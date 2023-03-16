use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x0000000a {
    eax: Eax,
}

impl Eax0x0000000a {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000a;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
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
    version_id: u8,
    general_purpose_performance_monitoring_counter: u8,
    bit_width_of_general_purpose_performance_monitoring_counter: u8,
    length_of_ebx_bit_vector: u8,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let [
            version_id,
            general_purpose_performance_monitoring_counter,
            bit_width_of_general_purpose_performance_monitoring_counter,
            length_of_ebx_bit_vector,
        ]: [u8; 4] = eax.to_le_bytes();
        Self {
            version_id,
            general_purpose_performance_monitoring_counter,
            bit_width_of_general_purpose_performance_monitoring_counter,
            length_of_ebx_bit_vector,
        }
    }
}

