#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

mod asm;
mod serial;
mod uefi;

use {
    core::{
        fmt::Write,
        panic::PanicInfo,
    },
    uefi::{
        types::{
            handle,
            status,
        },
        tables::system,
    },
};

#[no_mangle]
fn efi_main(image_handle: handle::Handle, system_table: &system::System) -> status::Status {
    let mut com1 = serial::Serial::new(serial::COM1PORT, serial::BAUD);
    serial_println!(com1, "Hello, World!");
    serial_println!(com1, "image_handle = {:#x?}", image_handle);
    serial_println!(com1, "system_table = {:#x?}", system_table);
    let status: status::Status = system_table.con_out.reset(false);
    serial_println!(com1, "status = {}", status);
    let status: status::Status = system_table.con_out.output_string("Hello, World!\n");
    serial_println!(com1, "status = {}", status);
    loop {
        asm::hlt();
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    loop {
        asm::hlt();
    }
}

