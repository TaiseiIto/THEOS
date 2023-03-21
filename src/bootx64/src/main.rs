#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

mod allocator;
mod asm;
mod elf;
mod serial;
mod uefi;

use {
    alloc::vec::Vec,
    asm::cpuid,
    core::panic::PanicInfo,
    uefi::{
        protocols::media_access::simple_file_system,
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
    use_boot_services();
    let _memory_map: memory_allocation::Map = system::exit_boot_services();
    serial_println!("Succeeded in exiting boot services.");
    loop {
        asm::hlt();
    }
}

fn use_boot_services() {
    uefi_println!("Hello, World!");
    uefi_println!("image_handle = {:#x?}", system::image());
    uefi_println!("system_table = {:#x?}", system::system());
    let memory_map = memory_allocation::Map::new();
    let memory_size: memory_allocation::PhysicalAddress = memory_map.get_memory_size();
    let memory_map: Vec<memory_allocation::MemoryDescriptor> = (&memory_map).into();
    uefi_println!("memory_map = {:#x?}", memory_map);
    uefi_println!("memory_size = {:#x}", memory_size);
    let cpuid: Option<cpuid::Cpuid> = cpuid::Cpuid::new();
    uefi_println!("cpuid = {:#x?}", cpuid);
    let supports_5_level_paging: bool = match cpuid {
        Some(cpuid) => cpuid.supports_5_level_paging(),
        None => false,
    };
    uefi_println!("supports_5_level_paging = {}", supports_5_level_paging);
    let cr0: u64 = asm::get_cr0();
    uefi_println!("cr0 = {:#018x}", cr0);
    let cr3: u64 = asm::get_cr3();
    uefi_println!("cr3 = {:#018x}", cr3);
    // Open the file system.
    let simple_file_system = simple_file_system::SimpleFileSystem::new();
    uefi_println!("simple_file_system = {:#x?}", simple_file_system);
    let kernel_elf: Vec<u8> = simple_file_system.read_file("/kernel.elf");
    let kernel_elf = elf::Elf::new(&kernel_elf[..]);
    uefi_println!("kernel_elf = {:#x?}", kernel_elf);
    // Close kernel.elf and the root directory.
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

