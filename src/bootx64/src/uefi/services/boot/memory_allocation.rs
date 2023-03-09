// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.2 Memory Allocation Services

use {
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

#[allow(dead_code)]
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

pub type PhysicalAddress = u64;

#[derive(WrappedFunction)]
#[repr(C)]
pub struct FreePages(pub extern "efiapi" fn(PhysicalAddress, usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct GetMemoryMap(pub extern "efiapi" fn(&mut usize, &mut u8, &mut usize, &mut usize, &mut u32) -> status::Status);

#[derive(Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
    memory_type: u32,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

pub type VirtualAddress = u64;

#[derive(WrappedFunction)]
#[repr(C)]
pub struct AllocatePool(pub extern "efiapi" fn(MemoryType, usize, &mut &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct FreePool(pub extern "efiapi" fn(&void::Void) -> status::Status);

