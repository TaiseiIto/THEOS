use super::asm;

#[derive(Debug)]
pub struct ConfigurationAddress {
    bus: u8,
    device: u8,
    function: u8,
}

impl ConfigurationAddress {
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
    const BUS_SHIFT_LENGTH: usize = 8;
    const BUS_SHIFT_END: usize = Self::BUS_SHIFT_BEGIN + Self::BUS_SHIFT_LENGTH;
    const ENABLE_BIT_SHIFT: usize = 31;
    const ENABLE_BIT: u32 = 1 << Self::ENABLE_BIT_SHIFT;

    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        if device <= Self::DEVICE_MAX && function <= Self::FUNCTION_MAX {
            Self {
                bus,
                device,
                function,
            }
        } else {
            panic!("Can't create a PCI configuration address!");
        }
    }

    pub fn address(&self, offset: u8) -> u32 {
        let Self {
            bus,
            device,
            function
        } = self;
        Self::ENABLE_BIT + ((*bus as u32) << Self::BUS_SHIFT_BEGIN) + ((*device as u32) << Self::DEVICE_SHIFT_BEGIN) + ((*function as u32) << Self::FUNCTION_SHIFT_BEGIN)
    }
}
