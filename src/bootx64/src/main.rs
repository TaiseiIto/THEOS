#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

mod asm;
mod serial;

use core::{
    fmt::Write,
    panic::PanicInfo,
};

#[no_mangle]
fn efi_main(_image_handle: u64, _system_table: u64) -> u64 {
    let mut com1 = serial::Serial::new(serial::COM1, serial::BAUD);
    com1.write_str("Hello, World!\n");
    loop {
        asm::hlt();
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        asm::hlt();
    }
}

