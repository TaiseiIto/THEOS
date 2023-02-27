use core::arch::asm;

pub type Port = u16;

pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

