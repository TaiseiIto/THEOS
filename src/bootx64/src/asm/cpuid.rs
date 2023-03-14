pub mod eax0x00000000;
pub mod eax0x00000001;

use core::arch::asm;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cpuid {
    eax0x00000000: eax0x00000000::Eax0x00000000,
    eax0x00000001: Option<eax0x00000001::Eax0x00000001>,
}

impl Cpuid {
    pub fn new() -> Self {
        let eax0x00000000 = eax0x00000000::Eax0x00000000::new();
        let eax0x00000001 = eax0x00000001::Eax0x00000001::new(&eax0x00000000);
        Self {
            eax0x00000000,
            eax0x00000001,
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

