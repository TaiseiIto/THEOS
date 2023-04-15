#![no_std]
#![no_main]

mod asm;
mod serial;
mod uefi;

use {
    core::panic::PanicInfo,
    uefi::tables::system,
};

#[no_mangle]
pub extern "C" fn main(serial: &serial::Serial, system: &system::System) -> ! {
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
    serial_println!("system = {:#x?}", system);
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

