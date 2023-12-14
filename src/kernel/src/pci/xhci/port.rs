use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.4 Host Controller Operational Registers
// Table 5-19: Host Controller USB Port Register Set
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    portsc: Portsc,
    portpmsc: u32,
    portli: u32,
    porthlpmc: u32,
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.4.8 Port Status and Control Register (PORTSC)
#[derive(Clone, Copy)]
struct Portsc(u32);

impl Portsc {
    const CCS_SHIFT: usize = 0;
    const PED_SHIFT: usize = Self::CCS_SHIFT + 1;
    const OCA_SHIFT: usize = Self::PED_SHIFT + 2;
    const PR_SHIFT: usize = Self::OCA_SHIFT + 1;
    const PLS_BEGIN: usize = Self::PR_SHIFT + 1;
    const PLS_LENGTH: usize = 4;
    const PLS_END: usize = Self::PLS_BEGIN + Self::PLS_LENGTH;
    const PP_SHIFT: usize = Self::PLS_END;
    const PORT_SPEED_BEGIN: usize = Self::PP_SHIFT + 1;
    const PORT_SPEED_LENGTH: usize = 4;
    const PORT_SPEED_END: usize = Self::PORT_SPEED_BEGIN + Self::PORT_SPEED_LENGTH;
    const PIC_BEGIN: usize = Self::PORT_SPEED_END;
    const PIC_LENGTH: usize = 2;
    const PIC_END: usize = Self::PIC_BEGIN + Self::PIC_LENGTH;
    const LWS_SHIFT: usize = Self::PIC_END;
    const CSC_SHIFT: usize = Self::LWS_SHIFT + 1;
    const PEC_SHIFT: usize = Self::CSC_SHIFT + 1;
    const WRC_SHIFT: usize = Self::PEC_SHIFT + 1;
    const OCC_SHIFT: usize = Self::WRC_SHIFT + 1;
    const PRC_SHIFT: usize = Self::OCC_SHIFT + 1;
    const PLC_SHIFT: usize = Self::PRC_SHIFT + 1;
    const CEC_SHIFT: usize = Self::PLC_SHIFT + 1;
    const CAS_SHIFT: usize = Self::CEC_SHIFT + 1;
    const WCE_SHIFT: usize = Self::CAS_SHIFT + 1;
    const WDE_SHIFT: usize = Self::WCE_SHIFT + 1;
    const WOE_SHIFT: usize = Self::WDE_SHIFT + 1;
    const DR_SHIFT: usize = Self::WOE_SHIFT + 3;
    const WPR_SHIFT: usize = Self::DR_SHIFT + 1;

    const CCS_MASK: u32 = 1 << Self::CCS_SHIFT;
    const PED_MASK: u32 = 1 << Self::PED_SHIFT;
    const OCA_MASK: u32 = 1 << Self::OCA_SHIFT;
    const PR_MASK: u32 = 1 << Self::PR_SHIFT;
    const PLS_MASK: u32 = (1 << Self::PLS_END) - (1 << Self::PLS_BEGIN);
    const PP_MASK: u32 = 1 << Self::PP_SHIFT;
    const PORT_SPEED_MASK: u32 = (1 << Self::PORT_SPEED_END) - (1 << Self::PORT_SPEED_BEGIN);
    const PIC_MASK: u32 = (1 << Self::PIC_END) - (1 << Self::PIC_BEGIN);
    const LWS_MASK: u32 = 1 << Self::LWS_SHIFT;
    const CSC_MASK: u32 = 1 << Self::CSC_SHIFT;
    const PEC_MASK: u32 = 1 << Self::PEC_SHIFT;
    const WRC_MASK: u32 = 1 << Self::WRC_SHIFT;
    const OCC_MASK: u32 = 1 << Self::OCC_SHIFT;
    const PRC_MASK: u32 = 1 << Self::PRC_SHIFT;
    const PLC_MASK: u32 = 1 << Self::PLC_SHIFT;
    const CEC_MASK: u32 = 1 << Self::CEC_SHIFT;
    const CAS_MASK: u32 = 1 << Self::CAS_SHIFT;
    const WCE_MASK: u32 = 1 << Self::WCE_SHIFT;
    const WDE_MASK: u32 = 1 << Self::WDE_SHIFT;
    const WOE_MASK: u32 = 1 << Self::WOE_SHIFT;
    const DR_MASK: u32 = 1 << Self::DR_SHIFT;
    const WPR_MASK: u32 = 1 << Self::WPR_SHIFT;

