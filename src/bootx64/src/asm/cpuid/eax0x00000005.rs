use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000005 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
}

impl Eax0x00000005 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        if 5 <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx,
                ecx: _,
            } = CpuidOutRegisters::cpuid(5);
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    largest_monitor_line_size: u16,
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let largest_monitor_line_size: u16 = ebx as u16;
        Self {
            largest_monitor_line_size,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    number_of_sub_c_states: [u8; 8],
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let mut number_of_sub_c_states: [u8; 8] = [0; 8];
        for i in 0..8 {
            number_of_sub_c_states[i] = ((edx >> (4 * i)) & 0x0f) as u8;
        }
        Self {
            number_of_sub_c_states,
        }
    }
}
