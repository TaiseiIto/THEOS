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
    hccparams1: Hccparams1,
    dbof: Dbof,
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

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.3.6 Capability Parameters 1 (HCCPARAMS1)
#[derive(Clone, Copy)]
struct Hccparams1(u32);

impl Hccparams1 {
    const AC64_SHIFT: usize = 0;
    const AC64_MASK: u32 = 1 << Self::AC64_SHIFT;
    const BNC_SHIFT: usize = Self::AC64_SHIFT + 1;
    const BNC_MASK: u32 = 1 << Self::BNC_SHIFT;
    const CSZ_SHIFT: usize = Self::BNC_SHIFT + 1;
    const CSZ_MASK: u32 = 1 << Self::CSZ_SHIFT;
    const PPC_SHIFT: usize = Self::CSZ_SHIFT + 1;
    const PPC_MASK: u32 = 1 << Self::PPC_SHIFT;
    const PIND_SHIFT: usize = Self::PPC_SHIFT + 1;
    const PIND_MASK: u32 = 1 << Self::PIND_SHIFT;
    const LHRC_SHIFT: usize = Self::PIND_SHIFT + 1;
    const LHRC_MASK: u32 = 1 << Self::LHRC_SHIFT;
    const LTC_SHIFT: usize = Self::LHRC_SHIFT + 1;
    const LTC_MASK: u32 = 1 << Self::LTC_SHIFT;
    const NSS_SHIFT: usize = Self::LTC_SHIFT + 1;
    const NSS_MASK: u32 = 1 << Self::NSS_SHIFT;
    const PAE_SHIFT: usize = Self::NSS_SHIFT + 1;
    const PAE_MASK: u32 = 1 << Self::PAE_SHIFT;
    const SPC_SHIFT: usize = Self::PAE_SHIFT + 1;
    const SPC_MASK: u32 = 1 << Self::SPC_SHIFT;
    const SEC_SHIFT: usize = Self::SPC_SHIFT + 1;
    const SEC_MASK: u32 = 1 << Self::SEC_SHIFT;
    const CFC_SHIFT: usize = Self::SEC_SHIFT + 1;
    const CFC_MASK: u32 = 1 << Self::CFC_SHIFT;
    const MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_BEGIN: usize = Self::CFC_SHIFT + 1;
    const MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_LENGTH: usize = 4;
    const MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_END: usize = Self::MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_BEGIN + Self::MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_LENGTH;
    const MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_MASK: u32 = (1 << Self::MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_END) - (1 << Self::MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_BEGIN);
    const XHCI_EXTENDED_CAPABILITIES_POINTER_BEGIN: usize = Self::MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_END;
    const XHCI_EXTENDED_CAPABILITIES_POINTER_MASK: u32 = u32::MAX - (1 << Self::XHCI_EXTENDED_CAPABILITIES_POINTER_BEGIN) + 1;

    fn ac64(&self) -> bool {
        (self.0 & Self::AC64_MASK) != 0
    }

    fn bnc(&self) -> bool {
        (self.0 & Self::BNC_MASK) != 0
    }

    fn csz(&self) -> bool {
        (self.0 & Self::CSZ_MASK) != 0
    }

    fn ppc(&self) -> bool {
        (self.0 & Self::PPC_MASK) != 0
    }

    fn pind(&self) -> bool {
        (self.0 & Self::PIND_MASK) != 0
    }

    fn lhrc(&self) -> bool {
        (self.0 & Self::LHRC_MASK) != 0
    }

    fn ltc(&self) -> bool {
        (self.0 & Self::LTC_MASK) != 0
    }

    fn nss(&self) -> bool {
        (self.0 & Self::NSS_MASK) != 0
    }

    fn pae(&self) -> bool {
        (self.0 & Self::PAE_MASK) != 0
    }

    fn spc(&self) -> bool {
        (self.0 & Self::SPC_MASK) != 0
    }

    fn sec(&self) -> bool {
        (self.0 & Self::SEC_MASK) != 0
    }

    fn cfc(&self) -> bool {
        (self.0 & Self::CFC_MASK) != 0
    }

    fn maximum_primary_stream_array_size(&self) -> u8 {
        ((self.0 & Self::MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_MASK) >> Self::MAXIMUM_PRIMARY_STREAM_ARRAY_SIZE_BEGIN) as u8
    }

    fn xhci_extended_capabilities_pointer(&self) -> u16 {
        ((self.0 & Self::XHCI_EXTENDED_CAPABILITIES_POINTER_MASK) >> Self::XHCI_EXTENDED_CAPABILITIES_POINTER_BEGIN) as u16
    }
}

impl fmt::Debug for Hccparams1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HCCPARAMS3")
            .field("ac64", &self.ac64())
            .field("bnc", &self.bnc())
            .field("csz", &self.csz())
            .field("ppc", &self.ppc())
            .field("pind", &self.pind())
            .field("lhrc", &self.lhrc())
            .field("ltc", &self.ltc())
            .field("nss", &self.nss())
            .field("pae", &self.pae())
            .field("spc", &self.spc())
            .field("sec", &self.sec())
            .field("cfc", &self.cfc())
            .field("maximum_primary_stream_array_size", &self.maximum_primary_stream_array_size())
            .field("xhci_extended_capabilities_pointer", &self.xhci_extended_capabilities_pointer())
            .finish()
    }
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.3.7 Doorbell Offset (DBOF)
#[derive(Clone, Copy)]
struct Dbof(u32);

impl Dbof {
    const DOORBELL_ARRAY_OFFSET_BEGIN: usize = 2;
    const DOORBELL_ARRAY_OFFSET_MASK: u32 = u32::MAX - (1 << Self::DOORBELL_ARRAY_OFFSET_BEGIN) + 1;

    fn doorbell_array_offset(&self) -> u32 {
        self.0 & Self::DOORBELL_ARRAY_OFFSET_MASK
    }
}

impl fmt::Debug for Dbof {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DBOF")
            .field("doorbell_array_offset", &self.doorbell_array_offset())
            .finish()
    }
}

