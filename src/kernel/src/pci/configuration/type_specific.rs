extern crate alloc;

mod base_address;
mod expansion_rom_base_address;

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
        cardbus_cis_pointer: u32,
        subsystem_vendor_id: u16,
        subsystem_id: u16,
        expansion_rom_base_address: expansion_rom_base_address::Register,
    },
    Type1,
    Reserved,
}

impl Registers {
    const BASE_ADDRESS_REGISTERS_BEGIN: usize = 0x10;
    const BASE_ADDRESS_REGISTERS_END: usize = 0x28;
    const CARDBUS_CIS_POINTER_BEGIN: usize = Self::BASE_ADDRESS_REGISTERS_END;
    const CARDBUS_CIS_POINTER_LENGTH: usize = mem::size_of::<u32>();
    const CARDBUS_CIS_POINTER_END: usize = Self::CARDBUS_CIS_POINTER_BEGIN + Self::CARDBUS_CIS_POINTER_LENGTH;
    const SUBSYSTEM_VENDOR_ID_BEGIN: usize = Self::CARDBUS_CIS_POINTER_END;
    const SUBSYSTEM_VENDOR_ID_LENGTH: usize = mem::size_of::<u16>();
    const SUBSYSTEM_VENDOR_ID_END: usize = Self::SUBSYSTEM_VENDOR_ID_BEGIN + Self::SUBSYSTEM_VENDOR_ID_LENGTH;
    const SUBSYSTEM_ID_BEGIN: usize = Self::SUBSYSTEM_VENDOR_ID_END;
    const SUBSYSTEM_ID_LENGTH: usize = mem::size_of::<u16>();
    const SUBSYSTEM_ID_END: usize = Self::SUBSYSTEM_ID_BEGIN + Self::SUBSYSTEM_ID_LENGTH;
    const EXPANSION_ROM_BASE_ADDRESS_BEGIN: usize = Self::SUBSYSTEM_ID_END;
    const EXPANSION_ROM_BASE_ADDRESS_LENGTH: usize = mem::size_of::<u32>();
    const EXPANSION_ROM_BASE_ADDRESS_END: usize = Self::EXPANSION_ROM_BASE_ADDRESS_BEGIN + Self::EXPANSION_ROM_BASE_ADDRESS_LENGTH;

    pub fn new(header_layout: &header_type::HeaderLayout, configuration: &[u8; CONFIGURATION_SIZE]) -> Self {
        match header_layout {
            header_type::HeaderLayout::Type0 => {
                let base_address_registers: [base_address::Register; NUM_BASE_ADDRESS_REGISTERS] = configuration[Self::BASE_ADDRESS_REGISTERS_BEGIN..Self::BASE_ADDRESS_REGISTERS_END]
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
                let cardbus_cis_pointer: [u8; Self::CARDBUS_CIS_POINTER_LENGTH] = configuration[Self::CARDBUS_CIS_POINTER_BEGIN..Self::CARDBUS_CIS_POINTER_END]
                    .try_into()
                    .expect("Can't get a PCI device cardbus cis pointer!");
                let cardbus_cis_pointer: u32 = u32::from_le_bytes(cardbus_cis_pointer);
                let subsystem_vendor_id: [u8; Self::SUBSYSTEM_VENDOR_ID_LENGTH] = configuration[Self::SUBSYSTEM_VENDOR_ID_BEGIN..Self::SUBSYSTEM_VENDOR_ID_END]
                    .try_into()
                    .expect("Can't get a PCI device subsystem vendor ID!");
                let subsystem_vendor_id: u16 = u16::from_le_bytes(subsystem_vendor_id);
                let subsystem_id: [u8; Self::SUBSYSTEM_ID_LENGTH] = configuration[Self::SUBSYSTEM_ID_BEGIN..Self::SUBSYSTEM_ID_END]
                    .try_into()
                    .expect("Can't get a PCI device subsystem ID!");
                let subsystem_id: u16 = u16::from_le_bytes(subsystem_id);
                let expansion_rom_base_address: [u8; Self::EXPANSION_ROM_BASE_ADDRESS_LENGTH] = configuration[Self::EXPANSION_ROM_BASE_ADDRESS_BEGIN..Self::EXPANSION_ROM_BASE_ADDRESS_END]
                    .try_into()
                    .expect("Can't get a expansion ROM base address!");
                let expansion_rom_base_address: expansion_rom_base_address::Register = u32::from_le_bytes(expansion_rom_base_address).into();
                Self::Type0 {
                    base_address_registers,
                    cardbus_cis_pointer,
                    subsystem_vendor_id,
                    subsystem_id,
                    expansion_rom_base_address,
                }
            },
            header_type::HeaderLayout::Type1 => Self::Type1,
            header_type::HeaderLayout::Reserved => Self::Reserved,
        }
    }
}

