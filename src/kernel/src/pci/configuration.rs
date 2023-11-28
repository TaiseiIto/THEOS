extern crate alloc;

mod bist;
mod command;
mod header_type;
mod status;
mod type_specific;

use {
    alloc::{
        collections::btree_map::BTreeMap,
        vec::Vec,
    },
    core::mem,
    super::super::asm,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    bus: u8,
    device: u8,
    function: u8,
}

impl Address {
    const OFFSET_SHIFT_BEGIN: usize = 0;
    const OFFSET_SHIFT_LENGTH: usize = 8;
    const OFFSET_SHIFT_END: usize = Self::OFFSET_SHIFT_BEGIN + Self::OFFSET_SHIFT_LENGTH;
    const FUNCTION_SHIFT_BEGIN: usize = Self::OFFSET_SHIFT_END;
    const FUNCTION_SHIFT_LENGTH: usize = 3;
    const FUNCTION_SHIFT_END: usize = Self::FUNCTION_SHIFT_BEGIN + Self::FUNCTION_SHIFT_LENGTH;
    const FUNCTION_MAX: u8 = (1 << Self::FUNCTION_SHIFT_LENGTH) - 1;
    const DEVICE_SHIFT_BEGIN: usize = Self::FUNCTION_SHIFT_END;
    const DEVICE_SHIFT_LENGTH: usize = 5;
    const DEVICE_SHIFT_END: usize = Self::DEVICE_SHIFT_BEGIN + Self::DEVICE_SHIFT_LENGTH;
    const DEVICE_MAX: u8 = (1 << Self::DEVICE_SHIFT_LENGTH) - 1;
    const BUS_SHIFT_BEGIN: usize = Self::DEVICE_SHIFT_END;
    #[allow(dead_code)]
    const BUS_SHIFT_LENGTH: usize = 8;
    #[allow(dead_code)]
    const BUS_SHIFT_END: usize = Self::BUS_SHIFT_BEGIN + Self::BUS_SHIFT_LENGTH;
    const ENABLE_BIT_SHIFT: usize = 31;
    const ENABLE_BIT: u32 = 1 << Self::ENABLE_BIT_SHIFT;
    const ADDRESS_PORT: u16 = 0x0cf8;
    const VALUE_PORT: u16 = 0x0cfc;

    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        assert!(device <= Self::DEVICE_MAX && function <= Self::FUNCTION_MAX);
        Self {
            bus,
            device,
            function,
        }
    }

    fn address(&self, offset: u8) -> u32 {
        let Self {
            bus,
            device,
            function
        } = self;
        Self::ENABLE_BIT + ((*bus as u32) << Self::BUS_SHIFT_BEGIN) + ((*device as u32) << Self::DEVICE_SHIFT_BEGIN) + ((*function as u32) << Self::FUNCTION_SHIFT_BEGIN) + (offset as u32)
    }

    fn read(&self, offset: u8) -> u32 {
        asm::outl(Self::ADDRESS_PORT, self.address(offset));
        asm::inl(Self::VALUE_PORT)
    }
}

const CONFIGURATION_SIZE: usize = 0x100;

impl Into<[u8; CONFIGURATION_SIZE]> for &Address {
    fn into(self) -> [u8; CONFIGURATION_SIZE] {
        (0..CONFIGURATION_SIZE)
            .step_by(mem::size_of::<u32>())
            .flat_map(|offset| self
                .read(offset as u8)
                .to_le_bytes()
                .into_iter())
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Can't get a PCI configuration!")
    }
}

impl Into<Option<Device>> for &Address {
    fn into(self) -> Option<Device> {
        let configuration: [u8; CONFIGURATION_SIZE] = self.into();
        let configuration: Device = configuration.into();
        match &configuration.vendor_id {
            0xffffu16 => None,
            _ => Some(configuration),
        }
    }
}

// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1 PCI-Compatible Configuration Registers
#[allow(dead_code)]
#[derive(Debug)]
pub struct Device {
    vendor_id: u16,
    device_id: u16,
    command: command::Register,
    status: status::Register,
    revision_id: u8,
    interface: u8,
    sub_class: u8,
    base_class: u8,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: header_type::Register,
    bist: bist::Register,
    type_specific: type_specific::Registers,
    capabilities_pointer: u8,
    interrupt_line: u8,
    interrupt_pin: u8,
}

impl Device {
    const VENDOR_ID_BEGIN: usize = 0;
    const VENDOR_ID_SIZE: usize = mem::size_of::<u16>();
    const VENDOR_ID_END: usize = Self::VENDOR_ID_BEGIN + Self::VENDOR_ID_SIZE;
    const DEVICE_ID_BEGIN: usize = Self::VENDOR_ID_END;
    const DEVICE_ID_SIZE: usize = mem::size_of::<u16>();
    const DEVICE_ID_END: usize = Self::DEVICE_ID_BEGIN + Self::DEVICE_ID_SIZE;
    const COMMAND_BEGIN: usize = Self::DEVICE_ID_END;
    const COMMAND_SIZE: usize = mem::size_of::<u16>();
    const COMMAND_END: usize = Self::COMMAND_BEGIN + Self::COMMAND_SIZE;
    const STATUS_BEGIN: usize = Self::COMMAND_END;
    const STATUS_SIZE: usize = mem::size_of::<u16>();
    const STATUS_END: usize = Self::STATUS_BEGIN + Self::STATUS_SIZE;
    const REVISION_ID_BEGIN: usize = Self::STATUS_END;
    const REVISION_ID_SIZE: usize = mem::size_of::<u8>();
    const REVISION_ID_END: usize = Self::REVISION_ID_BEGIN + Self::REVISION_ID_SIZE;
    const INTERFACE_BEGIN: usize = Self::REVISION_ID_END;
    const INTERFACE_SIZE: usize = mem::size_of::<u8>();
    const INTERFACE_END: usize = Self::INTERFACE_BEGIN + Self::INTERFACE_SIZE;
    const SUB_CLASS_BEGIN: usize = Self::INTERFACE_END;
    const SUB_CLASS_SIZE: usize = mem::size_of::<u8>();
    const SUB_CLASS_END: usize = Self::SUB_CLASS_BEGIN + Self::SUB_CLASS_SIZE;
    const BASE_CLASS_BEGIN: usize = Self::SUB_CLASS_END;
    const BASE_CLASS_SIZE: usize = mem::size_of::<u8>();
    const BASE_CLASS_END: usize = Self::BASE_CLASS_BEGIN + Self::BASE_CLASS_SIZE;
    const CACHE_LINE_SIZE_BEGIN: usize = Self::BASE_CLASS_END;
    const CACHE_LINE_SIZE_SIZE: usize = mem::size_of::<u8>();
    const CACHE_LINE_SIZE_END: usize = Self::CACHE_LINE_SIZE_BEGIN + Self::CACHE_LINE_SIZE_SIZE;
    const LATENCY_TIMER_BEGIN: usize = Self::CACHE_LINE_SIZE_END;
    const LATENCY_TIMER_SIZE: usize = mem::size_of::<u8>();
    const LATENCY_TIMER_END: usize = Self::LATENCY_TIMER_BEGIN + Self::LATENCY_TIMER_SIZE;
    const HEADER_TYPE_BEGIN: usize = Self::LATENCY_TIMER_END;
    const HEADER_TYPE_SIZE: usize = mem::size_of::<u8>();
    const HEADER_TYPE_END: usize = Self::HEADER_TYPE_BEGIN + Self::HEADER_TYPE_SIZE;
    const BIST_BEGIN: usize = Self::HEADER_TYPE_END;
    const BIST_SIZE: usize = mem::size_of::<u8>();
    const BIST_END: usize = Self::BIST_BEGIN + Self::BIST_SIZE;
    const CAPABILITIES_POINTER_BEGIN: usize = 0x34;
    #[allow(dead_code)]
    const CAPABILITIES_POINTER_SIZE: usize = mem::size_of::<u8>();
    #[allow(dead_code)]
    const CAPABILITIES_POINTER_END: usize = Self::CAPABILITIES_POINTER_BEGIN + Self::CAPABILITIES_POINTER_SIZE;
    const INTERRUPT_LINE_BEGIN: usize = 0x3c;
    #[allow(dead_code)]
    const INTERRUPT_LINE_SIZE: usize = mem::size_of::<u8>();
    #[allow(dead_code)]
    const INTERRUPT_LINE_END: usize = Self::INTERRUPT_LINE_BEGIN + Self::INTERRUPT_LINE_SIZE;
    const INTERRUPT_PIN_BEGIN: usize = 0x3d;
    const INTERRUPT_PIN_SIZE: usize = mem::size_of::<u8>();
    const INTERRUPT_PIN_END: usize = Self::INTERRUPT_PIN_BEGIN + Self::INTERRUPT_PIN_SIZE;

