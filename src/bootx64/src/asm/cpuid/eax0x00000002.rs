use {
    alloc::vec::Vec,
    super::{
        CpuidOutRegisters,
        eax0x00000000::Eax0x00000000,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000002 {
    descriptors: Vec<u8>,
}

impl Eax0x00000002 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 2;
        let ecx: u32 = 0;
        if eax <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx,
                ecx,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let mut eax: [u8; 4] = eax.to_le_bytes();
            let ebx: [u8; 4] = ebx.to_le_bytes();
            let edx: [u8; 4] = edx.to_le_bytes();
            let ecx: [u8; 4] = ecx.to_le_bytes();
            eax[0] = 0x00;
            let descriptors: [[u8; 4]; 4] = [eax, ebx, edx, ecx];
            let descriptors: Vec<u8> = descriptors
                .into_iter()
                .filter(|reg| reg[3] < 0x80)
                .map(|reg| reg
                    .into_iter()
                    .filter(|byte| *byte != 0)
                )
                .flatten()
                .collect();
            Some(Self {
                descriptors,
            })
        } else {
            None
        }
    }
}

