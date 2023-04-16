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
pub extern "C" fn main(kernel_arguments: &'static mut KernelArguments) -> ! {
    let KernelArguments {
        image,
        system,
        physical_page_present_bit_map,
        memory_map,
        serial,
    } = kernel_arguments;
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
    system::init_system(image, *system);
    let image: handle::Handle = system::image();
    let image: *const void::Void = image as *const void::Void;
    let image: usize = image as usize;
    serial_println!("image = {:#x}", image);
    serial_println!("system = {:#x?}", system::system());
    let physical_page_present_bit_map: &'static mut [u8] = *physical_page_present_bit_map;
    let memory_map: memory_allocation::MemoryDescriptors = (*memory_map).into();
    physical_page::Manager::init(physical_page_present_bit_map, &memory_map);
    loop {
        asm::hlt();
    }
}

pub struct KernelArguments<'a> {
    image: handle::Handle<'static>,
    system: &'a mut system::System<'a>,
    physical_page_present_bit_map: &'a mut [u8],
    memory_map: &'a memory_allocation::Map<'a>,
    serial: &'a serial::Serial,
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