    pub fn get_all_devices() -> BTreeMap<Address, Self> {
        let mut address2device = BTreeMap::<Address, Self>::new();
        let address = Address::new(0, 0, 0);
        let device: Option<Self> = (&address).into();
        let device: Self = device.expect("Can't get the host bridge!");
        address2device.insert(address, device);
        address2device
    }
}

impl From<[u8; CONFIGURATION_SIZE]> for Device {
    fn from(configuration: [u8; CONFIGURATION_SIZE]) -> Self {
        let vendor_id: [u8; Self::VENDOR_ID_SIZE] = configuration[Self::VENDOR_ID_BEGIN..Self::VENDOR_ID_END]
            .try_into()
            .expect("Can't get a PCI configuration!");
        let vendor_id: u16 = u16::from_le_bytes(vendor_id);
        let device_id: [u8; Self::DEVICE_ID_SIZE] = configuration[Self::DEVICE_ID_BEGIN..Self::DEVICE_ID_END]
            .try_into()
            .expect("Can't get a PCI configuration!");
        let device_id: u16 = u16::from_le_bytes(device_id);
        let command: [u8; Self::COMMAND_SIZE] = configuration[Self::COMMAND_BEGIN..Self::COMMAND_END]
            .try_into()
            .expect("Can't get a PCI configuration!");
        let command: command::Register = u16::from_le_bytes(command).into();
        let status: [u8; Self::STATUS_SIZE] = configuration[Self::STATUS_BEGIN..Self::STATUS_END]
            .try_into()
            .expect("Can't get a PCI configuration!");
        let status: status::Register = u16::from_le_bytes(status).into();
        let revision_id: u8 = configuration[Self::REVISION_ID_BEGIN];
        let interface: u8 = configuration[Self::INTERFACE_BEGIN];
        let sub_class: u8 = configuration[Self::SUB_CLASS_BEGIN];
        let base_class: u8 = configuration[Self::BASE_CLASS_BEGIN];
        let cache_line_size: u8 = configuration[Self::CACHE_LINE_SIZE_BEGIN];
        let latency_timer: u8 = configuration[Self::LATENCY_TIMER_BEGIN];
        let header_type: header_type::Register = configuration[Self::HEADER_TYPE_BEGIN].into();
        let bist: bist::Register = configuration[Self::BIST_BEGIN].into();
        let type_specific = type_specific::Registers::new(header_type.header_layout(), &configuration);
        let capabilities_pointer: u8 = configuration[Self::CAPABILITIES_POINTER_BEGIN];
        let interrupt_line: u8 = configuration[Self::INTERRUPT_LINE_BEGIN];
        let interrupt_pin: u8 = configuration[Self::INTERRUPT_PIN_BEGIN];
        Self {
            vendor_id,
            device_id,
            command,
            status,
            revision_id,
            interface,
            sub_class,
            base_class,
            cache_line_size,
            latency_timer,
            header_type,
            bist,
            type_specific,
            capabilities_pointer,
            interrupt_line,
            interrupt_pin,
        }
    }
}

