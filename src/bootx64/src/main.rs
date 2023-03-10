#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

mod asm;
mod serial;
mod uefi;

use {
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
fn efi_main(image_handle: handle::Handle, system_table: &mut system::System) -> status::Status {
    let mut com1 = serial::Serial::new(serial::COM1PORT, serial::BAUD);
    serial_println!(&mut com1, "Hello, World!");
    let status: status::Status = system_table.con_out.reset(false);
    serial_println!(&mut com1, "status = {}", status);
    uefi_println!(system_table, "Hello, World!");
    uefi_println!(system_table, "image_handle = {:#x?}", image_handle);
    uefi_println!(system_table, "system_table = {:#x?}", system_table.clone());
    let mut memory_map: [u8; 0x10000] = [0; 0x10000];
    let memory_map = memory_allocation::Map::new(&mut memory_map, system_table);
    uefi_println!(system_table, "memory_map = {:#x?}", memory_map);
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

