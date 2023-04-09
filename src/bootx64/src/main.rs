#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

mod allocator;
mod asm;
mod elf;
mod gdt;
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
    kernel.run();
    panic!("Can't run the kernel!");
}

#[allow(dead_code)]
#[derive(Debug)]
struct Kernel<'a> {
    elf: elf::Elf<'a>,
    cpuid: Option<cpuid::Cpuid>,
    paging: paging::State<'a>,
    page_map: BTreeMap<usize, usize>,
    stack: memory::Pages<'a>,
}

impl Kernel<'_> {
    fn new() -> Self {
        uefi_println!("Hello, World!");
        uefi_println!("image_handle = {:#x?}", system::image());
        uefi_println!("system_table = {:#x?}", system::system());
        let memory_map = memory_allocation::Map::new();
        let memory_size: memory_allocation::PhysicalAddress = memory_map.get_memory_size();
        let memory_size = memory_size as usize;
        let memory_map: Vec<memory_allocation::MemoryDescriptor> = (&memory_map).into();
        uefi_println!("memory_map = {:#x?}", memory_map);
        uefi_println!("memory_size = {:#x}", memory_size);
        let cpuid: Option<cpuid::Cpuid> = cpuid::Cpuid::new();
        let supports_5_level_paging: bool = match cpuid {
            Some(ref cpuid) => cpuid.supports_5_level_paging(),
            None => false,
        };
        uefi_println!("supports_5_level_paging = {:?}", supports_5_level_paging);
        let ia32_efer: Option<ia32_efer::Ia32Efer> = ia32_efer::Ia32Efer::get(&cpuid);
        let cr0 = control::register0::Cr0::get();
        let _cr2 = control::register2::Cr2::get();
        let cr3 = control::register3::Cr3::get();
        let cr4 = control::register4::Cr4::get();
        let mut paging = paging::State::new(&cr0, &cr3, &cr4, &ia32_efer, memory_size);
        // Open the file system.
        let simple_file_system = simple_file_system::SimpleFileSystem::new();
        let elf: Vec<u8> = simple_file_system.read_file("/kernel.elf");
        let elf = elf::Elf::new(&elf[..]);
        let mut page_map: BTreeMap<usize, usize> = elf.page_map();
        let stack = memory::Pages::new(1);
        page_map.insert(stack.physical_address() as usize, 0xfffffffffffff000);
        uefi_println!("page_map = {:#x?}", page_map);
        page_map
            .values()
            .for_each(|virtual_address| paging.divide_page(*virtual_address));
        let gdt: Vec<gdt::Descriptor> = gdt::Register::get().into();
        uefi_println!("gdt = {:#x?}", gdt);
        Self {
            elf,
            cpuid,
            paging,
            page_map,
            stack,
        }
    }

    fn run(&mut self) {
        self.page_map
            .iter()
            .for_each(|(physical_address, virtual_address)| self.paging.set_physical_address(*virtual_address, *physical_address));
        asm::set_cr3(self.paging.get_cr3());
        serial_println!("Kernel.run()");
        self.elf.run()
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

