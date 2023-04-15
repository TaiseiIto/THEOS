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
        types::{
            handle,
            void,
        },
    },
};

#[no_mangle]
pub extern "C" fn main(image: handle::Handle<'static>, system: &'static mut system::System, memory_map: &memory_allocation::Map, serial: &serial::Serial) -> ! {
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
    system::init_system(image, system);
    let image: handle::Handle = system::image();
    let image: *const void::Void = image as *const void::Void;
    let image: usize = image as usize;
    serial_println!("image = {:#x}", image);
    serial_println!("system = {:#x?}", system::system());
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

