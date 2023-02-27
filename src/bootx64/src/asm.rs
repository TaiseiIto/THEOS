use core::arch::asm;

pub type Port = u16;

pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
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

