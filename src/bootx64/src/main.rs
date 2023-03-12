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
    alloc::{
        vec,
        vec::Vec,
    },
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
fn efi_main(image_handle: handle::Handle, system_table: &'static mut system::System<'static>) -> status::Status {
    system::init_system(system_table);
    let mut com1 = serial::Serial::new(serial::COM1PORT, serial::BAUD);
    serial_println!(&mut com1, "Hello, World!");
    uefi_println!("Hello, World!");
    uefi_println!("image_handle = {:#x?}", image_handle);
    let memory_map_size: usize = memory_allocation::Map::get_size();
    uefi_println!("memory_map_size = {:#x}", memory_map_size);
    uefi_println!("vec![1, 2, 3] = {:#x?}", vec![1, 2, 3]);
    let mut vec = Vec::new();
    vec.push(4);
    vec.push(5);
    vec.push(6);
    uefi_println!("vec = {:#x?}", vec);
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

