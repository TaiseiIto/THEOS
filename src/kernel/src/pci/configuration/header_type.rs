// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.1.9 Header Type Register
#[allow(dead_code)]
#[derive(Debug)]
pub struct Register {
    header_layout: HeaderLayout,
    multi_function_device: bool,
}

impl Register {
    const MULTI_FUNCTION_DEVICE_SHIFT: usize = 7;
    const MULTI_FUNCTION_DEVICE_MASK: u8 = 1 << Self::MULTI_FUNCTION_DEVICE_SHIFT;

    pub fn header_layout(&self) -> &HeaderLayout {
        &self.header_layout
    }

    pub fn is_multi_function(&self) -> bool {
        self.multi_function_device
    }
}

impl From<u8> for Register {
    fn from(header_type: u8) -> Self {
        let header_layout: HeaderLayout = header_type.into();
        let multi_function_device: bool = header_type & Self::MULTI_FUNCTION_DEVICE_MASK != 0;
        Self {
            header_layout,
            multi_function_device,
        }
    }
}

#[derive(Clone, Debug)]
pub enum HeaderLayout {
    Type0,
    Type1,
    Reserved,
}

impl HeaderLayout {
    const SHIFT_BEGIN: usize = 0;
    const SHIFT_LENGTH:usize = 7;
    #[allow(dead_code)]
    const SHIFT_END: usize = Self::SHIFT_BEGIN + Self::SHIFT_LENGTH;
    const MASK: u8 = ((1 << Self::SHIFT_LENGTH) - 1) << Self::SHIFT_BEGIN;
}

impl From<u8> for HeaderLayout {
    fn from(header_type: u8) -> Self {
        match header_type & Self::MASK {
            0x00 => Self::Type0,
            0x01 => Self::Type1,
            0x02 => Self::Reserved,
            _ => panic!("Invalid header layout!"),
        }
    }
}

