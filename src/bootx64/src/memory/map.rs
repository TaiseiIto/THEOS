use super::super::uefi::{
    services::boot::memory_allocation,
    tables::system,
};

#[derive(Debug)]
pub struct Buffer<'a> {
    buffer: &'a u8,
    buffer_size: usize,
    key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

impl<'a> Buffer<'a> {
    pub fn new(buffer: &'a mut u8, mut buffer_size: usize, system: &system::System<'_>) -> Self {
        let mut key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;
        system.boot_services.get_memory_map(&mut buffer_size, buffer, &mut key, &mut descriptor_size, &mut descriptor_version);
        Self {
            buffer,
            buffer_size,
            key,
            descriptor_size,
            descriptor_version,
        }
    }
}

