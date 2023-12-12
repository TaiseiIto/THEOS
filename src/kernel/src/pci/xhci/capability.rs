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
    hcsparams3: Hcsparams3,
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
    const RESERVED_LENGTH: usize = 5;
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
    const EVENT_RING_SEGMENT_TABLE_MAX_BEGIN: usize = Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_END;
    const EVENT_RING_SEGMENT_TABLE_MAX_LENGTH: usize = 4;
    const EVENT_RING_SEGMENT_TABLE_MAX_END: usize = Self::EVENT_RING_SEGMENT_TABLE_MAX_BEGIN + Self::EVENT_RING_SEGMENT_TABLE_MAX_LENGTH;
    const EVENT_RING_SEGMENT_TABLE_MAX_MASK: u32 = (1 << Self::EVENT_RING_SEGMENT_TABLE_MAX_END) - (1 << Self::EVENT_RING_SEGMENT_TABLE_MAX_BEGIN);
    const RESERVED_BEGIN: usize = Self::EVENT_RING_SEGMENT_TABLE_MAX_END;
    const RESERVED_LENGTH: usize = 13;
    const RESERVED_END: usize = Self::RESERVED_BEGIN + Self::RESERVED_LENGTH;
    const MAX_SCRATCHPAD_BUFFERS_HIGH_BEGIN: usize = Self::RESERVED_END;
    const MAX_SCRATCHPAD_BUFFERS_HIGH_LENGTH: usize = 5;
    const MAX_SCRATCHPAD_BUFFERS_HIGH_END: usize = Self::MAX_SCRATCHPAD_BUFFERS_HIGH_BEGIN + Self::MAX_SCRATCHPAD_BUFFERS_HIGH_LENGTH;
    const MAX_SCRATCHPAD_BUFFERS_HIGH_MASK: u32 = (1 << Self::MAX_SCRATCHPAD_BUFFERS_HIGH_END) - (1 << Self::MAX_SCRATCHPAD_BUFFERS_HIGH_BEGIN);
    const SCRATCHPAD_RESTORE_BEGIN: usize = Self::MAX_SCRATCHPAD_BUFFERS_HIGH_END;
    const SCRATCHPAD_RESTORE_LENGTH: usize = 1;
    const SCRATCHPAD_RESTORE_END: usize = Self::SCRATCHPAD_RESTORE_BEGIN + Self::SCRATCHPAD_RESTORE_LENGTH;
    const SCRATCHPAD_RESTORE_MASK: u32 = (1 << Self::SCRATCHPAD_RESTORE_END) - (1 << Self::SCRATCHPAD_RESTORE_BEGIN);
    const MAX_SCRATCHPAD_BUFFERS_LOW_BEGIN: usize = Self::SCRATCHPAD_RESTORE_END;
    const MAX_SCRATCHPAD_BUFFERS_LOW_LENGTH: usize = 5;
    const MAX_SCRATCHPAD_BUFFERS_LOW_END: usize = Self::MAX_SCRATCHPAD_BUFFERS_LOW_BEGIN + Self::MAX_SCRATCHPAD_BUFFERS_LOW_LENGTH;
    const MAX_SCRATCHPAD_BUFFERS_LOW_MASK: u32 = u32::MAX - (1 << Self::MAX_SCRATCHPAD_BUFFERS_LOW_BEGIN) + 1;

    fn isochronous_scheduling_threshold(&self) -> u8 {
        ((self.0 & Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_MASK) >> Self::ISOCHRONOUS_SCHEDULING_THRESHOLD_BEGIN) as u8
    }

    fn event_ring_segment_table_max(&self) -> u8 {
        ((self.0 & Self::EVENT_RING_SEGMENT_TABLE_MAX_MASK) >> Self::EVENT_RING_SEGMENT_TABLE_MAX_BEGIN) as u8
    }

    fn max_scratchpad_buffers(&self) -> u16 {
        ((self.max_scratchpad_buffers_high() as u16) << Self::MAX_SCRATCHPAD_BUFFERS_LOW_LENGTH) + (self.max_scratchpad_buffers_low() as u16)
    }

    fn max_scratchpad_buffers_low(&self) -> u8 {
        ((self.0 & Self::MAX_SCRATCHPAD_BUFFERS_LOW_MASK) >> Self::MAX_SCRATCHPAD_BUFFERS_LOW_BEGIN) as u8
    }

    fn max_scratchpad_buffers_high(&self) -> u8 {
        ((self.0 & Self::MAX_SCRATCHPAD_BUFFERS_HIGH_MASK) >> Self::MAX_SCRATCHPAD_BUFFERS_HIGH_BEGIN) as u8
    }

    fn scratchpad_restore(&self) -> bool {
        self.0 & Self::SCRATCHPAD_RESTORE_MASK != 0
    }
}

impl fmt::Debug for Hcsparams2 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HCSPARAMS2")
            .field("isochronous_scheduling_threshold", &self.isochronous_scheduling_threshold())
            .field("event_ring_segment_table_max", &self.event_ring_segment_table_max())
            .field("max_scratchpad_buffers", &self.max_scratchpad_buffers())
            .field("scratchpad_restore", &self.scratchpad_restore())
            .finish()
    }
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.3.5  Structural Parameters 3 (HCSPARAMS3)
#[derive(Clone, Copy)]
struct Hcsparams3(u32);

impl Hcsparams3 {
    const U1_DEVICE_EXIT_LATENCY_BEGIN: usize = 0;
    const U1_DEVICE_EXIT_LATENCY_LENGTH: usize = 8;
    const U1_DEVICE_EXIT_LATENCY_END: usize = Self::U1_DEVICE_EXIT_LATENCY_BEGIN + Self::U1_DEVICE_EXIT_LATENCY_LENGTH;
    const U1_DEVICE_EXIT_LATENCY_MASK: u32 = (1 << Self::U1_DEVICE_EXIT_LATENCY_END) - (1 << Self::U1_DEVICE_EXIT_LATENCY_BEGIN);
    const U2_DEVICE_EXIT_LATENCY_BEGIN: usize = Self::U1_DEVICE_EXIT_LATENCY_END;
    const U2_DEVICE_EXIT_LATENCY_LENGTH: usize = 8;
    const U2_DEVICE_EXIT_LATENCY_END: usize = Self::U2_DEVICE_EXIT_LATENCY_BEGIN + Self::U2_DEVICE_EXIT_LATENCY_LENGTH;
    const U2_DEVICE_EXIT_LATENCY_MASK: u32 = (2 << Self::U2_DEVICE_EXIT_LATENCY_END) - (2 << Self::U2_DEVICE_EXIT_LATENCY_BEGIN);

    fn u1_device_exit_latency(&self) -> u8 {
        ((self.0 & Self::U1_DEVICE_EXIT_LATENCY_MASK) >> Self::U1_DEVICE_EXIT_LATENCY_BEGIN) as u8
    }

    fn u2_device_exit_latency(&self) -> u8 {
        ((self.0 & Self::U2_DEVICE_EXIT_LATENCY_MASK) >> Self::U2_DEVICE_EXIT_LATENCY_BEGIN) as u8
    }
}

impl fmt::Debug for Hcsparams3 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HCSPARAMS2")
            .field("u1_device_exit_latency", &self.u1_device_exit_latency())
            .field("u2_device_exit_latency", &self.u2_device_exit_latency())
            .finish()
    }
}

