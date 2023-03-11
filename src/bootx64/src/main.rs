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
    let allocate_type = memory_allocation::AllocateType::AllocateAnyPages;
    let memory_type = memory_allocation::MemoryType::LoaderData;
    let pages: usize = 1;
    let mut memory: memory_allocation::PhysicalAddress = 0;
    let status: status::Status = system_table.boot_services.allocate_pages(
        allocate_type,
        memory_type,
        pages,
        &mut memory,
    );
    uefi_println!(system_table, "memory = {:#x}", memory);
    uefi_println!(system_table, "status = {:#x}", status);
    let status: status::Status = system_table.boot_services.free_pages(memory, pages);
    uefi_println!(system_table, "status = {:#x}", status);
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

