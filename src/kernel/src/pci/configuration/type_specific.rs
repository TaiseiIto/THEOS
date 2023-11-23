extern crate alloc;

mod base_address;

use {
    alloc::vec::Vec,
    core::mem,
    super::{
        CONFIGURATION_SIZE,
        header_type,
    },
};

const NUM_BASE_ADDRESS_REGISTERS: usize = 6;

// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.2 Type 0 Configuration Space Header
// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.3 Type 1 Configuration Space Header
#[derive(Debug)]
pub enum Registers {
    Type0 {
        base_address_registers: [base_address::Register; NUM_BASE_ADDRESS_REGISTERS],
    },
    Type1,
    Reserved,
}

impl Registers {
    const BASE_ADDRESS_REGISTERS_SHIFT_BEGIN: usize = 0x10;
    const BASE_ADDRESS_REGISTERS_SHIFT_END: usize = 0x28;

    pub fn new(header_layout: &header_type::HeaderLayout, configuration: &[u8; CONFIGURATION_SIZE]) -> Self {
        match header_layout {
            header_type::HeaderLayout::Type0 => {
                let base_address_registers: [base_address::Register; NUM_BASE_ADDRESS_REGISTERS] = configuration[Self::BASE_ADDRESS_REGISTERS_SHIFT_BEGIN..Self::BASE_ADDRESS_REGISTERS_SHIFT_END]
                    .chunks_exact(mem::size_of::<u32>())
                    .map(|chunk| {
                        let chunk: [u8; mem::size_of::<u32>()] = chunk
                            .try_into()
                            .expect("Can't get a PCI device base address register!");
                        u32::from_le_bytes(chunk).into()
                    })
                    .collect::<Vec<base_address::Register>>()
                    .try_into()
                    .expect("Can't get a PCI device base address registers!");
                Self::Type0 {
                    base_address_registers,
                }
            },
            header_type::HeaderLayout::Type1 => Self::Type1,
            header_type::HeaderLayout::Reserved => Self::Reserved,
        }
    }
}

