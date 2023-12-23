extern crate alloc;

use {
    alloc::vec::Vec,
    core::mem,
    super::CONFIGURATION_SIZE,
};

#[derive(Clone, Debug)]
pub struct List {
    capability_id: u8,
    next_capability_pointer: u8,
    registers: Registers,
}

impl List {
    pub fn get_all(configuration: &[u8; CONFIGURATION_SIZE], first_capability_pointer: u8) -> Vec<Self> {
        ListIterator::new(configuration, first_capability_pointer).collect()
    }

    fn get_one(configuration: &[u8; CONFIGURATION_SIZE], capability_pointer: u8) -> Option<Self> {
        match capability_pointer {
            0x00 => None,
            capability_pointer => {
                let capability_pointer: usize = capability_pointer as usize;
                let capability_id: u8 = configuration[capability_pointer];
                let next_capability_pointer: u8 = configuration[capability_pointer + 1];
                let registers = Registers::new(configuration, capability_pointer);
                Some(Self {
                    capability_id,
                    next_capability_pointer,
                    registers,
                })
            },
        }
    }
}

#[derive(Clone)]
struct ListIterator<'a> {
    configuration: &'a [u8; CONFIGURATION_SIZE],
    structure: Option<List>,
}

impl<'a> ListIterator<'a> {
    fn new(configuration: &'a [u8; CONFIGURATION_SIZE], first_capability_pointer: u8) -> Self {
        let structure: Option<List> = List::get_one(configuration, first_capability_pointer);
        Self {
            configuration,
            structure,
        }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = List;

    fn next(&mut self) -> Option<Self::Item> {
        let ListIterator {
            configuration,
            structure,
        } = self.clone();
        if let Some(structure) = &structure {
            let next_capability_pointer: u8 = structure.next_capability_pointer;
            self.structure = List::get_one(configuration, next_capability_pointer);
        }
        structure
    }
}

// PCI Express Base Specification Revision 5.0 Version 1.0
// from 7.5.2 to 7.9.24
#[derive(Clone, Debug)]
enum Registers {
    // 7.5.2 PCI Power Management Capability Structure
    // Capability ID 0x01
    PCIPowerManagement,
    // 7.5.3 PCI Express Capability Structure
    // Capability ID 0x10
    PCIExpress,
    // 7.7.1 MSI Capability Structure
    // Capability ID 0x05
    MSI {
        message_control: u16,
    },
    // 7.7.2 MSI-X Capability and Table Structure
    // Capability ID 0x11
    MSIX {
        message_control: u16,
        table_offset: u32,
        pba_offset: u32,
    },
    // 7.8.5 Enhanced Allocation Capability Structure (EA)
    // Capability ID 0x14
    EnhancedAllocation,
    // 7.8.10 Flattening Portal Bridge (FPB) Capability
    // Capability ID 0x15
    FlatteningPortalBridge,
    // 7.9.4 Vender Specific Capability
    // Capability ID 0x09
    VendorSpecific,
    // 7.9.22 Conventional PCI Advanced Features Capability (AF)
    // Capability ID 0x13
    ConventionalPCIAdvancedFeatures,
    // 7.9.24 Subsystem ID and Subsystem Vendor ID Capability
    // Capability ID 0x0d
    SubsystemIDandSubsystemVendorID,
    Other,
}

impl Registers {
    const MSI_MESSAGE_CONTROL_OFFSET: usize = 2;
    const MSI_MESSAGE_CONTROL_SIZE: usize = mem::size_of::<u16>();

    const MSIX_MESSAGE_CONTROL_OFFSET: usize = 2;
    const MSIX_MESSAGE_CONTROL_SIZE: usize = mem::size_of::<u16>();
    const MSIX_TABLE_OFFSET_OFFSET: usize = Self::MSIX_MESSAGE_CONTROL_OFFSET + Self::MSIX_MESSAGE_CONTROL_SIZE;
    const MSIX_TABLE_OFFSET_SIZE: usize = mem::size_of::<u32>();
    const MSIX_PBA_OFFSET_OFFSET: usize = Self::MSIX_TABLE_OFFSET_OFFSET + Self::MSIX_TABLE_OFFSET_SIZE;
    const MSIX_PBA_OFFSET_SIZE: usize = mem::size_of::<u32>();

    fn new(configuration: &[u8; CONFIGURATION_SIZE], capability_pointer: usize) -> Self {
        match configuration[capability_pointer] {
            0x01 => Self::PCIPowerManagement,
            0x05 => {
                let message_control_begin: usize = capability_pointer + Self::MSI_MESSAGE_CONTROL_OFFSET;
                let message_control_end: usize = message_control_begin + Self::MSI_MESSAGE_CONTROL_SIZE;
                let message_control: &[u8] = &configuration[message_control_begin..message_control_end];
                let message_control: [u8; Self::MSI_MESSAGE_CONTROL_SIZE] = message_control
                    .try_into()
                    .expect("Can't get message control!");
                let message_control: u16 = u16::from_le_bytes(message_control);
                Self::MSI {
                    message_control,
                }
            },
            0x09 => Self::VendorSpecific,
            0x0d => Self::SubsystemIDandSubsystemVendorID,
            0x10 => Self::PCIExpress,
            0x11 => {
                let message_control_begin: usize = capability_pointer + Self::MSIX_MESSAGE_CONTROL_OFFSET;
                let message_control_end: usize = message_control_begin + Self::MSIX_MESSAGE_CONTROL_SIZE;
                let message_control: &[u8] = &configuration[message_control_begin..message_control_end];
                let message_control: [u8; Self::MSIX_MESSAGE_CONTROL_SIZE] = message_control
                    .try_into()
                    .expect("Can't get message control!");
                let message_control: u16 = u16::from_le_bytes(message_control);
                let table_offset_begin: usize = capability_pointer + Self::MSIX_TABLE_OFFSET_OFFSET;
                let table_offset_end: usize = table_offset_begin + Self::MSIX_TABLE_OFFSET_SIZE;
                let table_offset: &[u8] = &configuration[table_offset_begin..table_offset_end];
                let table_offset: [u8; Self::MSIX_TABLE_OFFSET_SIZE] = table_offset
                    .try_into()
                    .expect("Can't get table offset!");
                let table_offset: u32 = u32::from_le_bytes(table_offset);
                let pba_offset_begin: usize = capability_pointer + Self::MSIX_PBA_OFFSET_OFFSET;
                let pba_offset_end: usize = pba_offset_begin + Self::MSIX_PBA_OFFSET_SIZE;
                let pba_offset: &[u8] = &configuration[pba_offset_begin..pba_offset_end];
                let pba_offset: [u8; Self::MSIX_PBA_OFFSET_SIZE] = pba_offset
                    .try_into()
                    .expect("Can't get pba offset!");
                let pba_offset: u32 = u32::from_le_bytes(pba_offset);
                Self::MSIX {
                    message_control,
                    table_offset,
                    pba_offset,
                }
            },
            0x13 => Self::ConventionalPCIAdvancedFeatures,
            0x14 => Self::EnhancedAllocation,
            0x15 => Self::FlatteningPortalBridge,
            _ => Self::Other,
        }
    }
}

