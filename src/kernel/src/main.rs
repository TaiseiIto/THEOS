#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod asm;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    asm::outb(0x03f8, 0x48);
    asm::outb(0x03f8, 0x65);
    asm::outb(0x03f8, 0x6c);
    asm::outb(0x03f8, 0x6c);
    asm::outb(0x03f8, 0x6f);
    asm::outb(0x03f8, 0x2c);
    asm::outb(0x03f8, 0x20);
    asm::outb(0x03f8, 0x6b);
    asm::outb(0x03f8, 0x65);
    asm::outb(0x03f8, 0x72);
    asm::outb(0x03f8, 0x6e);
    asm::outb(0x03f8, 0x65);
    asm::outb(0x03f8, 0x6c);
    asm::outb(0x03f8, 0x2e);
    asm::outb(0x03f8, 0x65);
    asm::outb(0x03f8, 0x6c);
    asm::outb(0x03f8, 0x66);
    asm::outb(0x03f8, 0x0a);
    serial_println!("Hello, kernel.elf!");
    loop {
        asm::hlt();
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

