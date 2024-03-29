use {
    alloc::{
        string::String,
        vec::Vec,
    },
    super::CpuidOutRegisters,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000000 {
    max_eax: u32,
    vendor: String,
}

impl Eax0x00000000 {
    pub fn new() -> Self {
        let eax: u32 = 0x00000000;
        let ecx: u32 = 0x00000000;
        let CpuidOutRegisters {
            eax,
            ebx,
            edx,
            ecx,
        } = CpuidOutRegisters::cpuid(eax, ecx);
        let max_eax: u32 = eax;
        let vendor: [u32; 3] = [ebx, edx, ecx];
        let vendor: Vec<u8> = vendor
            .into_iter()
            .map(|dword| dword
                .to_le_bytes()
                .into_iter())
            .flatten()
            .collect();
        let vendor = String::from_utf8(vendor).expect("Can't get CPUID(EAX=0x00000000)!");
        Self {
            max_eax,
            vendor,
        }
    }

    pub fn max_eax(&self) -> u32 {
        self.max_eax
    }
}


