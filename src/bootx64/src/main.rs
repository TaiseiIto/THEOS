#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

mod asm;
mod serial;

use core::panic::PanicInfo;

#[no_mangle]
fn efi_main(_image_handle: u64, _system_table: u64) -> u64 {
    serial::COM1.put_char('A' as u8);
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

