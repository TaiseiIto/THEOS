extern crate alloc;

use {
    alloc::vec::Vec,
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
    MSI,
    // 7.7.2 MSI-X Capability and Table Structure
    // Capability ID 0x11
    MSIX,
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
    fn new(configuration: &[u8; CONFIGURATION_SIZE], capability_pointer: usize) -> Self {
        match configuration[capability_pointer] {
            0x01 => Self::PCIPowerManagement,
            0x05 => Self::MSI,
            0x09 => Self::VendorSpecific,
            0x0d => Self::SubsystemIDandSubsystemVendorID,
            0x10 => Self::PCIExpress,
            0x11 => Self::MSIX,
            0x13 => Self::ConventionalPCIAdvancedFeatures,
            0x14 => Self::EnhancedAllocation,
            0x15 => Self::FlatteningPortalBridge,
            _ => Self::Other,
        }
    }
}

