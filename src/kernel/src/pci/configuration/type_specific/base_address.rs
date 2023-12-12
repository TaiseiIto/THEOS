extern crate alloc;

use {
    alloc::collections::btree_set::BTreeSet,
    core::mem,
};

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

impl Into<u32> for &Register {
    fn into(self) -> u32 {
        match self {
            Register::Memory {
                memory_type,
                prefetchable,
                base_address,
            } => (*memory_type as u32) << Register::MEMORY_TYPE_SHIFT_BEGIN
                | if *prefetchable {
                    Register::PREFETCHABLE_MASK
                } else {
                    0
                } | base_address,
            Register::IO {
                base_address,
            } => base_address | Register::MEMORY_IO_MASK,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Address {
    Memory32(u32),
    Memory64(u64),
    IO(u16),
}

impl Address {
    pub fn get<'a, I>(register_iterator: I) -> BTreeSet<Self> where I: Iterator<Item = &'a Register> {
        register_iterator
            .fold((BTreeSet::<Self>::new(), None::<&Register>), |(mut addresses, last_address), next_address| match last_address {
                Some(last_address) => match next_address {
                    next_address @ Register::Memory {
                        memory_type: _,
                        prefetchable: _,
                        base_address: _,
                    } => {
                        let lower_address: u32 = last_address.into();
                        let lower_address: u32 = lower_address & Register::MEMORY_BASE_ADDRESS_MASK;
                        let lower_address: u64 = lower_address as u64;
                        let higher_address: u32 = next_address.into();
                        let higher_address: u64 = higher_address as u64;
                        let higher_address: u64 = higher_address << 32;
                        let address: u64 = lower_address | higher_address;
                        let address = Self::Memory64(address);
                        addresses.insert(address);
                        (addresses, None)
                    },
                    Register::IO {
                        base_address: _,
                    } => panic!("Can't get a PCI device memory mapped address!"),
                },
                None => match next_address {
                    next_address @ Register::Memory {
                        memory_type,
                        prefetchable: _,
                        base_address: _,
                    } => match memory_type {
                        0 => { // Memory type 0 indicates that this is 32 bit address.
                            let address: u32 = next_address.into();
                            let address = Self::Memory32(address);
                            addresses.insert(address);
                            (addresses, None)
                        },
                        2 => (addresses, Some(next_address)), // Memory type 2 indicates that this is 64 bit address.
                        _ => (addresses, None),
                    },
                    next_address @ Register::IO {
                        base_address: _,
                    } => {
                        let address: u32 = next_address.into();
                        let address: u32 = address & Register::IO_BASE_ADDRESS_MASK;
                        let address: u16 = address as u16;
                        let address = Self::IO(address);
                        addresses.insert(address);
                        (addresses, None)
                    },
                },
            })
            .0
    }

    pub fn is_not_null(&self) -> bool {
        let address: usize = match self {
            Self::Memory32(address) => *address as usize,
            Self::Memory64(address) => *address as usize,
            Self::IO(address) => *address as usize,
        };
        address != 0
    }
}

