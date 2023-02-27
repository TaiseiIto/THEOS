#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

use core::arch::asm;
use log::info;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize UEFI services.
    uefi_services::init(&mut system_table).unwrap();

    // Print a simple sentence.
    info!("Hello, World!");

    // Infinite loop.
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

