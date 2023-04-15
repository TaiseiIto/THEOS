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
pub extern "C" fn main(serial: &serial::Serial, system: &system::System, memory_map: &memory_allocation::Map) -> ! {
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
    serial_println!("system = {:#x?}", system);
    serial_println!("memory_map = {:#x?}", memory_map);
    let memory_map: memory_allocation::MemoryDescriptors = memory_map.into();
    memory_map.for_each(|memory_descriptor| {
        serial_println!("memory_descriptor = {:#x?}", memory_descriptor);
    });
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

