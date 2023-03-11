#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

mod allocator;
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
fn efi_main(image_handle: handle::Handle, system_table: &'static mut system::System<'static>) -> status::Status {
    allocator::Allocator::set_system(system_table.clone());
    let mut com1 = serial::Serial::new(serial::COM1PORT, serial::BAUD);
    serial_println!(&mut com1, "Hello, World!");
    let status: status::Status = system_table.con_out.reset(false);
    serial_println!(&mut com1, "status = {}", status);
    uefi_println!(system_table, "Hello, World!");
    uefi_println!(system_table, "image_handle = {:#x?}", image_handle);
    uefi_println!(system_table, "system_table = {:#x?}", system_table.clone());

    let memory_map_size: usize = memory_allocation::Map::get_size(system_table);
    uefi_println!(system_table, "memory_map_size = {:#x}", memory_map_size);

    let memory_type = memory_allocation::MemoryType::LoaderData;
    let memory_map = void::Void::new();
    let mut memory_map = &memory_map;
    let status: status::Status = system_table.boot_services.allocate_pool(
        memory_type,
        memory_map_size,
        &mut memory_map,
    );
    uefi_println!(system_table, "status = {:#x}", status);

    let status: status::Status = system_table.boot_services.free_pool(memory_map);
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

