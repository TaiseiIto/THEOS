// References
// Intel 64 an IA-32 Architectures Software Developer's Manual, Volume 4 Model Specific Registers, Chapter 2 Model Specific Registers (MSRs)

pub mod architectural;

use core::arch::asm;

fn rdmsr(address: u32) -> u64 {
    let ecx: u32 = address;
    let mut eax: u32;
    let mut edx: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") ecx,
            out("eax") eax,
            out("edx") edx,
        );
    }
    (edx as u64) << 32 | (eax as u64)
}

fn wrmsr(address: u32, value: u64) {
    let eax: u32 = (value & 0x00000000ffffffff) as u32;
    let ecx: u32 = address;
    let edx: u32 = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("eax") eax,
            in("ecx") ecx,
            in("edx") edx,
        );
    }
}

