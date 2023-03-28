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
    alloc::{
        collections::btree_map::BTreeMap,
        vec::Vec,
    },
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
    let mut kernel = Kernel::new();
    let _memory_map: memory_allocation::Map = system::exit_boot_services();
    kernel.start();
    loop {
        asm::hlt();
    }
}

#[allow(dead_code)]
struct Kernel<'a> {
    elf: elf::Elf<'a>,
    cpuid: Option<cpuid::Cpuid>,
    paging: paging::State<'a>,
    page_map: BTreeMap<usize, usize>,
}

impl Kernel<'_> {
    fn new() -> Self {
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
        let mut paging = paging::State::get(&cr0, &cr3, &cr4, &ia32_efer);
        // Open the file system.
        let simple_file_system = simple_file_system::SimpleFileSystem::new();
        uefi_println!("simple_file_system = {:#x?}", simple_file_system);
        let elf: Vec<u8> = simple_file_system.read_file("/kernel.elf");
        let elf = elf::Elf::new(&elf[..]);
        uefi_println!("elf = {:#x?}", elf);
        let page_map: BTreeMap<usize, usize> = elf.page_map();
        page_map
            .keys()
            .chain(page_map.values())
            .for_each(|virtual_address| paging.divide_page(*virtual_address));
        uefi_println!("page_map = {:#x?}", page_map);
        Self {
            elf,
            cpuid,
            paging,
            page_map,
        }
    }

    fn start(&mut self) {
        self.page_map
            .iter()
            .for_each(|(physical_address, virtual_address)| self.paging.swap_pages(*physical_address, *virtual_address));
        serial_println!("Succeeded in swapping pages!");
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

