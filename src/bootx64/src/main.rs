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
    alloc::{
        vec::Vec,
        string::String,
    },
    asm::cpuid,
    core::panic::PanicInfo,
    uefi::{
        protocols::media_access::{
            file_protocol,
            simple_file_system,
        },
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
    // Open the root directory.
    let mut root: &file_protocol::FileProtocol = simple_file_system.open_volume();
    uefi_println!("root = {:#x?}", root);
    // Find kernel.elf.
    let kernel_elf: file_protocol::FileInformation = root
        .find(|file_information| file_information.file_name() == String::from("kernel.elf"))
        .expect("kernel.elf is nou found!");
    uefi_println!("kernel_elf = {:#x?}", kernel_elf);
    let kernel_elf_size: usize = kernel_elf.file_size();
    uefi_println!("kernel_elf_size = {:#x}", kernel_elf_size);
    // Open kernel.elf.
    let read = true;
    let write = false;
    let create = false;
    let open_mode = file_protocol::OpenMode::new(
        read,
        write,
        create,
    );
    let read_only: bool = false;
    let hidden: bool = false;
    let system: bool = false;
    let reserved: bool = false;
    let directory: bool = false;
    let archive: bool = false;
    let attributes = file_protocol::Attributes::new(
        read_only,
        hidden,
        system,
        reserved,
        directory,
        archive,
    );
    let kernel_elf: &file_protocol::FileProtocol = root
        .open_child(&kernel_elf, &open_mode, &attributes)
        .expect("Can't open kernel.elf!");
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

