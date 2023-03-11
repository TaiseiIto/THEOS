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
    allocator::Allocator::set_system(system_table.clone());
    let mut com1 = serial::Serial::new(serial::COM1PORT, serial::BAUD);
    serial_println!(&mut com1, "Hello, World!");
    let status: status::Status = system_table.con_out.reset(false);
    serial_println!(&mut com1, "status = {}", status);
    uefi_println!(system_table, "Hello, World!");
    uefi_println!(system_table, "image_handle = {:#x?}", image_handle);
    uefi_println!(system_table, "system_table = {:#x?}", system_table.clone());
    let memory_map_size: usize = memory_allocation::Map::get_size(system_table);
    uefi_println!(system_table, "memory_map_size = {:#x}", memory_map_size);
    uefi_println!(system_table, "vec![1, 2, 3] = {:#x?}", vec![1, 2, 3]);
    let mut vec = Vec::new();
    vec.push(4);
    vec.push(5);
    vec.push(6);
    uefi_println!(system_table, "vec = {:#x?}", vec);
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

