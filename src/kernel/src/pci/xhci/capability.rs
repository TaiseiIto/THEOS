use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.3 Host Controller Capability Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    caplength: u8,
    rsvd: u8,
    hciversion: u16,
    hcsparams1: Hcsparams1,
    hcsparams2: Hcsparams2,
    hcsparams3: u32,
    hccparams1: u32,
    dbof: u32,
    rtsoff: u32,
    hccparams2: u32,
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.3.3 Structural Parameters 1 (HCSPARAMS1)
#[derive(Clone, Copy)]
struct Hcsparams1(u32);

impl Hcsparams1 {
    const MAX_DEVICE_SLOTS_BEGIN: usize = 0;
    const MAX_DEVICE_SLOTS_LENGTH: usize = 8;
    const MAX_DEVICE_SLOTS_END: usize = Self::MAX_DEVICE_SLOTS_BEGIN + Self::MAX_DEVICE_SLOTS_LENGTH;
    const MAX_DEVICE_SLOTS_MASK: u32 = (1 << Self::MAX_DEVICE_SLOTS_END) - (1 << Self::MAX_DEVICE_SLOTS_BEGIN);
    const MAX_INTERRUPTERS_BEGIN: usize = Self::MAX_DEVICE_SLOTS_END;
    const MAX_INTERRUPTERS_LENGTH: usize = 11;
    const MAX_INTERRUPTERS_END: usize = Self::MAX_INTERRUPTERS_BEGIN + Self::MAX_INTERRUPTERS_LENGTH;
    const MAX_INTERRUPTERS_MASK: u32 = (1 << Self::MAX_INTERRUPTERS_END) - (1 << Self::MAX_INTERRUPTERS_BEGIN);
    const RESERVED_BEGIN: usize = Self::MAX_INTERRUPTERS_END;
    const RESERVED_LENGTH: usize = 4;
    const RESERVED_END: usize = Self::RESERVED_BEGIN + Self::RESERVED_LENGTH;
    const MAX_PORTS_BEGIN: usize = Self::RESERVED_END;
    const MAX_PORTS_MASK: u32 = u32::MAX - (1 << Self::MAX_PORTS_BEGIN) + 1;

    fn max_device_slots(&self) -> u8 {
        ((self.0 & Self::MAX_DEVICE_SLOTS_MASK) >> Self::MAX_DEVICE_SLOTS_BEGIN) as u8
    }

    fn max_interrupters(&self) -> u16 {
        ((self.0 & Self::MAX_INTERRUPTERS_MASK) >> Self::MAX_INTERRUPTERS_BEGIN) as u16
    }

    fn max_ports(&self) -> u8 {
        ((self.0 & Self::MAX_PORTS_MASK) >> Self::MAX_PORTS_BEGIN) as u8
    }
}

impl fmt::Debug for Hcsparams1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HCSPARAMS1")
            .field("max_device_slots", &self.max_device_slots())
            .field("max_interrupters", &self.max_interrupters())
            .field("max_ports", &self.max_ports())
            .finish()
    }
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.3.4 Structural Parameters 2 (HCSPARAMS2)
#[derive(Clone, Copy)]
struct Hcsparams2(u32);

impl Hcsparams2 {
    const ISOCHRONOUS_SCHEDULING_THRESHOLD_BEGIN: usize = 0;
    const ISOCHRONOUS_SCHEDULING_THRESHOLD_LENGTH: usize = 4;
    const ISOCHRONOUS_SCHEDULING_THRESHOLD_END: usize = Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_BEGIN + Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_LENGTH;
    const ISOCHRONOUS_SCHEDULING_THRESHOLD_MASK: u32 = (1 << Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_END) - (1 << Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_BEGIN);

    fn isochronous_scheduling_threshold(&self) -> u8 {
        ((self.0 & Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_MASK) >> Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_BEGIN) as u8
    }
}

impl fmt::Debug for Hcsparams2 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HCSPARAMS2")
            .field("isochronous_scheduling_threshold", &self.isochronous_scheduling_threshold())
            .finish()
    }
}

