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
    alloc::vec::Vec,
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
fn efi_main(image_handle: handle::Handle<'static>, system_table: &'static mut system::System<'static>) -> status::Status {
    serial::Serial::init_com1();
    serial_println!("Hello, World!");
    system::init_system(image_handle, system_table);
    uefi_println!("Hello, World!");
    uefi_println!("image_handle = {:#x?}", system::image());
    uefi_println!("system_table = {:#x?}", system::system());
    let memory_map = memory_allocation::Map::new();
    let memory_size: memory_allocation::PhysicalAddress = memory_map.get_memory_size();
    let memory_map: Vec<memory_allocation::MemoryDescriptor> = (&memory_map).into();
    uefi_println!("memory_map = {:#x?}", memory_map);
    uefi_println!("memory_size = {:#x}", memory_size);
    let _memory_map: memory_allocation::Map = system::exit_boot_services();
    serial_println!("Succeeded in exiting boot services.");
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

