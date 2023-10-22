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
        protocols::{
            console_support::graphics_output,
            media_access::simple_file_system,
        },
        services::boot::memory_allocation,
        tables::system,
        types::{
            handle,
            status,
            void,
        },
    },
};

#[no_mangle]
fn efi_main(image_handle: handle::Handle<'static>, system_table: &'static mut system::System<'static>) -> status::Status {
    serial::Serial::init_com1();
    system::init_system(image_handle, system_table);
    uefi_println!("Hello, World!");
    uefi_println!("image_handle = {:#x?}", system::image());
    uefi_println!("system_table = {:#x?}", system::system());
    let mut kernel = Kernel::new();
    let memory_map: &memory_allocation::Map = &system::exit_boot_services();
    serial_println!("memory_map = {:#x?}", memory_map);
    let memory_map: memory_allocation::PassedMap = memory_map.into();
    kernel.run(system::image(), system::system(), &memory_map, serial::Serial::com1());
    panic!("Can't run the kernel!");
}

#[allow(dead_code)]
#[derive(Debug)]
struct Kernel<'a> {
    elf: elf::Elf<'a>,
    cpuid: Option<cpuid::Cpuid>,
    gdt: gdt::Gdt,
    physical_page_present_bit_map: memory::PhysicalPagePresentBitMap,
    page_map: BTreeMap<usize, usize>,
    paging: paging::State<'a>,
    stack: memory::Pages<'a>,
    stack_floor: &'a void::Void,
    cr0: control::register0::Cr0,
    cr2: control::register2::Cr2,
    cr3: control::register3::Cr3,
    cr4: control::register4::Cr4,
    ia32_efer: Option<ia32_efer::Ia32Efer>,
    graphics_output: &'a graphics_output::GraphicsOutput<'a>,
}

impl Kernel<'_> {
    fn new() -> Self {
        let memory_map = memory_allocation::Map::new();
        let memory_size: memory_allocation::PhysicalAddress = memory_map.get_memory_size();
        let memory_size = memory_size as usize;
        serial_println!("memory_size = {:#x?}", memory_size);
        let memory_map: Vec<memory_allocation::MemoryDescriptor> = (&memory_map).into();
        let physical_page_present_bit_map: memory::PhysicalPagePresentBitMap = (&memory_map).into();
        let cpuid: Option<cpuid::Cpuid> = cpuid::Cpuid::new();
        let supports_5_level_paging: bool = match cpuid {
            Some(ref cpuid) => cpuid.supports_5_level_paging(),
            None => false,
        };
        serial_println!("supports_5_level_paging = {:?}", supports_5_level_paging);
        let ia32_efer: Option<ia32_efer::Ia32Efer> = ia32_efer::Ia32Efer::get(&cpuid);
        let cr0 = control::register0::Cr0::get();
        let cr2 = control::register2::Cr2::get();
        let cr3 = control::register3::Cr3::get();
        let cr4 = control::register4::Cr4::get();
        let mut paging = paging::State::new(&cr0, &cr3, &cr4, &ia32_efer, memory_size);
        paging.map_highest_parallel(memory_size);
        // Open the file system.
        let simple_file_system = simple_file_system::SimpleFileSystem::new();
        let elf: Vec<u8> = simple_file_system.read_file("/kernel.elf");
        let elf = elf::Elf::new(&elf[..]);
        let gdt: Vec<gdt::Descriptor> = gdt::Register::get().into();
        serial_println!("old gdt = {:#x?}", gdt);
        let gdt = gdt::Gdt::new();
        serial_println!("new gdt = {:#x?}", gdt);
        let mut page_map: BTreeMap<usize, usize> = elf.page_map();
        let stack_floor = usize::MAX - (memory_size - 1);
        let stack = memory::Pages::new(0x10);
        let stack_pages: usize = stack.pages();
        stack
            .physical_addresses()
            .enumerate()
            .map(|(i, physical_address)| (stack_floor - (stack_pages - i) * memory_allocation::PAGE_SIZE, physical_address))
            .for_each(|(virtual_address, physical_address)| {
                page_map.insert(physical_address as usize, virtual_address);
            });
        let stack_floor: &void::Void = stack_floor.into();
        page_map
            .values()
            .for_each(|virtual_address| paging.divide_page(*virtual_address));
        page_map
            .iter()
            .for_each(|(physical_address, virtual_address)| paging.set_physical_address(*virtual_address, *physical_address));
        // Get a graphic output protocol.
        let graphics_output: &graphics_output::GraphicsOutput = graphics_output::GraphicsOutput::new();
        Self {
            elf,
            cpuid,
            gdt,
            physical_page_present_bit_map,
            page_map,
            paging,
            stack,
            stack_floor,
            cr0,
            cr2,
            cr3,
            cr4,
            ia32_efer,
            graphics_output,
        }
    }

    fn run(
        &mut self,
        image: handle::Handle<'static>,
        system: &system::System,
        memory_map: &memory_allocation::PassedMap,
        serial: &serial::Serial
    ) {
        let physical_page_present_bit_map: &[u8] = (&self.physical_page_present_bit_map).into();
        let stack_floor: &void::Void = self.stack_floor;
        let cr0: &control::register0::Cr0 = &(self.cr0);
        let cr2: &control::register2::Cr2 = &(self.cr2);
        let cr4: &control::register4::Cr4 = &(self.cr4);
        let ia32_efer: &Option<ia32_efer::Ia32Efer> = &(self.ia32_efer);
        let cr3: &control::register3::Cr3 = &control::register3::Cr3::set(self.paging.get_cr3());
        let graphics_output = self.graphics_output;
        let kernel_arguments = elf::KernelArguments::new(
            image,
            system,
            physical_page_present_bit_map,
            memory_map,
            stack_floor,
            cr0,
            cr2,
            cr3,
            cr4,
            ia32_efer,
            serial,
            graphics_output,
        );
        self.gdt.set();
        serial_println!("Kernel.run()");
        self.elf.run(kernel_arguments)
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

