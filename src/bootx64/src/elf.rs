// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod header;
pub mod program;
pub mod section;

use {
    alloc::{
        collections::{
            btree_map::BTreeMap,
            btree_set::BTreeSet,
        },
        vec::Vec,
    },
    crate::{
        serial_print,
        serial_println,
    },
    super::{
        asm::{
            control,
            msr::architectural::ia32_efer,
        },
        memory,
        serial,
        uefi::{
            protocols::console_support::graphics_output,
            services::boot::memory_allocation,
            tables::system,
            types::{
                handle,
                void,
            },
        },
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Elf<'a> {
    header: header::Header,
    programs: Vec<program::Program>,
    sections: Vec<section::Section>,
    deployed: BTreeMap<memory::PageRange, memory::Pages<'a>>,
}

impl Elf<'_> {
    pub fn new(elf: &[u8]) -> Self {
        let header: header::Header = elf.into();
        let programs = program::Program::read(&header, elf);
        let sections = section::Section::read(&header, elf);
        let deployed: BTreeMap<memory::PageRange, memory::Pages> = programs
            .iter()
            .map(|program| program.necessary_page_numbers())
            .fold(
                BTreeSet::<usize>::new(),
                |page_numbers, next_page_numbers|
                    page_numbers
                        .union(&next_page_numbers)
                        .cloned()
                        .collect()
            )
            .into_iter()
            .fold(
                BTreeSet::<memory::PageRange>::new(),
                |mut page_ranges, page_number| {
                    let mut page_range_start: usize = page_number;
                    let mut page_range_end: usize = page_number + 1;
                    if let Some(page_range) = page_ranges
                        .clone()
                        .iter()
                        .find(|page_range| page_range_end == page_range.start()) {
                        page_range_end = page_range.end();
                        page_ranges.remove(page_range);
                    }
                    if let Some(page_range) = page_ranges
                        .clone()
                        .iter()
                        .find(|page_range| page_range.end() == page_range_start) {
                        page_range_start = page_range.start();
                        page_ranges.remove(page_range);
                    }
                    let page_range = memory::PageRange::new(page_range_start..page_range_end);
                    page_ranges.insert(page_range);
                    page_ranges
                }
            )
            .into_iter()
            .map(|page_range| {
                let mut pages = memory::Pages::new(page_range.size());
                let page_range: memory::PageRange = page_range.clone();
                programs
                    .iter()
                    .for_each(|program| {
                        let start_page: usize = program.start_page();
                        let start_offset: usize = program.start_offset();
                        if page_range.contains(start_page) {
                            pages.write(start_page - page_range.start(), start_offset, program.bytes());
                        }
                    });
                (page_range, pages)
            })
            .collect();
        Self {
            header,
            programs,
            sections,
            deployed,
        }
    }

    pub fn page_map(&self) -> BTreeMap<usize, usize> {
        self.deployed
            .iter()
            .map(|(page_range, pages)| page_range
                .clone()
                .enumerate()
                .map(|(i, page)| (pages.physical_address() as usize + i * memory_allocation::PAGE_SIZE, page * memory_allocation::PAGE_SIZE))
            )
            .flatten()
            .collect()
    }

    pub fn run(&self, kernel_arguments: KernelArguments) {
        serial_println!("Elf.run()");
        self.header.run(kernel_arguments)
    }
}

#[allow(dead_code)]
pub struct KernelArguments<'a> {
    image: handle::Handle<'static>,
    system: &'a system::System<'a>,
    memory_size: usize,
    highest_parallel_offset: usize,
    physical_page_present_bit_map: &'a [u8],
    memory_map: &'a memory_allocation::PassedMap<'a>,
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

impl<'a> KernelArguments<'a> {
    pub fn new(
        image: handle::Handle<'static>,
        system: &'a system::System<'a>,
        memory_size: usize,
        highest_parallel_offset: usize,
        physical_page_present_bit_map: &'a [u8],
        memory_map: &'a memory_allocation::PassedMap,
        stack_floor: &'a void::Void,
        cr0: &'a control::register0::Cr0,
        cr2: &'a control::register2::Cr2,
        cr3: &'a control::register3::Cr3,
        cr4: &'a control::register4::Cr4,
        ia32_efer: &'a Option<ia32_efer::Ia32Efer>,
        com1: &'a serial::Serial,
        com2: &'a serial::Serial,
        graphics_output: &'a graphics_output::GraphicsOutput<'a>,
    ) -> Self {
        Self {
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
        }
    }

    pub fn move_to_higher_half(self, highest_parallel_offset: usize) -> Self {
        let Self {
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
        } = self;
        serial_println!("Move kernel arguments to higher half");
        serial_println!("highest parallel offset = {:#x?}", highest_parallel_offset);
        Self {
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
        }
    }

    pub fn higher_address(self) -> usize {
        let lower_address: &Self = &self;
        let lower_address: *const Self = lower_address as *const Self;
        let lower_address: usize = lower_address as usize;
        self.highest_parallel_offset + lower_address
    }
}

