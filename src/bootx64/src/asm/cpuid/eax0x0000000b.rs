use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x0000000b {
    eax: Eax,
}

impl Eax0x0000000b {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000b;
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
    unique_topology_id_shift: u8,
}

impl Eax {
    const UNIQUE_TOPOLOGY_ID_SHIFT_MASK: u32 = 0x0000001f;
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Eax {
        let unique_topology_id_shift: u8 = (eax & Self::UNIQUE_TOPOLOGY_ID_SHIFT_MASK) as u8;
        Self {
            unique_topology_id_shift,
        }
    }
}

