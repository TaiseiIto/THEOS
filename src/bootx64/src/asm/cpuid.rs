use {
    alloc::{
        string::String,
        vec::Vec,
    },
    core::arch::asm,
};

#[derive(Debug)]
pub struct CpuidEax0x00000000 {
    max_eax: u32,
    vendor: String,
}

impl CpuidEax0x00000000 {
    pub fn new() -> Self {
        let CpuidOutRegisters {
            eax,
            ebx,
            edx,
            ecx,
        } = CpuidOutRegisters::cpuid(0);
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
}

#[derive(Debug)]
pub struct CpuidOutRegisters {
    eax: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
}

impl CpuidOutRegisters {
    pub fn cpuid(eax_input: u32) -> Self {
        let mut eax: u32;
        let mut ebx: u32;
        let mut edx: u32;
        let mut ecx: u32;
        unsafe {
            asm!(
                "cpuid",
                "mov esi, ebx",
                in("eax") eax_input,
                lateout("eax") eax,
                out("esi") ebx,
                out("edx") edx,
                out("ecx") ecx,
            );
        }
        Self {
            eax,
            ebx,
            edx,
            ecx,
        }
    }
}

