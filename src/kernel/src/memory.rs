use super::uefi::services::boot::memory_allocation;

pub struct PhysicalPagePresentBitMap<'a>(&'a mut [u8]);

impl<'a> PhysicalPagePresentBitMap<'a> {
    pub fn new(
        physical_page_present_bit_map: &'a mut [u8],
        map: &memory_allocation::MemoryDescriptors,
    ) -> Self {
        map
            .clone()
            .for_each(|descriptor| {
                let physical_start: usize = descriptor.physical_start() as usize;
                let physical_page_start: usize = physical_start / memory_allocation::PAGE_SIZE;
                let physical_end: usize = descriptor.physical_end() as usize;
                let physical_page_end: usize = physical_end / memory_allocation::PAGE_SIZE;
                let memory_type: memory_allocation::MemoryType = descriptor.memory_type();
                (physical_page_start..physical_page_end)
                    .for_each(|page| {
                        let bit_map_index: usize = page / 8;
                        let bit_map_offset: usize = page % 8;
                        let byte: &mut u8 = &mut physical_page_present_bit_map[bit_map_index];
                        let mask: u8 = 0x01u8 << bit_map_offset;
                        match memory_type {
                            memory_allocation::MemoryType::BootServicesCode
                            | memory_allocation::MemoryType::BootServicesData
                            | memory_allocation::MemoryType::ConventionalMemory => *byte &= !mask,
                            _ => *byte |= mask,
                        }
                    });
            });
        Self(physical_page_present_bit_map)
    }
}
