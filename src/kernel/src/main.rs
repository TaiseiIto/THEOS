#![no_std]
#![no_main]

mod allocator;
mod asm;
mod display;
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
        memory_size,
        highest_parallel_offset,
        physical_page_present_bit_map,
        memory_map,
        stack_floor,
        cr0,
        cr2,
        cr3,
        cr4,
        ia32_efer,
        com1,
        com2,
        graphics_output,
    } = kernel_arguments;
    serial::Serial::init_com1(com1);
    serial::Serial::init_com2(com2);
    serial_println!("Hello, kernel.elf!");
    serial_println!("stack_floor = {:#x?}", stack_floor);
    serial_println!("RSP = {:#x?}", asm::get_rsp());
    system::init_system(image, *system);
    let image: handle::Handle = system::image();
    let image: *const void::Void = image as *const void::Void;
    let image: usize = image as usize;
    serial_println!("image = {:#x?}", image);
    serial_println!("system = {:#x?}", system::system());
    serial_println!("memory_size = {:#x?}", memory_size);
    serial_println!("highest_parallel_offset = {:#x?}", highest_parallel_offset);
    let physical_page_present_bit_map: &'static mut [u8] = *physical_page_present_bit_map;
    serial_println!("physical_page_present_bit_map");
    let memory_map: memory_allocation::MemoryDescriptors = (*memory_map).into();
    serial_println!("memory_map");
    physical_page::Manager::init(physical_page_present_bit_map, &memory_map);
    serial_println!("cr0 = {:#x?}", cr0);
    serial_println!("cr2 = {:#x?}", cr2);
    serial_println!("cr3 = {:#x?}", cr3);
    serial_println!("cr4 = {:#x?}", cr4);
    serial_println!("ia32_efer = {:#x?}", ia32_efer);
    serial_println!("graphics_output = {:#x?}", graphics_output);
    let display: display::Display = display::Display::new(graphics_output);
    for red in 0x00u8..0xffu8 {
        for green in 0x00u8..0xffu8 {
            let blue = 0x00u8;
            let x = red as u32;
            let y = green as u32;
            let coordinates = display::Coordinates::new(x, y);
            let color = display::Color::new(red, green, blue);
            display.write_pixel(&coordinates, &color);
        }
    }
    loop {
        asm::hlt();
    }
}

pub struct KernelArguments<'a> {
    image: handle::Handle<'static>,
    system: &'a mut system::System<'a>,
    memory_size: usize,
    highest_parallel_offset: usize,
    physical_page_present_bit_map: &'a mut [u8],
    memory_map: &'a memory_allocation::Map<'a>,
    stack_floor: &'a void::Void,
    cr0: &'a control::register0::Cr0,
    cr2: &'a control::register2::Cr2,
    cr3: &'a control::register3::Cr3,
    cr4: &'a control::register4::Cr4,
    ia32_efer: &'a Option<ia32_efer::Ia32Efer>,
    com1: &'a serial::Serial,
    com2: &'a serial::Serial,
    graphics_output: &'a graphics_output::GraphicsOutput<'a>,
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    serial_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

