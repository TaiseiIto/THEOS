#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

mod allocator;
mod asm;
mod elf;
mod memory;
mod serial;
mod uefi;

use {
    alloc::vec::Vec,
    asm::{
        control,
        cpuid,
        msr::architectural::ia32_efer,
    },
    core::panic::PanicInfo,
    memory::paging,
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
        Some(ref cpuid) => cpuid.supports_5_level_paging(),
        None => false,
    };
    uefi_println!("supports_5_level_paging = {}", supports_5_level_paging);
    let ia32_efer: Option<ia32_efer::Ia32Efer> = ia32_efer::Ia32Efer::get(&cpuid);
    uefi_println!("IA32_EFER = {:#x?}", ia32_efer);
    let cr0 = control::register0::Cr0::get();
    uefi_println!("CR0 = {:#x?}", cr0);
    let cr2 = control::register2::Cr2::get();
    uefi_println!("CR2 = {:#x?}", cr2);
    let cr3 = control::register3::Cr3::get();
    uefi_println!("CR3 = {:#x?}", cr3);
    let cr4 = control::register4::Cr4::get();
    uefi_println!("CR4 = {:#x?}", cr4);
    let paging = paging::State::get(&cr0, &cr3, &cr4, &ia32_efer);
    uefi_println!("paging = {:#x?}", paging);
    // Open the file system.
    let simple_file_system = simple_file_system::SimpleFileSystem::new();
    uefi_println!("simple_file_system = {:#x?}", simple_file_system);
    let kernel_elf: Vec<u8> = simple_file_system.read_file("/kernel.elf");
    let kernel_elf = elf::Elf::new(&kernel_elf[..]);
    uefi_println!("kernel_elf = {:#x?}", kernel_elf);
    uefi_println!("kernel_elf.necessary_page_numbers() = {:#x?}", kernel_elf.necessary_page_numbers());
    // Close kernel.elf and the root directory.
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

