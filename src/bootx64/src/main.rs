#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

mod asm;
mod serial;
mod uefi;

use {
    core::panic::PanicInfo,
    serial::print,
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
    serial_println!("Hello, World!");
    serial_println!("image_handle = {:#x?}", image_handle);
    serial_println!("system_table = {:#x?}", system_table);
    let status: status::Status = system_table.con_out.reset(false);
    serial_println!("status = {}", status);
    let status: status::Status = system_table.con_out.output_string("Hello, World!\n");
    serial_println!("status = {}", status);
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

