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
    uefi::system_table::SystemTable,
};

#[no_mangle]
fn efi_main(image_handle: u64, system_table: &SystemTable) -> u64 {
    serial_println!("Hello, World!");
    serial_println!("image_handle = {:#018x}", image_handle);
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

