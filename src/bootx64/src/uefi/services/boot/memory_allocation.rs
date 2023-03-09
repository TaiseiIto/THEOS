// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.2 Memory Allocation Services

use {
    core::{
        fmt,
        mem,
    },
    super::super::super::types::{
        status,
        void,
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

