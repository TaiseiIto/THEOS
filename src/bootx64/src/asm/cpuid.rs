pub mod eax0x00000000;
pub mod eax0x00000001;
pub mod eax0x00000002;
pub mod eax0x00000004;
pub mod eax0x00000005;

use {
    core::arch::asm,
    super::rflags,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cpuid {
    eax0x00000000: eax0x00000000::Eax0x00000000,
    eax0x00000001: Option<eax0x00000001::Eax0x00000001>,
    eax0x00000002: Option<eax0x00000002::Eax0x00000002>,
    eax0x00000004: Option<eax0x00000004::Eax0x00000004>,
    eax0x00000005: Option<eax0x00000005::Eax0x00000005>,
}

impl Cpuid {
    pub fn new() -> Option<Self> {
        if rflags::Rflags::cpuid_is_supported() {
            let eax0x00000000 = eax0x00000000::Eax0x00000000::new();
            let eax0x00000001 = eax0x00000001::Eax0x00000001::new(&eax0x00000000);
            let eax0x00000002 = eax0x00000002::Eax0x00000002::new(&eax0x00000000);
            let eax0x00000004 = eax0x00000004::Eax0x00000004::new(&eax0x00000000);
            let eax0x00000005 = eax0x00000005::Eax0x00000005::new(&eax0x00000000);
            Some(Self {
                eax0x00000000,
                eax0x00000001,
                eax0x00000002,
                eax0x00000004,
                eax0x00000005,
            })
        } else {
            None
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

