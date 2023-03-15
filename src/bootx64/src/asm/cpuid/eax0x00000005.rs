use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000005 {
    eax: Eax,
}

impl Eax0x00000005 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        if 5 <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx: _,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(5);
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
    smallest_monitor_line_size: u16,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let smallest_monitor_line_size: u16 = eax as u16;
        Self {
            smallest_monitor_line_size,
        }
    }
}

