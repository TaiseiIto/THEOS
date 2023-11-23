use super::{
    CONFIGURATION_SIZE,
    header_type,
};

// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.2 Type 0 Configuration Space Header
// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.3 Type 1 Configuration Space Header
#[derive(Debug)]
pub enum Registers {
    Type0,
    Type1,
    Reserved,
}

impl Registers {
    pub fn new(header_layout: &header_type::HeaderLayout, configuration: &[u8; CONFIGURATION_SIZE]) -> Self {
        match header_layout {
            header_type::HeaderLayout::Type0 => Self::Type0,
            header_type::HeaderLayout::Type1 => Self::Type1,
            header_type::HeaderLayout::Reserved => Self::Reserved,
        }
    }
}

