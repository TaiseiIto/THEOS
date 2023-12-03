// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.1.10 BIST Register
#[allow(dead_code)]
#[derive(Debug)]
pub struct Register {
    completion_code: u8,
    start_bist: bool,
    bist_capable: bool,
}

impl Register {
    const COMPLETION_CODE_SHIFT_BEGIN: usize = 0;
    const COMPLETION_CODE_SHIFT_LENGTH: usize = 4;
    #[allow(dead_code)]
    const COMPLETION_CODE_SHIFT_END: usize = Self::COMPLETION_CODE_SHIFT_BEGIN + Self::COMPLETION_CODE_SHIFT_LENGTH;
    const START_BIST_SHIFT: usize = 6;
    const BIST_CAPABLE_SHIFT: usize = 7;

    const COMPLETION_CODE_MASK: u8 = ((1 << Self::COMPLETION_CODE_SHIFT_LENGTH) - 1) << Self::COMPLETION_CODE_SHIFT_BEGIN;
    const START_BIST_MASK: u8 = 1 << Self::START_BIST_SHIFT;
    const BIST_CAPABLE_MASK: u8 = 1 << Self::BIST_CAPABLE_SHIFT;
}

impl From<u8> for Register {
    fn from(bist: u8) -> Self {
        let completion_code: u8 = (bist & Self::COMPLETION_CODE_MASK) >> Self::COMPLETION_CODE_SHIFT_BEGIN;
        let start_bist: bool = bist & Self::START_BIST_MASK != 0;
        let bist_capable: bool = bist & Self::BIST_CAPABLE_MASK != 0;
        Self {
            completion_code,
            start_bist,
            bist_capable,
        }
    }
}

