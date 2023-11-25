extern crate alloc;

mod base_address;
mod bridge_control;
mod expansion_rom_base_address;
mod secondary_status;

use {
    alloc::vec::Vec,
    core::mem,
    super::{
        CONFIGURATION_SIZE,
        Device,
        header_type,
    },
};

const TYPE0_NUM_BASE_ADDRESS_REGISTERS: usize = 6;
const TYPE1_NUM_BASE_ADDRESS_REGISTERS: usize = 2;

// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.2 Type 0 Configuration Space Header
// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.3 Type 1 Configuration Space Header
#[derive(Debug)]
pub enum Registers {
    Type0 {
        base_address_registers: [base_address::Register; TYPE0_NUM_BASE_ADDRESS_REGISTERS],
        cardbus_cis_pointer: u32,
        subsystem_vendor_id: u16,
        subsystem_id: u16,
        expansion_rom_base_address: expansion_rom_base_address::Register,
        min_gnt: u8,
        max_lat: u8,
    },
    Type1 {
        base_address_registers: [base_address::Register; TYPE1_NUM_BASE_ADDRESS_REGISTERS],
        primary_bus_number: u8,
        secondary_bus_number: u8,
        subordinate_bus_number: u8,
        secondary_latency_timer: u8,
        io_base: u8,
        io_limit: u8,
        secondary_status: secondary_status::Register,
        memory_base: u16,
        memory_limit: u16,
        prefetchable_memory_base: u16,
        prefetchable_memory_limit: u16,
        prefetchable_memory_base_upper_32bits: u32,
        prefetchable_memory_limit_upper_32bits: u32,
        io_base_upper_16bits: u16,
        io_base_limit_16bits: u16,
        expansion_rom_base_address: expansion_rom_base_address::Register,
        bridge_control: bridge_control::Register,
    },
    Reserved,
}

impl Registers {
    const BASE_ADDRESS_REGISTERS_BEGIN: usize = Device::BIST_END;
    const BASE_ADDRESS_REGISTER_LENGTH: usize = mem::size_of::<u32>();
    const TYPE0_BASE_ADDRESS_REGISTERS_LENGTH: usize = TYPE0_NUM_BASE_ADDRESS_REGISTERS * Self::BASE_ADDRESS_REGISTER_LENGTH;
    const TYPE0_BASE_ADDRESS_REGISTERS_END: usize = Self::BASE_ADDRESS_REGISTERS_BEGIN + Self::TYPE0_BASE_ADDRESS_REGISTERS_LENGTH;
    const CARDBUS_CIS_POINTER_BEGIN: usize = Self::TYPE0_BASE_ADDRESS_REGISTERS_END;
    const CARDBUS_CIS_POINTER_LENGTH: usize = mem::size_of::<u32>();
    const CARDBUS_CIS_POINTER_END: usize = Self::CARDBUS_CIS_POINTER_BEGIN + Self::CARDBUS_CIS_POINTER_LENGTH;
    const SUBSYSTEM_VENDOR_ID_BEGIN: usize = Self::CARDBUS_CIS_POINTER_END;
    const SUBSYSTEM_VENDOR_ID_LENGTH: usize = mem::size_of::<u16>();
    const SUBSYSTEM_VENDOR_ID_END: usize = Self::SUBSYSTEM_VENDOR_ID_BEGIN + Self::SUBSYSTEM_VENDOR_ID_LENGTH;
    const SUBSYSTEM_ID_BEGIN: usize = Self::SUBSYSTEM_VENDOR_ID_END;
    const SUBSYSTEM_ID_LENGTH: usize = mem::size_of::<u16>();
    const SUBSYSTEM_ID_END: usize = Self::SUBSYSTEM_ID_BEGIN + Self::SUBSYSTEM_ID_LENGTH;
    const TYPE0_EXPANSION_ROM_BASE_ADDRESS_BEGIN: usize = Self::SUBSYSTEM_ID_END;
    const EXPANSION_ROM_BASE_ADDRESS_LENGTH: usize = mem::size_of::<u32>();
    const TYPE0_EXPANSION_ROM_BASE_ADDRESS_END: usize = Self::TYPE0_EXPANSION_ROM_BASE_ADDRESS_BEGIN + Self::EXPANSION_ROM_BASE_ADDRESS_LENGTH;
    const MIN_GNT_BEGIN: usize = Device::INTERRUPT_PIN_END;
    const MIN_GNT_LENGTH: usize = mem::size_of::<u8>();
    const MIN_GNT_END: usize = Self::MIN_GNT_BEGIN + Self::MIN_GNT_LENGTH;
    const MAX_LAT_BEGIN: usize = Self::MIN_GNT_END;
    #[allow(dead_code)]
    const MAX_LAT_LENGTH: usize = mem::size_of::<u8>();
    #[allow(dead_code)]
    const MAX_LAT_END: usize = Self::MAX_LAT_BEGIN + Self::MAX_LAT_LENGTH;

