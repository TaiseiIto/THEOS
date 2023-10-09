// References
// Intel 64 an IA-32 Architectures Software Developer's Manual

pub mod control;
pub mod msr;

use core::arch::asm;

pub type Port = u16;

pub fn get_rsp() -> usize {
    let mut rsp: usize;
    unsafe {
        asm!(
            "mov rax, rsp",
            out("rax") rsp,
        );
    }
    rsp
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

