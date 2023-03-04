#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

mod asm;
mod serial;

use {
    core::{
        fmt::Write,
        panic::PanicInfo,
    },
    serial::print_format,
};

#[no_mangle]
fn efi_main(_image_handle: u64, _system_table: u64) -> u64 {
    println!("Hello, World!");
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

