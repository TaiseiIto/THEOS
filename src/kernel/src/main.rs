#![no_std]
#![no_main]

mod asm;
mod memory;
mod serial;
mod uefi;

use {
    asm::{
        control,
        msr::architectural::ia32_efer,
    },
    core::panic::PanicInfo,
    memory::physical_page,
    uefi::{
        protocols::console_support::graphics_output,
        services::boot::memory_allocation,
        tables::system,
        types::{
            handle,
            void,
        },
    },
};

#[no_mangle]
pub extern "C" fn main(kernel_arguments: &'static mut KernelArguments) -> ! {
    let KernelArguments {
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
    } = kernel_arguments;
    serial::Serial::init_com1(serial);
    serial_println!("Hello, kernel.elf!");
    serial_println!("RSP = {:#x}", asm::get_rsp());
    system::init_system(image, *system);
    let image: handle::Handle = system::image();
    let image: *const void::Void = image as *const void::Void;
    let image: usize = image as usize;
    serial_println!("image = {:#x}", image);
    serial_println!("system = {:#x?}", system::system());
    let physical_page_present_bit_map: &'static mut [u8] = *physical_page_present_bit_map;
    let memory_map: memory_allocation::MemoryDescriptors = (*memory_map).into();
    physical_page::Manager::init(physical_page_present_bit_map, &memory_map);
    serial_println!("cr0 = {:#x?}", cr0);
    serial_println!("cr2 = {:#x?}", cr2);
    serial_println!("cr3 = {:#x?}", cr3);
    serial_println!("cr4 = {:#x?}", cr4);
    serial_println!("ia32_efer = {:#x?}", ia32_efer);
    serial_println!("graphics_output = {:#x?}", graphics_output);
    graphics_output.test();
    loop {
        asm::hlt();
    }
}

pub struct KernelArguments<'a> {
    image: handle::Handle<'static>,
    system: &'a mut system::System<'a>,
    physical_page_present_bit_map: &'a mut [u8],
    memory_map: &'a memory_allocation::Map<'a>,
    stack_floor: &'a void::Void,
    cr0: &'a control::register0::Cr0,
    cr2: &'a control::register2::Cr2,
    cr3: &'a control::register3::Cr3,
    cr4: &'a control::register4::Cr4,
    ia32_efer: &'a Option<ia32_efer::Ia32Efer>,
    serial: &'a serial::Serial,
    graphics_output: &'a graphics_output::GraphicsOutput<'a>,
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