    const TYPE1_BASE_ADDRESS_REGISTERS_LENGTH: usize = TYPE1_NUM_BASE_ADDRESS_REGISTERS * Self::BASE_ADDRESS_REGISTER_LENGTH;
    const TYPE1_BASE_ADDRESS_REGISTERS_END: usize = Self::BASE_ADDRESS_REGISTERS_BEGIN + Self::TYPE1_BASE_ADDRESS_REGISTERS_LENGTH;
    const PRIMARY_BUS_NUMBER_BEGIN: usize = Self::TYPE1_BASE_ADDRESS_REGISTERS_END;
    const PRIMARY_BUS_NUMBER_LENGTH: usize = mem::size_of::<u8>();
    const PRIMARY_BUS_NUMBER_END: usize = Self::PRIMARY_BUS_NUMBER_BEGIN + Self::PRIMARY_BUS_NUMBER_LENGTH;
    const SECONDARY_BUS_NUMBER_BEGIN: usize = Self::PRIMARY_BUS_NUMBER_END;
    const SECONDARY_BUS_NUMBER_LENGTH: usize = mem::size_of::<u8>();
    const SECONDARY_BUS_NUMBER_END: usize = Self::SECONDARY_BUS_NUMBER_BEGIN + Self::SECONDARY_BUS_NUMBER_LENGTH;
    const SUBORDINATE_BUS_NUMBER_BEGIN: usize = Self::SECONDARY_BUS_NUMBER_END;
    const SUBORDINATE_BUS_NUMBER_LENGTH: usize = mem::size_of::<u8>();
    const SUBORDINATE_BUS_NUMBER_END: usize = Self::SUBORDINATE_BUS_NUMBER_BEGIN + Self::SUBORDINATE_BUS_NUMBER_LENGTH;
    const SECONDARY_LATENCY_TIMER_BEGIN: usize = Self::SUBORDINATE_BUS_NUMBER_END;
    const SECONDARY_LATENCY_TIMER_LENGTH: usize = mem::size_of::<u8>();
    const SECONDARY_LATENCY_TIMER_END: usize = Self::SECONDARY_LATENCY_TIMER_BEGIN + Self::SECONDARY_LATENCY_TIMER_LENGTH;
    const IO_BASE_BEGIN: usize = Self::SECONDARY_LATENCY_TIMER_END;
    const IO_BASE_LENGTH: usize = mem::size_of::<u8>();
    const IO_BASE_END: usize = Self::IO_BASE_BEGIN + Self::IO_BASE_LENGTH;
    const IO_LIMIT_BEGIN: usize = Self::IO_BASE_END;
    const IO_LIMIT_LENGTH: usize = mem::size_of::<u8>();
    const IO_LIMIT_END: usize = Self::IO_LIMIT_BEGIN + Self::IO_LIMIT_LENGTH;
    const SECONDARY_STATUS_BEGIN: usize = Self::IO_LIMIT_END;
    const SECONDARY_STATUS_LENGTH: usize = mem::size_of::<u16>();
    const SECONDARY_STATUS_END: usize = Self::SECONDARY_STATUS_BEGIN + Self::SECONDARY_STATUS_LENGTH;
    const MEMORY_BASE_BEGIN: usize = Self::SECONDARY_STATUS_END;
    const MEMORY_BASE_LENGTH: usize = mem::size_of::<u16>();
    const MEMORY_BASE_END: usize = Self::MEMORY_BASE_BEGIN + Self::MEMORY_BASE_LENGTH;
    const MEMORY_LIMIT_BEGIN: usize = Self::MEMORY_BASE_END;
    const MEMORY_LIMIT_LENGTH: usize = mem::size_of::<u16>();
    const MEMORY_LIMIT_END: usize = Self::MEMORY_LIMIT_BEGIN + Self::MEMORY_LIMIT_LENGTH;
    const PREFETCHABLE_MEMORY_BASE_BEGIN: usize = Self::MEMORY_LIMIT_END;
    const PREFETCHABLE_MEMORY_BASE_LENGTH: usize = mem::size_of::<u16>();
    const PREFETCHABLE_MEMORY_BASE_END: usize = Self::PREFETCHABLE_MEMORY_BASE_BEGIN + Self::PREFETCHABLE_MEMORY_BASE_LENGTH;
    const PREFETCHABLE_MEMORY_LIMIT_BEGIN: usize = Self::PREFETCHABLE_MEMORY_BASE_END;
    const PREFETCHABLE_MEMORY_LIMIT_LENGTH: usize = mem::size_of::<u16>();
    const PREFETCHABLE_MEMORY_LIMIT_END: usize = Self::PREFETCHABLE_MEMORY_LIMIT_BEGIN + Self::PREFETCHABLE_MEMORY_LIMIT_LENGTH;
    const PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_BEGIN: usize = Self::PREFETCHABLE_MEMORY_LIMIT_END;
    const PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_LENGTH: usize = mem::size_of::<u32>();
    const PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_END: usize = Self::PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_BEGIN + Self::PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_LENGTH;
    const PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_BEGIN: usize = Self::PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_END;
    const PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_LENGTH: usize = mem::size_of::<u32>();
    const PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_END: usize = Self::PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_BEGIN + Self::PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_LENGTH;
    const IO_BASE_UPPER_16BITS_BEGIN: usize = Self::PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_END;
    const IO_BASE_UPPER_16BITS_LENGTH: usize = mem::size_of::<u16>();
    const IO_BASE_UPPER_16BITS_END: usize = Self::IO_BASE_UPPER_16BITS_BEGIN + Self::IO_BASE_UPPER_16BITS_LENGTH;
    const IO_BASE_LIMIT_16BITS_BEGIN: usize = Self::IO_BASE_UPPER_16BITS_END;
    const IO_BASE_LIMIT_16BITS_LENGTH: usize = mem::size_of::<u16>();
    const IO_BASE_LIMIT_16BITS_END: usize = Self::IO_BASE_LIMIT_16BITS_BEGIN + Self::IO_BASE_LIMIT_16BITS_LENGTH;
    const TYPE1_EXPANSION_ROM_BASE_ADDRESS_END: usize = Device::INTERRUPT_LINE_BEGIN;
    const TYPE1_EXPANSION_ROM_BASE_ADDRESS_BEGIN: usize = Self::TYPE1_EXPANSION_ROM_BASE_ADDRESS_END - Self::EXPANSION_ROM_BASE_ADDRESS_LENGTH;
    const BRIDGE_CONTROL_BEGIN: usize = Device::INTERRUPT_PIN_END;
    const BRIDGE_CONTROL_LENGTH: usize = mem::size_of::<u16>();
    const BRIDGE_CONTROL_END: usize = Self::BRIDGE_CONTROL_BEGIN + Self::BRIDGE_CONTROL_LENGTH;

