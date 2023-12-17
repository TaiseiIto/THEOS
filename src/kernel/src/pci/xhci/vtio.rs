use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.7 VTIO Registers
#[allow(dead_code)]
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    cap: Cap,
    ca: u32,
    da: [u32; 8],
    reserved0: [u8; 8],
    ia: [u32; 32],
    reserved1: [u8; 0x50],
    ea: [u32; 255],
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.7.1 VTIO Capability Register (VTIOCAP)
#[derive(Clone, Copy)]
struct Cap(u32);

impl Cap {
    const PDMAID_BEGIN: usize = 0;
    const PDMAID_LENGTH: usize = 16;
    const PDMAID_END: usize = Self::PDMAID_BEGIN + Self::PDMAID_LENGTH;
    const ADMAID_BEGIN: usize = Self::PDMAID_END;
    const ADMAID_LENGTH: usize = 16;
    const ADMAID_END: usize = Self::ADMAID_BEGIN + Self::ADMAID_LENGTH;

    const PDMAID_MASK: u32 = (1 << Self::PDMAID_END) - (1 << Self::PDMAID_BEGIN);
    const ADMAID_MASK: u32 = u32::MAX - (1 << Self::ADMAID_BEGIN) + 1;

    fn pdmaid(&self) -> u16 {
        ((self.0 & Self::PDMAID_MASK) >> Self::PDMAID_BEGIN) as u16
    }

    fn admaid(&self) -> u16 {
        ((self.0 & Self::ADMAID_MASK) >> Self::ADMAID_BEGIN) as u16
    }
}

impl fmt::Debug for Cap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CAP")
            .field("self", &self.0)
            .field("PDMAID", &self.pdmaid())
            .field("ADMAID", &self.admaid())
            .finish()
    }
}