    fn ccs(&self) -> bool {
        self.0 & Self::CCS_MASK != 0
    }

    fn ped(&self) -> bool {
        self.0 & Self::PED_MASK != 0
    }

    fn oca(&self) -> bool {
        self.0 & Self::OCA_MASK != 0
    }

    fn pr(&self) -> bool {
        self.0 & Self::PR_MASK != 0
    }

    fn pls(&self) -> u8 {
        ((self.0 & Self::PLS_MASK) >> Self::PLS_BEGIN) as u8
    }

    fn pp(&self) -> bool {
        self.0 & Self::PP_MASK != 0
    }

    fn port_speed(&self) -> u8 {
        ((self.0 & Self::PORT_SPEED_MASK) >> Self::PORT_SPEED_BEGIN) as u8
    }

    fn pic(&self) -> u8 {
        ((self.0 & Self::PIC_MASK) >> Self::PIC_BEGIN) as u8
    }

    fn lws(&self) -> bool {
        self.0 & Self::LWS_MASK != 0
    }

    fn csc(&self) -> bool {
        self.0 & Self::CSC_MASK != 0
    }

    fn pec(&self) -> bool {
        self.0 & Self::PEC_MASK != 0
    }

    fn wrc(&self) -> bool {
        self.0 & Self::WRC_MASK != 0
    }

    fn occ(&self) -> bool {
        self.0 & Self::OCC_MASK != 0
    }

    fn prc(&self) -> bool {
        self.0 & Self::PRC_MASK != 0
    }

    fn plc(&self) -> bool {
        self.0 & Self::PLC_MASK != 0
    }

    fn cec(&self) -> bool {
        self.0 & Self::CEC_MASK != 0
    }

    fn cas(&self) -> bool {
        self.0 & Self::CAS_MASK != 0
    }

    fn wce(&self) -> bool {
        self.0 & Self::WCE_MASK != 0
    }

    fn wde(&self) -> bool {
        self.0 & Self::WDE_MASK != 0
    }

    fn woe(&self) -> bool {
        self.0 & Self::WOE_MASK != 0
    }

    fn dr(&self) -> bool {
        self.0 & Self::DR_MASK != 0
    }

    fn wpr(&self) -> bool {
        self.0 & Self::WPR_MASK != 0
    }
}

impl fmt::Debug for Portsc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PORTSC")
            .field("self", &self.0)
            .field("CCS", &self.ccs())
            .field("PED", &self.ped())
            .field("OCA", &self.oca())
            .field("PR", &self.pr())
            .field("PLS", &self.pls())
            .field("PP", &self.pp())
            .field("PORT_SPEED", &self.port_speed())
            .field("PIC", &self.pic())
            .field("LWS", &self.lws())
            .field("CSC", &self.csc())
            .field("PEC", &self.pec())
            .field("WRC", &self.wrc())
            .field("OCC", &self.occ())
            .field("PRC", &self.prc())
            .field("PLC", &self.plc())
            .field("CEC", &self.cec())
            .field("CAS", &self.cas())
            .field("WCE", &self.wce())
            .field("WDE", &self.wde())
            .field("WOE", &self.woe())
            .field("DR", &self.dr())
            .field("WPR", &self.wpr())
            .finish()
    }
}

