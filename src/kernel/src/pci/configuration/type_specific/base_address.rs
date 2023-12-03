use core::mem;

// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.2.1 Base Address Registers
#[derive(Debug)]
pub enum Register {
    Memory {
        memory_type: u8,
        prefetchable: bool,
        base_address: u32,
    },
    IO {
        base_address: u32,
    },
}

impl Register {
    const MEMORY_IO_SHIFT: usize = 0;
    const MEMORY_TYPE_SHIFT_BEGIN: usize = Self::MEMORY_IO_SHIFT + 1;
    const MEMORY_TYPE_SHIFT_LENGTH: usize = 2;
    const MEMORY_TYPE_SHIFT_END: usize = Self::MEMORY_TYPE_SHIFT_BEGIN + Self::MEMORY_TYPE_SHIFT_LENGTH;
    const PREFETCHABLE_SHIFT: usize = Self::MEMORY_TYPE_SHIFT_END;
    const MEMORY_BASE_ADDRESS_SHIFT_BEGIN: usize = Self::PREFETCHABLE_SHIFT + 1;
    const MEMORY_BASE_ADDRESS_SHIFT_END: usize = 8 * mem::size_of::<u32>();
    const MEMORY_BASE_ADDRESS_SHIFT_LENGTH: usize =  Self::MEMORY_BASE_ADDRESS_SHIFT_END - Self::MEMORY_BASE_ADDRESS_SHIFT_BEGIN;
    const IO_BASE_ADDRESS_SHIFT_BEGIN: usize = 2;
    const IO_BASE_ADDRESS_SHIFT_END: usize = 8 * mem::size_of::<u32>();
    const IO_BASE_ADDRESS_SHIFT_LENGTH: usize =  Self::IO_BASE_ADDRESS_SHIFT_END - Self::IO_BASE_ADDRESS_SHIFT_BEGIN;

    const MEMORY_IO_MASK: u32 = 1 << Self::MEMORY_IO_SHIFT;
    const MEMORY_TYPE_MASK: u32 = ((1 << Self::MEMORY_TYPE_SHIFT_LENGTH) - 1) << Self::MEMORY_TYPE_SHIFT_BEGIN;
    const PREFETCHABLE_MASK: u32 = 1 << Self::PREFETCHABLE_SHIFT;
    const MEMORY_BASE_ADDRESS_MASK: u32 = ((1 << Self::MEMORY_BASE_ADDRESS_SHIFT_LENGTH) - 1) << Self::MEMORY_BASE_ADDRESS_SHIFT_BEGIN;
    const IO_BASE_ADDRESS_MASK: u32 = ((1 << Self::IO_BASE_ADDRESS_SHIFT_LENGTH) - 1) << Self::IO_BASE_ADDRESS_SHIFT_LENGTH;
}

impl From<u32> for Register {
    fn from(register: u32) -> Self {
        if register & Self::MEMORY_IO_MASK == 0 {
            let memory_type: u8 = ((register & Self::MEMORY_TYPE_MASK) >> Self::MEMORY_TYPE_SHIFT_BEGIN) as u8;
            let prefetchable: bool = register & Self::PREFETCHABLE_MASK != 0;
            let base_address: u32 = register & Self::MEMORY_BASE_ADDRESS_MASK;
            Self::Memory {
                memory_type,
                prefetchable,
                base_address,
            }
        } else {
            let base_address: u32 = register & Self::IO_BASE_ADDRESS_MASK;
            Self::IO {
                base_address,
            }
        }
    }
}
