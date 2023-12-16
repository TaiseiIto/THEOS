use core::{
    fmt,
    mem,
};

mod interrupter;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5 Host Controller Runtime Registers
#[allow(dead_code)]
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    mfindex: Mfindex,
    reserved: [u8; 0x20 - mem::size_of::<Mfindex>()],
    interrupter: [interrupter::Registers; 0x400],
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.1 Microframe Index Register (MFINDEX)
#[derive(Clone, Copy)]
struct Mfindex(u32);

impl Mfindex {
    const MICROFRAME_INDEX_BEGIN: usize = 0;
    const MICROFRAME_INDEX_LENGTH: usize = 14;
    const MICROFRAME_INDEX_END: usize = Self::MICROFRAME_INDEX_BEGIN + Self::MICROFRAME_INDEX_LENGTH;

    const MICROFRAME_INDEX_MASK: u32 = (1 << Self::MICROFRAME_INDEX_END) - (1 << Self::MICROFRAME_INDEX_BEGIN);

    fn microframe_index(&self) -> u16 {
        ((self.0 & Self::MICROFRAME_INDEX_MASK) >> Self::MICROFRAME_INDEX_BEGIN) as u16
    }
}

impl fmt::Debug for Mfindex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MFINDEX")
            .field("self", &self.0)
            .field("MICROFRAME_INDEX", &self.microframe_index())
            .finish()
    }
}

