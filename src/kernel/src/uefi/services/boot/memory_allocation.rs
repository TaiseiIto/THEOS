// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.2 Memory Allocation Services

extern crate alloc;

use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
    },
    super::super::super::{
        super::allocator,
        tables::system,
        types::{
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(WrappedFunction)]
#[repr(C)]
pub struct AllocatePages(pub extern "efiapi" fn(AllocateType, MemoryType, usize, &mut PhysicalAddress) -> status::Status);

#[allow(dead_code)]
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[derive(Debug)]
#[repr(C)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
    MaxMemoryType,
}

impl From<u8> for MemoryType {
    fn from(memory_type: u8) -> MemoryType {
        match memory_type {
            0x0 => Self::ReservedMemoryType,
            0x1 => Self::LoaderCode,
            0x2 => Self::LoaderData,
            0x3 => Self::BootServicesCode,
            0x4 => Self::BootServicesData,
            0x5 => Self::RuntimeServicesCode,
            0x6 => Self::RuntimeServicesData,
            0x7 => Self::ConventionalMemory,
            0x8 => Self::UnusableMemory,
            0x9 => Self::ACPIReclaimMemory,
            0xa => Self::ACPIMemoryNVS,
            0xb => Self::MemoryMappedIO,
            0xc => Self::MemoryMappedIOPortSpace,
            0xd => Self::PalCode,
            0xe => Self::PersistentMemory,
            0xf => Self::UnacceptedMemoryType,
            0x10 => Self::MaxMemoryType,
            _ => panic!("Unknown memory type!"),
        }
    }
}

impl From<u32> for MemoryType {
    fn from(memory_type: u32) -> MemoryType {
        (memory_type as u8).into()
    }
}

pub type PhysicalAddress = u64;

