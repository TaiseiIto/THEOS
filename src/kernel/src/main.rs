#![no_std]
#![no_main]

mod asm;
mod memory;
mod serial;
mod uefi;

use {
    core::panic::PanicInfo,
    memory::physical_page,
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
pub extern "C" fn main(
    image: handle::Handle<'static>,
    system: &'static mut system::System,
    physical_page_present_bit_map: &'static mut &'static mut [u8],
    memory_map: &memory_allocation::Map,
    serial: &serial::Serial
) -> ! {
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
    system::init_system(image, system);
    let image: handle::Handle = system::image();
    let image: *const void::Void = image as *const void::Void;
    let image: usize = image as usize;
    serial_println!("image = {:#x}", image);
    serial_println!("system = {:#x?}", system::system());
    let physical_page_present_bit_map: &'static mut [u8] = *physical_page_present_bit_map;
    let memory_map: memory_allocation::MemoryDescriptors = memory_map.into();
    physical_page::PresentBitMap::init(physical_page_present_bit_map, &memory_map);
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

