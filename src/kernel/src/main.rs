#![no_std]
#![no_main]

mod asm;
mod serial;
mod uefi;

use {
    core::panic::PanicInfo,
    uefi::{
        services::boot::memory_allocation,
        tables::system,
    },
};

#[no_mangle]
pub extern "C" fn main(serial: &serial::Serial, system: &system::System, memory_map: &memory_allocation::PassedMap) -> ! {
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
    serial_println!("system = {:#x?}", system);
    serial_println!("memory_map = {:#x?}", memory_map);
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

