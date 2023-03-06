// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.2 Memory Allocation Services

#[derive(Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
    memory_type: u32,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

type PhysicalAddress = u64;
type VirtualAddress = u64;

