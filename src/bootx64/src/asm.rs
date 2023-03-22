// References
// Intel 64 an IA-32 Architectures Software Developer's Manual

pub mod control;
pub mod cpuid;
pub mod rflags;

use core::arch::asm;

pub type Port = u16;

pub fn get_cr3() -> u64 {
    let mut cr3: u64;
    unsafe {
        asm!(
            "mov rax, cr3",
            out("rax") cr3,
        );
    }
    cr3
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
            "push rax",
            "popfq",
            in("rax") rflags,
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

