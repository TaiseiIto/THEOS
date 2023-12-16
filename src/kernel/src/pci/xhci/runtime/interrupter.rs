use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.2 Interrupter Register Set
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Registers {
    iman: Iman,
    imod: u32,
    erstsz: u32,
    reserved: u32,
    erstba: u64,
    erdp: u64,
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.2.1 Interrupter Management Register (IMAN)
#[derive(Clone, Copy)]
struct Iman(u32);

impl Iman {
    const IP_SHIFT: usize = 0;
    const IE_SHIFT: usize = Self::IP_SHIFT + 1;

    const IP_MASK: u32 = 1 << Self::IP_SHIFT;
    const IE_MASK: u32 = 1 << Self::IE_SHIFT;

    fn ip(&self) -> bool {
        self.0 & Self::IP_MASK != 0
    }

    fn ie(&self) -> bool {
        self.0 & Self::IE_MASK != 0
    }
}

impl fmt::Debug for Iman {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IMAN")
            .field("self", &self.0)
            .field("IP", &self.ip())
            .field("IE", &self.ie())
            .finish()
    }
}

