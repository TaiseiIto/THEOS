use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.2 Interrupter Register Set
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Registers {
    iman: Iman,
    imod: Imod,
    erstsz: Erstsz,
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

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.2.2 Interruper Moderation Register (IMOD)
#[derive(Clone, Copy)]
struct Imod(u32);

impl Imod {
    const IMODI_BEGIN: usize = 0;
    const IMODI_LENGTH: usize = 16;
    const IMODI_END: usize = Self::IMODI_BEGIN + Self::IMODI_LENGTH;
    const IMODC_BEGIN: usize = Self::IMODI_END;
    const IMODC_LENGTH: usize = 16;
    #[allow(dead_code)]
    const IMODC_END: usize = Self::IMODC_BEGIN + Self::IMODC_LENGTH;

    const IMODI_MASK: u32 = (1 << Self::IMODI_END) - (1 << Self::IMODI_BEGIN);
    const IMODC_MASK: u32 = u32::MAX - (1 << Self::IMODC_BEGIN) + 1;

    fn imodi(&self) -> u16 {
        ((self.0 & Self::IMODI_MASK) >> Self::IMODI_BEGIN) as u16
    }

    fn imodc(&self) -> u16 {
        ((self.0 & Self::IMODC_MASK) >> Self::IMODC_BEGIN) as u16
    }
}

impl fmt::Debug for Imod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IMOD")
            .field("self", &self.0)
            .field("IMODI", &self.imodi())
            .field("IMODC", &self.imodc())
            .finish()
    }
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.2.3.1 Event Ring Segment Table Size Register (ERSTSZ)
#[derive(Clone, Copy)]
struct Erstsz(u32);

impl Erstsz {
    const ERSTSZ_BEGIN: usize = 0;
    const ERSTSZ_LENGTH: usize = 16;
    const ERSTSZ_END: usize = Self::ERSTSZ_BEGIN + Self::ERSTSZ_LENGTH;

    const ERSTSZ_MASK: u32 = (1 << Self::ERSTSZ_END) - (1 << Self::ERSTSZ_BEGIN);

    fn erstsz(&self) -> u16 {
        ((self.0 & Self::ERSTSZ_MASK) >> Self::ERSTSZ_BEGIN) as u16
    }
}

impl fmt::Debug for Erstsz {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ERSTSZ")
            .field("self", &self.0)
            .field("ERSTSZ", &self.erstsz())
            .finish()
    }
}

