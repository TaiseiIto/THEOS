use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.4 Host Controller Operational Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    usbcmd: Usbcmd,
    usbsts: u32,
    pagesize: u32,
    rsvd0: u64,
    dnctrl: u32,
    crcr: u32,
    rsvd1: u128,
    dcbaap: u64,
    config: u32,
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.4.1 USB Command Register (USBCMD)
#[derive(Clone, Copy)]
struct Usbcmd(u32);

impl Usbcmd {
    const RS_SHIFT: usize = 0;
    const RS_MASK: u32 = 1 << Self::RS_SHIFT;
    const HCRST_SHIFT: usize = Self::RS_SHIFT + 1;
    const HCRST_MASK: u32 = 1 << Self::HCRST_SHIFT;
    const INTE_SHIFT: usize = Self::HCRST_SHIFT + 1;
    const INTE_MASK: u32 = 1 << Self::INTE_SHIFT;
    const HSEE_SHIFT: usize = Self::INTE_SHIFT + 1;
    const HSEE_MASK: u32 = 1 << Self::HSEE_SHIFT;
    const LHCRST_SHIFT: usize = Self::HSEE_SHIFT + 4;
    const LHCRST_MASK: u32 = 1 << Self::LHCRST_SHIFT;
    const CSS_SHIFT: usize = Self::LHCRST_SHIFT + 1;
    const CSS_MASK: u32 = 1 << Self::CSS_SHIFT;
    const CRS_SHIFT: usize = Self::CSS_SHIFT + 1;
    const CRS_MASK: u32 = 1 << Self::CRS_SHIFT;
    const EWE_SHIFT: usize = Self::CRS_SHIFT + 1;
    const EWE_MASK: u32 = 1 << Self::EWE_SHIFT;
    const EU3S_SHIFT: usize = Self::EWE_SHIFT + 1;
    const EU3S_MASK: u32 = 1 << Self::EU3S_SHIFT;
    const CME_SHIFT: usize = Self::EU3S_SHIFT + 2;
    const CME_MASK: u32 = 1 << Self::CME_SHIFT;
    const ETE_SHIFT: usize = Self::CME_SHIFT + 1;
    const ETE_MASK: u32 = 1 << Self::ETE_SHIFT;
    const TSCEN_SHIFT: usize = Self::ETE_SHIFT + 1;
    const TSCEN_MASK: u32 = 1 << Self::TSCEN_SHIFT;
    const VTIOEN_SHIFT: usize = Self::TSCEN_SHIFT + 1;
    const VTIOEN_MASK: u32 = 1 << Self::VTIOEN_SHIFT;

    fn rs(&self) -> bool {
        self.0 & Self::RS_MASK != 0
    }

    fn hcrst(&self) -> bool {
        self.0 & Self::HCRST_MASK != 0
    }

    fn inte(&self) -> bool {
        self.0 & Self::INTE_MASK != 0
    }

    fn hsee(&self) -> bool {
        self.0 & Self::HSEE_MASK != 0
    }

    fn lhcrst(&self) -> bool {
        self.0 & Self::LHCRST_MASK != 0
    }

    fn css(&self) -> bool {
        self.0 & Self::CSS_MASK != 0
    }

    fn crs(&self) -> bool {
        self.0 & Self::CRS_MASK != 0
    }

    fn ewe(&self) -> bool {
        self.0 & Self::EWE_MASK != 0
    }

    fn eu3s(&self) -> bool {
        self.0 & Self::EU3S_MASK != 0
    }

    fn cme(&self) -> bool {
        self.0 & Self::CME_MASK != 0
    }

    fn ete(&self) -> bool {
        self.0 & Self::ETE_MASK != 0
    }

    fn tscen(&self) -> bool {
        self.0 & Self::TSCEN_MASK != 0
    }

    fn vtioen(&self) -> bool {
        self.0 & Self::VTIOEN_MASK != 0
    }
}

impl fmt::Debug for Usbcmd {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("USBCMD")
            .field("RS", &self.rs())
            .field("HCRST", &self.hcrst())
            .field("INTE", &self.inte())
            .field("HSEE", &self.hsee())
            .field("LHCRST", &self.lhcrst())
            .field("CSS", &self.css())
            .field("CRS", &self.crs())
            .field("EWE", &self.ewe())
            .field("EU3S", &self.eu3s())
            .field("CME", &self.cme())
            .field("ETE", &self.ete())
            .field("TSCEN", &self.tscen())
            .field("VTIOEN", &self.vtioen())
            .finish()
    }
}

