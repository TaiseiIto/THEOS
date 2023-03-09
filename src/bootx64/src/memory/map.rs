use {
    core::fmt,
    super::super::uefi::{
        services::boot::memory_allocation,
        tables::system,
    },
};

#[derive(Clone)]
pub struct Buffer<'a> {
    buffer: &'a [u8],
    key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

impl<'a> Buffer<'a> {
    pub fn new(buffer: &'a mut [u8], system: &system::System<'_>) -> Self {
        let mut buffer_size: usize = buffer.len();
        let buffer_address: &mut u8 = &mut buffer[0];
        let mut key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;
        system.boot_services.get_memory_map(&mut buffer_size, buffer_address, &mut key, &mut descriptor_size, &mut descriptor_version);
        let buffer: &[u8] = &buffer[..buffer_size];
        Self {
            buffer,
            key,
            descriptor_size,
            descriptor_version,
        }
    }
}

impl fmt::Debug for Buffer<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let memory_descriptors: MemoryDescriptors = self.into();
        formatter
            .debug_struct("Buffer")
            .field("key", &self.key)
            .field("descriptor_size", &self.descriptor_size)
            .field("descriptor_version", &self.descriptor_version)
            .field("descriptors", &memory_descriptors)
            .finish()
    }
}

impl<'a> Into<MemoryDescriptors<'a>> for &Buffer<'a> {
    fn into(self) -> MemoryDescriptors<'a> {
        let buffer: &[u8] = self.buffer;
        let descriptor_size: usize = self.descriptor_size;
        MemoryDescriptors {
            buffer,
            descriptor_size,
        }
    }
}

#[derive(Clone)]
pub struct MemoryDescriptors<'a> {
    buffer: &'a [u8],
    descriptor_size: usize,
}

impl fmt::Debug for MemoryDescriptors<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.clone())
            .finish()
    }
}

impl Iterator for MemoryDescriptors<'_> {
    type Item = memory_allocation::MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer.len() {
            0 => None,
            _ => {
                let mut memory_descriptor: [u8; memory_allocation::MEMORY_DESCRIPTOR_SIZE] = [0; memory_allocation::MEMORY_DESCRIPTOR_SIZE];
                for (i, byte) in self.buffer[..memory_allocation::MEMORY_DESCRIPTOR_SIZE].iter().enumerate() {
                    memory_descriptor[i] = *byte;
                }
                let memory_descriptor: memory_allocation::MemoryDescriptor = memory_descriptor.into();
                self.buffer = &self.buffer[self.descriptor_size..];
                Some(memory_descriptor)
            },
        }
    }
}