    pub fn new(header_layout: &header_type::HeaderLayout, configuration: &[u8; CONFIGURATION_SIZE]) -> Self {
        match header_layout {
            header_type::HeaderLayout::Type0 => {
                let base_address_registers: [base_address::Register; TYPE0_NUM_BASE_ADDRESS_REGISTERS] = configuration[Self::BASE_ADDRESS_REGISTERS_BEGIN..Self::TYPE0_BASE_ADDRESS_REGISTERS_END]
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
                let expansion_rom_base_address: [u8; Self::EXPANSION_ROM_BASE_ADDRESS_LENGTH] = configuration[Self::TYPE0_EXPANSION_ROM_BASE_ADDRESS_BEGIN..Self::TYPE0_EXPANSION_ROM_BASE_ADDRESS_END]
                    .try_into()
                    .expect("Can't get a expansion ROM base address!");
                let expansion_rom_base_address: expansion_rom_base_address::Register = u32::from_le_bytes(expansion_rom_base_address).into();
                let min_gnt: u8 = configuration[Self::MIN_GNT_BEGIN];
                let max_lat: u8 = configuration[Self::MAX_LAT_BEGIN];
                Self::Type0 {
                    base_address_registers,
                    cardbus_cis_pointer,
                    subsystem_vendor_id,
                    subsystem_id,
                    expansion_rom_base_address,
                    min_gnt,
                    max_lat,
                }
            },
            header_type::HeaderLayout::Type1 => {
                let base_address_registers: [base_address::Register; TYPE1_NUM_BASE_ADDRESS_REGISTERS] = configuration[Self::BASE_ADDRESS_REGISTERS_BEGIN..Self::TYPE1_BASE_ADDRESS_REGISTERS_END]
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
                let primary_bus_number: u8 = configuration[Self::PRIMARY_BUS_NUMBER_BEGIN];
                let secondary_bus_number: u8 = configuration[Self::SECONDARY_BUS_NUMBER_BEGIN];
                let subordinate_bus_number: u8 = configuration[Self::SUBORDINATE_BUS_NUMBER_BEGIN];
                let secondary_latency_timer: u8 = configuration[Self::SECONDARY_LATENCY_TIMER_BEGIN];
                let io_base: u8 = configuration[Self::IO_BASE_BEGIN];
                let io_limit: u8 = configuration[Self::IO_LIMIT_BEGIN];
                let secondary_status: [u8; Self::SECONDARY_STATUS_LENGTH] = configuration[Self::SECONDARY_STATUS_BEGIN..Self::SECONDARY_STATUS_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let secondary_status: secondary_status::Register = u16::from_le_bytes(secondary_status).into();
                let memory_base: [u8; Self::MEMORY_BASE_LENGTH] = configuration[Self::MEMORY_BASE_BEGIN..Self::MEMORY_BASE_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let memory_base: u16 = u16::from_le_bytes(memory_base);
                let memory_limit: [u8; Self::MEMORY_LIMIT_LENGTH] = configuration[Self::MEMORY_LIMIT_BEGIN..Self::MEMORY_LIMIT_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let memory_limit: u16 = u16::from_le_bytes(memory_limit);
                let prefetchable_memory_base: [u8; Self::PREFETCHABLE_MEMORY_BASE_LENGTH] = configuration[Self::PREFETCHABLE_MEMORY_BASE_BEGIN..Self::PREFETCHABLE_MEMORY_BASE_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let prefetchable_memory_base: u16 = u16::from_le_bytes(prefetchable_memory_base);
                let prefetchable_memory_limit: [u8; Self::PREFETCHABLE_MEMORY_LIMIT_LENGTH] = configuration[Self::PREFETCHABLE_MEMORY_LIMIT_BEGIN..Self::PREFETCHABLE_MEMORY_LIMIT_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let prefetchable_memory_limit: u16 = u16::from_le_bytes(prefetchable_memory_limit);
                let prefetchable_memory_base_upper_32bits: [u8; Self::PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_LENGTH] = configuration[Self::PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_BEGIN..Self::PREFETCHABLE_MEMORY_BASE_UPPER_32BITS_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let prefetchable_memory_base_upper_32bits: u32 = u32::from_le_bytes(prefetchable_memory_base_upper_32bits);
                let prefetchable_memory_limit_upper_32bits: [u8; Self::PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_LENGTH] = configuration[Self::PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_BEGIN..Self::PREFETCHABLE_MEMORY_LIMIT_UPPER_32BITS_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let prefetchable_memory_limit_upper_32bits: u32 = u32::from_le_bytes(prefetchable_memory_limit_upper_32bits);
                let io_base_upper_16bits: [u8; Self::IO_BASE_UPPER_16BITS_LENGTH] = configuration[Self::IO_BASE_UPPER_16BITS_BEGIN..Self::IO_BASE_UPPER_16BITS_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let io_base_upper_16bits: u16 = u16::from_le_bytes(io_base_upper_16bits);
                let io_base_limit_16bits: [u8; Self::IO_BASE_LIMIT_16BITS_LENGTH] = configuration[Self::IO_BASE_LIMIT_16BITS_BEGIN..Self::IO_BASE_LIMIT_16BITS_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let io_base_limit_16bits: u16 = u16::from_le_bytes(io_base_limit_16bits);
                let expansion_rom_base_address: [u8; Self::EXPANSION_ROM_BASE_ADDRESS_LENGTH] = configuration[Self::TYPE1_EXPANSION_ROM_BASE_ADDRESS_BEGIN..Self::TYPE1_EXPANSION_ROM_BASE_ADDRESS_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let expansion_rom_base_address: expansion_rom_base_address::Register = u32::from_le_bytes(expansion_rom_base_address).into();
                let bridge_control: [u8; Self::BRIDGE_CONTROL_LENGTH] = configuration[Self::BRIDGE_CONTROL_BEGIN..Self::BRIDGE_CONTROL_END]
                    .try_into()
                    .expect("Can't get a PCI device!");
                let bridge_control: bridge_control::Register = u16::from_le_bytes(bridge_control).into();
                Self::Type1 {
                    base_address_registers,
                    primary_bus_number,
                    secondary_bus_number,
                    subordinate_bus_number,
                    secondary_latency_timer,
                    io_base,
                    io_limit,
                    secondary_status,
                    memory_base,
                    memory_limit,
                    prefetchable_memory_base,
                    prefetchable_memory_limit,
                    prefetchable_memory_base_upper_32bits,
                    prefetchable_memory_limit_upper_32bits,
                    io_base_upper_16bits,
                    io_base_limit_16bits,
                    expansion_rom_base_address,
                    bridge_control,
                }
            },
            header_type::HeaderLayout::Reserved => Self::Reserved,
        }
    }
}

