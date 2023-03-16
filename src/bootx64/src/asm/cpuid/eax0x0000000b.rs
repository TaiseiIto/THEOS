use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x0000000b {
    eax: Eax,
    ebx: Ebx,
}

impl Eax0x0000000b {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000b;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(eax, ecx);
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    number_of_logical_processors: u16,
}

impl Ebx {
    const NUMBER_OF_LOGICAL_PROCESSORS_MASK: u32 = 0x0000001f;
}

impl From<u32> for Ebx {
    fn from(eax: u32) -> Ebx {
        let number_of_logical_processors: u16 = (eax & Self::NUMBER_OF_LOGICAL_PROCESSORS_MASK) as u16;
        Self {
            number_of_logical_processors,
        }
    }
}

