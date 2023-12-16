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
    erstba: Erstba,
    erdp: Erdp,
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

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.2.3.2 Event Ring Segment Table Base Address Register (ERSTBA)
#[derive(Clone, Copy)]
struct Erstba(u64);

impl Erstba {
    const ERSTBA_BEGIN: usize = 6;
    const ERSTBA_MASK: u64 = u64::MAX - (1 << Self::ERSTBA_BEGIN) + 1;

    fn erstba(&self) -> u64 {
        self.0 & Self::ERSTBA_MASK
    }
}

impl fmt::Debug for Erstba {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ERSTBA")
            .field("self", &self.0)
            .field("ERSTBA", &self.erstba())
            .finish()
    }
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.5.2.3.3 Event Ring Dequeue Pointer Register (ERDP)
#[derive(Clone, Copy)]
struct Erdp(u64);

impl Erdp {
    const DESI_BEGIN: usize = 0;
    const DESI_LENGTH: usize = 3;
    const DESI_END: usize = Self::DESI_BEGIN + Self::DESI_LENGTH;
    const EHB_BEGIN: usize = Self::DESI_END;
    const EHB_LENGTH: usize = 1;
    const EHB_END: usize = Self::EHB_BEGIN + Self::EHB_LENGTH;
    const EVENT_RING_DEQUEUE_POINTER_BEGIN: usize = Self::EHB_END;

    const DESI_MASK: u64 = (1 << Self::DESI_END) - (1 << Self::DESI_BEGIN);
    const EHB_MASK: u64 = (1 << Self::EHB_END) - (1 << Self::EHB_BEGIN);
    const EVENT_RING_DEQUEUE_POINTER_MASK: u64 = u64::MAX - (1 << Self::EVENT_RING_DEQUEUE_POINTER_BEGIN) + 1;

    fn desi(&self) -> u8 {
        ((self.0 & Self::DESI_MASK) >> Self::DESI_BEGIN) as u8
    }

    fn ehb(&self) -> bool {
        self.0 & Self::EHB_MASK != 0
    }

    fn event_ring_dequeue_pointer(&self) -> u64 {
        self.0 & Self::EVENT_RING_DEQUEUE_POINTER_MASK
    }
}

impl fmt::Debug for Erdp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ERDP")
            .field("self", &self.0)
            .field("DESI", &self.desi())
            .field("EHB", &self.ehb())
            .field("EVENT_RING_DEQUEUE_POINTER", &self.event_ring_dequeue_pointer())
            .finish()
    }
}

