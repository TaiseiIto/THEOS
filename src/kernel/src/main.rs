#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod asm;
mod serial;

#[no_mangle]
pub extern "C" fn main() -> ! {
    serial::Serial::init_com1();
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

