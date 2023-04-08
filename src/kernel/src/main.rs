#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod asm;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
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

