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
            void,
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

    let mut memory_map_size: usize = 0;
    let mut memory_map: u8 = 0;
    let mut map_key: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;
    let status: status::Status = system_table.boot_services.get_memory_map(
        &mut memory_map_size,
        &mut memory_map,
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );
    uefi_println!(system_table, "memory_map_size = {:#x}", memory_map_size);
    uefi_println!(system_table, "map_key = {:#x}", map_key);
    uefi_println!(system_table, "descriptor_size = {:#x}", descriptor_size);
    uefi_println!(system_table, "descriptor_version = {:#x}", descriptor_version);
    uefi_println!(system_table, "status = {:#x}", status);

    let memory_type = memory_allocation::MemoryType::LoaderData;
    let memory_map = void::Void::new();
    let status: status::Status = system_table.boot_services.allocate_pool(
        memory_type,
        memory_map_size,
        &mut &memory_map,
    );
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

