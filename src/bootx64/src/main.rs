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
        handle,
        status,
        system_table,
    },
};

#[no_mangle]
fn efi_main(image_handle: handle::Handle, system_table: &system_table::SystemTable) -> status::Status {
    serial_println!("Hello, World!");
    serial_println!("image_handle = {:#x?}", image_handle);
    serial_println!("system_table = {:#x?}", system_table);
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

