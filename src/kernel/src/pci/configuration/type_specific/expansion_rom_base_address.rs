use core::mem;

// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.2.4 Expansion ROM Base Address Register
#[derive(Debug)]
pub struct Register {
    enable: bool,
    validation_status: u8,
    validation_details: u8,
    base_address: u32,
}

impl Register {
    const ENABLE_SHIFT_BEGIN: usize = 0;
    const ENABLE_SHIFT_LENGTH: usize = 1;
    const ENABLE_SHIFT_END: usize = Self::ENABLE_SHIFT_BEGIN + Self::ENABLE_SHIFT_LENGTH;
    const VALIDATION_STATUS_SHIFT_BEGIN: usize = Self::ENABLE_SHIFT_END;
    const VALIDATION_STATUS_SHIFT_LENGTH: usize = 3;
    const VALIDATION_STATUS_SHIFT_END: usize = Self::VALIDATION_STATUS_SHIFT_BEGIN + Self::VALIDATION_STATUS_SHIFT_LENGTH;
    const VALIDATION_DETAILS_SHIFT_BEGIN: usize = Self::VALIDATION_STATUS_SHIFT_END;
    const VALIDATION_DETAILS_SHIFT_LENGTH: usize = 4;
    const VALIDATION_DETAILS_SHIFT_END: usize = Self::VALIDATION_DETAILS_SHIFT_BEGIN + Self::VALIDATION_DETAILS_SHIFT_LENGTH;
    const BASE_ADDRESS_SHIFT_BEGIN: usize = 11;
    const BASE_ADDRESS_SHIFT_END: usize = 8 * mem::size_of::<u32>();
    const BASE_ADDRESS_SHIFT_LENGTH: usize = Self::BASE_ADDRESS_SHIFT_END - Self::BASE_ADDRESS_SHIFT_BEGIN;

    const ENABLE_MASK: u32 = ((1 << Self::ENABLE_SHIFT_LENGTH) - 1) << Self::ENABLE_SHIFT_BEGIN;
    const VALIDATION_STATUS_MASK: u32 = ((1 << Self::VALIDATION_STATUS_SHIFT_LENGTH) - 1) << Self::VALIDATION_STATUS_SHIFT_BEGIN;
    const VALIDATION_DETAILS_MASK: u32 = ((1 << Self::VALIDATION_DETAILS_SHIFT_LENGTH) - 1) << Self::VALIDATION_DETAILS_SHIFT_BEGIN;
    const BASE_ADDRESS_MASK: u32 = ((1 << Self::BASE_ADDRESS_SHIFT_LENGTH) - 1) << Self::BASE_ADDRESS_SHIFT_BEGIN;
}

impl From<u32> for Register {
    fn from(register: u32) -> Self {
        let enable: bool = register & Self::ENABLE_MASK != 0;
        let validation_status: u8 = ((register & Self::VALIDATION_STATUS_MASK) >> Self::VALIDATION_STATUS_SHIFT_BEGIN) as u8;
        let validation_details: u8 = ((register & Self::VALIDATION_DETAILS_MASK) >> Self::VALIDATION_DETAILS_SHIFT_BEGIN) as u8;
        let base_address: u32 = register & Self::BASE_ADDRESS_MASK;
        Self {
            enable,
            validation_status,
            validation_details,
            base_address,
        }
    }
}

