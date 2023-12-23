// References
// Intel 64 an IA-32 Architectures Software Developer's Manual

pub mod control;
pub mod cpuid;
pub mod msr;
pub mod rflags;

use core::arch::asm;

pub type Port = u16;

pub fn get_rip() -> u64 {
        let mut rip: u64;
        unsafe {
            asm!(
                "call 0f",
                "0:",
                "pop rax",
                out("rax") rip,
            );
        }
        rip
}

fn get_rflags() -> u64 {
    let mut rflags: u64;
    unsafe {
        asm!(
            "pushfq",
            "pop rax",
            out("rax") rflags,
        );
    }
    rflags
}

fn set_rflags(rflags: u64) {
    unsafe {
        asm!(
            "push rcx",
            "popfq",
            in("rcx") rflags,
        );
    }
}

pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

pub fn inb(port: Port) -> u8 {
    let mut value: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
        );
    }
    value
}

pub fn outb(port: Port, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
        );
    }
}

