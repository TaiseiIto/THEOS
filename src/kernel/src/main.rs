#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod asm;
mod serial;

#[no_mangle]
pub extern "C" fn main(serial: &serial::Serial) -> ! {
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
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