#[derive(WrappedFunction)]
#[repr(C)]
pub struct FreePages(pub extern "efiapi" fn(PhysicalAddress, usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct GetMemoryMap(pub extern "efiapi" fn(&mut usize, &mut u8, &mut usize, &mut usize, &mut u32) -> status::Status);

#[repr(C)]
pub struct MemoryDescriptor {
    memory_type: u32,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

pub const PAGE_SIZE: usize = 0x1000;

impl MemoryDescriptor {
    pub fn physical_start(&self) -> PhysicalAddress {
        self.physical_start
    }

    pub fn physical_end(&self) -> PhysicalAddress {
        self.physical_start + (self.number_of_pages as PhysicalAddress) * (PAGE_SIZE as PhysicalAddress)
    }

    pub fn memory_type(&self) -> MemoryType {
        self.memory_type.into()
    }
}

impl fmt::Debug for MemoryDescriptor {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let memory_type: MemoryType = self.memory_type.into();
        let attribute: MemoryAttribute = self.attribute.into();
        formatter
            .debug_struct("MemoryDescriptor")
            .field("memory_type", &memory_type)
            .field("physical_start", &self.physical_start)
            .field("virtual_start", &self.virtual_start)
            .field("number_of_pages", &self.number_of_pages)
            .field("attribute", &attribute)
            .finish()
    }
}

impl From<[u8; MEMORY_DESCRIPTOR_SIZE]> for MemoryDescriptor {
    fn from(bytes: [u8; MEMORY_DESCRIPTOR_SIZE]) -> Self {
        unsafe {
            mem::transmute::<[u8; MEMORY_DESCRIPTOR_SIZE], Self>(bytes)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MemoryAttribute {
    uc: bool,
    wc: bool,
    wt: bool,
    wb: bool,
    uce: bool,
    wp: bool,
    rp: bool,
    xp: bool,
    nv: bool,
    more_reliable: bool,
    ro: bool,
    sp: bool,
    cpu_crypto: bool,
    runtime: bool,
}

impl From<u64> for MemoryAttribute {
    fn from(memory_attribute: u64) -> Self {
        let uc: bool = memory_attribute & 0x0000000000000001 != 0;
        let wc: bool = memory_attribute & 0x0000000000000002 != 0;
        let wt: bool = memory_attribute & 0x0000000000000004 != 0;
        let wb: bool = memory_attribute & 0x0000000000000008 != 0;
        let uce: bool = memory_attribute & 0x0000000000000010 != 0;
        let wp: bool = memory_attribute & 0x0000000000001000 != 0;
        let rp: bool = memory_attribute & 0x0000000000002000 != 0;
        let xp: bool = memory_attribute & 0x0000000000004000 != 0;
        let nv: bool = memory_attribute & 0x0000000000008000 != 0;
        let more_reliable: bool = memory_attribute & 0x0000000000010000 != 0;
        let ro: bool = memory_attribute & 0x0000000000020000 != 0;
        let sp: bool = memory_attribute & 0x0000000000040000 != 0;
        let cpu_crypto: bool = memory_attribute & 0x0000000000080000 != 0;
        let runtime: bool = memory_attribute & 0x8000000000000000 != 0;
        Self {
            uc,
            wc,
            wt,
            wb,
            uce,
            wp,
            rp,
            xp,
            nv,
            more_reliable,
            ro,
            sp,
            cpu_crypto,
            runtime,
        }
    }
}

pub const MEMORY_DESCRIPTOR_SIZE: usize = mem::size_of::<MemoryDescriptor>();

pub type VirtualAddress = u64;

#[derive(WrappedFunction)]
#[repr(C)]
pub struct AllocatePool(pub extern "efiapi" fn(MemoryType, usize, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct FreePool(pub extern "efiapi" fn(&void::Void) -> status::Status);

pub struct Map<'a> {
    buffer: allocator::Allocated<'a>,
    key: usize,
    descriptors: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

impl<'a> Map<'a> {
    pub fn new() -> Self {
        let (mut buffer_size, mut descriptor_size): (usize, usize) = Self::get_map_buffer_size();
        buffer_size *= 2;
        let mut buffer = allocator::Allocated::new(buffer_size, descriptor_size);
        let mut key: usize = 0;
        let mut descriptor_version: u32 = 0;
        let buffer_slice: &mut [u8] = buffer.get_mut();
        let buffer_address: &mut u8 = &mut buffer_slice[0];
        system::system()
            .boot_services
            .get_memory_map(
                &mut buffer_size,
                buffer_address,
                &mut key,
                &mut descriptor_size,
                &mut descriptor_version
            )
            .expect("Can't get memory map!");
        let descriptors: usize = buffer_size / descriptor_size;
        Self {
            buffer,
            key,
            descriptors,
            descriptor_size,
            descriptor_version,
        }
    }

    pub fn key(&self) -> usize {
        self.key
    }

    pub fn get_memory_size(&self) -> PhysicalAddress {
        self
            .iter()
            .map(|memory_descriptor| memory_descriptor.physical_end())
            .max()
            .expect("Can't get memory size!")
    }

    fn get_map_buffer_size() -> (usize, usize) {
        let mut size: usize = 0;
        let mut buffer: u8 = 0;
        let mut map_key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;
        match system::system()
            .boot_services
            .get_memory_map(
                &mut size,
                &mut buffer,
                &mut map_key,
                &mut descriptor_size,
                &mut descriptor_version,
            ) {
            _ => (),
        }
        (size, descriptor_size)
    }

    fn iter(&self) -> MemoryDescriptors {
        self.into()
    }
}

impl fmt::Debug for Map<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("Map")
            .field("key", &self.key)
            .field("descriptor_size", &self.descriptor_size)
            .field("descriptor_version", &self.descriptor_version)
            .field("descriptors", &self.iter())
            .finish()
    }
}

impl<'a> Into<MemoryDescriptors<'a>> for &'a Map<'a> {
    fn into(self) -> MemoryDescriptors<'a> {
        let buffer: &[u8] = self.buffer.get_ref();
        let descriptors: usize = self.descriptors;
        let descriptor_size: usize = self.descriptor_size;
        MemoryDescriptors {
            buffer,
            descriptors,
            descriptor_size,
        }
    }
}

impl Into<Vec<MemoryDescriptor>> for &Map<'_> {
    fn into(self) -> Vec<MemoryDescriptor> {
        let memory_descriptors: MemoryDescriptors = self.into();
        memory_descriptors.collect()
    }
}

#[allow(dead_code)]
pub struct PassedMap<'a> {
    buffer: &'a [u8],
    key: usize,
    descriptors: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

impl<'a> From<&'a Map<'a>> for PassedMap<'a> {
    fn from(map: &'a Map<'a>) -> Self {
        let Map {
            buffer,
            key,
            descriptors,
            descriptor_size,
            descriptor_version,
        } = map;
        let buffer: &[u8] = buffer.get_ref();
        let key: usize = *key;
        let descriptors: usize = *descriptors;
        let descriptor_size: usize = *descriptor_size;
        let descriptor_version: u32 = *descriptor_version;
        Self {
            buffer,
            key,
            descriptors,
            descriptor_size,
            descriptor_version,
        }
    }
}

#[derive(Clone)]
pub struct MemoryDescriptors<'a> {
    buffer: &'a [u8],
    descriptors: usize,
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
    type Item = MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        match self.descriptors {
            0 => None,
            _ => {
                let memory_descriptor: [u8; MEMORY_DESCRIPTOR_SIZE] = self
                    .buffer[..MEMORY_DESCRIPTOR_SIZE]
                    .try_into()
                    .expect("Can't get a memory descriptor!");
                let memory_descriptor: MemoryDescriptor = memory_descriptor.into();
                self.buffer = &self.buffer[self.descriptor_size..];
                self.descriptors -= 1;
                Some(memory_descriptor)
            },
        }
    }
}

