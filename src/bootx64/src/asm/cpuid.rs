pub mod eax0x00000000;
pub mod eax0x00000001;
pub mod eax0x00000002;
pub mod eax0x00000004;
pub mod eax0x00000005;
pub mod eax0x00000006;
pub mod eax0x00000007;
pub mod eax0x00000009;
pub mod eax0x0000000a;
pub mod eax0x0000000b;
pub mod eax0x0000000d;
pub mod eax0x80000000;
pub mod eax0x80000001;

use {
    core::arch::asm,
    crate::{
        uefi_print,
        uefi_println,
    },
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
    eax0x00000006: Option<eax0x00000006::Eax0x00000006>,
    eax0x00000007: Option<eax0x00000007::Eax0x00000007>,
    eax0x00000009: Option<eax0x00000009::Eax0x00000009>,
    eax0x0000000a: Option<eax0x0000000a::Eax0x0000000a>,
    eax0x0000000b: Option<eax0x0000000b::Eax0x0000000b>,
    eax0x0000000d: Option<eax0x0000000d::Eax0x0000000d>,
    eax0x80000000: eax0x80000000::Eax0x80000000,
    eax0x80000001: Option<eax0x80000001::Eax0x80000001>,
}

impl Cpuid {
    pub fn new() -> Option<Self> {
        if rflags::Rflags::cpuid_is_supported() {
            let eax0x00000000 = eax0x00000000::Eax0x00000000::new();
            let eax0x00000001 = eax0x00000001::Eax0x00000001::new(&eax0x00000000);
            let eax0x00000002 = eax0x00000002::Eax0x00000002::new(&eax0x00000000);
            let eax0x00000004 = eax0x00000004::Eax0x00000004::new(&eax0x00000000);
            let eax0x00000005 = eax0x00000005::Eax0x00000005::new(&eax0x00000000);
            let eax0x00000006 = eax0x00000006::Eax0x00000006::new(&eax0x00000000);
            let eax0x00000007 = eax0x00000007::Eax0x00000007::new(&eax0x00000000);
            let eax0x00000009 = eax0x00000009::Eax0x00000009::new(&eax0x00000000);
            let eax0x0000000a = eax0x0000000a::Eax0x0000000a::new(&eax0x00000000);
            let eax0x0000000b = eax0x0000000b::Eax0x0000000b::new(&eax0x00000000);
            let eax0x0000000d = eax0x0000000d::Eax0x0000000d::new(&eax0x00000000);
            let eax0x80000000 = eax0x80000000::Eax0x80000000::new();
            let eax0x80000001 = eax0x80000001::Eax0x80000001::new(&eax0x00000000);
            Some(Self {
                eax0x00000000,
                eax0x00000001,
                eax0x00000002,
                eax0x00000004,
                eax0x00000005,
                eax0x00000006,
                eax0x00000007,
                eax0x00000009,
                eax0x0000000a,
                eax0x0000000b,
                eax0x0000000d,
                eax0x80000000,
                eax0x80000001,
            })
        } else {
            None
        }
    }

    pub fn supports_5_level_paging(&self) -> bool {
        match &self.eax0x00000007 {
            Some(eax0x00000007) => eax0x00000007.supports_5_level_paging(),
            None => false,
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
    pub fn cpuid(eax_input: u32, ecx_input: u32) -> Self {
        let mut eax: u32;
        let mut ebx: u32;
        let mut edx: u32;
        let mut ecx: u32;
        unsafe {
            asm!(
                "cpuid",
                "mov esi, ebx",
                in("eax") eax_input,
                in("ecx") ecx_input,
                lateout("eax") eax,
                out("esi") ebx,
                out("edx") edx,
                lateout("ecx") ecx,
            );
        }
        uefi_println!("CPUID instruction!");
        uefi_println!("in eax = {:#?}", eax_input);
        uefi_println!("in ecx = {:#?}", ecx_input);
        uefi_println!("out eax = {:#?}", eax);
        uefi_println!("out ebx = {:#?}", ebx);
        uefi_println!("out edx = {:#?}", edx);
        uefi_println!("out ecx = {:#?}", ecx);
        Self {
            eax,
            ebx,
            edx,
            ecx,
        }
    }
}

