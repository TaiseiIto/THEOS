#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

mod allocator;
mod asm;
mod serial;
mod uefi;

use {
    alloc::vec::Vec,
    core::panic::PanicInfo,
    uefi::{
        services::boot::memory_allocation,
        types::{
            handle,
            status,
        },
        tables::system,
    },
};

#[no_mangle]
fn efi_main(image_handle: handle::Handle<'static>, system_table: &'static mut system::System<'static>) -> status::Status {
    system::init_system(image_handle, system_table);
    let mut com1 = serial::Serial::new(serial::COM1PORT, serial::BAUD);
    serial_println!(&mut com1, "Hello, World!");
    uefi_println!("Hello, World!");
    uefi_println!("image_handle = {:#x?}", system::image());
    uefi_println!("system_table = {:#x?}", system::system());
    let memory_map = memory_allocation::Map::new();
    let memory_map: Vec<memory_allocation::MemoryDescriptor> = (&memory_map).into();
    uefi_println!("memory_map = {:#x?}", memory_map);
    let _memory_map: memory_allocation::Map = system::exit_boot_services();
    loop {
        asm::hlt();
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    let mut com1 = serial::Serial::new(serial::COM1PORT, serial::BAUD);
    serial_println!(&mut com1, "{}", panic);
    loop {
        asm::hlt();
    }
}

